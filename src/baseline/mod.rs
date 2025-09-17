use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use mago_database::DatabaseReader;
use mago_database::ReadDatabase;
use mago_reporting::IssueCollection;

use crate::error::Error;

/// Calculates a simple hash for issue fingerprinting
fn calculate_hash(content: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Creates a simple fingerprint for an issue based on the issue content (supports multi-line)
fn create_simple_issue_fingerprint(code: &str, file_contents: &str, start_offset: u32, end_offset: u32) -> String {
    let lines: Vec<&str> = file_contents.lines().collect();

    // Convert offsets to line numbers
    let mut start_line = 1;
    let mut end_line = 1;
    let mut current_offset = 0;

    for (line_idx, line) in lines.iter().enumerate() {
        let line_end = current_offset + line.len() as u32 + 1; // +1 for newline

        if current_offset <= start_offset && start_offset < line_end {
            start_line = (line_idx + 1) as u32;
        }
        if current_offset <= end_offset && end_offset < line_end {
            end_line = (line_idx + 1) as u32;
        }

        current_offset = line_end;
    }

    // Get all lines from start to end (inclusive)
    let issue_lines: Vec<&str> = if start_line > 0 && end_line >= start_line && (end_line as usize) <= lines.len() {
        lines[(start_line as usize - 1)..(end_line as usize)].iter().map(|line| line.trim()).collect()
    } else {
        // Fallback to just the start line
        if start_line > 0 && (start_line as usize) <= lines.len() {
            vec![lines[start_line as usize - 1].trim()]
        } else {
            vec![""]
        }
    };

    // Join all issue lines
    let issue_content = issue_lines.join(" ");

    // Create fingerprint: rule_id + trimmed_issue_content
    let fingerprint_data = format!("{}:{}", code, issue_content);
    calculate_hash(&fingerprint_data)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct BaselineSourceIssue {
    pub code: String,
    pub start_line: u32,
    pub end_line: u32,
    // Optional fingerprint for enhanced matching (v2 format)
    pub fingerprint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BaselineEntry {
    pub issues: Vec<BaselineSourceIssue>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Baseline {
    entries: HashMap<Cow<'static, str>, BaselineEntry>,
}

/// Generates a `Baseline` from a collection of issues with optional fingerprinting.
pub fn generate_baseline_from_issues(
    issues: IssueCollection,
    database: &ReadDatabase,
    use_fingerprints: bool,
) -> Result<Baseline, Error> {
    let mut baseline = Baseline::default();

    for issue in issues {
        let Some(code) = issue.code else { continue };
        let Some(annotation) = issue
            .annotations
            .iter()
            .find(|a| a.is_primary())
            .or_else(|| issue.annotations.iter().find(|a| !a.is_primary()))
        else {
            tracing::warn!("Issue with code '{code}' has no annotations, it will not be included in the baseline.");

            continue;
        };

        let start = annotation.span.start;
        let end = annotation.span.end;
        let source_file = database.get(&annotation.span.file_id)?;

        let entry = baseline.entries.entry(source_file.name.clone()).or_default();

        let fingerprint = if use_fingerprints {
            Some(create_simple_issue_fingerprint(&code, &source_file.contents, start.offset, end.offset))
        } else {
            None
        };

        entry.issues.push(BaselineSourceIssue {
            code: code.to_string(),
            start_line: source_file.line_number(start.offset),
            end_line: source_file.line_number(end.offset),
            fingerprint,
        });
    }

    Ok(baseline)
}

/// Serializes a `Baseline` to a TOML file.
///
/// If a file already exists at the given path, it will be handled based on the `backup` flag.
///
/// # Arguments
///
/// * `path` - The path to write the baseline file to.
/// * `baseline` - The `Baseline` object to serialize.
/// * `backup` - If `true`, renames an existing baseline file to `[path].bkp`. If `false`, deletes it.
pub fn serialize_baseline(path: &Path, baseline: &Baseline, backup: bool) -> Result<(), Error> {
    if path.exists() {
        if backup {
            let backup_path = path.with_extension("toml.bkp");
            fs::rename(path, backup_path).map_err(Error::CreatingBaselineFile)?;
        } else {
            fs::remove_file(path).map_err(Error::CreatingBaselineFile)?;
        }
    }

    let toml_string = toml::to_string_pretty(baseline).map_err(Error::SerializingToml)?;
    fs::write(path, toml_string).map_err(Error::CreatingBaselineFile)?;
    Ok(())
}

/// Deserializes a `Baseline` from a TOML file.
pub fn unserialize_baseline(path: &Path) -> Result<Baseline, Error> {
    let toml_string = fs::read_to_string(path).map_err(Error::ReadingBaselineFile)?;
    toml::from_str(&toml_string).map_err(Error::DeserializingToml)
}

/// Filters a collection of `Issue` objects against a baseline with optional fingerprinting.
pub fn filter_issues(
    baseline: &Baseline,
    issues: IssueCollection,
    database: &ReadDatabase,
    use_fingerprints: bool,
) -> Result<(IssueCollection, usize, bool), Error> {
    let baseline_sets: HashMap<Cow<'static, str>, HashSet<BaselineSourceIssue>> =
        baseline.entries.iter().map(|(path, entry)| (path.clone(), entry.issues.iter().cloned().collect())).collect();

    let mut filtered_issues = IssueCollection::new();
    let mut seen_baseline_issues: HashMap<Cow<'static, str>, HashSet<BaselineSourceIssue>> = HashMap::new();

    for issue in issues {
        let Some(annotation) = issue
            .annotations
            .iter()
            .find(|a| a.is_primary())
            .or_else(|| issue.annotations.iter().find(|a| !a.is_primary()))
        else {
            filtered_issues.push(issue);
            continue;
        };

        let source_file = database.get(&annotation.span.file_id)?;

        let Some(baseline_issue_set) = baseline_sets.get(&source_file.name) else {
            // File is not in the baseline, so the issue is new.
            filtered_issues.push(issue);
            continue;
        };

        let Some(code) = &issue.code else {
            filtered_issues.push(issue);
            continue;
        };

        let fingerprint = if use_fingerprints {
            Some(create_simple_issue_fingerprint(
                code,
                &source_file.contents,
                annotation.span.start.offset,
                annotation.span.end.offset,
            ))
        } else {
            None
        };

        let issue_to_check = BaselineSourceIssue {
            code: code.to_string(),
            start_line: source_file.line_number(annotation.span.start.offset),
            end_line: source_file.line_number(annotation.span.end.offset),
            fingerprint,
        };

        // Check for match using appropriate strategy
        let is_match = if use_fingerprints {
            // Fingerprint-based matching: check if any baseline issue has the same fingerprint
            baseline_issue_set.iter().any(|baseline_issue| {
                baseline_issue.code == issue_to_check.code
                    && baseline_issue.fingerprint == issue_to_check.fingerprint
                    && baseline_issue.fingerprint.is_some()
            })
        } else {
            // Line-based matching: exact match on line numbers
            baseline_issue_set.contains(&issue_to_check)
        };

        if is_match {
            // Issue is in the baseline, so we ignore it and mark it as "seen".
            seen_baseline_issues.entry(source_file.name.clone()).or_default().insert(issue_to_check);
        } else {
            // Issue is not in the baseline, so it's a new one.
            filtered_issues.push(issue);
        }
    }

    let seen_count = seen_baseline_issues.values().map(|set| set.len()).sum();

    // Check for dead issues (in baseline but not "seen").
    let mut has_dead_issues = false;
    for (path, baseline_issue_set) in &baseline_sets {
        if let Some(seen_set) = seen_baseline_issues.get(path) {
            if seen_set.len() != baseline_issue_set.len() {
                has_dead_issues = true;
                break;
            }
        } else {
            // If we have a baseline for a file but saw no issues from it, all its baseline issues are dead.
            // This can happen if all issues in a file were fixed.
            has_dead_issues = true;
            break;
        }
    }

    Ok((filtered_issues, seen_count, has_dead_issues))
}

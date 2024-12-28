use std::path::Path;
use std::path::PathBuf;

use ahash::HashSet;
use async_walkdir::Filtering;
use async_walkdir::WalkDir;
use futures::StreamExt;

use mago_interner::ThreadedInterner;
use mago_source::SourceCategory;
use mago_source::SourceManager;

use crate::config::source::SourceConfiguration;
use crate::consts::PHP_STUBS;
use crate::error::Error;

/// Load the source manager by scanning and processing the sources
/// as per the given configuration.
///
/// # Arguments
///
/// * `interner` - The interner to use for string interning.
/// * `configuration` - The configuration to use for loading the sources.
/// * `include_stubs` - Whether to include stubs in the source manager.
///
/// # Returns
///
/// A `Result` containing the new source manager or a `SourceError` if
/// an error occurred during the build process.
pub async fn load(
    interner: &ThreadedInterner,
    configuration: &SourceConfiguration,
    include_stubs: bool,
) -> Result<SourceManager, Error> {
    let SourceConfiguration { root, paths, includes, excludes, extensions } = configuration;

    let mut starting_paths = Vec::new();

    if paths.is_empty() {
        starting_paths.push((root.clone(), true));
    } else {
        for source in paths {
            starting_paths.push((source.clone(), true));
        }
    }

    for include in includes {
        starting_paths.push((include.clone(), false));
    }

    if paths.is_empty() && includes.is_empty() {
        starting_paths.push((root.clone(), true));
    }

    let excludes_set: HashSet<Exclusion> = excludes
        .iter()
        .map(|exclude| {
            // if it contains a wildcard, treat it as a pattern
            if exclude.contains('*') {
                Exclusion::Pattern(exclude.clone())
            } else {
                let path = Path::new(exclude);

                if path.is_absolute() {
                    Exclusion::Path(path.to_path_buf())
                } else {
                    Exclusion::Path(root.join(path))
                }
            }
        })
        .collect();

    let extensions: HashSet<&String> = extensions.iter().collect();

    let manager = SourceManager::new(interner.clone());
    for (path, user_defined) in starting_paths.into_iter() {
        let mut entries = WalkDir::new(path)
            // filter out .git directories
            .filter(|entry| async move {
                if entry.path().starts_with(".") {
                    Filtering::IgnoreDir
                } else {
                    Filtering::Continue
                }
            });

        // Check for errors after processing all entries in the current path
        while let Some(entry) = entries.next().await {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }

            // Skip user-defined sources if they are included in the `includes` list.
            if user_defined && includes.iter().any(|include| path.starts_with(include)) {
                continue;
            }

            // Skip excluded files and directories.
            if is_excluded(&path, &excludes_set) {
                continue;
            }

            // Skip files that do not have an accepted extension.
            if !is_accepted_file(&path, &extensions) {
                continue;
            }

            let name = match path.strip_prefix(root) {
                Ok(rel_path) => rel_path.display().to_string(),
                Err(_) => path.display().to_string(),
            };

            manager.insert_path(
                name,
                path.clone(),
                if user_defined { SourceCategory::UserDefined } else { SourceCategory::External },
            );
        }
    }

    if include_stubs {
        for (stub, content) in PHP_STUBS {
            manager.insert_content(stub.to_owned(), content.to_owned(), SourceCategory::BuiltIn);
        }
    }

    Ok(manager)
}

fn is_excluded(path: &Path, excludes: &HashSet<Exclusion>) -> bool {
    for exclusion in excludes {
        return match exclusion {
            Exclusion::Path(p) if path.starts_with(p) => true,
            Exclusion::Pattern(p) if glob_match::glob_match(p, path.to_string_lossy().as_ref()) => true,
            _ => continue,
        };
    }

    false
}

fn is_accepted_file(path: &Path, extensions: &HashSet<&String>) -> bool {
    if extensions.is_empty() {
        path.extension().and_then(|s| s.to_str()).map(|ext| ext.eq_ignore_ascii_case("php")).unwrap_or(false)
    } else {
        path.extension().and_then(|s| s.to_str()).map(|ext| extensions.contains(&ext.to_string())).unwrap_or(false)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Exclusion {
    Path(PathBuf),
    Pattern(String),
}

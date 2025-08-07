use std::path::PathBuf;

use mago_database::DatabaseReader;
use mago_database::ReadDatabase;
use mago_database::error::DatabaseError;
use mago_database::file::FileId;
use mago_database::file::FileType;
use serde::Deserialize;
use serde::Serialize;

use mago_fixer::FixPlan;
use mago_span::Position;
use mago_span::Span;

use crate::Annotation;
use crate::AnnotationKind;
use crate::Issue;
use crate::IssueCollection;
use crate::Level;

pub mod emitter;
pub mod writer;

/// Expanded representation of a file id.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ExpandedFileId {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<PathBuf>,
    pub size: usize,
    pub file_type: FileType,
}

/// Expanded representation of a position within a file.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ExpandedPosition {
    pub file_id: ExpandedFileId,
    pub offset: usize,
    pub line: usize,
}

/// Expanded representation of a span, including start and end positions.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct ExpandedSpan {
    pub start: ExpandedPosition,
    pub end: ExpandedPosition,
}

/// Expanded annotation, enriched with resolved spans.
#[derive(Debug, PartialEq, Eq, Ord, Clone, Hash, PartialOrd, Deserialize, Serialize)]
pub struct ExpandedAnnotation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub kind: AnnotationKind,
    pub span: ExpandedSpan,
}

/// Expanded issue, containing detailed information for display or external reporting.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExpandedIssue {
    pub level: Level,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<ExpandedAnnotation>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub suggestions: Vec<(ExpandedFileId, FixPlan)>,
}

/// A collection of expanded issues.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ExpandedIssueCollection {
    issues: Vec<ExpandedIssue>,
}

pub trait Expandable<T> {
    fn expand(&self, database: &ReadDatabase) -> Result<T, DatabaseError>;
}

impl Expandable<ExpandedFileId> for FileId {
    fn expand(&self, database: &ReadDatabase) -> Result<ExpandedFileId, DatabaseError> {
        let file = database.get_by_id(self)?;

        Ok(ExpandedFileId {
            name: file.name.clone(),
            path: file.path.clone(),
            size: file.size,
            file_type: file.file_type,
        })
    }
}

impl Expandable<ExpandedPosition> for Position {
    fn expand(&self, database: &ReadDatabase) -> Result<ExpandedPosition, DatabaseError> {
        let file = database.get_by_id(&self.file_id)?;

        Ok(ExpandedPosition {
            file_id: self.file_id.expand(database)?,
            offset: self.offset,
            line: file.line_number(self.offset),
        })
    }
}

impl Expandable<ExpandedSpan> for Span {
    fn expand(&self, manager: &ReadDatabase) -> Result<ExpandedSpan, DatabaseError> {
        Ok(ExpandedSpan { start: self.start.expand(manager)?, end: self.end.expand(manager)? })
    }
}

impl Expandable<ExpandedAnnotation> for Annotation {
    fn expand(&self, database: &ReadDatabase) -> Result<ExpandedAnnotation, DatabaseError> {
        Ok(ExpandedAnnotation { message: self.message.clone(), kind: self.kind, span: self.span.expand(database)? })
    }
}

impl Expandable<ExpandedIssue> for Issue {
    fn expand(&self, database: &ReadDatabase) -> Result<ExpandedIssue, DatabaseError> {
        let mut annotations = Vec::new();
        for annotation in &self.annotations {
            annotations.push(annotation.expand(database)?);
        }

        let mut suggestions = Vec::new();
        for (file_id, fix) in &self.suggestions {
            suggestions.push((file_id.expand(database)?, fix.clone()));
        }

        Ok(ExpandedIssue {
            level: self.level,
            code: self.code.clone(),
            message: self.message.clone(),
            notes: self.notes.clone(),
            help: self.help.clone(),
            link: self.link.clone(),
            annotations,
            suggestions,
        })
    }
}

impl Expandable<ExpandedIssueCollection> for IssueCollection {
    fn expand(&self, database: &ReadDatabase) -> Result<ExpandedIssueCollection, DatabaseError> {
        let mut expanded_issues = Vec::new();
        for issue in self.issues.iter() {
            expanded_issues.push(issue.expand(database)?);
        }

        Ok(ExpandedIssueCollection { issues: expanded_issues })
    }
}

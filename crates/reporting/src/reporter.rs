use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use strum::VariantNames;

use mago_database::ReadDatabase;

use crate::Issue;
use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;
use crate::internal::emitter::Emitter;
use crate::internal::writer::ReportWriter;

/// Defines the output target for the `ReportWriter`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReportingTarget {
    /// Direct output to standard output (stdout).
    #[default]
    Stdout,
    /// Direct output to standard error (stderr).
    Stderr,
}

/// The format to use when writing the report.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum ReportingFormat {
    #[default]
    Rich,
    Medium,
    Short,
    Ariadne,
    Github,
    Gitlab,
    Json,
    Count,
    Checkstyle,
    Emacs,
}

pub struct Reporter {
    database: ReadDatabase,
    target: ReportingTarget,
    writer: ReportWriter,
}

impl Reporter {
    pub fn new(manager: ReadDatabase, target: ReportingTarget) -> Self {
        Self { database: manager, target, writer: ReportWriter::new(target) }
    }

    pub fn report(
        &self,
        issues: impl IntoIterator<Item = Issue>,
        format: ReportingFormat,
    ) -> Result<Option<Level>, ReportingError> {
        format.emit(&mut self.writer.lock(), &self.database, IssueCollection::from(issues))
    }
}

unsafe impl Send for Reporter {}
unsafe impl Sync for Reporter {}

impl std::fmt::Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Reporter")
            .field("manager", &self.database)
            .field("target", &self.target)
            .finish_non_exhaustive()
    }
}

impl FromStr for ReportingTarget {
    type Err = ReportingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stdout" | "out" => Ok(Self::Stdout),
            "stderr" | "err" => Ok(Self::Stderr),
            _ => Err(ReportingError::InvalidTarget(s.to_string())),
        }
    }
}

impl FromStr for ReportingFormat {
    type Err = ReportingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rich" => Ok(Self::Rich),
            "medium" => Ok(Self::Medium),
            "short" => Ok(Self::Short),
            "ariadne" => Ok(Self::Ariadne),
            "github" => Ok(Self::Github),
            "gitlab" => Ok(Self::Gitlab),
            "json" => Ok(Self::Json),
            "count" => Ok(Self::Count),
            "checkstyle" => Ok(Self::Checkstyle),
            "emacs" => Ok(Self::Emacs),
            _ => Err(ReportingError::InvalidFormat(s.to_string())),
        }
    }
}

use codespan_reporting::files::Error as FilesError;
use serde_json::Error as JsonError;
use std::io::Error as IoError;

use mago_database::error::DatabaseError;

#[derive(Debug)]
pub enum ReportingError {
    DatabaseError(DatabaseError),
    JsonError(JsonError),
    FilesError(FilesError),
    IoError(IoError),
    InvalidTarget(String),
    InvalidFormat(String),
}

impl std::fmt::Display for ReportingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DatabaseError(error) => write!(f, "{error}"),
            Self::JsonError(error) => write!(f, "Json error: {error}"),
            Self::FilesError(error) => write!(f, "Files error: {error}"),
            Self::IoError(error) => write!(f, "IO error: {error}"),
            Self::InvalidTarget(target) => write!(f, "Invalid target: {target}"),
            Self::InvalidFormat(format) => write!(f, "Invalid format: {format}"),
        }
    }
}

impl std::error::Error for ReportingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::DatabaseError(error) => Some(error),
            Self::JsonError(error) => Some(error),
            Self::FilesError(error) => Some(error),
            Self::IoError(error) => Some(error),
            Self::InvalidTarget(_) => None,
            Self::InvalidFormat(_) => None,
        }
    }
}

impl From<DatabaseError> for ReportingError {
    fn from(error: DatabaseError) -> Self {
        Self::DatabaseError(error)
    }
}

impl From<JsonError> for ReportingError {
    fn from(error: JsonError) -> Self {
        Self::JsonError(error)
    }
}

impl From<FilesError> for ReportingError {
    fn from(error: FilesError) -> Self {
        Self::FilesError(error)
    }
}

impl From<IoError> for ReportingError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

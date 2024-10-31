use std::path::PathBuf;

use serde::Deserialize;
use serde::Serialize;

/// Configuration options for source discovery.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceConfiguration {
    /// The root directory from which to start scanning.
    ///
    /// Defaults to the current working directory.
    pub root: PathBuf,

    /// Paths to user defined source files.
    ///
    /// If empty, all files in the root directory are included.
    ///
    /// Defaults to `[]`.
    pub paths: Vec<PathBuf>,

    /// Paths to non-user defined files to include in the scan.
    ///
    /// Defaults to `[]`.
    pub includes: Vec<PathBuf>,

    /// Patterns to exclude from the scan.
    ///
    /// Defaults to `[]`.
    pub excludes: Vec<String>,

    /// File extensions to filter by.
    ///
    /// Defaults to `[".php"]`.
    pub extensions: Vec<String>,
}

impl SourceConfiguration {
    /// Creates a new `SourceConfiguration` with the given root directory.
    ///
    /// # Arguments
    ///
    /// * `root` - The root directory from which to start scanning.
    ///
    /// # Returns
    ///
    /// A new `SourceConfiguration` with the given root directory.
    pub fn from_root(root: PathBuf) -> Self {
        Self { root, paths: vec![], includes: vec![], excludes: vec![], extensions: vec![] }
    }
}

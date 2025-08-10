use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::PathBuf;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use walkdir::WalkDir;

use crate::Database;
use crate::error::DatabaseError;
use crate::exclusion::Exclusion;
use crate::file::File;
use crate::file::FileType;
use crate::utils::read_file;

/// Configures and builds a `Database` by scanning the filesystem and memory.
pub struct DatabaseLoader {
    database: Option<Database>,
    workspace: PathBuf,
    paths: Vec<PathBuf>,
    includes: Vec<PathBuf>,
    excludes: Vec<Exclusion>,
    memory_sources: Vec<(&'static str, &'static str, FileType)>,
    extensions: Vec<String>,
}

impl DatabaseLoader {
    /// Creates a new loader with the given configuration.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        workspace: PathBuf,
        paths: Vec<PathBuf>,
        includes: Vec<PathBuf>,
        excludes: Vec<Exclusion>,
        extensions: Vec<String>,
    ) -> Self {
        Self { workspace, paths, includes, excludes, memory_sources: vec![], extensions, database: None }
    }

    /// Adds a memory source to the loader.
    ///
    /// This allows you to include files that are not on the filesystem but should be part of the database.
    ///
    /// # Arguments
    ///
    /// * `name` - The logical name of the file, typically its path relative to the workspace.
    /// * `contents` - The contents of the file as a string.
    /// * `file_type` - The type of the file, indicating whether it's a host file or a vendored file.
    pub fn add_memory_source(&mut self, name: &'static str, contents: &'static str, file_type: FileType) {
        self.memory_sources.push((name, contents, file_type));
    }

    /// Scans sources according to the configuration and builds a `Database`.
    ///
    /// This is the main entry point that orchestrates the entire loading process.
    /// It returns a `Result` as some pre-processing, like compiling globs, can fail.
    pub fn load(mut self) -> Result<Database, DatabaseError> {
        let mut db = if let Some(existing_db) = self.database.take() { existing_db } else { Database::new() };

        let extensions_set: HashSet<OsString> = self.extensions.iter().map(OsString::from).collect();

        let mut glob_builder = GlobSetBuilder::new();
        for ex in &self.excludes {
            if let Exclusion::Pattern(pat) = ex {
                glob_builder.add(Glob::new(pat)?);
            }
        }

        let glob_excludes = glob_builder.build()?;
        let host_files = self.load_paths(&self.paths, FileType::Host, &extensions_set, &glob_excludes)?;
        let vendored_files = self.load_paths(&self.includes, FileType::Vendored, &extensions_set, &glob_excludes)?;

        for file in host_files.into_iter().chain(vendored_files.into_iter()) {
            db.add(file);
        }

        for (name, contents, file_type) in self.memory_sources {
            let file = File::new(Cow::Borrowed(name), file_type, None, Cow::Borrowed(contents));

            db.add(file);
        }

        Ok(db)
    }

    /// Discovers and reads all files from a set of root paths in parallel.
    fn load_paths(
        &self,
        roots: &[PathBuf],
        file_type: FileType,
        extensions: &HashSet<OsString>,
        glob_excludes: &GlobSet,
    ) -> Result<Vec<File>, DatabaseError> {
        // 2. Discover all file paths first. This part is still synchronous and fast.
        let path_excludes: HashSet<_> = self
            .excludes
            .iter()
            .filter_map(|ex| match ex {
                Exclusion::Path(p) => p.canonicalize().ok(),
                _ => None,
            })
            .collect();

        let mut paths_to_process = Vec::new();
        for root in roots {
            for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    paths_to_process.push(entry.into_path());
                }
            }
        }

        // 3. Use a parallel iterator to process all discovered paths.
        let files: Vec<File> = paths_to_process
            .into_par_iter() // This is the magic from rayon!
            .filter_map(|path| {
                // Apply filters in parallel
                if glob_excludes.is_match(&path) {
                    return None;
                }
                if let Ok(p) = path.canonicalize() {
                    if path_excludes.contains(&p) {
                        return None;
                    }
                }
                if let Some(ext) = path.extension() {
                    if !extensions.contains(ext) {
                        return None;
                    }
                } else {
                    return None;
                }

                // Read the file. `read_file` is a blocking operation, but since it's
                // running in a limited number of threads, it's efficient.
                match read_file(&self.workspace, &path, file_type) {
                    Ok(file) => Some(Ok(file)),
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<Vec<File>, _>>()?; // Collect results, propagating any errors.

        Ok(files)
    }
}

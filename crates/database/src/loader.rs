use std::borrow::Cow;
use std::collections::HashSet;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

use globset::Glob;
use globset::GlobSet;
use globset::GlobSetBuilder;
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

        self.load_paths(&mut db, &self.paths, FileType::Host, &extensions_set, &glob_excludes)?;
        self.load_paths(&mut db, &self.includes, FileType::Vendored, &extensions_set, &glob_excludes)?;

        for (name, contents, file_type) in self.memory_sources {
            let file = File::new(Cow::Borrowed(name), file_type, None, Cow::Borrowed(contents));

            db.add(file);
        }

        Ok(db)
    }

    fn load_paths(
        &self,
        db: &mut Database,
        roots: &[PathBuf],
        file_type: FileType,
        extensions: &HashSet<OsString>,
        glob_excludes: &GlobSet,
    ) -> Result<(), DatabaseError> {
        let path_excludes: HashSet<_> = self
            .excludes
            .iter()
            .filter_map(|ex| match ex {
                Exclusion::Path(p) => p.canonicalize().ok(),
                _ => None,
            })
            .collect();

        for root in roots {
            for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
                if entry.file_type().is_file() {
                    self.process_path(db, entry.path(), file_type, extensions, glob_excludes, &path_excludes)?;
                }
            }
        }

        Ok(())
    }

    /// The "File Processor" part: applies all filters to a single path.
    fn process_path(
        &self,
        db: &mut Database,
        path: &Path,
        file_type: FileType,
        extensions: &HashSet<OsString>,
        glob_excludes: &GlobSet,
        path_excludes: &HashSet<PathBuf>,
    ) -> Result<(), DatabaseError> {
        // Filter 1: Check against pre-compiled glob patterns.
        if glob_excludes.is_match(path) {
            return Ok(());
        }

        // Filter 2: Check against specific paths.
        if let Ok(canonical_path) = path.canonicalize()
            && path_excludes.contains(&canonical_path)
        {
            return Ok(());
        }

        // Filter 3: Check file extension.
        let extension = path.extension();

        if let Some(ext) = extension
            && !extensions.contains(ext)
        {
            return Ok(());
        } else if extension.is_none() {
            return Ok(()); // No extension, so we skip it.
        }

        let file = read_file(&self.workspace, path, file_type)?;

        db.add(file);

        Ok(())
    }
}

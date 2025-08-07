use std::path::Path;
use std::path::PathBuf;

use mago_database::Database;
use mago_database::exclusion::Exclusion;
use mago_database::file::FileType;
use mago_database::loader::DatabaseLoader;

use crate::config::source::SourceConfiguration;
use crate::consts::PHP_STUBS;
use crate::error::Error;

pub fn from_paths(
    configuration: &SourceConfiguration,
    paths: Vec<PathBuf>,
    include_stubs: bool,
) -> Result<Database, Error> {
    let SourceConfiguration { workspace, extensions, .. } = configuration;

    let mut loader = DatabaseLoader::new(workspace.clone(), paths, vec![], vec![], extensions.clone());
    if include_stubs {
        for (stub_name, stub_content) in PHP_STUBS {
            loader.add_memory_source(stub_name, stub_content, FileType::Builtin);
        }
    }

    loader.load().map_err(Error::Database)
}

pub fn load(
    configuration: &SourceConfiguration,
    include_externals: bool,
    include_stubs: bool,
) -> Result<Database, Error> {
    let SourceConfiguration { workspace, paths, includes, excludes, extensions } = configuration;

    let mut loader = DatabaseLoader::new(
        workspace.clone(),
        paths.clone(),
        if include_externals { includes.clone() } else { vec![] },
        create_excludes_set(excludes, workspace),
        extensions.clone(),
    );

    if include_stubs {
        for (stub_name, stub_content) in PHP_STUBS {
            loader.add_memory_source(stub_name, stub_content, FileType::Builtin);
        }
    }

    loader.load().map_err(Error::Database)
}

fn create_excludes_set(excludes: &[String], root: &Path) -> Vec<Exclusion> {
    excludes
        .iter()
        .map(|exclude| {
            // if it contains a wildcard, treat it as a pattern
            if exclude.contains('*') {
                let mut exclude = exclude.clone();
                // if it starts with `./`, replace it with the root path
                if exclude.starts_with("./") {
                    exclude.replace_range(..1, root.to_string_lossy().trim_end_matches('/'));
                }

                Exclusion::Pattern(exclude)
            } else {
                let path = Path::new(&exclude);
                let path_buf = if path.is_absolute() { path.to_path_buf() } else { root.join(path) };

                Exclusion::Path(path_buf.canonicalize().unwrap_or(path_buf))
            }
        })
        .collect()
}

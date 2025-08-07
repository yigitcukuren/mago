use std::sync::Arc;

use ahash::HashSet;
use mago_database::DatabaseReader;
use mago_database::ReadDatabase;
use mago_database::file::File;
use tokio::task::JoinHandle;

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::populator::populate_codebase;
use mago_codex::reference::SymbolReferences;
use mago_codex::scanner::scan_program;
use mago_interner::ThreadedInterner;
use mago_names::resolver::NameResolver;
use mago_syntax::parser::parse_file;

use crate::error::Error;
use crate::utils::progress::ProgressBarTheme;
use crate::utils::progress::create_progress_bar;
use crate::utils::progress::remove_progress_bar;

/// Compiles complete `CodebaseMetadata` from a database of source files.
///
/// This function orchestrates the analysis of an entire codebase in three main stages:
///
/// 1. **Concurrent Analysis:** Spawns a Tokio task for each file to parse it and extract its local metadata in parallel.
/// 2. **Aggregation:** Merges the metadata from all files into a single `CodebaseMetadata` object.
/// 3. **Population:** Performs a final, global pass to resolve symbol references across the entire codebase, connecting definitions with their usages.
///
/// A progress bar is displayed to track the concurrent analysis phase.
///
/// # Arguments
///
/// * `database`: An `Arc<ReadDatabase>` containing the source files to be analyzed.
/// * `symbol_references`: Pre-existing symbol reference information to be integrated during the final population stage.
/// * `interner`: The shared string interner for efficient string management.
///
/// # Errors
///
/// Returns an [`Error`] if any of the concurrent analysis tasks fail (e.g., due
/// to an I/O error when a file is unexpectedly unavailable) or if a task panics.
pub async fn compile_codebase_for_sources(
    database: &Arc<ReadDatabase>,
    symbol_references: &mut SymbolReferences,
    interner: &ThreadedInterner,
) -> Result<CodebaseMetadata, Error> {
    let file_ids = database.file_ids().collect::<Vec<_>>();

    let mut compiled_codebase = CodebaseMetadata::new();

    let total_files = file_ids.len();
    let progress_bar = create_progress_bar(total_files, "Compiling", ProgressBarTheme::Magenta);

    let mut compilation_tasks: Vec<JoinHandle<Result<CodebaseMetadata, Error>>> = Vec::with_capacity(total_files);

    for file_id in file_ids {
        let interner_clone = interner.clone();
        let database = database.clone();
        let progress_bar_clone = progress_bar.clone();

        compilation_tasks.push(tokio::spawn(async move {
            let source_file = database.get_by_id(&file_id)?;
            let metadata = extract_metadata_from_source(source_file, &interner_clone);

            progress_bar_clone.inc(1);

            Ok(metadata)
        }));
    }

    for task_handle in compilation_tasks {
        let source_metadata = task_handle.await??;

        compiled_codebase.extend(source_metadata);
    }

    remove_progress_bar(progress_bar);

    tracing::trace!("Populating codebase with symbol references...");
    populate_codebase(&mut compiled_codebase, interner, symbol_references, HashSet::default(), HashSet::default());
    tracing::trace!("Codebase population complete.");

    Ok(compiled_codebase)
}

/// The single-file worker function that extracts `CodebaseMetadata`.
///
/// This function performs the core analysis for one source file by:
///
/// 1. Parsing the source text into an AST (`Program`).
/// 2. Resolving all names (classes, functions, etc.) within the AST.
/// 3. Scanning the resolved AST to produce semantic metadata.
fn extract_metadata_from_source(source_file: &File, interner: &ThreadedInterner) -> CodebaseMetadata {
    let (program, parse_issues) = parse_file(interner, source_file);
    if parse_issues.is_some() {
        tracing::warn!(
            "Encountered parsing issue in {} during codebase compilation. Analysis may be incomplete.",
            source_file.name
        );
    }

    let resolver = NameResolver::new(interner);
    let resolved_names = resolver.resolve(&program);

    scan_program(interner, source_file, &program, &resolved_names)
}

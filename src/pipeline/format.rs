use mago_database::ReadDatabase;
use mago_database::change::ChangeLog;
use mago_formatter::Formatter;
use mago_formatter::settings::FormatSettings;
use mago_interner::ThreadedInterner;
use mago_php_version::PHPVersion;
use mago_syntax::parser::parse_file;

use crate::error::Error;
use crate::pipeline::StatelessParallelPipeline;
use crate::pipeline::StatelessReducer;
use crate::utils;

/// The "reduce" step for the formatting pipeline.
///
/// This struct aggregates the boolean results from each parallel formatting
/// task into a final count of how many files were changed.
#[derive(Debug, Clone)]
pub struct FormatReducer;

impl StatelessReducer<bool, usize> for FormatReducer {
    fn reduce(&self, results: Vec<bool>) -> Result<usize, Error> {
        Ok(results.into_iter().filter(|&changed| changed).count())
    }
}

/// Shared, read-only context provided to each parallel formatting task.
#[derive(Clone)]
pub struct FormatContext {
    /// The target PHP version for formatting rules.
    pub php_version: PHPVersion,
    /// The configured settings for the formatter.
    pub settings: FormatSettings,
    /// If `true`, the pipeline will only check for changes and not modify files.
    pub dry_run: bool,
    /// A thread-safe log for recording formatting changes.
    pub change_log: ChangeLog,
}

/// The main entry point for running the parallel formatting pipeline.
///
/// This function orchestrates the formatting of all `Host` files in the database
/// using a stateless parallel pipeline, which is highly efficient for tasks that
/// can process each file in isolation.
///
/// # Arguments
///
/// * `interner`: The shared string interner.
/// * `database`: The read-only database containing the files to format.
/// * `context`: The shared [`FormatContext`] for the formatting run.
///
/// # Returns
///
/// A `Result` containing the total number of files that were changed, or an [`Error`].
pub fn run_format_pipeline(
    interner: &ThreadedInterner,
    database: ReadDatabase,
    context: FormatContext,
) -> Result<usize, Error> {
    StatelessParallelPipeline::new("✨ Formatting", database, interner, context, Box::new(FormatReducer)).run(
        |context, interner, file| {
            let (program, error) = parse_file(&interner, &file);

            if let Some(error) = error {
                tracing::error!("Skipping formatting for '{}': {}.", file.name, error);
                return Ok(false);
            }

            let formatter = Formatter::new(&interner, context.php_version, context.settings);
            let formatted_content = formatter.format(&file, &program);

            utils::apply_update(&context.change_log, &file, formatted_content, context.dry_run)
        },
    )
}

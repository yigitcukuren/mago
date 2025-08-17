use std::borrow::Cow;
use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use mago_database::change::ChangeLog;
use mago_database::error::DatabaseError;
use mago_database::file::File;
use mago_formatter::Formatter;
use mago_interner::ThreadedInterner;

use crate::config::Configuration;
use crate::database;
use crate::error::Error;
use crate::pipeline::format::FormatContext;
use crate::pipeline::format::run_format_pipeline;

/// Represents the `format` command, which is responsible for formatting source files
/// according to specified rules in the configuration file.
#[derive(Parser, Debug)]
#[command(
    name = "format",
    aliases = ["fmt"],
    about = "Format source files to match defined style rules",
    long_about = r#"
The `format` command applies consistent formatting to source files based on the rules defined in the configuration file.

This command helps maintain a consistent codebase style, improving readability and collaboration.
"#
)]
pub struct FormatCommand {
    /// Format specific files or directories, overriding the source configuration.
    #[arg(help = "Format specific files or directories, overriding the source configuration")]
    pub path: Vec<PathBuf>,

    /// Perform a dry run to check if files are already formatted.
    #[arg(long, short = 'd', help = "Check if the source files are already formatted without making changes")]
    pub dry_run: bool,

    #[arg(long, short = 'i', help = "Read input from STDIN, format it, and write to STDOUT")]
    pub stdin_input: bool,
}

/// Executes the format command with the provided configuration and options.
///
/// # Arguments
///
/// * `command` - The `FormatCommand` structure containing user-specified options.
/// * `configuration` - The application configuration loaded from file or defaults.
///
/// # Returns
///
/// Exit code: `0` if successful or no changes were needed, `1` if issues were found during the check.
pub fn execute(command: FormatCommand, mut configuration: Configuration) -> Result<ExitCode, Error> {
    let interner = ThreadedInterner::new();

    configuration.source.excludes.extend(std::mem::take(&mut configuration.formatter.excludes));

    if command.stdin_input {
        let file = create_file_from_stdin()?;
        let formatter = Formatter::new(&interner, configuration.php_version, configuration.formatter.settings);
        return Ok(match formatter.format_file(&file) {
            Ok(formatted) => {
                print!("{formatted}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                tracing::error!("Failed to format input: {}", error);
                ExitCode::FAILURE
            }
        });
    }

    let mut database = if !command.path.is_empty() {
        database::from_paths(&configuration.source, command.path, false)?
    } else {
        database::load(&configuration.source, false, false)?
    };

    // 1. Create the shared ChangeLog and context for the pipeline.
    let change_log = ChangeLog::new();
    let shared_context = FormatContext {
        php_version: configuration.php_version,
        settings: configuration.formatter.settings,
        dry_run: command.dry_run,
        change_log: change_log.clone(),
    };

    let changed_count = run_format_pipeline(&interner, database.read_only(), shared_context)?;
    if !command.dry_run {
        database.commit(change_log, true)?;
    }

    if changed_count == 0 {
        tracing::info!("All files are already formatted.");
        return Ok(ExitCode::SUCCESS);
    }

    Ok(if command.dry_run {
        tracing::info!("Found {} file(s) that need formatting.", changed_count);
        ExitCode::FAILURE
    } else {
        tracing::info!("Formatted {} file(s) successfully.", changed_count);
        ExitCode::SUCCESS
    })
}

/// Creates an ephemeral file from standard input.
fn create_file_from_stdin() -> Result<File, Error> {
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).map_err(|e| Error::Database(DatabaseError::IOError(e)))?;

    Ok(File::ephemeral(Cow::Borrowed("<stdin>"), Cow::Owned(content)))
}

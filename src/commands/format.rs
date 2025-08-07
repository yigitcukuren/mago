use std::io::Read;
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Arc;

use clap::Parser;

use mago_database::Database;
use mago_database::DatabaseReader;
use mago_database::ReadDatabase;
use mago_database::change::ChangeLog;
use mago_database::error::DatabaseError;
use mago_database::file::File;
use mago_database::file::FileId;
use mago_database::file::FileType;
use mago_formatter::Formatter;
use mago_formatter::settings::FormatSettings;
use mago_interner::ThreadedInterner;
use mago_php_version::PHPVersion;
use mago_syntax::parser::parse_file;

use crate::config::Configuration;
use crate::database;
use crate::error::Error;
use crate::utils;
use crate::utils::progress::ProgressBarTheme;
use crate::utils::progress::create_progress_bar;
use crate::utils::progress::remove_progress_bar;

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
pub async fn execute(command: FormatCommand, mut configuration: Configuration) -> Result<ExitCode, Error> {
    // Initialize the interner for managing identifiers.
    let interner = ThreadedInterner::new();

    configuration.source.excludes.extend(std::mem::take(&mut configuration.format.excludes));

    if command.stdin_input {
        let file = create_file_from_stdin()?;
        let formatter = Formatter::new(&interner, configuration.php_version, configuration.format.settings);

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

    let database = if !command.path.is_empty() {
        database::from_paths(&configuration.source, command.path, false)?
    } else {
        database::load(&configuration.source, false, false)?
    };

    // Format all files and get the count of changed files.
    let changed =
        format_all(interner, database, configuration.php_version, configuration.format.settings, command.dry_run)
            .await?;

    // Provide feedback and return appropriate exit code.
    if changed == 0 {
        tracing::info!("All files are already formatted.");

        return Ok(ExitCode::SUCCESS);
    }

    Ok(if command.dry_run {
        tracing::info!("Found {} file(s) that need formatting.", changed);

        ExitCode::FAILURE
    } else {
        tracing::info!("Formatted {} file(s) successfully.", changed);

        ExitCode::SUCCESS
    })
}

/// Formats all files using the provided settings.
///
/// # Arguments
///
/// * `interner` - The interner to be used for identifier management.
/// * `database` - The database containing files.
/// * `php_version` - The PHP version to use for formatting.
/// * `settings` - Formatting settings to apply.
/// * `check` - A flag to determine whether to check or apply formatting.
///
/// # Returns
///
/// A result containing the number of changed files or an error.
#[inline]
async fn format_all(
    interner: ThreadedInterner,
    mut database: Database,
    php_version: PHPVersion,
    settings: FormatSettings,
    dry_run: bool,
) -> Result<usize, Error> {
    let read_database = Arc::new(database.read_only());
    let change_log = ChangeLog::new();

    let file_ids: Vec<_> = read_database.file_ids_with_type(FileType::Host).collect();
    let length = file_ids.len();
    let progress_bar = create_progress_bar(length, "✨ Formatting", ProgressBarTheme::Green);

    let mut handles = Vec::with_capacity(length);
    for file_id in file_ids.into_iter() {
        handles.push(tokio::spawn({
            let interner = interner.clone();
            let db = read_database.clone();
            let progress_bar = progress_bar.clone();
            let change_log = change_log.clone();

            async move {
                let result = format_file(&interner, &db, &file_id, &change_log, php_version, settings, dry_run);

                progress_bar.inc(1);

                result
            }
        }));
    }

    let mut changed = 0;

    // Process each formatting task and update progress bar.
    for handle in handles {
        if handle.await?? {
            changed += 1;
        }
    }

    database.commit(change_log)?;

    remove_progress_bar(progress_bar);

    Ok(changed)
}

/// Formats a single file.
///
/// # Arguments
///
/// * `interner` - Reference to the interner to be used for parsing and formatting.
/// * `database` - The read-only database containing the files.
/// * `file_id` - The identifier of the file to format.
/// * `change_log` - The change log to record changes made during formatting.
/// * `php_version` - The PHP version to use for formatting.
/// * `settings` - Formatting settings to apply.
/// * `check` - A flag to determine whether to check or apply formatting.
///
/// # Returns
///
/// A result indicating whether the file was changed or an error occurred.
#[inline]
fn format_file(
    interner: &ThreadedInterner,
    database: &ReadDatabase,
    file_id: &FileId,
    change_log: &ChangeLog,
    php_version: PHPVersion,
    settings: FormatSettings,
    dry_run: bool,
) -> Result<bool, Error> {
    let file = database.get_by_id(file_id)?;

    let (program, error) = parse_file(interner, file);

    // Handle parsing errors and perform formatting.
    let changed = match error {
        Some(error) => {
            tracing::error!("Skipping formatting for file '{}': {}.", file.name, error);

            false
        }
        None => {
            let formatter = Formatter::new(interner, php_version, settings);
            let formatted = formatter.format(file, &program);

            utils::apply_update(change_log, file, formatted, dry_run)?
        }
    };

    Ok(changed)
}

/// Creates an ephemeral file from standard input.
///
/// # Returns
///
/// A result containing the file or an error.
fn create_file_from_stdin() -> Result<File, Error> {
    let mut content = String::new();
    std::io::stdin().read_to_string(&mut content).map_err(|e| Error::Database(DatabaseError::IOError(e)))?;

    Ok(File::ephemeral("<stdin>".to_owned(), content))
}

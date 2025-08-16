use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use mago_database::DatabaseReader;
use mago_interner::ThreadedInterner;

use crate::commands::args::reporting::ReportingArgs;
use crate::config::Configuration;
use crate::database;
use crate::error::Error;
use crate::pipeline::analysis::run_analysis_pipeline;

/// Command to perform static type analysis on PHP source code.
///
/// This command identifies potential type errors, unused code, and other
/// type-related issues within the specified PHP project or files.
#[derive(Parser, Debug)]
#[command(
    name = "analyze",
    // Alias for the British
    alias = "analyse",
    about = "Find typing issues in the project source code using configurable type checker settings.",
    long_about = "The `analyze` command is a fast type checker for PHP. It scans your codebase, \
                  builds a model of its symbols and types, and then analyzes it to find \
                  potential type errors, unused code, and other configurable checks."
)]
pub struct AnalyzeCommand {
    /// Specific files or directories to analyze.
    /// If provided, this overrides the source configuration from `mago.toml`.
    #[arg(help = "Analyze specific files or directories, overriding source configuration")]
    pub path: Vec<PathBuf>,

    /// Disable the use of stubs (e.g., for built-in PHP functions or popular libraries).
    /// Disabling stubs might lead to more reported issues if type information for external symbols is missing.
    #[arg(long, help = "Disable stubs, potentially leading to more issues", default_value_t = false)]
    pub no_stubs: bool,

    /// Enable or disable finding unused expressions.
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Find unused expressions (e.g., function calls whose results are not used)")]
    pub find_unused_expressions: Option<bool>,

    /// Enable or disable finding unused definitions (e.g., unreferenced functions, classes).
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Find unused definitions (functions, classes, constants, etc.)")]
    pub find_unused_definitions: Option<bool>,

    /// Enable or disable analysis of code known to be unreachable (dead code).
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Analyze dead code for errors (code known to be unreachable)")]
    pub analyze_dead_code: Option<bool>,

    /// Enable or disable memoization of properties.
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Enable memoization for property assignments and accesses")]
    pub memoize_properties: Option<bool>,

    /// Enable or disable the use of `include` construct in the analysis.
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Allow the use of `include` construct in the analysis")]
    pub allow_include: Option<bool>,

    /// Enable or disable the use of `eval` construct in the analysis.
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Allow the use of `eval` construct in the analysis")]
    pub allow_eval: Option<bool>,

    /// Enable or disable the use of `empty` construct in the analysis.
    /// Overrides the corresponding setting in `mago.toml` if provided.
    #[arg(long, help = "Allow the use of `empty` construct in the analysis")]
    pub allow_empty: Option<bool>,

    /// Enable or disable checking for thrown exceptions.
    /// This setting controls whether the analysis will report issues related to exceptions that are thrown but not caught.
    #[arg(long, help = "Check for thrown exceptions that are not caught")]
    pub check_throws: Option<bool>,

    /// Arguments related to reporting and fixing issues.
    #[clap(flatten)]
    pub reporting: ReportingArgs,

    /// Run the analysis sequentially instead of concurrently.
    #[arg(long, help = "Run the analysis sequentially instead of concurrently, which can be useful for debugging.")]
    pub sequential: bool,
}

/// Executes the analyze command.
///
/// This function orchestrates the process of:
///
/// 1. Loading source files.
/// 2. Compiling a codebase model from these files (with progress).
/// 3. Analyzing the user-defined sources against the compiled codebase (with progress).
/// 4. Reporting any found issues.
pub fn execute(command: AnalyzeCommand, configuration: Configuration) -> Result<ExitCode, Error> {
    let interner = ThreadedInterner::new();

    let database = if !command.path.is_empty() {
        database::from_paths(&configuration.source, command.path, !command.no_stubs)?
    } else {
        database::load(&configuration.source, true, !command.no_stubs)?
    };

    if database.is_empty() {
        tracing::info!("No files found to analyze.");

        return Ok(ExitCode::SUCCESS);
    }

    let mut analyzer_settings = configuration.analyze.to_setttings(configuration.php_version);
    if let Some(analyze_dead_code) = command.analyze_dead_code {
        analyzer_settings.analyze_dead_code = analyze_dead_code;
    }

    if let Some(find_unused_definitions) = command.find_unused_definitions {
        analyzer_settings.find_unused_definitions = find_unused_definitions;
    }

    if let Some(find_unused_expressions) = command.find_unused_expressions {
        analyzer_settings.find_unused_expressions = find_unused_expressions;
    }

    if let Some(memoize_properties) = command.memoize_properties {
        analyzer_settings.memoize_properties = memoize_properties;
    }

    if let Some(allow_include) = command.allow_include {
        analyzer_settings.allow_include = allow_include;
    }

    if let Some(allow_eval) = command.allow_eval {
        analyzer_settings.allow_eval = allow_eval;
    }

    if let Some(allow_empty) = command.allow_empty {
        analyzer_settings.allow_empty = allow_empty;
    }

    if let Some(check_throws) = command.check_throws {
        analyzer_settings.check_throws = check_throws;
    }

    let analysis_results = run_analysis_pipeline(&interner, database.read_only(), analyzer_settings)?;

    command.reporting.process_issues(analysis_results.issues, configuration, interner, database)
}

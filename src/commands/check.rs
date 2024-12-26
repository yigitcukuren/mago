use std::process::ExitCode;

use clap::Parser;

use mago_feedback::create_progress_bar;
use mago_feedback::remove_progress_bar;
use mago_feedback::ProgressBarTheme;
use mago_interner::ThreadedInterner;
use mago_reporting::reporter::Reporter;
use mago_reporting::reporter::ReportingFormat;
use mago_reporting::reporter::ReportingTarget;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_reporting::Level;
use mago_semantics::Semantics;
use mago_source::SourceManager;

use crate::config::Configuration;
use crate::enum_variants;
use crate::error::Error;
use crate::source;

#[derive(Parser, Debug)]
#[command(
    name = "check",
    about = "Check the project for parsing and semantic issues",
    long_about = r#"
Check the project for parsing and semantic issues.

This command analyzes the project's source code and highlights syntax and semantic issues in your PHP codebase.

For more in-depth analysis, consider using the `lint` command.
    "#
)]
pub struct CheckCommand {
    #[arg(long, default_value_t, help = "The issue reporting target to use.", ignore_case = true, value_parser = enum_variants!(ReportingTarget))]
    pub reporting_target: ReportingTarget,

    #[arg(long, default_value_t, help = "The issue reporting format to use.", ignore_case = true, value_parser = enum_variants!(ReportingFormat))]
    pub reporting_format: ReportingFormat,
}

pub async fn execute(command: CheckCommand, configuration: Configuration) -> Result<ExitCode, Error> {
    let interner = ThreadedInterner::new();
    let source_manager = source::load(&interner, &configuration.source, false).await?;

    let issues = process_sources(&interner, &source_manager).await?;

    let issues_contain_errors = issues.get_highest_level().is_some_and(|level| level >= Level::Error);

    let reporter = Reporter::new(interner, source_manager, command.reporting_target);

    reporter.report(issues, command.reporting_format)?;

    Ok(if issues_contain_errors { ExitCode::FAILURE } else { ExitCode::SUCCESS })
}

#[inline]
pub(super) async fn process_sources(
    interner: &ThreadedInterner,
    manager: &SourceManager,
) -> Result<IssueCollection, Error> {
    // Collect all user-defined sources.
    let sources: Vec<_> = manager.user_defined_source_ids().collect();
    let length = sources.len();

    let progress_bar = create_progress_bar(length, "🩻 Checking", ProgressBarTheme::Green);

    let mut handles = Vec::with_capacity(length);
    for source_id in sources {
        handles.push(tokio::spawn({
            let interner = interner.clone();
            let manager = manager.clone();
            let progress_bar = progress_bar.clone();

            async move {
                let source = manager.load(&source_id)?;
                let semantics = Semantics::build(&interner, source);
                progress_bar.inc(1);

                Result::<_, Error>::Ok(semantics)
            }
        }));
    }

    let mut results = Vec::with_capacity(length);
    for handle in handles {
        let semantic = handle.await??;

        if let Some(error) = &semantic.parse_error {
            results.push(Into::<Issue>::into(error));
        }

        results.extend(semantic.issues);
    }

    remove_progress_bar(progress_bar);

    Ok(IssueCollection::from(results.into_iter()))
}

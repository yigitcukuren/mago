use clap::Parser;

use mago_interner::ThreadedInterner;

use crate::service::config::Configuration;
use crate::service::linter::LintService;
use crate::service::source::SourceService;
use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "fix",
    about = "Fix lint issues identified during the linting process",
    long_about = r#"
Fix lint issues identified during the linting process.

Automatically applies fixes where possible, based on the rules in the `mago.toml` or the default settings.
    "#
)]
pub struct FixCommand {
    #[arg(long, short, help = "Apply fixes that are marked as unsafe, including potentially unsafe fixes")]
    pub r#unsafe: bool,
    #[arg(long, short, help = "Apply fixes that are marked as potentially unsafe")]
    pub potentially_unsafe: bool,
    #[arg(long, short, help = "Run the command without writing any changes to disk")]
    pub dry_run: bool,
}

pub async fn execute(command: FixCommand, configuration: Configuration) -> i32 {
    let interner = ThreadedInterner::new();

    let source_service = SourceService::new(interner.clone(), configuration.source);
    let source_manager = source_service.load().await.unwrap_or_else(bail);

    let service = LintService::new(configuration.linter, interner.clone(), source_manager.clone());

    let result = service.fix(command.r#unsafe, command.potentially_unsafe, command.dry_run).await.unwrap_or_else(bail);

    if result.skipped_unsafe > 0 {
        mago_feedback::warn!(
            "Skipped {} fixes because they were marked as unsafe. To apply those fixes, use the `--unsafe` flag.",
            result.skipped_unsafe
        );
    }

    if result.skipped_potentially_unsafe > 0 {
        mago_feedback::warn!(
            "Skipped {} fixes because they were marked as potentially unsafe. To apply those fixes, use the `--potentially-unsafe` flag.",
            result.skipped_potentially_unsafe
        );
    }

    if result.changed == 0 {
        mago_feedback::info!("No fixes were applied");

        return 0;
    }

    if command.dry_run {
        mago_feedback::info!("Found {} fixes that can be applied", result.changed);

        1
    } else {
        mago_feedback::info!("Applied {} fixes successfully", result.changed);

        0
    }
}

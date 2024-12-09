use clap::Parser;

use mago_fixer::SafetyClassification;
use mago_interner::ThreadedInterner;
use mago_service::config::Configuration;
use mago_service::linter::LintService;
use mago_service::source::SourceService;
use mago_source::error::SourceError;

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

impl FixCommand {
    pub fn get_safety_classification(&self) -> SafetyClassification {
        if self.r#unsafe {
            SafetyClassification::Unsafe
        } else if self.potentially_unsafe {
            SafetyClassification::PotentiallyUnsafe
        } else {
            SafetyClassification::Safe
        }
    }
}

pub async fn execute(command: FixCommand, configuration: Configuration) -> i32 {
    let interner = ThreadedInterner::new();

    let source_service = SourceService::new(interner.clone(), configuration.source);
    let source_manager = source_service.load().await.unwrap_or_else(bail);

    let service = LintService::new(configuration.linter, interner.clone(), source_manager.clone());

    let classification = command.get_safety_classification();
    let mut unsafe_fixes = 0;
    let mut potentially_unsafe_fixes = 0;
    let fix_plans = service.run().await.unwrap_or_else(bail).to_fix_plans().into_iter().filter_map(|(source, plan)| {
        if plan.is_empty() {
            return None;
        }

        let safe_plan = plan.to_minimum_safety_classification(classification);
        if safe_plan.is_empty() {
            match plan.get_minimum_safety_classification() {
                SafetyClassification::Unsafe => {
                    unsafe_fixes += 1;

                    mago_feedback::warn!("Skipping `{}` because it contains unsafe fixes.", interner.lookup(&source.0));
                }
                SafetyClassification::PotentiallyUnsafe => {
                    potentially_unsafe_fixes += 1;

                    mago_feedback::warn!(
                        "Skipping `{}` because it contains potentially unsafe fixes.",
                        interner.lookup(&source.0)
                    );
                }
                _ => {}
            }

            None
        } else {
            Some((source, safe_plan))
        }
    });

    let mut handles = vec![];
    for (source, plan) in fix_plans.into_iter() {
        handles.push(tokio::spawn({
            let source_manager = source_manager.clone();
            let interner = interner.clone();

            async move {
                let source = source_manager.load(source).unwrap_or_else(bail);
                let source_name = interner.lookup(&source.identifier.value());
                let source_content = interner.lookup(&source.content);

                mago_feedback::info!("Fixing {} issues in `{}`", plan.len(), source_name);

                let code = plan.execute(source_content);

                if command.dry_run {
                    // todo, print the diff in a pretty way
                    println!("TOO LAZY TO PRETTY PRINT: {:#?}", code);
                } else {
                    source_manager.write(source.identifier, code.get_fixed())?
                }

                Ok::<(), SourceError>(())
            }
        }));
    }

    if unsafe_fixes > 0 {
        mago_feedback::warn!(
            "Skipped {} fixes because they were marked as unsafe. To apply these fixes, use the `--unsafe` flag.",
            unsafe_fixes
        );
    }

    if potentially_unsafe_fixes > 0 {
        mago_feedback::warn!(
            "Skipped {} fixes because they were marked as potentially unsafe. To apply these fixes, use the `--potentially-unsafe` flag.",
            potentially_unsafe_fixes
        );
    }

    for handle in handles {
        handle.await.unwrap_or_else(bail).unwrap_or_else(bail);
    }

    0
}

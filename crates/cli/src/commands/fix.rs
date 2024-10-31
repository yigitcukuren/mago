use std::collections::hash_map::Entry;

use ahash::HashMap;
use clap::Parser;

use fennec_fixer::FixPlan;
use fennec_fixer::SafetyClassification;
use fennec_interner::ThreadedInterner;
use fennec_service::config::Configuration;
use fennec_service::linter::LintService;
use fennec_service::source::SourceService;
use fennec_source::SourceIdentifier;

use crate::utils::bail;

#[derive(Parser, Debug)]
#[command(
    name = "fix",
    about = "Fix lint issues identified during the linting process",
    long_about = r#"
Fix lint issues identified during the linting process.

Automatically applies fixes where possible, based on the rules in the `fennec.toml` or the default settings.
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
    let issues = service.run().await.unwrap_or_else(bail);

    let mut plans: HashMap<SourceIdentifier, Vec<FixPlan>> = HashMap::default();
    for issue in issues.into_iter() {
        for suggestion in issue.suggestions.into_iter() {
            match plans.entry(suggestion.0) {
                Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut().push(suggestion.1);
                }
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![suggestion.1]);
                }
            }
        }
    }

    let classification = command.get_safety_classification();
    let mut handles = vec![];
    for mut source_plans in plans.into_iter() {
        handles.push(tokio::spawn({
            let source_manager = source_manager.clone();
            let interner = interner.clone();

            let source = source_plans.0;
            let plan = source_plans.1.drain(..).collect::<FixPlan>();

            async move {
                let source = source_manager.load(source).await.unwrap_or_else(bail);
                let source_name = interner.lookup(source.identifier.value());
                let source_content = interner.lookup(source.content);

                if plan.get_minimum_safety_classification() > classification {
                    let required = classification.to_string();
                    let cuurent = plan.get_minimum_safety_classification().to_string();

                    fennec_feedback::debug!(
                        "skipping fix for `{}` because it requires a higher safety level ({} > {})",
                        source_name,
                        required,
                        cuurent
                    );
                } else {
                    fennec_feedback::info!("fixing issue in `{}` ( {} fix operations )", source_name, plan.len());

                    let code = plan.execute(source_content, classification);

                    if command.dry_run {
                        // todo, print the diff in a pretty way
                        println!("TOO LAZY TO PRETTY PRINT: {:#?}", code);
                    } else if let Some(path) = source.path {
                        std::fs::write(path, code.get_fixed())?;

                        fennec_feedback::info!("fixed issue in `{}`", source_name);
                    } else {
                        unreachable!();
                    }
                }

                Ok::<(), std::io::Error>(())
            }
        }));
    }

    for handle in handles {
        handle.await.unwrap_or_else(bail).unwrap_or_else(bail);
    }

    0
}

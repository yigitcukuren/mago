use indoc::indoc;

use mago_collector::pragma::Pragma;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_syntax::ast::*;

use crate::COLLECTOR_CATEGORY;
use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoUncategorizedPragmaRule;

impl Rule for NoUncategorizedPragmaRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("No Uncategorized Pragma", Level::Warning).with_description(indoc! {"
            Enforces that all Mago pragmas (`@mago-ignore` and `@mago-expect`) have a category specifier.

            This ensures that suppression comments target the correct tool (e.g., `lint:` or `analysis:`)
            and prevents them from being ignored in future Mago versions.
        "})
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::Program(program) = node else { return LintDirective::Abort };

        let pragmas = Pragma::extract(context.source, program.trivia.as_slice(), context.interner, None);
        for pragma in pragmas.iter() {
            if pragma.category.is_some() {
                continue;
            }

            let issue = Issue::new(context.level(), "Pragma is missing a category specifier.")
                .with_annotation(
                    Annotation::primary(pragma.span)
                        .with_message("This pragma needs a category to target a specific Mago tool."),
                )
                .with_annotation(Annotation::secondary(pragma.trivia_span).with_message("...within this comment."))
                .with_note("Pragmas must be categorized to ensure they apply to the intended tool (e.g., 'lint').")
                .with_note("Uncategorized pragmas are deprecated and will be ignored in future versions of Mago.")
                .with_help(format!(
                    "If this pragma is intended for the linter, add the '{COLLECTOR_CATEGORY}' prefix."
                ));

            context.propose(issue, |plan| {
                plan.insert(
                    pragma.code_span.start.offset,
                    format!("{COLLECTOR_CATEGORY}:"),
                    SafetyClassification::Safe,
                );
            });
        }

        LintDirective::Abort
    }
}

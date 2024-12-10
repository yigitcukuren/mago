use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct UseWhileInsteadOfForRule;

impl Rule for UseWhileInsteadOfForRule {
    fn get_name(&self) -> &'static str {
        "use-while-instead-of-for"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Note)
    }
}

impl<'a> Walker<LintContext<'a>> for UseWhileInsteadOfForRule {
    fn walk_in_for<'ast>(&self, r#for: &'ast For, context: &mut LintContext<'a>) {
        if !r#for.initializations.is_empty() || !r#for.increments.is_empty() {
            return;
        }

        let issue = Issue::new(
            context.level(),
            "use `while` instead of `for`",
        )
        .with_annotation(Annotation::primary(r#for.span()))
        .with_note("this `for` loop can be simplified to a `while` loop since it doesn't have initializations or increments.")
        .with_help("use a `while` loop instead of a `for` loop.");

        context.report_with_fix(issue, |plan| {
            plan.delete(r#for.r#for.span.to_range(), SafetyClassification::Safe);
            plan.insert(r#for.r#for.span.start.offset, "while", SafetyClassification::Safe);

            plan.delete(r#for.initializations_semicolon.to_range(), SafetyClassification::Safe);
            if r#for.conditions.is_empty() {
                plan.insert(r#for.initializations_semicolon.end.offset, "true", SafetyClassification::Safe);
            } else {
                for semicolon in r#for.conditions.tokens.iter() {
                    plan.replace(semicolon.span.to_range(), " && ", SafetyClassification::Safe);
                }
            }

            plan.delete(r#for.conditions_semicolon.to_range(), SafetyClassification::Safe);
            if let ForBody::ColonDelimited(for_colon_delimited_body) = &r#for.body {
                plan.replace(for_colon_delimited_body.end_for.span.to_range(), "endwhile", SafetyClassification::Safe);
            }
        });
    }
}

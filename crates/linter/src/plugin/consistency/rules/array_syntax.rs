use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct ArraySyntaxRule;

impl Rule for ArraySyntaxRule {
    fn get_name(&self) -> &'static str {
        "array-syntax"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Note)
    }
}

impl<'a> Walker<LintContext<'a>> for ArraySyntaxRule {
    fn walk_in_legacy_array<'ast>(&self, arr: &'ast LegacyArray, context: &mut LintContext<'a>) {
        if context.option("syntax").and_then(|o| o.as_str()).map(|v| v.to_lowercase().eq("long")).unwrap_or(false) {
            return;
        }

        let issue = Issue::new(context.level(), "Short array syntax `[..]` is preferred over `array(..)`.")
            .with_annotation(
                Annotation::primary(arr.span()).with_message("This array uses the long array syntax `array(..)`."),
            )
            .with_help("Use the short array syntax `[..]` instead");

        context.report_with_fix(issue, |plan| {
            plan.replace(arr.array.span.join(arr.left_parenthesis).to_range(), "[", SafetyClassification::Safe);
            plan.replace(arr.right_parenthesis.to_range(), "]", SafetyClassification::Safe);
        });
    }

    fn walk_in_array<'ast>(&self, arr: &'ast Array, context: &mut LintContext<'a>) {
        if !context.option("syntax").and_then(|o| o.as_str()).map(|v| v.to_lowercase().eq("long")).unwrap_or(false) {
            return;
        }

        let issue = Issue::new(context.level(), "Long array syntax `array(..)` is preferred over `[..]`.")
            .with_annotation(
                Annotation::primary(arr.span()).with_message("This array uses the short array syntax `[..]`."),
            )
            .with_help("Use the long array syntax `array(..)` instead");

        context.report_with_fix(issue, |plan| {
            plan.replace(arr.left_bracket.to_range(), "array(", SafetyClassification::Safe);
            plan.replace(arr.right_bracket.to_range(), ")", SafetyClassification::Safe)
        });
    }
}

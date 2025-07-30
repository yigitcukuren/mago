use indoc::indoc;

use mago_reporting::*;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoSprintfConcatenationRule;

impl Rule for NoSprintfConcatenationRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("No Sprintf Concatenation", Level::Warning)
            .with_description(indoc! {"
            Disallows string concatenation with the result of an `sprintf` call.

            Concatenating with `sprintf` is less efficient and can be less readable than
            incorporating the string directly into the format template. This pattern
            creates an unnecessary intermediate string and can make the final output
            harder to see at a glance.
        "})
            .with_example(RuleUsageExample::invalid(
                "Using `sprintf` with concatenation",
                indoc! {r#"
                    <?php

                    $name = 'World';
                    $greeting = 'Hello, ' . sprintf('%s!', $name);
                "#},
            ))
            .with_example(RuleUsageExample::valid(
                "Incorporating directly into `sprintf`",
                indoc! {r#"
                    <?php

                    $name = 'World';
                    $greeting = sprintf('Hello, %s!', $name);
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::Binary(Binary { lhs, operator: BinaryOperator::StringConcat(dot), rhs }) = node else {
            return LintDirective::default();
        };

        let (sprintf_call_expr, other_expr) = if is_sprintf_call(lhs, context) {
            (lhs, rhs)
        } else if is_sprintf_call(rhs, context) {
            (rhs, lhs)
        } else {
            return LintDirective::default();
        };

        context.report(
            Issue::new(context.level(), "String concatenation with `sprintf` can be simplified")
                .with_annotation(
                    Annotation::primary(dot.span())
                        .with_message("This concatenation can be avoided"),
                )
                .with_annotation(
                    Annotation::secondary(sprintf_call_expr.span())
                        .with_message("The result of this `sprintf` call..."),
                )
                .with_annotation(
                    Annotation::secondary(other_expr.span())
                        .with_message("...is being joined with this expression."),
                )
                .with_note(
                    "Combining all parts into a single `sprintf` call is more efficient and makes the code more readable.",
                )
                .with_help(
                    "Incorporate the concatenated content into the `sprintf` format argument."
                )
        );

        LintDirective::default()
    }
}

fn is_sprintf_call(expression: &Expression, context: &LintContext<'_>) -> bool {
    let Expression::Call(Call::Function(FunctionCall { function, .. })) = expression else {
        return false;
    };

    let Expression::Identifier(function_identifier) = function.as_ref() else { return false };

    let called_function_name = context.resolve_function_name(function_identifier);

    called_function_name.eq_ignore_ascii_case("sprintf")
}

use indoc::indoc;

use mago_php_version::PHPVersion;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct LiteralNamedArgumentRule;

impl Rule for LiteralNamedArgumentRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Literal Named Argument", Level::Warning)
            .with_description(indoc! {"
                Enforces that literal values used as arguments in function or method calls are passed as named arguments.
                This improves code clarity by making the purpose of the literal value self-evident at the call site.

                This rule helps avoid ambiguity, especially for boolean flags, numeric constants, or `null` values
                where the meaning isn't immediately clear from the value itself.
            "})
            .with_example(RuleUsageExample::valid(
                "Literal argument passed with its parameter name for clarity.",
                indoc! {"
                    <?php

                    function set_option(string $key, bool $enable_feature) {
                        // function implementation
                    }

                    set_option(key: 'feature_x', enable_feature: true); // correct usage
                "}
            ))
            .with_example(RuleUsageExample::invalid(
                "Literal argument passed positionally, making its purpose less clear.",
                indoc! {"
                    <?php

                    function set_option(string $key, bool $enable_feature) {
                        // function implementation
                    }

                    set_option('feature_x', true);
                "}
            ))
            .with_minimum_supported_php_version(PHPVersion::PHP80)
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::PositionalArgument(positional_argument) = node else { return LintDirective::default() };

        let Expression::Literal(literal) = &positional_argument.value else {
            return LintDirective::default();
        };

        let literal_value = match literal {
            Literal::String(literal_string) => context.interner.lookup(&literal_string.raw),
            Literal::Integer(literal_integer) => context.interner.lookup(&literal_integer.raw),
            Literal::Float(literal_float) => context.interner.lookup(&literal_float.raw),
            Literal::True(_) => "true",
            Literal::False(_) => "false",
            Literal::Null(_) => "null",
        };

        context.report(
            Issue::new(
                context.level(),
                format!("Literal argument `{literal_value}` should be passed as a named argument for clarity."),
            )
            .with_annotation(
                Annotation::primary(literal.span())
                    .with_message("Consider using a named argument for this literal value."),
            )
            .with_note("Using named arguments for literals, especially booleans, numbers, or `null`, makes the function/method call more self-documenting by clarifying the purpose of the value.")
            .with_help(format!("Consider changing the call to `function_name(literal: {literal_value})` instead of `function_name({literal_value})`.")),
        );

        LintDirective::default()
    }
}

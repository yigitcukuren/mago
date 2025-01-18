use indoc::indoc;

use mago_ast::Argument;
use mago_php_version::PHPVersion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NamedArgumentsFeatureRule;

impl Rule for NamedArgumentsFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Named Arguments Feature", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP80)
            .with_description(indoc! {r#"
                Detects usage of named function arguments, introduced in PHP 8.0. This feature allows
                calling functions like `foo(bar: 1, baz: 2)` rather than relying on positional parameters.
            "#})
            .with_example(RuleUsageExample::valid(
                "Using positional arguments (compatible with <8.0)",
                indoc! {r#"
                    <?php

                    function greet(string $first, string $last): void {
                        echo "Hello {$first} {$last}!";
                    }

                    greet("John", "Doe");
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using named arguments (PHP 8.0+)",
                indoc! {r#"
                    <?php

                    function greet(string $first, string $last): void {
                        echo "Hello {$first} {$last}!";
                    }

                    greet(last: "Doe", first: "John");
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for NamedArgumentsFeatureRule {
    fn walk_in_argument(&self, argument: &Argument, context: &mut LintContext<'_>) {
        let Argument::Named(named_argument) = argument else {
            return;
        };

        let issue = Issue::new(context.level(), "Named arguments are only available in PHP 8.0 and above.")
            .with_annotation(Annotation::primary(named_argument.span()).with_message("Named argument used here."));

        context.report(issue);
    }
}

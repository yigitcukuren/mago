use indoc::indoc;

use mago_ast::ClosureCreation;
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
pub struct ClosureCreationFeatureRule;

impl Rule for ClosureCreationFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Closure Creation Feature", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP81)
            .with_description(indoc! {r#"
                Detects usage of the closure creation syntax (e.g. `$var = foo(...)`)
                introduced in PHP 8.1. This feature allows creating a closure
                from an existing function without an explicit closure.
            "#})
            .with_example(RuleUsageExample::valid(
                "Using a normal closure or function reference (pre-8.1)",
                indoc! {r#"
                    <?php

                    $func = function(string $arg): string {
                        return foo($arg);
                    };
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using the closure creation feature (PHP 8.1+)",
                indoc! {r#"
                    <?php

                    $func = foo(...);
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for ClosureCreationFeatureRule {
    fn walk_closure_creation(&self, closure_creation: &ClosureCreation, context: &mut LintContext<'_>) {
        let issue = Issue::new(context.level(), "The closure creation syntax is only available in PHP 8.1 and above.")
            .with_annotation(
                Annotation::primary(closure_creation.span()).with_message("Closure creation syntax used here."),
            );

        context.report(issue);
    }
}

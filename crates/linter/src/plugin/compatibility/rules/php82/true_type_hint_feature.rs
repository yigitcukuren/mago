use indoc::indoc;

use mago_ast::Hint;
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
pub struct TrueTypeHintFeatureRule;

impl Rule for TrueTypeHintFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("True Type Hint Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP82)
            .with_description(indoc! {r#"
                Detects usage of the `true` type hint, which is only available in PHP 8.2 and above.
                Prior to PHP 8.2, `true` cannot be used as a type hint.
            "#})
            .with_example(RuleUsageExample::valid(
                "Type hint without `true` (compatible with <8.2)",
                indoc! {r#"
                    <?php

                    function foo(bool $value): void {}
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Type hint with `true` (PHP 8.2+)",
                indoc! {r#"
                    <?php

                    function foo(true $value): void {}
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for TrueTypeHintFeatureRule {
    fn walk_in_hint(&self, hint: &Hint, context: &mut LintContext<'_>) {
        if let Hint::True(r#true) = hint {
            let issue = Issue::new(context.level(), "The `true` type hint is only available in PHP 8.2 and above.")
                .with_annotation(Annotation::primary(r#true.span()).with_message("`true` type hint used here."));

            context.report(issue);
        }
    }
}

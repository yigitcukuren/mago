use indoc::indoc;

use mago_ast::Class;
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
pub struct ReadonlyClassFeatureRule;

impl Rule for ReadonlyClassFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Readonly Class Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP82)
            .with_description(indoc! {r#"
                Detects usage of readonly classes, which are only available in PHP 8.2 and above.
                Prior to PHP 8.2, classes cannot be marked as readonly.
            "#})
            .with_example(RuleUsageExample::valid(
                "Regular class (compatible with <8.2)",
                indoc! {r#"
                    <?php

                    class Example {}
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Readonly class (PHP 8.2+)",
                indoc! {r#"
                    <?php

                    readonly class Example {}
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for ReadonlyClassFeatureRule {
    fn walk_in_class(&self, class: &Class, context: &mut LintContext<'_>) {
        if let Some(modifier) = class.modifiers.get_readonly() {
            let issue = Issue::new(context.level(), "Readonly classes are only available in PHP 8.2 and above.")
                .with_annotation(Annotation::primary(modifier.span()).with_message("Readonly modifier used here."));

            context.report(issue);
        }
    }
}

use indoc::indoc;

use mago_ast::PlainProperty;
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
pub struct AsymmetricVisibilityFeatureRule;

impl Rule for AsymmetricVisibilityFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Asymmetric Visibility Feature", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP84)
            .with_description(indoc! {r#"
                Detects usage of asymmetric visibility on properties or methods
                (e.g., `public protected(set)`, `protected private(set)`), introduced in PHP 8.4.
                This allows different access levels for reading vs. writing a property.
            "#})
            .with_example(RuleUsageExample::valid(
                "Using uniform visibility (compatible with <8.4)",
                indoc! {r#"
                    <?php

                    class Foo {
                        private string $bar;

                        public function getBar(): string {
                            return $this->bar;
                        }

                        private function setBar(string $value): void {
                            $this->bar = $value;
                        }
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using asymmetric visibility (PHP 8.4+)",
                indoc! {r#"
                    <?php

                    class Foo {
                        public private(set) string $bar;
                    }
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for AsymmetricVisibilityFeatureRule {
    fn walk_plain_property(&self, plain_property: &PlainProperty, context: &mut LintContext<'_>) {
        let Some(write_visibility) = plain_property.modifiers.get_first_write_visibility() else {
            return;
        };

        let issue = Issue::new(context.level(), "Asymmetric visibility is only available in PHP 8.4 and above.")
            .with_annotation(
                Annotation::primary(write_visibility.span()).with_message("Asymmetric visibility used here."),
            );

        context.report(issue);
    }
}

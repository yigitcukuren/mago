use indoc::indoc;

use mago_ast::HookedProperty; // Fixed missing import
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
pub struct HookedPropertyFeatureRule;

impl Rule for HookedPropertyFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Hooked Property Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP84)
            .with_description(indoc! {r#"
                Detects usage of hooked properties introduced in PHP 8.4.
                Hooked properties allow defining custom get/set handlers directly
                in the property declaration using `get` and `set` blocks.
            "#})
            .with_example(RuleUsageExample::valid(
                "Regular property without hooks (compatible with <8.4)",
                indoc! {r#"
                    <?php

                    class Foo {
                        public string $bar;
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Property with both get and set hooks",
                indoc! {r#"
                    <?php

                    class User {
                        public string $name = 'default' {
                            get {
                                return strtoupper($this->name);
                            }
                            set(string $value) {
                                $this->name = trim($value);
                            }
                        }
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Property with only a get hook",
                indoc! {r#"
                    <?php

                    class Logger {
                        public array $logs = [] {
                            get {
                                return array_reverse($this->logs);
                            }
                        }
                    }
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for HookedPropertyFeatureRule {
    fn walk_hooked_property(&self, hooked_property: &HookedProperty, context: &mut LintContext<'_>) {
        let issue = Issue::new(context.level(), "Hooked properties are only available in PHP 8.4 and above.")
            .with_annotation(
                Annotation::primary(hooked_property.span()).with_message("Hooked property declaration used here."),
            );

        context.report(issue);
    }
}

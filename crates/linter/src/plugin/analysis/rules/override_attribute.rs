use indoc::indoc;

use mago_ast::*;
use mago_php_version::PHPVersion;
use mago_reporting::*;
use mago_span::HasSpan;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct OverrideAttributeRule;

impl Rule for OverrideAttributeRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Override Attribute", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP83)
            .with_description(indoc! {"
                Ensures proper usage of the #[Override] attribute in PHP code.

                Validates three main scenarios:

                1. Overriding methods must have the #[Override] attribute
                2. Non-overriding methods must NOT have the attribute
                3. Constructors cannot use the #[Override] attribute

                Helps prevent subtle inheritance bugs and improves code clarity.
            "})
            .with_example(RuleUsageExample::valid(
                "Correct #[Override] usage",
                indoc! {r#"
                    <?php

                    class ParentClass {
                        protected function process(): void {}
                    }

                    class ChildClass extends ParentClass {
                        #[Override]
                        protected function process(): void {}
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Missing #[Override] attribute in overriding method",
                indoc! {r#"
                    <?php

                    class ParentClass {
                        public function save(): void {}
                    }

                    class ChildClass extends ParentClass {
                        public function save(): void {}  // Missing attribute
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Unnecessary #[Override] attribute in non-overriding method",
                indoc! {r#"
                    <?php

                    class Example {
                        #[Override]
                        public function uniqueMethod(): void {}  // Not overriding
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Invalid #[Override] attribute in constructor",
                indoc! {r#"
                    <?php

                    class ParentClass {
                        public function __construct() {}
                    }

                    class ChildClass extends ParentClass {
                        #[Override]
                        public function __construct() {}  // Constructors cannot use the attribute
                    }
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let (reflection, members) = match node {
            Node::Class(class) => {
                let name = context.module.names.get(&class.name);
                let Some(reflection) = context.codebase.get_class(context.interner, name) else {
                    return LintDirective::default();
                };

                (reflection, class.members.iter())
            }
            Node::Enum(r#enum) => {
                let name = context.module.names.get(&r#enum.name);
                let Some(reflection) = context.codebase.get_enum(context.interner, name) else {
                    return LintDirective::default();
                };

                (reflection, r#enum.members.iter())
            }
            Node::Interface(r#interface) => {
                let name = context.module.names.get(&r#interface.name);
                let Some(reflection) = context.codebase.get_interface(context.interner, name) else {
                    return LintDirective::default();
                };

                (reflection, r#interface.members.iter())
            }
            Node::Trait(r#trait) => {
                let name = context.module.names.get(&r#trait.name);
                let Some(reflection) = context.codebase.get_trait(context.interner, name) else {
                    return LintDirective::default();
                };

                (reflection, r#trait.members.iter())
            }
            Node::AnonymousClass(anonymous_class) => {
                let Some(reflection) = context.codebase.get_anonymous_class(&anonymous_class) else {
                    return LintDirective::default();
                };

                (reflection, anonymous_class.members.iter())
            }
            _ => return LintDirective::default(),
        };

        for member in members {
            let ClassLikeMember::Method(method) = member else {
                continue;
            };

            let override_attribute = 'outer: {
                for attribute_list in method.attribute_lists.iter() {
                    for attribute in attribute_list.attributes.iter() {
                        let name = context.lookup_name(&attribute.name);

                        if name.eq_ignore_ascii_case("Override") {
                            break 'outer Some(attribute);
                        }
                    }
                }

                None
            };

            let name = context.interner.lookup(&method.name.value);
            if name.eq_ignore_ascii_case("__construct") {
                if let Some(attribute) = override_attribute {
                    let issue = Issue::new(context.level(), "Invalid `#[Override]` attribute on constructor.")
                        .with_annotation(
                            Annotation::primary(attribute.span())
                                .with_message("Constructors cannot be marked with `#[Override]`."),
                        )
                        .with_note("PHP constructors don't override parent constructors.")
                        .with_help("Remove the `#[Override]` attribute from the constructor.");

                    context.report(issue);
                }

                continue;
            }

            let Some(method_reflection) = reflection.methods.members.get(&method.name.value) else {
                continue;
            };

            if !method_reflection.is_overriding {
                if let Some(attribute) = override_attribute {
                    let classname = reflection.name.get_key(context.interner);
                    let issue = Issue::new(
                        context.level(),
                        format!("Unnecessary `#[Override]` attribute on `{}::{}`.", classname, name),
                    )
                    .with_annotation(
                        Annotation::primary(attribute.span())
                            .with_message("This method doesn't override any parent method."),
                    )
                    .with_note("The attribute should only be used when explicitly overriding a parent method.")
                    .with_help(format!("Remove the `#[Override]` attribute from `{}` or verify inheritance.", name));

                    context.report(issue);
                }

                continue;
            } else if override_attribute.is_some() {
                continue;
            }

            let Some(parent_class_names) = reflection.methods.overriden_members.get(&method.name.value) else {
                continue;
            };

            let Some(parent_class_name) = parent_class_names.iter().next() else {
                continue;
            };

            let classname = reflection.name.get_key(context.interner);
            let parent_class = parent_class_name.get_key(context.interner);

            let issue = Issue::new(
                context.level(),
                format!("Missing `#[Override]` attribute on overriding method `{}::{}`.", classname, name),
            )
            .with_annotation(
                Annotation::primary(method.name.span)
                    .with_message(format!("This method overrides `{}::{}`.", parent_class, name)),
            )
            .with_note("The `#[Override]` attribute clarifies intent and prevents accidental signature mismatches.")
            .with_help("Add `#[Override]` attribute to method declaration.");

            context.report(issue);
        }

        LintDirective::default()
    }
}

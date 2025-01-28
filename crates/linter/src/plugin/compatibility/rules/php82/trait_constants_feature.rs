use indoc::indoc;

use mago_ast::ClassLikeMember;
use mago_ast::Trait;
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
pub struct TraitConstantsFeatureRule;

impl Rule for TraitConstantsFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Trait Constants Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP82)
            .with_description(indoc! {r#"
                Detects usage of constants in traits, which are only available in PHP 8.2 and above.
                Prior to PHP 8.2, traits cannot define constants.
            "#})
            .with_example(RuleUsageExample::valid(
                "Trait without constants (compatible with <8.2)",
                indoc! {r#"
                    <?php

                    trait Example {
                        public function foo(): void {}
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Trait with constants (PHP 8.2+)",
                indoc! {r#"
                    <?php

                    trait Example {
                        public const FOO = 'bar';

                        public function foo(): void {}
                    }
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for TraitConstantsFeatureRule {
    fn walk_in_trait(&self, r#trait: &Trait, context: &mut LintContext<'_>) {
        for member in r#trait.members.iter() {
            if let ClassLikeMember::Constant(constant) = member {
                let issue = Issue::new(context.level(), "Constants in traits are only available in PHP 8.2 and above.")
                    .with_annotation(Annotation::primary(constant.span()).with_message("Constant defined in trait."))
                    .with_annotation(Annotation::secondary(r#trait.span()).with_message("Trait defined here."));

                context.report(issue);
            }
        }
    }
}

use indoc::indoc;

use mago_ast::FunctionLikeParameter;
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
pub struct PromotedPropertiesFeatureRule;

impl Rule for PromotedPropertiesFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Promoted Properties Feature", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP80)
            .with_description(indoc! {r#"
                Detects usage of constructor property promotion, introduced in PHP 8.0.
                This syntax allows property definitions in the constructor signature (e.g.
                `public function __construct(private int $id) {}`) rather than separate property
                declarations.
            "#})
            .with_example(RuleUsageExample::valid(
                "Defining properties without promotion (compatible with <8.0)",
                indoc! {r#"
                    <?php

                    class User {
                        private int $id;

                        public function __construct(int $id) {
                            $this->id = $id;
                        }
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using promoted properties (PHP 8.0+)",
                indoc! {r#"
                    <?php

                    class User {
                        public function __construct(private int $id) {}
                    }
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for PromotedPropertiesFeatureRule {
    fn walk_in_function_like_parameter(
        &self,
        function_like_parameter: &FunctionLikeParameter,
        context: &mut LintContext<'_>,
    ) {
        if !function_like_parameter.is_promoted_property() {
            return;
        }

        let issue = Issue::new(context.level(), "Promoted properties are only available in PHP 8.0 and above.")
            .with_annotation(
                Annotation::primary(function_like_parameter.span()).with_message("Promoted property used here."),
            );

        context.report(issue);
    }
}

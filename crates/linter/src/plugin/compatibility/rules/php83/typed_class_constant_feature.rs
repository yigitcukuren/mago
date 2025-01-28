use indoc::indoc;

use mago_ast::ClassLikeConstant;
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
pub struct TypedClassConstantFeatureRule;

impl Rule for TypedClassConstantFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Typed Class Constant Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP83)
            .with_description(indoc! {r#"
                Detects usage of typed class constants, which are only available in PHP 8.3 and above.
                Prior to PHP 8.3, class constants cannot have type declarations.
            "#})
            .with_example(RuleUsageExample::valid(
                "Untyped class constant (compatible with <8.3)",
                indoc! {r#"
                    <?php

                    class Example {
                        public const FOO = 'bar';
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Typed class constant (PHP 8.3+)",
                indoc! {r#"
                    <?php

                    class Example {
                        public const string FOO = 'bar';
                    }
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for TypedClassConstantFeatureRule {
    fn walk_in_class_like_constant(&self, class_like_constant: &ClassLikeConstant, context: &mut LintContext<'_>) {
        let Some(type_hint) = &class_like_constant.hint else {
            return;
        };

        let issue = Issue::new(context.level(), "Typed class constants are only available in PHP 8.3 and above.")
            .with_annotation(Annotation::primary(type_hint.span()).with_message("Type hint used here."));

        context.report(issue);
    }
}

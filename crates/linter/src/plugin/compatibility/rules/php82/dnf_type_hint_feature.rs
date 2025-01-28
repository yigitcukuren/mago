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
pub struct DnfTypeHintFeatureRule;

impl Rule for DnfTypeHintFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Disjunctive Normal Form Type Hint Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP82)
            .with_description(indoc! {r#"
                Detects usage of Disjunctive Normal Form (DNF) types, which are only available in PHP 8.2 and above.
                Prior to PHP 8.2, union types cannot contain intersection types, and vice versa.
            "#})
            .with_example(RuleUsageExample::valid(
                "Simple union or intersection types (compatible with <8.2)",
                indoc! {r#"
                    <?php

                    function foo(int|string $value): void {}
                    function bar(Countable&Traversable $value): void {}
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "DNF types (PHP 8.2+)",
                indoc! {r#"
                    <?php

                    function foo((Countable&Traversable)|false $value): void {}
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for DnfTypeHintFeatureRule {
    fn walk_in_hint(&self, hint: &Hint, context: &mut LintContext<'_>) {
        let is_dnf = match hint {
            Hint::Intersection(inter) if inter.left.is_union() || inter.right.is_union() => true,
            Hint::Union(union) if union.left.is_intersection() || union.right.is_intersection() => true,
            _ => false,
        };

        if !is_dnf {
            return;
        }

        context.report(
            Issue::new(context.level(), "Disjunctive Normal Form (DNF) types are only available in PHP 8.2 and above.")
                .with_annotation(Annotation::primary(hint.span()).with_message("DNF type used here.")),
        );
    }
}

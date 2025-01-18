use indoc::indoc;

use mago_ast::ast::*;
use mago_php_version::PHPVersion;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct NullCoalesceAssignmentFeatureRule;

impl Rule for NullCoalesceAssignmentFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Null Coalesce Assignment Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP74)
            .with_description(indoc! {"
                Flags any usage of the `??=` operator, which was introduced in PHP 7.4.

                In environments running older versions of PHP, this operator is unavailable.
                For backwards compatibility, you can use `$var = $var ?? <default>` instead.
            "})
            .with_example(RuleUsageExample::valid(
                "Assigning a default value manually",
                indoc! {r#"
                    <?php

                    // Works in all PHP versions (pre-7.4 included):
                    $a = $a ?? 1;
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using the null coalesce assign feature",
                indoc! {r#"
                    <?php

                    // Only valid in PHP 7.4+:
                    $a ??= 1;
                "#},
            ))
    }
}

impl<'a> Walker<LintContext<'a>> for NullCoalesceAssignmentFeatureRule {
    fn walk_in_assignment(&self, assignment: &Assignment, context: &mut LintContext<'a>) {
        let AssignmentOperator::Coalesce(operator) = assignment.operator else {
            return;
        };

        let issue = Issue::new(
            context.level(),
            "The `??=` (null coalesce assignment) operator is only available in PHP 7.4 and later.",
        )
        .with_annotation(
            Annotation::primary(operator.span()).with_message("Null coalesce assignment operator `??=` used here."),
        )
        .with_note("Use a manual check-and-assignment approach if you need compatibility with older PHP versions.")
        .with_help("Replace `$var ??= <default>` with `$var = $var ?? <default>`.");

        context.report(issue);
    }
}

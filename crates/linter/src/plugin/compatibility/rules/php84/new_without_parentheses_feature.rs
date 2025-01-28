use indoc::indoc;

use mago_ast::*;
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
pub struct NewWithoutParenthesesRule;

impl Rule for NewWithoutParenthesesRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("New Without Parentheses Feature", Level::Error)
            .with_maximum_supported_php_version(PHPVersion::PHP84)
            .with_description(indoc! {r#"
                Detects direct method/property access on `new` expressions without parentheses.
                Prior to PHP 8.4, parentheses are required around `new` expressions when
                immediately accessing methods or properties (e.g., `(new Foo())->bar()`).
            "#})
            .with_example(RuleUsageExample::valid(
                "Using parentheses with new expression (compatible with <8.4)",
                indoc! {r#"
                    <?php

                    (new Foo())->method();
                    (new Bar())?->method();
                    (new Baz())->property;
                    (new Qux())?->property;
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Omitting parentheses around new expression (PHP 8.4+)",
                indoc! {r#"
                    <?php

                    new Foo()->method();
                    new Bar()?->method();
                    new Baz()->property;
                    new Qux()?->property;
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Chained calls without parentheses",
                indoc! {r#"
                    <?php

                    new Foo()->bar()?->baz();
                    new Logger()?->log()->flush();
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for NewWithoutParenthesesRule {
    fn walk_in_method_call(&self, method_call: &MethodCall, context: &mut LintContext<'_>) {
        check(&method_call.object, context, "method call");
    }

    fn walk_in_null_safe_method_call(&self, null_safe_call: &NullSafeMethodCall, context: &mut LintContext<'_>) {
        check(&null_safe_call.object, context, "nullsafe method call");
    }

    fn walk_in_property_access(&self, property_access: &PropertyAccess, context: &mut LintContext<'_>) {
        check(&property_access.object, context, "property access");
    }

    fn walk_in_null_safe_property_access(
        &self,
        null_safe_access: &NullSafePropertyAccess,
        context: &mut LintContext<'_>,
    ) {
        check(&null_safe_access.object, context, "nullsafe property access");
    }
}

#[inline]
fn check(object_expr: &Expression, context: &mut LintContext<'_>, operation: &str) {
    let Expression::Instantiation(instantiation) = object_expr else {
        return;
    };

    let issue = Issue::new(
        context.level(),
        format!("Direct {operation} on `new` expressions without parentheses is only available in PHP 8.4 and above."),
    )
    .with_annotation(
        Annotation::primary(instantiation.span())
            .with_message(format!("Unparenthesized `new` expression used for {operation}.")),
    );

    context.report(issue);
}

use indoc::indoc;

use mago_ast::FunctionLikeReturnTypeHint;
use mago_ast::Hint;
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
pub struct UnionTypeHintFeatureRule;

impl Rule for UnionTypeHintFeatureRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Union Type Hint Feature", Level::Error)
            .with_minimum_supported_php_version(PHPVersion::PHP80)
            .with_description(indoc! {"
                Detects usage of union type hints (e.g. `int|float` or `A|B`) which were introduced in PHP 8.0.
            "})
            .with_example(RuleUsageExample::valid(
                "Avoiding union type hints for pre-8.0 compatibility",
                indoc! {r#"
                    <?php

                    /**
                     * @param int|float $val
                     */
                    function calculate($val): void
                    {
                        if (!is_int($val) && !is_float($val)) {
                            throw new InvalidArgumentException('Expected int or float');
                        }

                        // ...
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using union type hints (PHP 8.0+)",
                indoc! {r#"
                    <?php

                    function calculate(int|float $val): void
                    {
                        // ...
                    }
                "#},
            ))
    }
}

impl<'a> Walker<LintContext<'a>> for UnionTypeHintFeatureRule {
    fn walk_function_like_parameter(
        &self,
        function_like_parameter: &mago_ast::FunctionLikeParameter,
        context: &mut LintContext<'a>,
    ) {
        if let Some(Hint::Union(union_hint)) = &function_like_parameter.hint {
            context.report(
                Issue::new(
                    context.level(),
                    "Union type hints (e.g. `int|float`) are only available in PHP 8.0 and above.",
                )
                .with_annotation(Annotation::primary(union_hint.span()).with_message("Union type hint used here."))
                .with_note(
                    "Union type hints are only available in PHP 8.0 and above. Consider using a different approach.",
                ),
            );
        }
    }

    fn walk_function_like_return_type_hint(
        &self,
        function_like_return_type_hint: &FunctionLikeReturnTypeHint,
        context: &mut LintContext<'a>,
    ) {
        if let Hint::Union(union_hint) = &function_like_return_type_hint.hint {
            context.report(
                Issue::new(
                    context.level(),
                    "Union type hints (e.g. `int|float`) are only available in PHP 8.0 and above.",
                )
                .with_annotation(Annotation::primary(union_hint.span()).with_message("Union type hint used here."))
                .with_note(
                    "Union type hints are only available in PHP 8.0 and above. Consider using a different approach.",
                ),
            );
        }
    }

    fn walk_plain_property(&self, plain_property: &PlainProperty, context: &mut LintContext<'a>) {
        if let Some(Hint::Union(union_hint)) = &plain_property.hint {
            context.report(
                Issue::new(
                    context.level(),
                    "Union type hints (e.g. `int|float`) are only available in PHP 8.0 and above.",
                )
                .with_annotation(Annotation::primary(union_hint.span()).with_message("Union type hint used here."))
                .with_note(
                    "Union type hints are only available in PHP 8.0 and above. Consider using a different approach.",
                ),
            );
        }
    }
}

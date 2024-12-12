use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct CombineConsecutiveIssetsRule;

impl Rule for CombineConsecutiveIssetsRule {
    fn get_name(&self) -> &'static str {
        "combine-consecutive-issets"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for CombineConsecutiveIssetsRule {
    fn walk_in_binary(&self, binary: &Binary, context: &mut LintContext<'a>) {
        let BinaryOperator::And(_) = binary.operator else {
            return;
        };

        let Some((left_parenthesized, left_isset)) = get_isset_construct(binary.lhs.as_ref(), true) else {
            return;
        };

        let Some((right_parenthesized, right_isset)) = get_isset_construct(binary.rhs.as_ref(), false) else {
            return;
        };

        let issue = Issue::new(context.level(), "consecutive isset calls can be combined")
            .with_annotation(Annotation::primary(left_isset.span()))
            .with_annotation(Annotation::primary(right_isset.span()))
            .with_annotation(Annotation::secondary(binary.span()))
            .with_help("combine the isset calls into a single call, e.g. `isset($a, $b)`");

        // don't bother fixing if either of the isset calls is already parenthesized
        // this can be messy to fix and is not worth the effort.
        if left_parenthesized || right_parenthesized {
            return context.report(issue);
        }

        context.report_with_fix(issue, |plan| {
            let to_replace = left_isset.right_parenthesis.join(binary.operator.span());
            let to_delete = right_isset.isset.span.join(right_isset.left_parenthesis);

            plan.replace(to_replace.to_range(), ",".to_string(), SafetyClassification::Safe);
            plan.delete(to_delete.to_range(), SafetyClassification::Safe);
        });
    }
}

fn get_isset_construct(mut expression: &Expression, select_binary_rhs: bool) -> Option<(bool, &IssetConstruct)> {
    let mut between_parentheses = false;

    while let Expression::Parenthesized(parenthesized) = expression {
        expression = parenthesized.expression.as_ref();
        between_parentheses = true;
    }

    match expression {
        Expression::Construct(construct) => {
            if let Construct::Isset(isset) = construct.as_ref() {
                Some((between_parentheses, isset))
            } else {
                None
            }
        }
        Expression::Binary(binary) if select_binary_rhs => {
            if let BinaryOperator::And(_) = binary.operator {
                let (lhs_between_parentheses, lhs_isset) = get_isset_construct(binary.rhs.as_ref(), true)?;

                Some((between_parentheses || lhs_between_parentheses, lhs_isset))
            } else {
                None
            }
        }
        _ => None,
    }
}

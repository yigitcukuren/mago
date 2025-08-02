use std::rc::Rc;

use mago_codex::ttype::TType;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::get_mixed_any;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::artifacts::get_expression_range;
use crate::code::Code;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

/// Analyzes the Elvis operator (`?:`).
///
/// The result type is determined based on the "falsiness" of the left-hand side (LHS):
/// - If LHS is always falsy (e.g., `false`, `null`, `0`, `""`), the result is the type of the RHS.
///   A hint is issued about the LHS always being falsy.
/// - If LHS is never falsy (e.g., `true`, non-empty string, non-zero number, object),
///   the result is the type of the LHS. The RHS is still analyzed for side effects.
///   A hint is issued about the RHS being redundant.
/// - If LHS can be falsy (e.g., `bool`, `int`, `string`), the result is a union of the
///   "truthy" parts of the LHS type and the RHS type.
/// - If LHS is `mixed`, the result is a union of `mixed` and the RHS type.
///
/// Data flow is established from the operand(s) that contribute to the result.
pub fn analyze_elvis_operation<'a>(
    binary: &Binary,
    context: &mut Context<'a>,
    block_context: &mut BlockContext<'a>,
    artifacts: &mut AnalysisArtifacts,
) -> Result<(), AnalysisError> {
    binary.lhs.analyze(context, block_context, artifacts)?;
    let lhs_type_option = artifacts.get_rc_expression_type(&binary.lhs).cloned();

    let Some(lhs_type) = lhs_type_option else {
        binary.rhs.analyze(context, block_context, artifacts)?;
        artifacts.set_expression_type(binary, get_mixed_any());
        return Ok(());
    };

    let result_type: TUnion;

    if lhs_type.is_always_falsy() {
        context.collector.report_with_code(
            Code::REDUNDANT_ELVIS,
            Issue::help("Redundant Elvis operator: left-hand side is always falsy.")
                .with_annotation(Annotation::primary(binary.lhs.span()).with_message("This is always falsy"))
                .with_annotation(
                    Annotation::secondary(binary.rhs.span())
                        .with_message("This right-hand side will always be evaluated"),
                )
                .with_note("The Elvis operator `?:` evaluates the right-hand side if the left-hand side is falsy.")
                .with_help("Consider directly using the right-hand side expression."),
        );

        binary.rhs.analyze(context, block_context, artifacts)?;
        result_type = artifacts.get_expression_type(&binary.rhs).cloned().unwrap_or_else(get_mixed_any);
    } else if lhs_type.is_always_truthy() {
        context.collector.report_with_code(
            Code::REDUNDANT_ELVIS,
            Issue::help("Redundant Elvis operator: left-hand side is always truthy.")
                .with_annotation(Annotation::primary(binary.lhs.span()).with_message(format!(
                    "This expression (type `{}`) is always truthy",
                    lhs_type.get_id(Some(context.interner))
                )))
                .with_annotation(
                    Annotation::secondary(binary.rhs.span())
                        .with_message("This right-hand side will never be evaluated"),
                )
                .with_note("The Elvis operator `?:` only evaluates the right-hand side if the left-hand side is falsy.")
                .with_help("Consider removing the `?:` operator and the right-hand side expression."),
        );

        result_type = (*lhs_type).clone();
        binary.rhs.analyze(context, block_context, artifacts)?;
    } else {
        binary.rhs.analyze(context, block_context, artifacts)?;
        let rhs_type = artifacts.get_expression_type(&binary.rhs).cloned().unwrap_or_else(get_mixed_any);

        let truthy_lhs_type = lhs_type.to_truthy();

        result_type = combine_union_types(&truthy_lhs_type, &rhs_type, context.codebase, context.interner, false);
    }

    artifacts.expression_types.insert(get_expression_range(binary), Rc::new(result_type));

    Ok(())
}

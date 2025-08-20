use mago_codex::metadata::class_like::ClassLikeMetadata;
use mago_codex::metadata::function_like::FunctionLikeMetadata;
use mago_syntax::ast::ClassLikeMember;
use mago_syntax::ast::FunctionLikeParameter;
use mago_syntax::ast::Statement;

use crate::context::Context;
use crate::statement::function_like::FunctionLikeBody;

mod avoid_catching_error;
mod override_attribute;
mod unused_parameter;

pub fn check_function_like(
    metadata: &FunctionLikeMetadata,
    params: &[FunctionLikeParameter],
    body: FunctionLikeBody,
    ctx: &mut Context<'_>,
) {
    if !ctx.settings.perform_heuristic_checks {
        return;
    }

    unused_parameter::check_unused_params(metadata, params, body, ctx);
}

pub fn check_class_like(metadata: &ClassLikeMetadata, members: &[ClassLikeMember], ctx: &mut Context<'_>) {
    if !ctx.settings.perform_heuristic_checks {
        return;
    }

    override_attribute::check_override_attribute(metadata, members, ctx);
}

pub fn check_statement(stmt: &Statement, ctx: &mut Context<'_>) {
    if !ctx.settings.perform_heuristic_checks {
        return;
    }

    if let Statement::Try(r#try) = stmt {
        avoid_catching_error::check_for_caught_error(r#try, ctx);
    }
}

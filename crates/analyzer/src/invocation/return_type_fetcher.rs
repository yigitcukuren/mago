use ahash::HashMap;

use mago_codex::ttype::get_mixed_any;
use mago_codex::ttype::template::TemplateResult;
use mago_codex::ttype::union::TUnion;
use mago_interner::StringIdentifier;

use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::invocation::Invocation;
use crate::invocation::resolver::resolve_invocation_type;
use crate::invocation::special_function_like_handler::handle_special_functions;

pub fn fetch_invocation_return_type<'a>(
    context: &mut Context<'a>,
    block_context: &BlockContext<'a>,
    artifacts: &mut AnalysisArtifacts,
    invocation: &Invocation<'_>,
    template_result: &TemplateResult,
    parameters: &HashMap<StringIdentifier, TUnion>,
) -> TUnion {
    if let Some(return_type) = handle_special_functions(context, block_context, artifacts, invocation) {
        return return_type;
    }

    let Some(return_type) = invocation.target.get_return_type().cloned() else {
        return get_mixed_any();
    };

    resolve_invocation_type(context, invocation, template_result, parameters, return_type)
}

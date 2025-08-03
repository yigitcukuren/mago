use mago_syntax::ast::ArgumentList;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl Analyzable for ArgumentList {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        let was_inside_call = block_context.inside_call;
        let was_inside_general_use = block_context.inside_general_use;

        block_context.inside_call = true;
        block_context.inside_general_use = true;

        for argument in self.arguments.iter() {
            argument.value().analyze(context, block_context, artifacts)?;
        }

        block_context.inside_call = was_inside_call;
        block_context.inside_general_use = was_inside_general_use;

        Ok(())
    }
}

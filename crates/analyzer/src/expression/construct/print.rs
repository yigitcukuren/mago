use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_literal_int;
use mago_codex::ttype::union::TUnion;
use mago_syntax::ast::*;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::common::construct::ConstructInput;
use crate::common::construct::analyze_construct_inputs;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl Analyzable for PrintConstruct {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        analyze_construct_inputs(
            context,
            block_context,
            artifacts,
            "print",
            self.print.span,
            ConstructInput::Expression(&self.value),
            TUnion::new(vec![TAtomic::Scalar(TScalar::string())]),
            false,
            false,
            true,
        )?;

        artifacts.set_expression_type(self, get_literal_int(1));

        Ok(())
    }
}

use mago_syntax::ast::*;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

mod empty;
mod eval;
mod exit;
mod include;
mod isset;
mod print;

impl Analyzable for Construct {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        match self {
            Construct::Isset(isset_construct) => isset_construct.analyze(context, block_context, artifacts),
            Construct::Include(construct) => construct.analyze(context, block_context, artifacts),
            Construct::IncludeOnce(construct) => construct.analyze(context, block_context, artifacts),
            Construct::Require(construct) => construct.analyze(context, block_context, artifacts),
            Construct::RequireOnce(construct) => construct.analyze(context, block_context, artifacts),
            Construct::Print(print_construct) => print_construct.analyze(context, block_context, artifacts),
            Construct::Exit(exit_construct) => exit_construct.analyze(context, block_context, artifacts),
            Construct::Die(die_construct) => die_construct.analyze(context, block_context, artifacts),
            Construct::Eval(eval_construct) => eval_construct.analyze(context, block_context, artifacts),
            Construct::Empty(empty_construct) => empty_construct.analyze(context, block_context, artifacts),
        }
    }
}

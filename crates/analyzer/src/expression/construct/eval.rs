use mago_codex::ttype::atomic::TAtomic;
use mago_codex::ttype::atomic::scalar::TScalar;
use mago_codex::ttype::get_mixed;
use mago_codex::ttype::union::TUnion;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_syntax::ast::*;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::common::construct::ConstructInput;
use crate::common::construct::analyze_construct_inputs;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl Analyzable for EvalConstruct {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        if !context.settings.allow_eval {
            context.collector.report_with_code(
                IssueCode::DisallowedConstruct,
                Issue::error("Use of `eval` is disallowed by project configuration.")
                    .with_annotation(Annotation::primary(self.eval.span).with_message("`eval` is disallowed here"))
                    .with_note(
                        "Executing arbitrary strings with `eval` is extremely dangerous and a common source of security vulnerabilities like code injection."
                    )
                    .with_help(
                        "The best practice is to refactor your code to avoid dynamic execution. If it is essential, consider using a sandboxed environment like a Lua or WebAssembly (WASM) runtime instead of `eval`."
                    ),
            );
        }

        analyze_construct_inputs(
            context,
            block_context,
            artifacts,
            "eval",
            self.eval.span,
            ConstructInput::Expression(&self.value),
            TUnion::new(vec![TAtomic::Scalar(TScalar::string())]),
            false, // is_variadic
            false, // is_optional
            true,  // has_side_effects
        )?;

        artifacts.set_expression_type(self, get_mixed());

        Ok(())
    }
}

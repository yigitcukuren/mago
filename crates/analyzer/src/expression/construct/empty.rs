use mago_codex::ttype::get_bool;
use mago_codex::ttype::get_false;
use mago_codex::ttype::get_true;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_syntax::ast::*;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::IssueCode;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::error::AnalysisError;

impl Analyzable for EmptyConstruct {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        if !context.settings.allow_empty {
            context.collector.report_with_code(
                IssueCode::DisallowedConstruct,
                Issue::error("Use of `empty` is disallowed by project configuration.")
                    .with_annotation(Annotation::primary(self.empty.span).with_message("`empty` is disallowed here"))
                    .with_note(
                        "The `empty()` construct uses loose comparisons and treats various values like `0`, `false`, `'0'`, and `[]` as 'empty', which can hide bugs and lead to unexpected behavior."
                    )
                    .with_help(
                        "For clarity and safety, use a strict comparison instead, such as `!== null`, `!== []`, or `!== ''`."
                    ),
            );
        }

        let was_inside_isset = block_context.inside_isset;
        block_context.inside_isset = true;
        self.value.analyze(context, block_context, artifacts)?;
        block_context.inside_isset = was_inside_isset;

        artifacts.set_expression_type(
            self,
            match artifacts.get_expression_type(&self.value) {
                Some(value_type) if value_type.is_always_truthy() => get_true(),
                Some(value_type) if value_type.is_always_falsy() => get_false(),
                _ => get_bool(),
            },
        );

        Ok(())
    }
}

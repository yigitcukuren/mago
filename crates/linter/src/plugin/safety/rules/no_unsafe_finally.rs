use mago_ast::*;
use mago_ast_utils::control_flow::find_control_flows_in_block;
use mago_ast_utils::control_flow::ControlFlow;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoUnsafeFinallyRule;

impl Rule for NoUnsafeFinallyRule {
    fn get_name(&self) -> &'static str {
        "no-unsafe-finally"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Error)
    }
}

impl<'a> Walker<LintContext<'a>> for NoUnsafeFinallyRule {
    fn walk_in_try(&self, r#try: &Try, context: &mut LintContext<'a>) {
        let Some(finally) = r#try.finally_clause.as_ref() else {
            return;
        };

        for control_flow in find_control_flows_in_block(&finally.block) {
            let kind = match control_flow {
                ControlFlow::Return(_) => "return",
                ControlFlow::Throw(_) => "throw",
                ControlFlow::Continue(_) => "continue",
                ControlFlow::Break(_) => "break",
            };

            let issue = Issue::new(context.level(), "Unsafe control flow in finally block.")
                .with_annotation(
                    Annotation::primary(control_flow.span())
                        .with_message(format!("Control flow statement `{}` in `finally` block.", kind)),
                )
                .with_annotation(
                    Annotation::secondary(r#try.span())
                        .with_message("This `finally` block is associated with this `try` block."),
                )
                .with_note(
                    "Control flow statements in `finally` blocks override control flows from `try` and `catch` blocks, leading to unexpected behavior.",
                )
                .with_help("Avoid using control flow statements in `finally` blocks.");

            context.report(issue);
        }
    }
}

use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoEmptyLoopRule;

impl Rule for NoEmptyLoopRule {
    fn get_name(&self) -> &'static str {
        "no-empty-loop"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Note)
    }
}

impl NoEmptyLoopRule {
    fn report(&self, r#loop: impl HasSpan, context: &mut LintContext<'_>) {
        let loop_span = r#loop.span();

        let issue = Issue::new(context.level(), "loop body is empty")
            .with_annotations([Annotation::primary(loop_span)
                .with_message("this loop body is empty and does not perform any actions.")])
            .with_help("consider removing this loop or adding meaningful logic to its body.");

        context.report_with_fix(issue, |plan| {
            plan.delete(loop_span.to_range(), SafetyClassification::PotentiallyUnsafe);
        });
    }
}

impl<'a> Walker<LintContext<'a>> for NoEmptyLoopRule {
    fn walk_in_foreach(&self, foreach: &Foreach, context: &mut LintContext<'a>) {
        let is_empty = match &foreach.body {
            ForeachBody::Statement(stmt) => is_statement_empty(stmt),
            ForeachBody::ColonDelimited(body) => are_statements_empty(body.statements.as_slice()),
        };

        if is_empty {
            self.report(foreach.span(), context);
        }
    }

    fn walk_in_for(&self, for_loop: &For, context: &mut LintContext<'a>) {
        let is_empty = match &for_loop.body {
            ForBody::Statement(stmt) => is_statement_empty(stmt),
            ForBody::ColonDelimited(body) => are_statements_empty(body.statements.as_slice()),
        };

        if is_empty {
            self.report(for_loop.span(), context);
        }
    }

    fn walk_in_while(&self, while_loop: &While, context: &mut LintContext<'a>) {
        let is_empty = match &while_loop.body {
            WhileBody::Statement(stmt) => is_statement_empty(stmt),
            WhileBody::ColonDelimited(body) => are_statements_empty(body.statements.as_slice()),
        };

        if is_empty {
            self.report(while_loop.span(), context);
        }
    }

    fn walk_in_do_while(&self, do_while: &DoWhile, context: &mut LintContext<'a>) {
        let is_empty = is_statement_empty(&do_while.statement);

        if is_empty {
            self.report(do_while.span(), context);
        }
    }
}

fn is_statement_empty(statement: &Statement) -> bool {
    match statement {
        Statement::Block(block) => are_statements_empty(block.statements.as_slice()),
        Statement::Noop(_) => true,
        _ => false,
    }
}

fn are_statements_empty(statements: &[Statement]) -> bool {
    statements.is_empty() || statements.iter().all(is_statement_empty)
}

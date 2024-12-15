use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RedundantContinueRule;

impl Rule for RedundantContinueRule {
    fn get_name(&self) -> &'static str {
        "redundant-continue"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Help)
    }
}

impl RedundantContinueRule {
    fn report(&self, r#continue: &Continue, r#loop: impl HasSpan, context: &mut LintContext<'_>) {
        let issue = Issue::new(context.level(), "Redundant continue statement in loop body.")
            .with_annotations([
                Annotation::primary(r#continue.span()).with_message(
                    "This `continue` statement is redundant because it is the last statement in the loop body.",
                ),
                Annotation::secondary(r#loop.span()),
            ])
            .with_help("Remove this `continue` statement, as it does not affect the loop's behavior.");

        context.report_with_fix(issue, |plan| {
            plan.delete(r#continue.span().to_range(), SafetyClassification::Safe);
        });
    }
}

impl<'a> Walker<LintContext<'a>> for RedundantContinueRule {
    fn walk_in_foreach(&self, foreach: &Foreach, context: &mut LintContext<'a>) {
        if let Some(cont) = match &foreach.body {
            ForeachBody::Statement(stmt) => statement_is_continue(stmt),
            ForeachBody::ColonDelimited(body) => statements_end_with_continue(body.statements.as_slice()),
        } {
            self.report(cont, foreach, context);
        }
    }

    fn walk_in_for(&self, r#for: &For, context: &mut LintContext<'a>) {
        if let Some(cont) = match &r#for.body {
            ForBody::Statement(stmt) => statement_is_continue(stmt),
            ForBody::ColonDelimited(body) => statements_end_with_continue(body.statements.as_slice()),
        } {
            self.report(cont, r#for, context);
        }
    }

    fn walk_in_while(&self, r#while: &While, context: &mut LintContext<'a>) {
        if let Some(cont) = match &r#while.body {
            WhileBody::Statement(stmt) => statement_is_continue(stmt),
            WhileBody::ColonDelimited(body) => statements_end_with_continue(body.statements.as_slice()),
        } {
            self.report(cont, r#while, context);
        }
    }

    fn walk_in_do_while(&self, do_while: &DoWhile, context: &mut LintContext<'a>) {
        if let Some(cont) = statement_is_continue(&do_while.statement) {
            self.report(cont, do_while, context);
        }
    }
}

fn statements_end_with_continue(statements: &[Statement]) -> Option<&Continue> {
    let last = statements.last()?;

    statement_is_continue(last)
}

fn statement_is_continue(statement: &Statement) -> Option<&Continue> {
    match statement {
        Statement::Block(block) => statement_is_continue(block.statements.last()?),
        Statement::Continue(cont) => match cont.level {
            None | Some(Expression::Literal(Literal::Integer(LiteralInteger { value: Some(1), .. }))) => Some(cont),
            Some(_) => None,
        },
        _ => None,
    }
}

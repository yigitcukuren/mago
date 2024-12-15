use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RedudnantClosingTagRule;

impl RedudnantClosingTagRule {
    fn report(sequence: &Sequence<Statement>, context: &mut LintContext<'_>) {
        let Some(last_statement) = sequence.last() else {
            return;
        };

        if let Statement::ClosingTag(closing_tag) = last_statement {
            let issue = Issue::new(context.level(), "Redundant closing tag ( `?>` ).")
                .with_annotation(Annotation::primary(closing_tag.span()).with_message("This closing tag is redundant."))
                .with_help("Remove the redundant closing tag ( `?>` ).");

            context
                .report_with_fix(issue, |plan| plan.delete(closing_tag.span().to_range(), SafetyClassification::Safe));

            return;
        }

        if let Statement::Inline(inline) = last_statement {
            let stmts_len = sequence.len();
            if stmts_len < 2 {
                return;
            }

            let value = context.interner.lookup(&inline.value);
            if value.bytes().all(|b| b.is_ascii_whitespace()) {
                let Some(Statement::ClosingTag(tag)) = sequence.get(stmts_len - 2) else {
                    return;
                };

                let issue =
                    Issue::new(context.level(), "Redundant closing tag ( `?>` ) followed by trailing whitespace.")
                        .with_annotation(Annotation::primary(tag.span()).with_message("This closing tag is redundant."))
                        .with_annotation(
                            Annotation::secondary(inline.span())
                                .with_message("This inline statement is contains only whitespace."),
                        )
                        .with_help("Remove the redundant closing tag ( `?>` ) and trailing whitespace.");

                context.report_with_fix(issue, |plan| {
                    plan.delete(inline.span().to_range(), SafetyClassification::Safe);
                    plan.delete(tag.span().to_range(), SafetyClassification::Safe);
                });
            }
        }

        if let Statement::Namespace(namespace) = last_statement {
            match &namespace.body {
                NamespaceBody::Implicit(namespace_implicit_body) => {
                    Self::report(&namespace_implicit_body.statements, context);
                }
                NamespaceBody::BraceDelimited(_) => {}
            }
        }
    }
}

impl Rule for RedudnantClosingTagRule {
    fn get_name(&self) -> &'static str {
        "redundant-closing-tag"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Help)
    }
}

impl<'a> Walker<LintContext<'a>> for RedudnantClosingTagRule {
    fn walk_program<'ast>(&self, program: &'ast Program, context: &mut LintContext<'a>) {
        Self::report(&program.statements, context);
    }
}

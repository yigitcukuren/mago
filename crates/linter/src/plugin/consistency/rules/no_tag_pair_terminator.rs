use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoTagPairTerminatorRule;

impl Rule for NoTagPairTerminatorRule {
    fn get_name(&self) -> &'static str {
        "no-tag-pair-terminator"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Note)
    }
}

impl<'a> Walker<LintContext<'a>> for NoTagPairTerminatorRule {
    fn walk_terminator<'ast>(&self, terminator: &'ast Terminator, context: &mut LintContext<'a>) {
        let Terminator::TagPair(close, open) = terminator else {
            return;
        };

        let issue = Issue::new(context.level(), "Semicolon terminator is preferred over tag-pair terminator")
            .with_annotation(
                Annotation::primary(close.span().join(open.span()))
                    .with_message("This tag-pair terminator `?><?php` is not recommended."),
            )
            .with_help("Replace `?><?php` with a `;`");

        context.report_with_fix(issue, |plan| {
            plan.replace(close.span().join(open.span()).to_range(), ";", SafetyClassification::Safe)
        });
    }
}

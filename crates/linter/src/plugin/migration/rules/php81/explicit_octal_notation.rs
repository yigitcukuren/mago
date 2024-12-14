use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct ExplicitOctalNotationRule;

impl Rule for ExplicitOctalNotationRule {
    fn get_name(&self) -> &'static str {
        "explicit-octal-notation"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for ExplicitOctalNotationRule {
    fn walk_in_literal_integer(&self, literal_integer: &LiteralInteger, context: &mut LintContext<'a>) {
        let literal_text = context.lookup(&literal_integer.raw);
        if !literal_text.starts_with('0') {
            return;
        }

        if !literal_text.as_bytes().get(1).copied().is_some_and(|c| {
            // check for `0o`, `0x`, or `0b` prefix
            c != b'o' && c != b'O' && c != b'x' && c != b'X' && c != b'b' && c != b'B'
        }) {
            return;
        }

        let issue = Issue::new(context.level(), "use explicit octal numeral notation")
            .with_annotation(
                Annotation::primary(literal_integer.span())
                    .with_message("implicit octal numeral notation")
            )
            .with_note("using `0o` makes the octal intent explicit and avoids confusion with other formats.")
            .with_help("replace the leading `0` with `0o` to make the octal intent explicit")
            .with_link("https://www.php.net/manual/en/migration81.new-features.php#migration81.new-features.core.octal-literal-prefix")
        ;

        let replacement = format!("0o{}", &literal_text[1..]);

        context.report_with_fix(issue, |plan| {
            plan.replace(literal_integer.span().to_range(), replacement, SafetyClassification::Safe);
        });
    }
}

use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct MissingAssertDescriptionRule;

impl Rule for MissingAssertDescriptionRule {
    #[inline]
    fn get_name(&self) -> &'static str {
        "missing-assert-description"
    }

    #[inline]
    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for MissingAssertDescriptionRule {
    fn walk_in_function_call(&self, function_call: &FunctionCall, context: &mut LintContext<'a>) {
        let Expression::Identifier(identifier) = function_call.function.as_ref() else {
            return;
        };

        let function_name = context.resolve_function_name(identifier);
        // we only care about the "assert" function
        if !function_name.eq_ignore_ascii_case("assert") {
            return;
        }

        if function_call.arguments.arguments.get(1).is_none() {
            let issue = Issue::new(context.level(), "Missing description in assert function.")
                .with_annotation(Annotation::primary(function_call.span()).with_message("`assert` function is called here."))
                .with_help("Add a description to the assert function to make it easier to understand the purpose of the assertion.");

            context.report(issue);
        }
    }
}

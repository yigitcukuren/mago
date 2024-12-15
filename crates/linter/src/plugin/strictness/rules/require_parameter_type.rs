use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RequireParameterTypeRule;

impl Rule for RequireParameterTypeRule {
    #[inline]
    fn get_name(&self) -> &'static str {
        "require-parameter-type"
    }

    #[inline]
    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for RequireParameterTypeRule {
    fn walk_in_function_like_parameter<'ast>(
        &self,
        function_like_parameter: &'ast FunctionLikeParameter,
        context: &mut LintContext<'a>,
    ) {
        if function_like_parameter.hint.is_some() {
            return;
        }

        let parameter_name = context.lookup(&function_like_parameter.variable.name);

        context.report(
            Issue::new(context.level(), format!("Parameter `{}` is missing a type hint.", parameter_name))
                .with_annotation(
                    Annotation::primary(function_like_parameter.span())
                        .with_message(format!("Parameter `{}` is declared here", parameter_name)),
                )
                .with_note("Type hints improve code readability and help prevent type-related errors.")
                .with_help(format!("Consider adding a type hint to parameter `{}`.", parameter_name)),
        );
    }
}

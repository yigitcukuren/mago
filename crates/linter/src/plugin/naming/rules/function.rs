use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct FunctionRule;

impl Rule for FunctionRule {
    fn get_name(&self) -> &'static str {
        "function"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Help)
    }
}

impl Walker<LintContext<'_>> for FunctionRule {
    fn walk_in_function(&self, function: &Function, context: &mut LintContext) {
        let name = context.lookup(&function.name.value);
        let fqfn = context.lookup_name(&function.name);
        let camel_case = context.option("camel").and_then(|v| v.as_bool()).unwrap_or(false);
        let either_case = context.option("either").and_then(|v| v.as_bool()).unwrap_or(false);

        if either_case {
            if !mago_casing::is_camel_case(name) && !mago_casing::is_snake_case(name) {
                context.report(
                    Issue::new(
                        context.level(),
                        format!("Function name `{}` should be in either camel case or snake case.", name),
                    )
                    .with_annotation(
                        Annotation::primary(function.name.span())
                            .with_message(format!("Function `{}` is declared here.`", name)),
                    )
                    .with_annotation(
                        Annotation::secondary(function.span())
                            .with_message(format!("Function `{}` is defined here.", fqfn)),
                    )
                    .with_note(format!(
                        "The function name `{}` does not follow either camel case or snake naming convention.",
                        name
                    ))
                    .with_help(format!(
                        "Consider renaming it to `{}` or `{}` to adhere to the naming convention.",
                        mago_casing::to_camel_case(name),
                        mago_casing::to_snake_case(name)
                    )),
                );
            }

            return;
        }

        if camel_case {
            if !mago_casing::is_camel_case(name) {
                context.report(
                    Issue::new(context.level(), format!("Function name `{}` should be in camel case.", name))
                        .with_annotation(
                            Annotation::primary(function.name.span())
                                .with_message(format!("Function `{}` is declared here.`", name)),
                        )
                        .with_annotation(
                            Annotation::secondary(function.span())
                                .with_message(format!("Function `{}` is defined here.", fqfn)),
                        )
                        .with_note(format!("The function name `{}` does not follow camel naming convention.", name))
                        .with_help(format!(
                            "Consider renaming it to `{}` to adhere to the naming convention.",
                            mago_casing::to_camel_case(name)
                        )),
                );
            }

            return;
        }

        if !mago_casing::is_snake_case(name) {
            context.report(
                Issue::new(context.level(), format!("Function name `{}` should be in snake case.", name))
                    .with_annotation(
                        Annotation::primary(function.name.span())
                            .with_message(format!("Function `{}` is declared here.`", name)),
                    )
                    .with_annotation(
                        Annotation::secondary(function.span())
                            .with_message(format!("Function `{}` is defined here.", fqfn)),
                    )
                    .with_note(format!("The function name `{}` does not follow snake naming convention.", name))
                    .with_help(format!(
                        "Consider renaming it to `{}` to adhere to the naming convention.",
                        mago_casing::to_snake_case(name)
                    )),
            );
        }
    }
}

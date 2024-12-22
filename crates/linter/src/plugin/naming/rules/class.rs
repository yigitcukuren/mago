use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct ClassRule;

impl Rule for ClassRule {
    fn get_name(&self) -> &'static str {
        "class"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Help)
    }
}

impl<'a> Walker<LintContext<'a>> for ClassRule {
    fn walk_in_class<'ast>(&self, class: &'ast Class, context: &mut LintContext<'a>) {
        let mut issues = vec![];

        let name = context.lookup(&class.name.value);

        if !mago_casing::is_class_case(name) {
            let issue = Issue::new(context.level(), format!("Class name `{}` should be in class case.", name))
                .with_annotations([
                    Annotation::primary(class.name.span()).with_message(format!("Class `{}` is declared here.", name))
                ])
                .with_note(format!("The class name `{}` does not follow class naming convention.", name))
                .with_help(format!(
                    "Consider renaming it to `{}` to adhere to the naming convention.",
                    mago_casing::to_class_case(name)
                ));

            issues.push(issue);
        }

        if class.modifiers.contains_abstract()
            && context.option("psr").and_then(|o| o.as_bool()).unwrap_or(true)
            && !name.starts_with("Abstract")
        {
            let suggested_name = format!("Abstract{}", mago_casing::to_class_case(name));

            issues.push(
                Issue::new(
                    context.level(),
                    format!("Abstract class name `{}` should be prefixed with `Abstract`.", name),
                )
                .with_annotations([
                    Annotation::primary(class.name.span()).with_message(format!("Class `{}` is declared here.", name))
                ])
                .with_note(format!("The abstract class name `{}` does not follow PSR naming convention.", name))
                .with_help(format!("Consider renaming it to `{}` to adhere to the naming convention.", suggested_name)),
            );
        }

        for issue in issues {
            context.report(issue);
        }
    }
}

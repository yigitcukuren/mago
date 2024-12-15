use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RequirePropertyTypeRule;

impl Rule for RequirePropertyTypeRule {
    #[inline]
    fn get_name(&self) -> &'static str {
        "require-property-type"
    }

    #[inline]
    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for RequirePropertyTypeRule {
    fn walk_in_property<'ast>(&self, property: &'ast Property, context: &mut LintContext<'a>) {
        if property.hint().is_some() {
            return;
        }

        let name = context.lookup(&property.first_variable().name);

        context.report(
            Issue::new(context.level(), format!("Property `{}` is missing a type hint.", name))
                .with_annotation(
                    Annotation::primary(property.span()).with_message(format!("Property `{}` is declared here.", name)),
                )
                .with_note("Adding a type hint to properties improves code readability and helps prevent type errors.")
                .with_help(format!("Consider specifying a type hint for `{}`.", name)),
        );
    }
}

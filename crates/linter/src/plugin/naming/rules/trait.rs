use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct TraitRule;

impl Rule for TraitRule {
    fn get_name(&self) -> &'static str {
        "trait"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Help)
    }
}

impl<'a> Walker<LintContext<'a>> for TraitRule {
    fn walk_in_trait<'ast>(&self, r#trait: &'ast Trait, context: &mut LintContext<'a>) {
        let mut issues = vec![];

        let name = context.lookup(&r#trait.name.value);
        let fqcn = context.lookup_name(&r#trait.name);

        if !mago_casing::is_class_case(name) {
            issues.push(
                Issue::new(context.level(), format!("trait name `{}` should be in class case", name))
                    .with_annotations([
                        Annotation::primary(r#trait.name.span()),
                        Annotation::secondary(r#trait.span())
                            .with_message(format!("trait `{}` is declared here", fqcn)),
                    ])
                    .with_note(format!("the trait name `{}` does not follow class naming convention.", name))
                    .with_help(format!(
                        "consider renaming it to `{}` to adhere to the naming convention.",
                        mago_casing::to_class_case(name)
                    )),
            );
        }

        if context.option("psr").and_then(|o| o.as_bool()).unwrap_or(true) && !name.ends_with("Trait") {
            issues.push(
                Issue::new(context.level(), format!("trait name `{}` should be suffixed with `Trait`", name))
                    .with_annotations([
                        Annotation::primary(r#trait.name.span()),
                        Annotation::secondary(r#trait.span())
                            .with_message(format!("trait `{}` is declared here", fqcn)),
                    ])
                    .with_note(format!("the trait name `{}` does not follow PSR naming convention.", name))
                    .with_help(format!("consider renaming it to `{}Trait` to adhere to the naming convention.", name)),
            );
        }

        for issue in issues {
            context.report(issue);
        }
    }
}

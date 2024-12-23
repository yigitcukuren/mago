use mago_ast::ast::*;
use mago_reflection::class_like::ClassLikeReflection;
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

impl RequirePropertyTypeRule {
    fn report(reflection: &ClassLikeReflection, members: &[ClassLikeMember], context: &mut LintContext<'_>) {
        for member in members {
            let ClassLikeMember::Property(property) = member else {
                continue;
            };

            if property.hint().is_some() {
                continue;
            }

            for variable in property.variables() {
                let Some(property_reflection) = reflection.get_property(&variable.name) else {
                    continue;
                };

                if property_reflection.is_overriding {
                    // This property is overriding a method from a parent class.
                    continue;
                }

                let name = context.lookup(&variable.name);

                context.report(
                    Issue::new(context.level(), format!("Property `{}` is missing a type hint.", name))
                        .with_annotation(
                            Annotation::primary(property.span())
                                .with_message(format!("Property `{}` is declared here.", name)),
                        )
                        .with_note(
                            "Adding a type hint to properties improves code readability and helps prevent type errors.",
                        )
                        .with_help(format!("Consider specifying a type hint for `{}`.", name)),
                );
            }
        }
    }
}

impl<'a> Walker<LintContext<'a>> for RequirePropertyTypeRule {
    fn walk_in_class(&self, class: &Class, context: &mut LintContext<'a>) {
        let name = context.semantics.names.get(&class.name);
        let Some(reflection) = context.codebase.get_class(context.interner, name) else {
            return;
        };

        Self::report(reflection, class.members.as_slice(), context);
    }

    fn walk_in_trait(&self, r#trait: &Trait, context: &mut LintContext<'a>) {
        let name = context.semantics.names.get(&r#trait.name);
        let Some(reflection) = context.codebase.get_trait(context.interner, name) else {
            return;
        };

        Self::report(reflection, r#trait.members.as_slice(), context);
    }
}

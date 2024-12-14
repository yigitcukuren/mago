use mago_ast::*;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct UnderscoreClassNameRule;

impl Rule for UnderscoreClassNameRule {
    fn get_name(&self) -> &'static str {
        "underscore-class-name"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for UnderscoreClassNameRule {
    fn walk_in_class(&self, class: &Class, context: &mut LintContext<'a>) {
        let class_name = context.lookup(&class.name.value);
        if class_name != "_" {
            return;
        }

        let issue = Issue::new(context.level(), "using `_` as a class name is deprecated")
            .with_annotation(
                Annotation::primary(class.name.span()).with_message("rename the class to something more descriptive"),
            )
            .with_note("class names consisting only of `_` are deprecated. consider using a meaningful name.");

        context.report(issue);
    }

    fn walk_in_interface(&self, interface: &Interface, context: &mut LintContext<'a>) {
        let interface_name = context.lookup(&interface.name.value);
        if interface_name != "_" {
            return;
        }

        let issue = Issue::new(context.level(), "using `_` as an interface name is deprecated")
            .with_annotation(
                Annotation::primary(interface.name.span())
                    .with_message("rename the interface to something more descriptive"),
            )
            .with_note("interface names consisting only of `_` are deprecated. consider using a meaningful name.");

        context.report(issue);
    }

    fn walk_in_trait(&self, r#trait: &Trait, context: &mut LintContext<'a>) {
        let trait_name = context.lookup(&r#trait.name.value);
        if trait_name != "_" {
            return;
        }

        let issue = Issue::new(context.level(), "using `_` as a trait name is deprecated")
            .with_annotation(
                Annotation::primary(r#trait.name.span()).with_message("rename the trait to something more descriptive"),
            )
            .with_note("trait names consisting only of `_` are deprecated. consider using a meaningful name.");

        context.report(issue);
    }

    fn walk_in_enum(&self, r#enum: &Enum, context: &mut LintContext<'a>) {
        let enum_name = context.lookup(&r#enum.name.value);
        if enum_name != "_" {
            return;
        }

        let issue = Issue::new(context.level(), "using `_` as an enum name is deprecated")
            .with_annotation(
                Annotation::primary(r#enum.name.span()).with_message("rename the enum to something more descriptive"),
            )
            .with_note("enum names consisting only of `_` are deprecated. consider using a meaningful name.");

        context.report(issue);
    }
}

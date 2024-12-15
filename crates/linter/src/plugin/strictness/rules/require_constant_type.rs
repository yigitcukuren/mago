use mago_ast::ast::*;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct RequireConstantTypeRule;

impl Rule for RequireConstantTypeRule {
    #[inline]
    fn get_name(&self) -> &'static str {
        "require-constant-type"
    }

    #[inline]
    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for RequireConstantTypeRule {
    fn walk_class_like_constant<'ast>(
        &self,
        class_like_constant: &'ast ClassLikeConstant,
        context: &mut LintContext<'a>,
    ) {
        if class_like_constant.hint.is_some() {
            return;
        }

        let item = class_like_constant.first_item();

        let constant_name = context.lookup(&item.name.value);

        context.report(
            Issue::new(context.level(), format!("Class constant `{}` is missing a type hint.", constant_name))
                .with_annotation(
                    Annotation::primary(class_like_constant.span())
                        .with_message(format!("Class constant `{}` is defined here.", constant_name)),
                )
                .with_note("Adding a type hint to constants improves code readability and helps prevent type errors.")
                .with_help(format!("Consider specifying a type hint for `{}`.", constant_name)),
        );
    }
}

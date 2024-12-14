use mago_ast::ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::*;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct ReturnByReferenceFromVoidFunctionRule;

impl Rule for ReturnByReferenceFromVoidFunctionRule {
    fn get_name(&self) -> &'static str {
        "return-by-reference-from-void-function"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for ReturnByReferenceFromVoidFunctionRule {
    fn walk_in_function(&self, function: &Function, context: &mut LintContext<'a>) {
        let Some(amperstand) = function.ampersand.as_ref() else {
            return;
        };

        let Some(return_type) = function.return_type_hint.as_ref() else {
            return;
        };

        if !matches!(return_type.hint, Hint::Void(_)) {
            return;
        }

        report(context, "function", function.span(), amperstand, false);
    }

    fn walk_in_method(&self, method: &Method, context: &mut LintContext<'a>) {
        let Some(amperstand) = method.ampersand.as_ref() else {
            return;
        };

        let Some(return_type) = method.return_type_hint.as_ref() else {
            return;
        };

        if !matches!(return_type.hint, Hint::Void(_)) {
            return;
        }

        report(context, "method", method.span(), amperstand, false);
    }

    fn walk_in_closure(&self, closure: &Closure, context: &mut LintContext<'a>) {
        let Some(amperstand) = closure.ampersand.as_ref() else {
            return;
        };

        let Some(return_type) = closure.return_type_hint.as_ref() else {
            return;
        };

        if !matches!(return_type.hint, Hint::Void(_)) {
            return;
        }

        report(context, "closure", closure.span(), amperstand, false);
    }

    fn walk_in_arrow_function(&self, arrow_function: &ArrowFunction, context: &mut LintContext<'a>) {
        let Some(amperstand) = arrow_function.ampersand.as_ref() else {
            return;
        };

        let Some(return_type) = arrow_function.return_type_hint.as_ref() else {
            return;
        };

        if !matches!(return_type.hint, Hint::Void(_)) {
            return;
        }

        report(context, "arrow function", arrow_function.span(), amperstand, false);
    }

    fn walk_in_property_hook(&self, property_hook: &PropertyHook, context: &mut LintContext<'a>) {
        let name = context.lookup(&property_hook.name.value);
        if "set" != name {
            return;
        }

        let Some(amperstand) = property_hook.ampersand.as_ref() else {
            return;
        };

        report(context, "set property hook", property_hook.span(), amperstand, true);
    }
}

fn report(
    context: &mut LintContext<'_>,
    kind: &'static str,
    span: Span,
    ampersand: &Span,
    is_set_hook: bool,
) {
    let message = if !is_set_hook {
        format!("returning by reference from a void {} is deprecated since PHP 8.0", kind)
    } else {
        "returning by reference from a set property hook is deprecated since PHP 8.0".to_string()
    };

    let issue = Issue::new(context.level(), message)
        .with_annotation(Annotation::primary(*ampersand).with_message("`&` indicating a return by reference"))
        .with_annotation(Annotation::secondary(span))
        .with_help("consider removing the `&` to comply with PHP 8.0 standards and avoid future issues.".to_string());

    context.report_with_fix(issue, |plan| {
        plan.delete(ampersand.to_range(), SafetyClassification::Safe);
    });
}

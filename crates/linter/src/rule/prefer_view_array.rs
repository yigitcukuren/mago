use indoc::indoc;
use serde::Deserialize;
use serde::Serialize;

use mago_php_version::PHPVersionRange;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::category::Category;
use crate::context::LintContext;
use crate::integration::Integration;
use crate::integration::IntegrationSet;
use crate::rule::Config;
use crate::rule::LintRule;
use crate::rule::utils::call::function_call_matches;
use crate::rule_meta::RuleMeta;
use crate::settings::RuleSettings;

#[derive(Debug, Clone)]
pub struct PreferViewArrayRule {
    meta: &'static RuleMeta,
    cfg: PreferViewArrayConfig,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case", deny_unknown_fields)]
pub struct PreferViewArrayConfig {
    pub level: Level,
}

impl Default for PreferViewArrayConfig {
    fn default() -> Self {
        Self { level: Level::Help }
    }
}

impl Config for PreferViewArrayConfig {
    fn level(&self) -> Level {
        self.level
    }
}

impl LintRule for PreferViewArrayRule {
    type Config = PreferViewArrayConfig;

    fn meta() -> &'static RuleMeta {
        const META: RuleMeta = RuleMeta {
            name: "Prefer View Array",
            code: "prefer-view-array",
            description: indoc! {"
                Prefer passing data to views using the array parameter in the `view()` function,
                rather than chaining the `with()` method.`

                Using the array parameter directly is more concise and readable.
            "},
            good_example: indoc! {"
                <?php

                return view('user.profile', [
                    'user' => $user,
                    'profile' => $profile,
                ]);
            "},
            bad_example: indoc! {"
                <?php

                return view('user.profile')->with([
                    'user' => $user,
                    'profile' => $profile,
                ]);
            "},
            category: Category::BestPractices,
            php: PHPVersionRange::any(),
            requires: IntegrationSet::only(Integration::Laravel),
        };

        &META
    }

    fn targets() -> &'static [NodeKind] {
        const TARGETS: &[NodeKind] = &[NodeKind::MethodCall];

        TARGETS
    }

    fn build(settings: RuleSettings<Self::Config>) -> Self {
        Self { meta: Self::meta(), cfg: settings.config }
    }

    fn check(&self, ctx: &mut LintContext, node: Node) {
        let Node::MethodCall(call @ MethodCall { object, method, .. }) = node else {
            return;
        };

        if !is_function_call_to(ctx, object.as_ref(), "view") || !is_method_named(ctx, method, "with") {
            return;
        }

        ctx.collector.report(
            Issue::new(self.cfg.level(), "Use array parameter in `view()` instead of chaining `with()`.")
                .with_code(self.meta.code)
                .with_annotation(
                    Annotation::primary(call.span())
                        .with_message("Chaining `with()` here is less readable and idiomatic"),
                )
                .with_note("Passing data directly as an array parameter to `view()` is preferred.")
                .with_help("Refactor the code to use the array parameter in the `view()` function."),
        );
    }
}

fn is_function_call_to(context: &LintContext, expression: &Expression, function_name: &str) -> bool {
    let Expression::Call(Call::Function(call)) = expression else {
        return false;
    };

    function_call_matches(context, call, function_name)
}

fn is_method_named(context: &LintContext, member: &ClassLikeMemberSelector, name: &str) -> bool {
    match member {
        ClassLikeMemberSelector::Identifier(method) => {
            context.interner.lookup(&method.value).eq_ignore_ascii_case(name)
        }
        _ => false,
    }
}

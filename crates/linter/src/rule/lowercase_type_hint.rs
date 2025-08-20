use indoc::indoc;
use serde::Deserialize;
use serde::Serialize;

use mago_php_version::PHPVersionRange;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::*;
use mago_syntax::ast::*;

use crate::category::Category;
use crate::context::LintContext;
use crate::integration::IntegrationSet;
use crate::rule::Config;
use crate::rule::LintRule;
use crate::rule_meta::RuleMeta;
use crate::settings::RuleSettings;

#[derive(Debug, Clone)]
pub struct LowercaseTypeHintRule {
    meta: &'static RuleMeta,
    cfg: LowercaseTypeHintConfig,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case", deny_unknown_fields)]
pub struct LowercaseTypeHintConfig {
    pub level: Level,
}

impl Default for LowercaseTypeHintConfig {
    fn default() -> Self {
        Self { level: Level::Help }
    }
}

impl Config for LowercaseTypeHintConfig {
    fn level(&self) -> Level {
        self.level
    }
}

impl LintRule for LowercaseTypeHintRule {
    type Config = LowercaseTypeHintConfig;

    fn meta() -> &'static RuleMeta {
        const META: RuleMeta = RuleMeta {
            name: "Lowercase Type Hint",
            code: "lowercase-type-hint",
            description: indoc! {"
                Enforces that PHP type hints (like `void`, `bool`, `int`, `float`, etc.) be written
                in lowercase. Using uppercase or mixed case is discouraged for consistency
                and readability.
            "},
            good_example: indoc! {r#"
                <?php

                function example(int $param): void {
                    return;
                }
            "#},
            bad_example: indoc! {r#"
                <?php

                function example(Int $param): VOID {
                    return;
                }
            "#},
            category: Category::Consistency,
            php: PHPVersionRange::any(),
            requires: IntegrationSet::empty(),
        };

        &META
    }

    fn targets() -> &'static [NodeKind] {
        const TARGETS: &[NodeKind] = &[NodeKind::Hint];

        TARGETS
    }

    fn build(settings: RuleSettings<Self::Config>) -> Self {
        Self { meta: Self::meta(), cfg: settings.config }
    }

    fn check(&self, ctx: &mut LintContext, node: Node) {
        let Node::Hint(hint) = node else {
            return;
        };

        let identifier = match hint {
            Hint::Void(identifier)
            | Hint::Never(identifier)
            | Hint::Float(identifier)
            | Hint::Bool(identifier)
            | Hint::Integer(identifier)
            | Hint::String(identifier)
            | Hint::Object(identifier)
            | Hint::Mixed(identifier)
            | Hint::Iterable(identifier) => identifier,
            _ => {
                return if hint.is_complex() {};
            }
        };

        let name = ctx.lookup(&identifier.value);
        let lowered = name.to_ascii_lowercase();
        if !lowered.eq(&name) {
            let issue = Issue::new(self.cfg.level(), format!("Type hint `{}` should be in lowercase.", name))
                .with_code(self.meta.code)
                .with_annotation(Annotation::primary(identifier.span()))
                .with_help(format!("Consider using `{}` instead of `{}`.", lowered, name));

            ctx.collector.report(issue);
        }
    }
}

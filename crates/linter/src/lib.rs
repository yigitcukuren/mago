use std::sync::Arc;

use mago_collector::Collector;
use mago_database::file::File;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;
use mago_php_version::PHPVersion;
use mago_reporting::IssueCollection;
use mago_syntax::ast::Node;
use mago_syntax::ast::Program;

use crate::context::LintContext;
use crate::legacy_rule_mappings::LEGACY_RULE_CODE_MAPPINGS;
use crate::registry::RuleRegistry;
use crate::rule::AnyRule;
use crate::scope::Scope;
use crate::settings::Settings;

pub mod category;
pub mod context;
pub mod integration;
pub mod legacy_rule_mappings;
pub mod registry;
pub mod rule;
pub mod rule_meta;
pub mod scope;
pub mod settings;

const COLLECTOR_CATEGORY: &str = "lint";

#[derive(Debug, Clone)]
pub struct Linter {
    interner: ThreadedInterner,
    registry: Arc<RuleRegistry>,
    php_version: PHPVersion,
}

impl Linter {
    /// Creates a new Linter instance.
    ///
    /// # Arguments
    ///
    /// * `only` - If `Some`, only the rules with the specified codes will be loaded.
    ///   If `None`, all rules enabled by the settings will be loaded.
    pub fn new(interner: ThreadedInterner, settings: Settings, only: Option<&[String]>) -> Self {
        Self { interner, php_version: settings.php_version, registry: Arc::new(RuleRegistry::build(settings, only)) }
    }

    pub fn rules(&self) -> &[AnyRule] {
        self.registry.rules()
    }

    pub fn lint(&self, source_file: &File, program: &Program, resolved_names: &ResolvedNames) -> IssueCollection {
        let mut collector = Collector::new(source_file, program, &self.interner, COLLECTOR_CATEGORY);

        // Set legacy rule code mappings for compatibility with the old linter.
        collector.set_aliases(LEGACY_RULE_CODE_MAPPINGS);

        let mut context = LintContext::new(self.php_version, &self.interner, source_file, resolved_names, collector);

        walk(Node::Program(program), &mut context, &self.registry);

        context.collector.finish()
    }
}

fn walk<'a>(node: Node<'a>, ctx: &mut LintContext<'a>, reg: &RuleRegistry) {
    let mut in_scope = false;
    if let Some(scope) = Scope::for_node(ctx, node) {
        ctx.scope.push(scope);

        in_scope = true;
    }

    let rules_to_run = reg.for_kind(node.kind());

    for &rule_index in rules_to_run {
        let rule = reg.rule(rule_index);

        rule.check(ctx, node);
    }

    for child in node.children() {
        walk(child, ctx, reg);
    }

    if in_scope {
        ctx.scope.pop();
    }
}

use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;

use mago_codex::metadata::CodebaseMetadata;
use mago_collector::Collector;
use mago_database::file::File;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;
use mago_reporting::IssueCollection;
use mago_reporting::Level;
use mago_syntax::ast::Node;
use mago_syntax::ast::Program;

use crate::ast::PreComputedNode;
use crate::context::LintContext;
use crate::plugin::Plugin;
use crate::rule::ConfiguredRule;
use crate::rule::Rule;
use crate::settings::RuleSettings;
use crate::settings::Settings;

pub mod consts;
pub mod context;
pub mod definition;
pub mod directive;
pub mod plugin;
pub mod rule;
pub mod scope;
pub mod settings;

mod ast;

const COLLECTOR_CATEGORY: &str = "lint";

/// The main linter instance that orchestrates the linting process.
///
/// This struct holds the configuration, registered rules, and shared data
/// required to analyze source files.
#[derive(Debug, Clone)]
pub struct Linter {
    settings: Settings,
    interner: ThreadedInterner,
    rules: Arc<RwLock<Vec<ConfiguredRule>>>,
}

impl Linter {
    /// Creates a new linter with the given configuration.
    ///
    /// # Parameters
    ///
    /// - `settings`: The settings to use for the linter.
    /// - `interner`: The interner to use, typically shared with the parser.
    /// - `codebase`: The codebase metadata for project-wide analysis.
    pub fn new(settings: Settings, interner: ThreadedInterner) -> Self {
        Self { settings, interner, rules: Arc::new(RwLock::new(Vec::new())) }
    }

    /// Creates a new linter and enables all available default plugins.
    ///
    /// This is a convenience constructor for quickly setting up a linter with a
    /// comprehensive set of rules.
    pub fn with_all_plugins(settings: Settings, interner: ThreadedInterner) -> Self {
        let mut linter = Self::new(settings, interner);
        crate::foreach_plugin!(|plugin| linter.add_plugin(plugin));
        linter
    }

    /// Registers a plugin and configures its rules based on the linter's settings.
    pub fn add_plugin(&mut self, plugin: impl Plugin) {
        let plugin_definition = plugin.get_definition();
        let plugin_slug = plugin_definition.get_slug();

        let is_explicitly_enabled = self.settings.plugins.iter().any(|p| p.eq_ignore_ascii_case(&plugin_slug));
        let is_default_enabled = self.settings.default_plugins && plugin_definition.enabled_by_default;

        if !is_explicitly_enabled && !is_default_enabled {
            return;
        }

        for rule in plugin.get_rules() {
            self.add_rule(&plugin_slug, rule);
        }
    }

    /// Registers a single rule from a plugin if it is supported and enabled.
    pub fn add_rule(&mut self, plugin_slug: impl Into<String>, rule: Box<dyn Rule>) {
        let rule_definition = rule.get_definition();
        let plugin_slug = plugin_slug.into();
        let slug = format!("{}/{}", plugin_slug, rule_definition.get_slug());

        tracing::trace!("Initializing rule `{slug}`...");

        // Skip rule if it doesn't support the configured PHP version.
        if !rule_definition.supports_php_version(self.settings.php_version) {
            tracing::trace!("Rule `{slug}` skipped due to PHP version mismatch.");
            return;
        }

        let settings = self
            .settings
            .get_rule_settings(&slug)
            .cloned()
            .unwrap_or_else(|| RuleSettings::from_level(rule_definition.level));

        if !settings.enabled {
            tracing::trace!("Rule `{slug}` has been disabled.");
            return;
        }

        // Determine the final severity level for the rule.
        let level = match settings.level.or(rule_definition.level) {
            Some(level) => level,
            None => {
                tracing::trace!("Rule `{slug}` is disabled because no level is configured.");
                return;
            }
        };

        tracing::trace!("Rule `{slug}` is enabled with level `{level}`.");
        self.rules.write().expect("Unable to add rule: poisoned lock").push(ConfiguredRule {
            slug,
            level,
            settings,
            rule,
        });
    }

    /// Returns a read-only view of the currently configured rules.
    ///
    /// # Panics
    ///
    /// This method will panic if the underlying `RwLock` is poisoned.
    pub fn get_configured_rules(&self) -> RwLockReadGuard<'_, Vec<ConfiguredRule>> {
        self.rules.read().expect("Unable to get rule: poisoned lock")
    }

    /// Retrieves the configured level of a specific rule by its slug.
    ///
    /// # Parameters
    ///
    /// - `slug`: The fully qualified slug of the rule (e.g., `"best-practices/no-eval"`).
    ///
    /// # Returns
    ///
    /// An [`Option<Level>`] containing the rule's level if it is configured, otherwise `None`.
    pub fn get_rule_level(&self, slug: &str) -> Option<Level> {
        self.get_configured_rules().iter().find(|r| r.slug == slug).map(|r| r.level)
    }

    /// Lints a given source file and returns a collection of found issues.
    ///
    /// This is the main entry point for linting a file. It performs the following steps:
    ///
    /// 1. Creates a `Collector` to gather issues for the "lint" category.
    /// 2. Pre-computes an AST representation for efficient traversal by rules.
    /// 3. Iterates through each configured rule.
    /// 4. For each rule, creates a `LintContext` and runs the rule's logic.
    /// 5. Issues from each rule are passed to the main collector, which handles suppression logic.
    /// 6. Finally, finalizes the collector to report unused pragmas and returns the complete set of issues.
    pub fn lint(
        &self,
        source_file: &File,
        program: &Program,
        resolved_names: &ResolvedNames,
        codebase: &CodebaseMetadata,
    ) -> IssueCollection {
        let configured_rules = self.rules.read().expect("Unable to read rules: poisoned lock");
        if configured_rules.is_empty() {
            tracing::warn!("Linting aborted - no rules configured.");
            return IssueCollection::new();
        }

        let mut collector = Collector::new(source_file, program, &self.interner, COLLECTOR_CATEGORY);
        let node = PreComputedNode::from(Node::Program(program));

        for configured_rule in configured_rules.iter() {
            let mut context = LintContext::new(
                self.settings.php_version,
                configured_rule,
                &self.interner,
                codebase,
                source_file,
                resolved_names,
            );

            context.lint(&node);

            collector.extend(context.finish());
        }

        collector.finish()
    }
}

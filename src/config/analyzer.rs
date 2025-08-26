use config::ConfigBuilder;
use config::Value;
use config::ValueKind;
use config::builder::BuilderState;
use mago_analyzer::settings::Settings;
use mago_php_version::PHPVersion;
use serde::Deserialize;
use serde::Serialize;

use crate::config::ConfigurationEntry;
use crate::error::Error;

/// Configuration options for the static analyzer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case", deny_unknown_fields)]
pub struct AnalyzerConfiguration {
    /// A list of patterns to exclude from analysis.
    pub excludes: Vec<String>,

    /// Ignore specific issues based on their code.
    pub ignore: Vec<String>,

    /// Report all issues related to the use of `mixed` types.
    pub mixed_issues: bool,

    /// Report all issues related to possibly `false` values.
    pub falsable_issues: bool,

    /// Report all issues related to possibly `null` values.
    pub nullable_issues: bool,

    /// Report all issues related to redundant code.
    pub redundancy_issues: bool,

    /// Report all issues related to by-reference variables.
    pub reference_issues: bool,

    /// Report all issues related to unreachable code.
    pub unreachable_issues: bool,

    /// Report all issues related to using deprecated code.
    pub deprecation_issues: bool,

    /// Report all issues related to logically impossible conditions.
    pub impossibility_issues: bool,

    /// Report all issues related to ambiguous code constructs.
    pub ambiguity_issues: bool,

    /// Report all issues related to the existence of symbols (e.g., classes, functions, constants).
    pub existence_issues: bool,

    /// Report all issues related to generic template types and their usage.
    pub template_issues: bool,

    /// Report all issues related to function arguments.
    pub argument_issues: bool,

    /// Report all issues related to operands in expressions.
    pub operand_issues: bool,

    /// Report all issues related to properties and their usage.
    pub property_issues: bool,

    /// Report all issues related to the use of generators.
    pub generator_issues: bool,

    /// Report all issues related to array operations and usage.
    pub array_issues: bool,

    /// Report issues related to the return type of functions and methods.
    pub return_issues: bool,

    /// Report issues related to methods and their usage.
    pub method_issues: bool,

    /// Report issues related to iterators and their usage.
    pub iterator_issues: bool,

    /// Whether to find unused expressions.
    pub find_unused_expressions: bool,

    /// Whether to find unused definitions.
    pub find_unused_definitions: bool,

    /// Whether to analyze dead code.
    pub analyze_dead_code: bool,

    /// Whether to memoize properties.
    pub memoize_properties: bool,

    /// Allow accessing array keys that may not be defined without reporting an issue.
    pub allow_possibly_undefined_array_keys: bool,

    /// Whether to check for thrown exceptions.
    pub check_throws: bool,

    /// Whether to perform heuristic checks.
    pub perform_heuristic_checks: bool,
}

impl AnalyzerConfiguration {
    pub fn to_setttings(&self, php_version: PHPVersion) -> Settings {
        Settings {
            version: php_version,
            mixed_issues: self.mixed_issues,
            falsable_issues: self.falsable_issues,
            nullable_issues: self.nullable_issues,
            redundancy_issues: self.redundancy_issues,
            reference_issues: self.reference_issues,
            unreachable_issues: self.unreachable_issues,
            deprecation_issues: self.deprecation_issues,
            impossibility_issues: self.impossibility_issues,
            ambiguity_issues: self.ambiguity_issues,
            existence_issues: self.existence_issues,
            template_issues: self.template_issues,
            argument_issues: self.argument_issues,
            operand_issues: self.operand_issues,
            property_issues: self.property_issues,
            generator_issues: self.generator_issues,
            array_issues: self.array_issues,
            return_issues: self.return_issues,
            method_issues: self.method_issues,
            iterator_issues: self.iterator_issues,
            analyze_dead_code: self.analyze_dead_code,
            find_unused_definitions: self.find_unused_definitions,
            find_unused_expressions: self.find_unused_expressions,
            memoize_properties: self.memoize_properties,
            allow_possibly_undefined_array_keys: self.allow_possibly_undefined_array_keys,
            check_throws: self.check_throws,
            perform_heuristic_checks: self.perform_heuristic_checks,
            diff: false,
        }
    }
}

impl ConfigurationEntry for AnalyzerConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        let defaults = Self::default();

        builder
            .set_default(
                "analyze.excludes",
                Value::new(None, ValueKind::Array(self.excludes.into_iter().map(Value::from).collect::<Vec<_>>())),
            )?
            .set_default(
                "analyze.ignore",
                Value::new(None, ValueKind::Array(self.ignore.into_iter().map(Value::from).collect::<Vec<_>>())),
            )?
            .set_default("analyze.mixed-issues", defaults.mixed_issues)?
            .set_default("analyze.falsable-issues", defaults.falsable_issues)?
            .set_default("analyze.nullable-issues", defaults.nullable_issues)?
            .set_default("analyze.redundancy-issues", defaults.redundancy_issues)?
            .set_default("analyze.reference-issues", defaults.reference_issues)?
            .set_default("analyze.unreachable-issues", defaults.unreachable_issues)?
            .set_default("analyze.deprecation-issues", defaults.deprecation_issues)?
            .set_default("analyze.impossibility-issues", defaults.impossibility_issues)?
            .set_default("analyze.ambiguity-issues", defaults.ambiguity_issues)?
            .set_default("analyze.existence-issues", defaults.existence_issues)?
            .set_default("analyze.template-issues", defaults.template_issues)?
            .set_default("analyze.argument-issues", defaults.argument_issues)?
            .set_default("analyze.operand-issues", defaults.operand_issues)?
            .set_default("analyze.property-issues", defaults.property_issues)?
            .set_default("analyze.generator-issues", defaults.generator_issues)?
            .set_default("analyze.array-issues", defaults.array_issues)?
            .set_default("analyze.return-issues", defaults.return_issues)?
            .set_default("analyze.method-issues", defaults.method_issues)?
            .set_default("analyze.iterator-issues", defaults.iterator_issues)?
            .set_default("analyze.find-unused-definitions", defaults.find_unused_definitions)?
            .set_default("analyze.find-unused-expressions", defaults.find_unused_expressions)?
            .set_default("analyze.analyze-dead-code", defaults.analyze_dead_code)?
            .set_default("analyze.memoize-properties", defaults.memoize_properties)?
            .set_default("analyze.allow-possibly-undefined-array-keys", defaults.allow_possibly_undefined_array_keys)?
            .set_default("analyze.check-throws", defaults.check_throws)?
            .set_default("analyze.perform-heuristic-checks", defaults.perform_heuristic_checks)
            .map_err(Error::from)
    }
}

impl Default for AnalyzerConfiguration {
    fn default() -> Self {
        let defaults = Settings::default();

        Self {
            excludes: vec![],
            ignore: vec![],
            mixed_issues: defaults.mixed_issues,
            falsable_issues: defaults.falsable_issues,
            nullable_issues: defaults.nullable_issues,
            redundancy_issues: defaults.redundancy_issues,
            reference_issues: defaults.reference_issues,
            unreachable_issues: defaults.unreachable_issues,
            deprecation_issues: defaults.deprecation_issues,
            impossibility_issues: defaults.impossibility_issues,
            ambiguity_issues: defaults.ambiguity_issues,
            existence_issues: defaults.existence_issues,
            template_issues: defaults.template_issues,
            argument_issues: defaults.argument_issues,
            operand_issues: defaults.operand_issues,
            property_issues: defaults.property_issues,
            generator_issues: defaults.generator_issues,
            array_issues: defaults.array_issues,
            return_issues: defaults.return_issues,
            method_issues: defaults.method_issues,
            iterator_issues: defaults.iterator_issues,
            find_unused_expressions: defaults.find_unused_expressions,
            find_unused_definitions: defaults.find_unused_definitions,
            analyze_dead_code: defaults.analyze_dead_code,
            memoize_properties: defaults.memoize_properties,
            allow_possibly_undefined_array_keys: defaults.allow_possibly_undefined_array_keys,
            check_throws: defaults.check_throws,
            perform_heuristic_checks: defaults.perform_heuristic_checks,
        }
    }
}

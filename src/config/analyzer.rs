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
#[serde(deny_unknown_fields)]
pub struct AnalyzeConfiguration {
    /// A list of patterns to exclude from analysis.
    pub excludes: Vec<String>,

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

    /// Whether to allow the use of `include` construct.
    pub allow_include: bool,

    /// Whether to allow the use of `eval` construct.
    pub allow_eval: bool,

    /// Whether to allow the use of `empty` construct.
    pub allow_empty: bool,

    /// Allow accessing array keys that may not be defined without reporting an issue.
    pub allow_possibly_undefined_array_keys: bool,

    /// Whether to check for thrown exceptions.
    pub check_throws: bool,
}

impl AnalyzeConfiguration {
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
            allow_include: self.allow_include,
            allow_eval: self.allow_eval,
            allow_empty: self.allow_empty,
            allow_possibly_undefined_array_keys: self.allow_possibly_undefined_array_keys,
            check_throws: self.check_throws,
            diff: false,
        }
    }
}

impl ConfigurationEntry for AnalyzeConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        let defaults = Self::default();

        builder
            .set_default(
                "analyze.excludes",
                Value::new(None, ValueKind::Array(self.excludes.into_iter().map(Value::from).collect::<Vec<_>>())),
            )?
            .set_default("analyze.mixed_issues", defaults.mixed_issues)?
            .set_default("analyze.falsable_issues", defaults.falsable_issues)?
            .set_default("analyze.nullable_issues", defaults.nullable_issues)?
            .set_default("analyze.redundancy_issues", defaults.redundancy_issues)?
            .set_default("analyze.reference_issues", defaults.reference_issues)?
            .set_default("analyze.unreachable_issues", defaults.unreachable_issues)?
            .set_default("analyze.deprecation_issues", defaults.deprecation_issues)?
            .set_default("analyze.impossibility_issues", defaults.impossibility_issues)?
            .set_default("analyze.ambiguity_issues", defaults.ambiguity_issues)?
            .set_default("analyze.existence_issues", defaults.existence_issues)?
            .set_default("analyze.template_issues", defaults.template_issues)?
            .set_default("analyze.argument_issues", defaults.argument_issues)?
            .set_default("analyze.operand_issues", defaults.operand_issues)?
            .set_default("analyze.property_issues", defaults.property_issues)?
            .set_default("analyze.generator_issues", defaults.generator_issues)?
            .set_default("analyze.array_issues", defaults.array_issues)?
            .set_default("analyze.return_issues", defaults.return_issues)?
            .set_default("analyze.method_issues", defaults.method_issues)?
            .set_default("analyze.iterator_issues", defaults.iterator_issues)?
            .set_default("analyze.find_unused_definitions", defaults.find_unused_definitions)?
            .set_default("analyze.find_unused_expressions", defaults.find_unused_expressions)?
            .set_default("analyze.analyze_dead_code", defaults.analyze_dead_code)?
            .set_default("analyze.memoize_properties", defaults.memoize_properties)?
            .set_default("analyze.allow_include", defaults.allow_include)?
            .set_default("analyze.allow_eval", defaults.allow_eval)?
            .set_default("analyze.allow_empty", defaults.allow_empty)?
            .set_default("analyze.allow_possibly_undefined_array_keys", defaults.allow_possibly_undefined_array_keys)?
            .set_default("analyze.check_throws", defaults.check_throws)
            .map_err(Error::from)
    }
}

impl Default for AnalyzeConfiguration {
    fn default() -> Self {
        let defaults = Settings::default();

        Self {
            excludes: vec![],
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
            allow_include: defaults.allow_include,
            allow_eval: defaults.allow_eval,
            allow_empty: defaults.allow_empty,
            allow_possibly_undefined_array_keys: defaults.allow_possibly_undefined_array_keys,
            check_throws: defaults.check_throws,
        }
    }
}

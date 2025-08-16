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

/// Configuration options for formatting source code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnalyzeConfiguration {
    /// A list of patterns to exclude from formatting.
    ///
    /// Defaults to `[]`.
    pub excludes: Vec<String>,

    /// Whether to find unused expressions.
    ///
    /// Defaults to `true`.
    pub find_unused_expressions: bool,

    /// Whether to find unused definitions.
    ///
    /// Defaults to `true`.
    pub find_unused_definitions: bool,

    /// Whether to analyze dead code.
    ///
    /// Defaults to `true`.
    pub analyze_dead_code: bool,

    /// Whether to memoize properties.
    ///
    /// Defaults to `true`.
    pub memoize_properties: bool,

    /// Whether to allow the use of `include` construct.
    ///
    /// Defaults to `true`.
    pub allow_include: bool,

    /// Whether to allow the use of `eval` construct.
    ///
    /// Defaults to `true`.
    pub allow_eval: bool,

    /// Whether to allow the use of `empty` construct.
    ///
    /// Defaults to `true`.
    pub allow_empty: bool,

    /// Whether to check for thrown exceptions.
    ///
    /// Defaults to `false`.
    pub check_throws: bool,
}

impl AnalyzeConfiguration {
    pub fn to_setttings(&self, php_version: PHPVersion) -> Settings {
        Settings {
            version: php_version,
            analyze_dead_code: self.analyze_dead_code,
            find_unused_definitions: self.find_unused_definitions,
            find_unused_expressions: self.find_unused_expressions,
            memoize_properties: self.memoize_properties,
            allow_include: self.allow_include,
            allow_eval: self.allow_eval,
            allow_empty: self.allow_empty,
            check_throws: self.check_throws,
            ..Default::default()
        }
    }
}

impl ConfigurationEntry for AnalyzeConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        builder
            .set_default("analyze.excludes", Value::new(None, ValueKind::Array(vec![])))?
            .set_default("analyze.find_unused_definitions", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.find_unused_expressions", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.analyze_dead_code", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.memoize_properties", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.allow_include", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.allow_eval", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.allow_empty", Value::new(None, ValueKind::Boolean(true)))?
            .set_default("analyze.check_throws", Value::new(None, ValueKind::Boolean(false)))
            .map_err(Error::from)
    }
}

impl Default for AnalyzeConfiguration {
    fn default() -> Self {
        Self {
            excludes: vec![],
            find_unused_expressions: true,
            find_unused_definitions: true,
            analyze_dead_code: true,
            memoize_properties: true,
            allow_include: true,
            allow_eval: true,
            allow_empty: true,
            check_throws: false,
        }
    }
}

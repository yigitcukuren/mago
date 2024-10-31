use ahash::HashMap;
use serde::Deserialize;
use serde::Serialize;
use toml::value::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LinterLevel {
    Off,
    Help,
    Note,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinterConfiguration {
    pub level: Option<LinterLevel>,
    pub external: Option<bool>,
    pub default_plugins: Option<bool>,
    pub plugins: Vec<String>,
    pub rules: Vec<LinterRuleConfiguration>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LinterRuleConfiguration {
    pub name: String,
    pub level: Option<LinterLevel>,
    pub options: Option<HashMap<String, Value>>,
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self { level: None, external: None, default_plugins: None, plugins: Vec::new(), rules: Vec::new() }
    }
}

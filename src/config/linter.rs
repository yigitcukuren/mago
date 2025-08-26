use config::ConfigBuilder;
use config::builder::BuilderState;
use mago_linter::integration::Integration;
use serde::Deserialize;
use serde::Serialize;

use mago_linter::settings::RulesSettings;

use crate::config::ConfigurationEntry;
use crate::error::Error;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct LinterConfiguration {
    /// A list of patterns to exclude from linting.
    pub excludes: Vec<String>,

    /// Integrations to enable during linting.
    pub integrations: Vec<Integration>,

    /// Settings for various linting rules.
    pub rules: RulesSettings,
}

impl ConfigurationEntry for LinterConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        use ::config::Value;
        use ::config::ValueKind;

        let builder = builder
            .set_default("linter.excludes", Value::new(None, ValueKind::Array(vec![])))?
            .set_default("linter.integrations", Value::new(None, ValueKind::Array(vec![])))?;

        Ok(builder)
    }
}

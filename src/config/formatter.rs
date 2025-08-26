use config::ConfigBuilder;
use config::Value;
use config::ValueKind;
use config::builder::BuilderState;
use serde::Deserialize;
use serde::Serialize;

use mago_formatter::settings::*;

use crate::config::ConfigurationEntry;
use crate::error::Error;

/// Configuration options for formatting source code.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "kebab-case", deny_unknown_fields)]
pub struct FormatterConfiguration {
    /// A list of patterns to exclude from formatting.
    ///
    /// Defaults to `[]`.
    pub excludes: Vec<String>,

    #[serde(flatten)]
    pub settings: FormatSettings,
}

impl ConfigurationEntry for FormatterConfiguration {
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, Error> {
        builder.set_default("format.excludes", Value::new(None, ValueKind::Array(vec![]))).map_err(Error::from)
    }
}

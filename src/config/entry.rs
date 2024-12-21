use super::*;

use config::builder::BuilderState;
use config::ConfigBuilder;

/// Internal trait for configuration entries.
pub(super) trait ConfigurationEntry {
    /// Configures the builder with the entry.
    fn configure<St: BuilderState>(self, builder: ConfigBuilder<St>) -> Result<ConfigBuilder<St>, ConfigurationError>;

    fn normalize(&mut self) -> Result<(), ConfigurationError> {
        Ok(())
    }
}

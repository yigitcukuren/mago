use std::process::ExitCode;

use clap::Parser;
use clap::ValueEnum;

use crate::config::Configuration;
use crate::config::analyzer::AnalyzerConfiguration;
use crate::config::formatter::FormatterConfiguration;
use crate::config::linter::LinterConfiguration;
use crate::config::source::SourceConfiguration;
use crate::consts::CURRENT_DIR;
use crate::error::Error;

#[derive(ValueEnum, Debug, Clone, Copy)]
#[value(rename_all = "kebab-case")]
enum ConfigSection {
    Source,
    Linter,
    Formatter,
    Analyzer,
}

/// Display the final, merged configuration that Mago is using.
///
/// This command is useful for debugging your setup. It prints the fully resolved
/// configuration, showing the combined result of your `mago.toml` file, any
/// environment variables, and the built-in default values.
#[derive(Parser, Debug)]
#[command(name = "config", about, long_about)]
pub struct ConfigCommand {
    /// Display only a specific section of the configuration.
    #[arg(long, value_enum)]
    show: Option<ConfigSection>,

    /// Show the default configuration values instead of the current ones.
    ///
    /// This ignores any `mago.toml` file or environment variables.
    #[arg(long, default_value_t = false)]
    default: bool,
}

impl ConfigCommand {
    pub fn execute(self, configuration: Configuration) -> Result<ExitCode, Error> {
        let json = if let Some(section) = self.show {
            match section {
                ConfigSection::Source => {
                    if self.default {
                        serde_json::to_string_pretty(&SourceConfiguration::from_workspace(CURRENT_DIR.clone()))?
                    } else {
                        serde_json::to_string_pretty(&configuration.source)?
                    }
                }
                ConfigSection::Linter => {
                    if self.default {
                        serde_json::to_string_pretty(&LinterConfiguration::default())?
                    } else {
                        serde_json::to_string_pretty(&configuration.linter)?
                    }
                }
                ConfigSection::Formatter => {
                    if self.default {
                        serde_json::to_string_pretty(&FormatterConfiguration::default())?
                    } else {
                        serde_json::to_string_pretty(&configuration.formatter)?
                    }
                }
                ConfigSection::Analyzer => {
                    if self.default {
                        serde_json::to_string_pretty(&AnalyzerConfiguration::default())?
                    } else {
                        serde_json::to_string_pretty(&configuration.analyzer)?
                    }
                }
            }
        } else {
            if self.default {
                serde_json::to_string_pretty(&Configuration::from_workspace(CURRENT_DIR.clone()))?
            } else {
                serde_json::to_string_pretty(&configuration)?
            }
        };

        println!("{}", json);

        Ok(ExitCode::SUCCESS)
    }
}

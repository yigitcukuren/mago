use crate::definition::PluginDefinition;
use crate::plugin::security::rules::no_insecure_comparison::NoInsecureComparisonRule;
use crate::plugin::security::rules::no_literal_password::NoLiteralPasswordRule;
use crate::plugin::security::rules::tainted_data_to_skin::TaintedDataToSinkRule;

use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct SecurityPlugin;

impl Plugin for SecurityPlugin {
    fn get_definition(&self) -> PluginDefinition {
        PluginDefinition {
            name: "Security",
            description: "Provides rules that enforce best practices for security.",
            enabled_by_default: true,
        }
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![Box::new(NoInsecureComparisonRule), Box::new(NoLiteralPasswordRule), Box::new(TaintedDataToSinkRule)]
    }
}

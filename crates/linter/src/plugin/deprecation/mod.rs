use crate::plugin::deprecation::rules::php84::implicitly_nullable_parameter::ImplicitlyNullableParameterRule;

use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct DeprecationPlugin;

impl Plugin for DeprecationPlugin {
    fn get_name(&self) -> &'static str {
        "deprecation"
    }

    fn is_enabled_by_default(&self) -> bool {
        true
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![
            // PHP 8.4
            Box::new(ImplicitlyNullableParameterRule),
        ]
    }
}

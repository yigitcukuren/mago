use crate::plugin::migration::rules::php80::str_contains::StrContainsRule;
use crate::plugin::migration::rules::php80::str_starts_with::StrStartsWithRule;

use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct MigrationPlugin;

impl Plugin for MigrationPlugin {
    fn get_name(&self) -> &'static str {
        "migration"
    }

    fn is_enabled_by_default(&self) -> bool {
        true
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![Box::new(StrStartsWithRule), Box::new(StrContainsRule)]
    }
}

use serde::Deserialize;
use serde::Serialize;

use mago_php_version::PHPVersionRange;

use crate::category::Category;
use crate::integration::IntegrationSet;

#[derive(Debug, PartialEq, Eq, Ord, Copy, Clone, PartialOrd, Deserialize, Serialize)]
pub struct RuleMeta {
    pub name: &'static str,
    pub code: &'static str,
    pub description: &'static str,
    pub good_example: &'static str,
    pub bad_example: &'static str,
    pub category: Category,
    pub requires: IntegrationSet,
    pub php: PHPVersionRange,
}

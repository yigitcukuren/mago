//! # Linter Rule Utilities
//!
//! This module provides helper functions for interacting with the linter's rule set,
//! such as retrieving metadata for all available rules.

use mago_interner::ThreadedInterner;
use mago_linter::Linter;
use mago_linter::rule::AnyRule;
use mago_linter::rule_meta::RuleMeta;

/// Retrieves metadata for all available linter rules based on the provided settings.
pub fn get_available_rules(settings: mago_linter::settings::Settings) -> Vec<&'static RuleMeta> {
    let interner = ThreadedInterner::new();
    let linter = Linter::new(interner, settings, None);
    linter.rules().iter().map(AnyRule::meta).collect()
}

use mago_linter::plugin::security::rules::no_insecure_comparison::NoInsecureComparisonRule;
use mago_linter::plugin::security::rules::no_literal_password::NoLiteralPasswordRule;
use mago_linter::plugin::security::rules::tainted_data_to_skin::TaintedDataToSinkRule;

use crate::rule_test;

rule_test!(test_insecure_comparison, NoInsecureComparisonRule);
rule_test!(test_literal_password, NoLiteralPasswordRule);
rule_test!(test_tainted_data_to_sink, TaintedDataToSinkRule);

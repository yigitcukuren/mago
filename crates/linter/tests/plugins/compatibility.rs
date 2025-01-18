use mago_linter::plugin::compatibility::rules::php74::null_coalesce_assignment_feature::NullCoalesceAssignmentFeatureRule;
use mago_linter::plugin::compatibility::rules::php80::named_arguments_feature::NamedArgumentsFeatureRule;
use mago_linter::plugin::compatibility::rules::php80::promoted_properties_feature::PromotedPropertiesFeatureRule;
use mago_linter::plugin::compatibility::rules::php80::union_type_hint_feature::UnionTypeHintFeatureRule;
use mago_linter::plugin::compatibility::rules::php81::closure_creation_feature::ClosureCreationFeatureRule;
use mago_linter::plugin::compatibility::rules::php84::asymmetric_visibility_feature::AsymmetricVisibilityFeatureRule;

use crate::rule_test;

rule_test!(test_null_coalesce_assignment_feature, NullCoalesceAssignmentFeatureRule);
rule_test!(test_named_arguments_feature, NamedArgumentsFeatureRule);
rule_test!(test_promoted_properties_feature, PromotedPropertiesFeatureRule);
rule_test!(test_union_type_hint_feature, UnionTypeHintFeatureRule);
rule_test!(test_closure_creation_feature, ClosureCreationFeatureRule);
rule_test!(test_asymmetric_visibility_feature, AsymmetricVisibilityFeatureRule);

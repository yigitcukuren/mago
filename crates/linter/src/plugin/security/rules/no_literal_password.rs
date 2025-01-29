use indoc::indoc;

use mago_ast::*;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::plugin::security::rules::utils::get_password;
use crate::plugin::security::rules::utils::is_password;
use crate::plugin::security::rules::utils::is_password_literal;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoLiteralPasswordRule;

impl Rule for NoLiteralPasswordRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("No Literal Password", Level::Error)
            .with_description(indoc! {r#"
                Detects the use of literal values for passwords or sensitive data.
                Storing passwords or sensitive information as literals in code is a security risk
                and should be avoided. Use environment variables or secure configuration management instead.
            "#})
            .with_example(RuleUsageExample::valid(
                "Using environment variables for sensitive data",
                indoc! {r#"
                    <?php

                    $password = getenv('DB_PASSWORD');
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values for passwords",
                indoc! {r#"
                    <?php

                    $password = "supersecret";
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values in arrays",
                indoc! {r#"
                    <?php

                    $config = [
                        'password' => 'supersecret',
                    ];
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values in constants",
                indoc! {r#"
                    <?php

                    const PASSWORD = 'supersecret';
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values in properties",
                indoc! {r#"
                    <?php

                    class Database {
                        private string $password = 'supersecret';
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values in function parameters",
                indoc! {r#"
                    <?php

                    function connect($password = 'supersecret') {}
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Using literal values in named arguments",
                indoc! {r#"
                    <?php

                    connect(password: 'supersecret');
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for NoLiteralPasswordRule {
    fn walk_in_assignment(&self, assignment: &Assignment, context: &mut LintContext<'_>) {
        let Some(password) = get_password(context, &assignment.lhs) else {
            return;
        };

        check(password, &assignment.rhs, context);
    }

    fn walk_in_array_element(&self, array_element: &ArrayElement, context: &mut LintContext<'_>) {
        let ArrayElement::KeyValue(kv) = array_element else {
            return;
        };

        let is_key_a_password = matches!(
            kv.key.as_ref(),
            Expression::Literal(Literal::String(literal_string)) if is_password_literal(context, literal_string),
        );

        if !is_key_a_password {
            return;
        }

        check(kv.key.as_ref(), kv.value.as_ref(), context);
    }

    fn walk_in_constant_item(&self, constant_item: &ConstantItem, context: &mut LintContext<'_>) {
        let constant_name = context.interner.lookup(&constant_item.name.value);
        if !is_password(constant_name) {
            return;
        }

        check(&constant_item.name, &constant_item.value, context);
    }

    fn walk_in_class_like_constant_item(
        &self,
        class_like_constant_item: &ClassLikeConstantItem,
        context: &mut LintContext<'_>,
    ) {
        let constant_name = context.interner.lookup(&class_like_constant_item.name.value);
        if !is_password(constant_name) {
            return;
        }

        check(&class_like_constant_item.name, &class_like_constant_item.value, context);
    }

    fn walk_in_property_concrete_item(
        &self,
        property_concrete_item: &PropertyConcreteItem,
        context: &mut LintContext<'_>,
    ) {
        let variable_name = context.interner.lookup(&property_concrete_item.variable.name);
        if !is_password(&variable_name[1..]) {
            return;
        }

        check(&property_concrete_item.variable, &property_concrete_item.value, context);
    }

    // check for `function foo($password = 'literal') {}`
    fn walk_in_function_like_parameter(
        &self,
        function_like_parameter: &FunctionLikeParameter,
        context: &mut LintContext<'_>,
    ) {
        let Some(default_value) = function_like_parameter.default_value.as_ref() else {
            return;
        };

        let parameter_name = context.interner.lookup(&function_like_parameter.variable.name);
        if !is_password(&parameter_name[1..]) {
            return;
        }

        check(&function_like_parameter.variable, &default_value.value, context);
    }

    // check for `foo(password: 'literal')`
    fn walk_in_named_argument(&self, named_argument: &NamedArgument, context: &mut LintContext<'_>) {
        let argument_name = context.interner.lookup(&named_argument.name.value);
        if !is_password(argument_name) {
            return;
        }

        check(&named_argument.name, &named_argument.value, context);
    }
}

fn check(name: impl HasSpan, value: &Expression, context: &mut LintContext) {
    let is_literal_password = match value {
        Expression::Literal(Literal::String(literal_string)) => {
            let value = context.interner.lookup(&literal_string.value);

            value.len() > 2 // at least 2 characters for the quotes, skip empty strings
        }
        Expression::Literal(Literal::Integer(_)) => true,
        _ => false,
    };

    if !is_literal_password {
        return;
    }

    let issue = Issue::new(context.level(), "Literal passwords or sensitive data should not be stored in code.")
        .with_annotation(Annotation::primary(name.span()).with_message("Sensitive item found here."))
        .with_annotation(Annotation::secondary(value.span()).with_message("Literal value used here."))
        .with_help("Use environment variables or secure configuration management instead.");

    context.report(issue);
}

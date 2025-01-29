use indoc::indoc;
use toml::Value;

use mago_ast::*;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::Level;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleOptionDefinition;
use crate::definition::RuleUsageExample;
use crate::plugin::security::rules::utils::is_user_input;
use crate::rule::Rule;

const PRINTF_FUNCTION: &str = "printf";
const KNOWN_SINK_FUNCTIONS: &str = "known_sink_functions";
const KNOWN_SINK_FUNCTIONS_DEFAULT: [&str; 1] = [PRINTF_FUNCTION];

#[derive(Clone, Debug)]
pub struct TaintedDataToSinkRule;

impl Rule for TaintedDataToSinkRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("Tainted Data to Sink", Level::Error)
            .with_description(indoc! {r#"
                Detects user (tainted) data being passed directly to sink functions or constructs
                (such as `echo`, `print`, or user-defined "log" functions). If these functions emit
                or store data without sanitization, it could lead to Cross-Site Scripting (XSS)
                or other injection attacks.
            "#})
            .with_option(RuleOptionDefinition {
                name: KNOWN_SINK_FUNCTIONS,
                r#type: "array<string>",
                description: "A list of sink functions that process or record data without sanitization.",
                default: Value::Array(vec![Value::String(PRINTF_FUNCTION.to_string())]),
            })
            .with_example(RuleUsageExample::valid(
                "Sanitizing user input before passing to a sink",
                indoc! {r#"
                    <?php

                    // Properly escape data before using a sink like `echo`
                    echo htmlspecialchars($_GET['name'] ?? '', ENT_QUOTES, 'UTF-8');
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Directly passing user input to `echo`",
                indoc! {r#"
                    <?php

                    // This is considered unsafe:
                    echo $_GET['name'] ?? '';
                "#},
            ))
    }
}

impl Walker<LintContext<'_>> for TaintedDataToSinkRule {
    fn walk_in_echo(&self, echo: &Echo, context: &mut LintContext<'_>) {
        for value in echo.values.iter() {
            check_tainted_data_to_sink(context, &echo.echo, value);
        }
    }

    fn walk_in_print_construct(&self, print_construct: &PrintConstruct, context: &mut LintContext<'_>) {
        check_tainted_data_to_sink(context, &print_construct.print, &print_construct.value);
    }

    fn walk_in_function_call(&self, function_call: &FunctionCall, context: &mut LintContext<'_>) {
        let Expression::Identifier(identifier) = function_call.function.as_ref() else {
            return;
        };

        let function_name = context.resolve_function_name(identifier);

        // Check if this function name is listed among known sinks
        let is_sink_function =
            if let Some(known_sinks) = context.option(KNOWN_SINK_FUNCTIONS).and_then(|o| o.as_array()) {
                known_sinks.iter().any(|f| f.as_str().is_some_and(|f| f.eq_ignore_ascii_case(function_name)))
            } else {
                KNOWN_SINK_FUNCTIONS_DEFAULT.iter().any(|f| f.eq_ignore_ascii_case(function_name))
            };

        if !is_sink_function {
            return;
        }

        // If it is indeed a known sink, check each argument
        for argument in function_call.argument_list.arguments.iter() {
            check_tainted_data_to_sink(context, &function_call.function, argument.value());
        }
    }
}

fn check_tainted_data_to_sink(context: &mut LintContext<'_>, used_in: &impl HasSpan, value: &Expression) {
    if !is_user_input(context, value) {
        return;
    }

    let issue = Issue::new(context.level(), "Tainted data passed to a sink function/construct.")
        .with_annotation(Annotation::primary(value.span()).with_message("This value originates from user input."))
        .with_annotation(
            Annotation::secondary(used_in.span()).with_message("Data is passed here without sanitization."),
        )
        .with_note("Tainted (user-supplied) data must be sanitized or escaped before being passed to sinks, or risk injection vulnerabilities.")
        .with_help("Ensure the data is validated or escaped prior to using this sink.");

    context.report(issue);
}

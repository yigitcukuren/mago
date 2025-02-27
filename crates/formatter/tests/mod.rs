use mago_formatter::Formatter;
use mago_formatter::settings::FormatSettings;
use mago_interner::ThreadedInterner;
use mago_php_version::PHPVersion;

pub mod comment;
pub mod format;
pub mod parens;

/// Test that the given code is formatted to the expected result.
///
/// This function will parse the given code, format it, parse the formatted code, and then format it again
/// to ensure that the formatter is idempotent.
///
/// # Arguments
///
/// * `code` - The code to format
/// * `expected` - The expected result of formatting the code
/// * `settings` - The settings to use when formatting the code
pub fn test_format(code: impl AsRef<str>, expected: &str, settings: FormatSettings) {
    test_format_with_version(code, expected, PHPVersion::PHP84, settings);
}

/// Test that the given code is formatted to the expected result.
///
/// This function will parse the given code, format it, parse the formatted code, and then format it again
/// to ensure that the formatter is idempotent.
///
/// # Arguments
///
/// * `code` - The code to format
/// * `expected` - The expected result of formatting the code
/// * `php_version` - The PHP version to use when formatting the code
/// * `settings` - The settings to use when formatting the code
pub fn test_format_with_version(
    code: impl AsRef<str>,
    expected: &str,
    php_version: PHPVersion,
    settings: FormatSettings,
) {
    let interner = ThreadedInterner::new();
    let formatter = Formatter::new(&interner, php_version, settings);

    let formatted_code = formatter.format_code("code.php", code.as_ref()).unwrap();
    pretty_assertions::assert_eq!(expected, formatted_code, "Formatted code does not match expected");

    let reformatted_code = formatter.format_code("formatted_code.php", &formatted_code).unwrap();
    pretty_assertions::assert_eq!(expected, reformatted_code, "Reformatted code does not match expected");
}

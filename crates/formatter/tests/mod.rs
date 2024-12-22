use mago_formatter::settings::FormatSettings;
use mago_interner::ThreadedInterner;
use mago_parser::parse_source;
use mago_source::error::SourceError;
use mago_source::SourceManager;

pub mod format;
pub mod parens;

pub fn test_format(code: impl AsRef<str>, expected: &str, settings: FormatSettings) -> Result<(), SourceError> {
    let interner = ThreadedInterner::new();
    let mut manager = SourceManager::new(interner.clone());
    let source_id = manager.insert_content("code.php".to_string(), code.as_ref().to_string(), true);
    let source = manager.load(&source_id)?;
    let (program, _) = parse_source(&interner, &source);
    let formatted = mago_formatter::format(settings, &interner, &source, &program);

    pretty_assertions::assert_eq!(expected, formatted, "Formatted code does not match expected");

    Ok(())
}

use ariadne::sources as ariadne_sources;
use ariadne::*;
use termcolor::WriteColor;

use mago_interner::ThreadedInterner;
use mago_source::HasSource;
use mago_source::SourceManager;

use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;

pub fn ariadne_format(
    mut writer: &mut dyn WriteColor,
    sources: &SourceManager,
    interner: &ThreadedInterner,
    issues: IssueCollection,
) -> Result<Option<Level>, ReportingError> {
    let highest_level = issues.get_highest_level();

    for issue in issues {
        let kind = match issue.level {
            Level::Help | Level::Note => ReportKind::Advice,
            Level::Warning => ReportKind::Warning,
            Level::Error => ReportKind::Error,
        };

        let color = match issue.level {
            Level::Help | Level::Note => Color::Blue,
            Level::Warning => Color::Yellow,
            Level::Error => Color::Red,
        };

        let (file_path, range) = match issue.annotations.iter().find(|annotation| annotation.is_primary()) {
            Some(annotation) => {
                let source = sources.load(&annotation.span.source())?;

                (
                    interner.lookup(&source.identifier.0).to_string(),
                    annotation.span.start.offset..annotation.span.end.offset,
                )
            }
            None => ("<unknown>".to_string(), 0..0),
        };

        let mut report = Report::build(kind, (file_path, range)).with_message(issue.message);

        if let Some(code) = issue.code {
            report = report.with_code(code);
        }

        for note in issue.notes {
            report = report.with_note(note);
        }

        if let Some(link) = issue.link {
            // Since ariadne doesn't support links, we can just set it as a note
            report = report.with_note(format!("For more information, see: {link}"));
        }

        if let Some(help) = issue.help {
            report = report.with_help(help);
        }

        let mut relevant_sources = vec![];
        for annotation in issue.annotations {
            let source = sources.load(&annotation.span.source())?;
            let file_path = interner.lookup(&source.identifier.0).to_string();
            let range = annotation.span.start.offset..annotation.span.end.offset;

            let mut label = Label::new((file_path.clone(), range));
            if annotation.is_primary() {
                label = label.with_color(color).with_priority(1);
            }

            if let Some(message) = annotation.message {
                report = report.with_label(label.with_message(message));
            } else {
                report = report.with_label(label);
            }

            relevant_sources.push((file_path, interner.lookup(&source.content)));
        }

        report.finish().write(ariadne_sources(relevant_sources), &mut writer).unwrap();
    }

    Ok(highest_level)
}

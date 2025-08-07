use ariadne::sources as ariadne_sources;
use ariadne::*;
use termcolor::WriteColor;

use mago_database::DatabaseReader;
use mago_database::ReadDatabase;
use mago_database::file::HasFileId;

use crate::IssueCollection;
use crate::Level;
use crate::error::ReportingError;

pub fn ariadne_format(
    mut writer: &mut dyn WriteColor,
    database: &ReadDatabase,
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
                let file = database.get_by_id(&annotation.span.file_id())?;

                (file.name.clone(), annotation.span.start.offset..annotation.span.end.offset)
            }
            None => ("<unknown>".to_owned(), 0..0),
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
            let file = database.get_by_id(&annotation.span.file_id())?;
            let range = annotation.span.start.offset..annotation.span.end.offset;

            let mut label = Label::new((file.name.clone(), range));
            if annotation.is_primary() {
                label = label.with_color(color).with_priority(1);
            }

            if let Some(message) = annotation.message {
                report = report.with_label(label.with_message(message));
            } else {
                report = report.with_label(label);
            }

            relevant_sources.push((file.name.clone(), &file.contents));
        }

        report.finish().write(ariadne_sources(relevant_sources), &mut writer).unwrap();
    }

    Ok(highest_level)
}

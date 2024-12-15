use termcolor::WriteColor;

use mago_interner::ThreadedInterner;
use mago_source::SourceManager;

use crate::error::ReportingError;
use crate::internal::Expandable;
use crate::IssueCollection;
use crate::Level;

pub fn json_format(
    writer: &mut dyn WriteColor,
    sources: &SourceManager,
    interner: &ThreadedInterner,
    issues: IssueCollection,
) -> Result<Option<Level>, ReportingError> {
    let highest_level = issues.get_highest_level();
    let issues = issues.expand(sources, interner)?;

    serde_json::to_writer_pretty(writer, &issues)?;

    Ok(highest_level)
}

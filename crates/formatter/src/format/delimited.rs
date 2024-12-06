use fennec_span::Span;

use crate::document::Document;
use crate::document::IfBreak;
use crate::document::Line;
use crate::Formatter;

/// Represents a pair of delimiters (e.g., parentheses, braces, brackets, attributes),
/// each with a corresponding `Span` for the opening and closing positions.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) enum Delimiter {
    Parentheses(Span, Span),
}

impl Delimiter {
    /// Returns the string representation of the opening delimiter.
    pub fn left_as_str(&self) -> &'static str {
        match self {
            Self::Parentheses(_, _) => "(",
        }
    }

    /// Returns the string representation of the closing delimiter.
    pub fn right_as_str(&self) -> &'static str {
        match self {
            Self::Parentheses(_, _) => ")",
        }
    }

    /// Determines if a space is required around the delimiters.
    /// - `true` for braces (`{}`).
    /// - `false` for other delimiters.
    pub fn needs_space(&self) -> bool {
        match self {
            Self::Parentheses(_, _) => false,
        }
    }

    /// Returns the spans for the opening and closing delimiters.
    #[inline]
    pub fn spans(&self) -> (&Span, &Span) {
        match self {
            Self::Parentheses(left, right) => (left, right),
        }
    }

    pub fn is_already_broken(&self, f: &Formatter<'_>) -> bool {
        let (left_delimiter, right_delimiter) = self.spans();

        let starting_line = f.source.line_number(left_delimiter.start.offset);
        let ending_line = f.source.line_number(right_delimiter.end.offset);

        if starting_line == ending_line {
            false
        } else {
            let previous = &f.source_text[right_delimiter.start.offset - 1..right_delimiter.start.offset];

            previous.chars().all(char::is_whitespace)
        }
    }

    pub fn format_left<'a>(&self, f: &mut Formatter<'a>) -> (Document<'a>, bool) {
        let left_delimiter = self.spans().0;

        let mut contents = vec![];
        if let Some(comments) = f.print_leading_comments(*left_delimiter) {
            contents.push(comments);
        }

        contents.push(Document::String(self.left_as_str()));

        let has_trailing_comments = if let Some(comments) = f.print_trailing_comments(*left_delimiter) {
            contents.push(Document::IfBreak(IfBreak::new(
                Document::Indent(vec![Document::Line(Line::default())]),
                Document::space(),
            )));

            contents.push(comments);

            true
        } else {
            false
        };

        if contents.len() == 1 {
            (contents.remove(0), false)
        } else {
            (Document::Array(contents), has_trailing_comments)
        }
    }

    pub fn format_right<'a>(&self, f: &mut Formatter<'a>) -> (Document<'a>, bool) {
        let right_delimiter = self.spans().1;

        let mut contents = vec![];
        let has_leading_comments = if let Some(leading_comments) = f.print_leading_comments(*right_delimiter) {
            contents.push(Document::Indent(vec![Document::Line(Line::hardline()), leading_comments]));

            true
        } else {
            false
        };

        if has_leading_comments {
            contents.push(Document::Line(Line::hardline()));
        }

        contents.push(Document::String(self.right_as_str()));

        if let Some(trailing_comments) = f.print_trailing_comments(*right_delimiter) {
            contents.push(trailing_comments);
        }

        (Document::Array(contents), has_leading_comments)
    }
}

use fennec_ast::sequence::TokenSeparatedSequence;
use fennec_span::HasSpan;

use crate::comment::CommentFlags;
use crate::document::IfBreak;
use crate::document::Line;
use crate::document::*;
use crate::format::Format;
use crate::Formatter;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(super) struct TokenSeparatedSequenceFormatter<'a> {
    pub separator: &'a str,
    pub trailing_separator: bool,
    pub force_break: bool,
    pub force_inline: bool,
    pub break_parent: bool,
}

impl<'a> TokenSeparatedSequenceFormatter<'a> {
    pub fn new(separator: &'a str) -> Self {
        Self { separator, trailing_separator: false, force_break: false, force_inline: false, break_parent: false }
    }

    pub fn with_trailing_separator(mut self, trailing_separator: bool) -> Self {
        self.trailing_separator = trailing_separator;
        self
    }

    pub fn format<T: Format<'a> + HasSpan>(
        self,
        f: &mut Formatter<'a>,
        nodes: &'a TokenSeparatedSequence<T>,
    ) -> Document<'a> {
        let mut contents = Vec::new();

        let mut must_break = self.force_break;
        let length = nodes.len();
        for (i, item, separator) in nodes.iter_with_tokens() {
            contents.push(item.format(f));
            if let Some(separator) = separator {
                // If a separator has a trailing single line comment, we must break
                // the sequence, otherwise a further node might get commented out.
                //
                // e.g.:
                //
                // ```
                // (
                //    $a, // comment
                //    $b,
                // )
                // ```
                //
                // If we don't break the sequence, the comment will be on the same line as the
                // rest of the sequence, which is not what we want.
                //
                // ```
                // ($a, // comment, $b)
                // ```
                //
                // resulting in `b` and the right parenthesis being commented out.
                must_break = must_break || f.has_comment(separator.span, CommentFlags::Trailing & CommentFlags::Line);

                let leading_comments = f.print_leading_comments(separator.span);
                let trailing_comments = f.print_trailing_comments(separator.span);
                let has_comments = leading_comments.is_some() || trailing_comments.is_some();
                let separator =
                    f.print_comments(leading_comments, Document::String(f.lookup(&separator.value)), trailing_comments);

                if (i < (length - 1)) || has_comments || must_break {
                    contents.push(separator);
                } else {
                    contents.push(Document::IfBreak(IfBreak::then(separator)));
                }
            } else if self.trailing_separator {
                if must_break {
                    contents.push(Document::String(self.separator));
                } else if !self.force_inline {
                    contents.push(Document::IfBreak(IfBreak::then(Document::String(self.separator))));
                }
            }

            if i < (length - 1) {
                if must_break {
                    contents.push(Document::Line(Line::hardline()));
                } else if self.force_inline {
                    contents.push(Document::space());
                } else {
                    contents.push(Document::Line(Line::default()));
                }
            }
        }

        Document::Array(contents)
    }
}

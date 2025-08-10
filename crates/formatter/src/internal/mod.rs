use std::iter::Peekable;
use std::vec::IntoIter;

use mago_database::file::File;
use mago_database::file::FileId;
use mago_database::file::HasFileId;
use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_php_version::PHPVersion;
use mago_span::Span;
use mago_syntax::ast::Node;
use mago_syntax::ast::Trivia;

use crate::document::group::GroupIdentifier;
use crate::document::group::GroupIdentifierBuilder;
use crate::settings::FormatSettings;

pub mod binaryish;
pub mod comment;
pub mod format;
pub mod macros;
pub mod parens;
pub mod printer;
pub mod utils;

#[derive(Debug, Clone, Copy, Default)]
pub struct ArgumentState {
    expand_first_argument: bool,
    expand_last_argument: bool,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParameterState {
    force_break: bool,
}

#[derive(Debug)]
pub struct FormatterState<'a> {
    interner: &'a ThreadedInterner,
    file: &'a File,
    php_version: PHPVersion,
    settings: FormatSettings,
    stack: Vec<Node<'a>>,
    comments: Peekable<IntoIter<Trivia>>,
    scripting_mode: bool,
    id_builder: GroupIdentifierBuilder,
    argument_state: ArgumentState,
    parameter_state: ParameterState,
    in_pipe_chain_arrow_segment: bool,
    in_script_terminating_statement: bool,
    in_condition: bool,
    halted_compilation: bool,
}

impl<'a> FormatterState<'a> {
    pub fn new(
        interner: &'a ThreadedInterner,
        file: &'a File,
        php_version: PHPVersion,
        settings: FormatSettings,
    ) -> Self {
        Self {
            interner,
            file,
            php_version,
            settings,
            stack: vec![],
            comments: vec![].into_iter().peekable(),
            scripting_mode: false,
            id_builder: GroupIdentifierBuilder::new(),
            argument_state: ArgumentState::default(),
            parameter_state: ParameterState::default(),
            in_pipe_chain_arrow_segment: false,
            in_condition: false,
            halted_compilation: false,
            in_script_terminating_statement: false,
        }
    }

    fn next_id(&mut self) -> GroupIdentifier {
        self.id_builder.next_id()
    }

    fn lookup(&self, string: &StringIdentifier) -> &'a str {
        self.interner.lookup(string)
    }

    #[inline]
    fn as_str(&self, string: impl AsRef<str>) -> &'a str {
        self.interner.interned_str(string)
    }

    #[inline]
    fn enter_node(&mut self, node: Node<'a>) {
        self.stack.push(node);
    }

    #[inline]
    fn leave_node(&mut self) {
        self.stack.pop();
    }

    #[inline]
    fn current_node(&self) -> Node<'a> {
        self.stack[self.stack.len() - 1]
    }

    #[inline]
    fn parent_node(&self) -> Node<'a> {
        self.stack[self.stack.len() - 2]
    }

    #[inline]
    fn grandparent_node(&self) -> Option<Node<'a>> {
        let len = self.stack.len();

        (len > 2).then(|| self.stack[len - 2 - 1])
    }

    #[inline]
    fn great_grandparent_node(&self) -> Option<Node<'a>> {
        let len = self.stack.len();
        (len > 3).then(|| self.stack[len - 3 - 1])
    }

    #[inline]
    fn nth_parent_kind(&self, n: u32) -> Option<Node<'a>> {
        let n = n as usize;
        let len = self.stack.len();

        (len > n).then(|| self.stack[len - n - 1])
    }

    #[inline]
    fn is_previous_line_empty(&self, start_index: u32) -> bool {
        let idx = start_index - 1;
        let idx = self.skip_spaces(Some(idx), true);
        let idx = self.skip_newline(idx, true);
        let idx = self.skip_spaces(idx, true);
        let idx2 = self.skip_newline(idx, true);
        idx != idx2
    }

    #[inline]
    fn is_next_line_empty(&self, span: Span) -> bool {
        self.is_next_line_empty_after_index(span.end.offset)
    }

    #[inline]
    fn is_next_line_empty_after_index(&self, start_index: u32) -> bool {
        let mut old_idx = None;
        let mut idx = Some(start_index);
        while idx != old_idx {
            old_idx = idx;
            idx = self.skip_to_line_end(idx);
            idx = self.skip_spaces(idx, /* backwards */ false);
        }

        idx = self.skip_inline_comments(idx);
        idx = self.skip_newline(idx, /* backwards */ false);
        idx.is_some_and(|idx| self.has_newline(idx, /* backwards */ false))
    }

    #[inline]
    fn skip_inline_comments(&self, start_index: Option<u32>) -> Option<u32> {
        let start_index = start_index?;
        let start_index_usize = start_index as usize;
        if start_index_usize + 1 >= self.file.contents.len() {
            return Some(start_index); // Not enough characters to check for comment
        }

        if self.file.contents[start_index_usize..].starts_with("//")
            || (self.file.contents[start_index_usize..].starts_with("#")
                && !self.file.contents[start_index_usize + 1..].starts_with("["))
        {
            return self.skip_everything_but_new_line(Some(start_index), false);
        }

        if self.file.contents[start_index_usize..].starts_with("/*") {
            // Find the closing */
            if let Some(end_pos) = self.file.contents[start_index_usize + 2..].find("*/") {
                let end_index = start_index_usize + 2 + end_pos + 2; // +2 for the "*/" itself

                // Check if there's a newline between /* and */
                let comment_text = &self.file.contents[start_index_usize..end_index];
                if !comment_text.contains('\n') && !comment_text.contains('\r') {
                    return Some(end_index as u32);
                }

                // If there's a newline, we don't consider it an inline comment
                // so we don't skip it
            }
        }

        Some(start_index)
    }

    #[inline]
    fn skip_to_line_end(&self, start_index: Option<u32>) -> Option<u32> {
        let mut index = self.skip(start_index, false, |c| matches!(c, b' ' | b'\t' | b',' | b';'));
        index = self.skip_inline_comments(index);
        index
    }

    #[inline]
    fn skip_spaces(&self, start_index: Option<u32>, backwards: bool) -> Option<u32> {
        self.skip(start_index, backwards, |c| matches!(c, b' ' | b'\t'))
    }

    #[inline]
    fn skip_spaces_and_new_lines(&self, start_index: Option<u32>, backwards: bool) -> Option<u32> {
        self.skip(start_index, backwards, |c| matches!(c, b' ' | b'\t' | b'\r' | b'\n'))
    }

    #[inline]
    fn skip_everything_but_new_line(&self, start_index: Option<u32>, backwards: bool) -> Option<u32> {
        self.skip(start_index, backwards, |c| !matches!(c, b'\r' | b'\n'))
    }

    #[inline]
    fn skip<F>(&self, start_index: Option<u32>, backwards: bool, f: F) -> Option<u32>
    where
        F: Fn(u8) -> bool,
    {
        let start_index = start_index? as usize;
        let mut index = start_index;
        if backwards {
            for c in self.file.contents[..=start_index].bytes().rev() {
                if !f(c) {
                    return Some(index as u32);
                }
                index -= 1;
            }
        } else {
            let source_bytes = self.file.contents.as_bytes();
            let text_len = source_bytes.len();
            while index < text_len {
                if !f(source_bytes[index]) {
                    return Some(index as u32);
                }

                index += 1;
            }
        }

        None
    }

    #[inline]
    fn skip_newline(&self, start_index: Option<u32>, backwards: bool) -> Option<u32> {
        let start_index = start_index?;
        let start_index_usize = start_index as usize;
        let c = if backwards {
            self.file.contents[..=start_index_usize].bytes().next_back()
        } else {
            self.file.contents[start_index_usize..].bytes().next()
        }?;

        if matches!(c, b'\n') {
            return Some(if backwards { start_index - 1 } else { start_index + 1 });
        }

        if matches!(c, b'\r') {
            let next_index = if backwards { start_index_usize - 1 } else { start_index_usize + 1 };
            let next_c = if backwards {
                self.file.contents[..=next_index].bytes().next_back()
            } else {
                self.file.contents[next_index..].bytes().next()
            }?;

            if matches!(next_c, b'\n') {
                return Some(if backwards { start_index - 2 } else { start_index + 2 });
            }
        }

        Some(start_index)
    }

    #[inline]
    fn has_newline(&self, start_index: u32, backwards: bool) -> bool {
        if (backwards && start_index == 0) || (!backwards && (start_index as usize) == self.file.contents.len()) {
            return false;
        }
        let start_index = if backwards { start_index - 1 } else { start_index };
        let idx = self.skip_spaces(Some(start_index), backwards);
        let idx2 = self.skip_newline(idx, backwards);
        idx != idx2
    }

    #[inline]
    fn split_lines(slice: &'a str) -> Vec<&'a str> {
        let mut lines = Vec::new();
        let mut remaining = slice;

        while !remaining.is_empty() {
            if let Some(pos) = remaining.find("\r\n") {
                lines.push(&remaining[..pos]);
                remaining = &remaining[pos + 2..];
            } else if let Some(pos) = remaining.find('\n') {
                lines.push(&remaining[..pos]);
                remaining = &remaining[pos + 1..];
            } else {
                // No more newlines
                if !remaining.is_empty() {
                    lines.push(remaining);
                }
                break;
            }
        }

        lines
    }

    #[inline]
    fn skip_leading_whitespace_up_to(s: &'a str, indent: usize) -> &'a str {
        let mut position = 0;
        for (count, (i, b)) in s.bytes().enumerate().enumerate() {
            // Check if the current byte represents whitespace
            if !b.is_ascii_whitespace() || count >= indent {
                break;
            }

            position = i + 1;
        }

        &s[position..]
    }
}

impl HasFileId for FormatterState<'_> {
    fn file_id(&self) -> FileId {
        self.file.id
    }
}

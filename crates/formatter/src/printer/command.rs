use crate::document::Document;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indent {
    pub root: bool,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Break,
    Flat,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Command<'a> {
    pub indent: Indent,
    pub mode: Mode,
    pub document: Document<'a>,
}

impl Indent {
    pub fn root() -> Self {
        Self { root: true, length: 0 }
    }

    pub fn new(length: usize) -> Self {
        Self { root: false, length }
    }
}

impl Mode {
    pub fn is_break(self) -> bool {
        self == Self::Break
    }

    pub fn is_flat(self) -> bool {
        self == Self::Flat
    }
}

impl<'a> Command<'a> {
    pub fn new(indent: Indent, mode: Mode, document: Document<'a>) -> Self {
        Self { indent, mode, document }
    }

    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }
}

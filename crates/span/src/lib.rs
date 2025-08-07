//! Provides fundamental types for source code location tracking.
//!
//! This crate defines the core primitives [`Position`] and [`Span`] used throughout
//! mago to identify specific locations in source files. It also provides
//! the generic traits [`HasPosition`] and [`HasSpan`] to abstract over any syntax
//! tree node or token that has a location.

use std::ops::Range;

use serde::Deserialize;
use serde::Serialize;

use mago_database::file::FileId;
use mago_database::file::HasFileId;

/// Represents a specific byte offset within a single source file.
///
/// This struct combines a [`FileId`] with a zero-based `offset` to create a
/// precise, unique location pointer.
///
/// The memory layout is specified as `#[repr(C)]` to ensure stability for
/// potential foreign function interfaces (FFI).
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(C)]
pub struct Position {
    /// The unique identifier of the file this position belongs to.
    pub file_id: FileId,
    /// The zero-based byte offset from the beginning of the file.
    pub offset: usize,
}

/// Represents a contiguous range of source code within a single file.
///
/// A `Span` is defined by a `start` and `end` [`Position`], marking the beginning
/// (inclusive) and end (exclusive) of a source code segment.
///
/// The memory layout is specified as `#[repr(C)]` for stability.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[repr(C)]
pub struct Span {
    /// The starting position of the span (inclusive).
    pub start: Position,
    /// The ending position of the span (exclusive).
    pub end: Position,
}

/// A trait for types that have a single, defined source position.
pub trait HasPosition {
    /// Returns the source position.
    fn position(&self) -> Position;

    /// A convenience method to get the byte offset of the position.
    #[inline]
    fn offset(&self) -> usize {
        self.position().offset
    }
}

/// A trait for types that cover a span of source code.
pub trait HasSpan {
    /// Returns the source span.
    fn span(&self) -> Span;

    /// A convenience method to get the starting position of the span.
    fn start_position(&self) -> Position {
        self.span().start
    }

    /// A convenience method to get the ending position of the span.
    fn end_position(&self) -> Position {
        self.span().end
    }
}

impl Position {
    /// Creates a new `Position` from a file ID and a byte offset.
    pub fn new(file_id: FileId, offset: usize) -> Self {
        Self { file_id, offset }
    }

    /// Creates a "dummy" position with a null file ID.
    ///
    /// This is useful for generated code or locations that don't map to a real file.
    pub fn dummy(offset: usize) -> Self {
        Self::new(FileId::zero(), offset)
    }

    /// Creates a position at the very beginning of a file.
    pub fn start_of(file_id: FileId) -> Self {
        Self::new(file_id, 0)
    }

    /// Returns a new position moved forward by the given offset.
    ///
    /// Uses saturating arithmetic to prevent overflow.
    pub const fn forward(&self, offset: usize) -> Self {
        Self { file_id: self.file_id, offset: self.offset.saturating_add(offset) }
    }

    /// Returns a new position moved backward by the given offset.
    ///
    /// Uses saturating arithmetic to prevent underflow.
    pub fn backward(&self, offset: usize) -> Self {
        Self { file_id: self.file_id, offset: self.offset.saturating_sub(offset) }
    }

    /// Creates a `Range<usize>` starting at this position's offset with a given length.
    pub fn range_for(&self, length: usize) -> Range<usize> {
        self.offset..self.offset.saturating_add(length)
    }
}

impl Span {
    /// Creates a new `Span` from a start and end position.
    ///
    /// # Panics
    ///
    /// In debug builds, this will panic if the start and end positions are not
    /// from the same file (unless one is a dummy position).
    pub fn new(start: Position, end: Position) -> Self {
        debug_assert!(
            start.file_id.is_zero() || end.file_id.is_zero() || start.file_id == end.file_id,
            "span start and end must be in the same file",
        );
        Self { start, end }
    }

    /// Creates a "dummy" span with a null file ID.
    pub fn dummy(start_offset: usize, end_offset: usize) -> Self {
        Self::new(Position::dummy(start_offset), Position::dummy(end_offset))
    }

    /// Creates a new span that starts at the beginning of the first span
    /// and ends at the conclusion of the second span.
    pub fn between(start: Span, end: Span) -> Self {
        start.join(end)
    }

    /// Creates a new span that encompasses both `self` and `other`.
    /// The new span starts at `self.start` and ends at `other.end`.
    pub fn join(self, other: Span) -> Span {
        Span::new(self.start, other.end)
    }

    /// Creates a new span that starts at the beginning of this span
    /// and ends at the specified position.
    pub fn to_end(&self, end: Position) -> Span {
        Span::new(self.start, end)
    }

    /// Creates a new span that starts at the specified position
    /// and ends at the end of this span.
    pub fn from_start(&self, start: Position) -> Span {
        Span::new(start, self.end)
    }

    /// Creates a new span that is a subspan of this span, defined by the given byte offsets.
    /// The `start` and `end` parameters are relative to the start of this span.
    pub fn subspan(&self, start: usize, end: usize) -> Span {
        Span::new(self.start.forward(start), self.start.forward(end))
    }

    /// Checks if a position is contained within this span's byte offsets.
    pub fn contains(&self, position: &impl HasPosition) -> bool {
        self.has_offset(position.offset())
    }

    /// Checks if a raw byte offset is contained within this span.
    pub fn has_offset(&self, offset: usize) -> bool {
        self.start.offset <= offset && offset <= self.end.offset
    }

    /// Converts the span to a `Range<usize>` of its byte offsets.
    pub fn to_range(&self) -> Range<usize> {
        self.start.offset..self.end.offset
    }

    /// Converts the span to a tuple of byte offsets.
    pub fn to_offset_tuple(&self) -> (usize, usize) {
        (self.start.offset, self.end.offset)
    }

    /// Returns the length of the span in bytes.
    pub fn length(&self) -> usize {
        self.end.offset.saturating_sub(self.start.offset)
    }

    pub fn is_before(&self, other: impl HasPosition) -> bool {
        self.end.offset <= other.position().offset
    }

    pub fn is_after(&self, other: impl HasPosition) -> bool {
        self.start.offset >= other.position().offset
    }
}

impl HasPosition for Position {
    fn position(&self) -> Position {
        *self
    }
}

impl HasSpan for Span {
    fn span(&self) -> Span {
        *self
    }
}

/// A blanket implementation that allows any `HasSpan` type to also be treated
/// as a `HasPosition` type, using the span's start as its position.
impl<T: HasSpan> HasPosition for T {
    fn position(&self) -> Position {
        self.start_position()
    }
}

impl HasFileId for Position {
    fn file_id(&self) -> FileId {
        self.file_id
    }
}

impl HasFileId for Span {
    fn file_id(&self) -> FileId {
        self.start.file_id
    }
}

/// Ergonomic blanket impl for references.
impl<T: HasSpan> HasSpan for &T {
    fn span(&self) -> Span {
        (*self).span()
    }
}

/// Ergonomic blanket impl for boxed values.
impl<T: HasSpan> HasSpan for Box<T> {
    fn span(&self) -> Span {
        self.as_ref().span()
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Range<usize> {
        span.to_range()
    }
}

impl From<&Span> for Range<usize> {
    fn from(span: &Span) -> Range<usize> {
        span.to_range()
    }
}

impl From<Position> for usize {
    fn from(position: Position) -> usize {
        position.offset
    }
}

impl From<&Position> for usize {
    fn from(position: &Position) -> usize {
        position.offset
    }
}

impl std::ops::Add<usize> for Position {
    type Output = Position;

    fn add(self, rhs: usize) -> Self::Output {
        self.forward(rhs)
    }
}

impl std::ops::Sub<usize> for Position {
    type Output = Position;

    fn sub(self, rhs: usize) -> Self::Output {
        self.backward(rhs)
    }
}

impl std::ops::AddAssign<usize> for Position {
    fn add_assign(&mut self, rhs: usize) {
        self.offset = self.offset.saturating_add(rhs);
    }
}

impl std::ops::SubAssign<usize> for Position {
    /// Moves the position backward in-place.
    fn sub_assign(&mut self, rhs: usize) {
        self.offset = self.offset.saturating_sub(rhs);
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.offset)
    }
}

impl std::fmt::Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start.offset, self.end.offset)
    }
}

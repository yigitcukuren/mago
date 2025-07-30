use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_span::HasSpan;
use mago_span::Span;

use crate::metadata::attribute::AttributeMetadata;
use crate::ttype::atomic::TAtomic;

/// Contains metadata associated with a specific `case` within a PHP `enum`.
///
/// Represents enum cases in both "pure" enums (e.g., `case Pending;` in `enum Status`)
/// and "backed" enums (e.g., `case Ok = 200;` in `enum HttpStatus: int`),
/// including associated attributes, values, and source locations.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnumCaseMetadata {
    pub attributes: Vec<AttributeMetadata>,
    pub name: StringIdentifier,
    pub name_span: Span,
    pub span: Span,
    pub value_type: Option<TAtomic>,
    pub is_backed: bool,
    pub is_deprecated: bool,
}

impl EnumCaseMetadata {
    /// Creates new `EnumCaseMetadata` for a case assumed initially to be non-backed (pure).
    ///
    /// Use modifier methods (`set_is_backed`, `with_is_backed`) later during analysis
    /// if the enum is determined to be backed.
    ///
    /// # Arguments
    /// * `name`: The identifier (name) of the enum case (e.g., `PENDING`).
    /// * `name_span`: The source code location of the name identifier.
    /// * `span`: The source code location of the entire case declaration.
    #[inline]
    pub fn new(name: StringIdentifier, name_span: Span, span: Span) -> Self {
        Self {
            attributes: Vec::new(),
            name,
            name_span,
            span,
            value_type: None,
            is_backed: false, // Assume pure initially
            is_deprecated: false,
        }
    }
}

impl HasSpan for EnumCaseMetadata {
    fn span(&self) -> Span {
        self.span
    }
}

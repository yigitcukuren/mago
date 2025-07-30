use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_span::HasSpan;
use mago_span::Span;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AttributeMetadata {
    pub name: StringIdentifier,
    pub span: Span,
}

impl HasSpan for AttributeMetadata {
    fn span(&self) -> Span {
        self.span
    }
}

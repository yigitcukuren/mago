use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::Type;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct IndexAccessType<'input> {
    pub target: Box<Type<'input>>,
    pub left_bracket: Span,
    pub index: Box<Type<'input>>,
    pub right_bracket: Span,
}

impl HasSpan for IndexAccessType<'_> {
    fn span(&self) -> Span {
        self.target.span().join(self.right_bracket)
    }
}

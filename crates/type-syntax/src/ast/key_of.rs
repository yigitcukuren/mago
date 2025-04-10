use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::generics::SingleGenericParameter;
use crate::ast::keyword::Keyword;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct KeyOfType<'input> {
    pub keyword: Keyword<'input>,
    pub parameter: SingleGenericParameter<'input>,
}

impl HasSpan for KeyOfType<'_> {
    fn span(&self) -> Span {
        self.keyword.span().join(self.parameter.span())
    }
}

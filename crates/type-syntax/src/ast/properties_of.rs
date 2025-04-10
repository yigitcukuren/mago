use serde::Serialize;
use strum::Display;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::generics::SingleGenericParameter;
use crate::ast::keyword::Keyword;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
#[repr(C)]
pub enum PropertiesOfFilter {
    All,
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct PropertiesOfType<'input> {
    pub filter: PropertiesOfFilter,
    pub keyword: Keyword<'input>,
    pub parameter: SingleGenericParameter<'input>,
}

impl HasSpan for PropertiesOfType<'_> {
    fn span(&self) -> Span {
        self.keyword.span().join(self.parameter.span())
    }
}

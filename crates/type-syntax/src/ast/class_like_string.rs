use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::generics::SingleGenericParameter;
use crate::ast::keyword::Keyword;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct ClassStringType<'input> {
    pub keyword: Keyword<'input>,
    pub parameter: Option<SingleGenericParameter<'input>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct InterfaceStringType<'input> {
    pub keyword: Keyword<'input>,
    pub parameter: Option<SingleGenericParameter<'input>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct EnumStringType<'input> {
    pub keyword: Keyword<'input>,
    pub parameter: Option<SingleGenericParameter<'input>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct TraitStringType<'input> {
    pub keyword: Keyword<'input>,
    pub parameter: Option<SingleGenericParameter<'input>>,
}

impl HasSpan for ClassStringType<'_> {
    fn span(&self) -> Span {
        match &self.parameter {
            Some(parameter) => self.keyword.span.join(parameter.span()),
            None => self.keyword.span,
        }
    }
}

impl HasSpan for InterfaceStringType<'_> {
    fn span(&self) -> Span {
        match &self.parameter {
            Some(parameter) => self.keyword.span.join(parameter.span()),
            None => self.keyword.span,
        }
    }
}

impl HasSpan for EnumStringType<'_> {
    fn span(&self) -> Span {
        match &self.parameter {
            Some(parameter) => self.keyword.span.join(parameter.span()),
            None => self.keyword.span,
        }
    }
}

impl HasSpan for TraitStringType<'_> {
    fn span(&self) -> Span {
        match &self.parameter {
            Some(parameter) => self.keyword.span.join(parameter.span()),
            None => self.keyword.span,
        }
    }
}

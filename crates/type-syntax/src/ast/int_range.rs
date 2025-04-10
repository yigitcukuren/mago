use serde::Serialize;
use strum::Display;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::keyword::Keyword;
use crate::ast::literal::LiteralIntType;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord, Display)]
#[serde(tag = "type", content = "value")]
#[repr(C)]
pub enum LiteralIntOrKeyword<'input> {
    LiteralInt(LiteralIntType<'input>),
    Keyword(Keyword<'input>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, PartialOrd, Ord)]
#[repr(C)]
pub struct IntRangeType<'input> {
    pub keyword: Keyword<'input>,
    pub less_than: Span,
    pub min: LiteralIntOrKeyword<'input>,
    pub comma: Span,
    pub max: LiteralIntOrKeyword<'input>,
    pub greater_than: Span,
}

impl HasSpan for LiteralIntOrKeyword<'_> {
    fn span(&self) -> Span {
        match self {
            LiteralIntOrKeyword::LiteralInt(literal) => literal.span(),
            LiteralIntOrKeyword::Keyword(keyword) => keyword.span(),
        }
    }
}

impl HasSpan for IntRangeType<'_> {
    fn span(&self) -> Span {
        self.keyword.span().join(self.greater_than.span())
    }
}

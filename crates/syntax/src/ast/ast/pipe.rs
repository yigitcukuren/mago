use serde::Deserialize;
use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::ast::expression::Expression;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Pipe {
    /// The expression whose value is passed as the first argument.
    pub input: Box<Expression>,
    /// The span of the pipe operator `|>`.
    pub operator: Span,
    /// The expression that must resolve to a callable.
    pub callable: Box<Expression>,
}

impl HasSpan for Pipe {
    fn span(&self) -> Span {
        self.input.span().join(self.callable.span())
    }
}

use serde::Deserialize;
use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

use crate::ast::ast::argument::ArgumentList;
use crate::ast::ast::expression::Expression;
use crate::ast::ast::keyword::Keyword;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Instantiation {
    pub new: Keyword,
    pub class: Box<Expression>,
    pub argument_list: Option<ArgumentList>,
}

impl HasSpan for Instantiation {
    fn span(&self) -> Span {
        if let Some(argument_list) = &self.argument_list {
            self.new.span().join(argument_list.span())
        } else {
            self.new.span().join(self.class.span())
        }
    }
}

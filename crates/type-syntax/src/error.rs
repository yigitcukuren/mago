use serde::Serialize;

use mago_span::HasPosition;
use mago_span::HasSpan;
use mago_span::Position;
use mago_span::Span;

use crate::token::TypeTokenKind;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub enum SyntaxError {
    UnexpectedToken(u8, Position),
    UnrecognizedToken(u8, Position),
    UnexpectedEndOfFile(Position),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum ParseError {
    SyntaxError(SyntaxError),
    UnexpectedEndOfFile(Vec<TypeTokenKind>, Position),
    UnexpectedToken(Vec<TypeTokenKind>, TypeTokenKind, Span),
    UnclosedLiteralString(Span),
}

impl From<SyntaxError> for ParseError {
    fn from(error: SyntaxError) -> Self {
        ParseError::SyntaxError(error)
    }
}

impl HasPosition for SyntaxError {
    fn position(&self) -> Position {
        match self {
            SyntaxError::UnexpectedToken(_, position) => *position,
            SyntaxError::UnrecognizedToken(_, position) => *position,
            SyntaxError::UnexpectedEndOfFile(position) => *position,
        }
    }
}

impl HasSpan for ParseError {
    fn span(&self) -> Span {
        match self {
            ParseError::SyntaxError(error) => {
                let position = error.position();

                Span::new(position, position)
            }
            ParseError::UnexpectedEndOfFile(_, position) => Span::new(*position, *position),
            ParseError::UnexpectedToken(_, _, span) => *span,
            ParseError::UnclosedLiteralString(span) => *span,
        }
    }
}

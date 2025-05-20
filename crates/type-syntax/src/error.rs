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

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::UnexpectedToken(token, position) => {
                write!(f, "Unexpected token {} at {}", token, position)
            }
            SyntaxError::UnrecognizedToken(token, position) => {
                write!(f, "Unrecognized token {} at {}", token, position)
            }
            SyntaxError::UnexpectedEndOfFile(position) => {
                write!(f, "Unexpected end of file at {}", position)
            }
        }
    }
}

impl std::error::Error for SyntaxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::SyntaxError(err) => write!(f, "{}", err),
            ParseError::UnexpectedEndOfFile(_, position) => {
                write!(f, "Unexpected end of file at {}", position)
            }
            ParseError::UnexpectedToken(_, token, span) => {
                write!(f, "Unexpected token {:?} at {}", token, span)
            }
            ParseError::UnclosedLiteralString(span) => {
                write!(f, "Unclosed literal string at {}", span)
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::SyntaxError(err) => Some(err),
            _ => None,
        }
    }
}

use serde::Deserialize;
use serde::Serialize;

use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasPosition;
use mago_span::HasSpan;
use mago_span::Position;
use mago_span::Span;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum SyntaxError {
    UnexpectedToken(u8, Position),
    UnrecognizedToken(u8, Position),
}

impl HasSpan for SyntaxError {
    fn span(&self) -> Span {
        let position = match self {
            Self::UnexpectedToken(_, p) => *p,
            Self::UnrecognizedToken(_, p) => *p,
        };

        Span::new(position, Position { offset: position.offset + 1, ..position })
    }
}

impl std::fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::UnexpectedToken(token, _) => &format!("Unexpected token `{}` (0x{:02X})", *token as char, token),
            Self::UnrecognizedToken(token, _) => &format!("Unrecognised token `{}` (0x{:02X})", *token as char, token),
        };

        write!(f, "{}", message)
    }
}

impl std::error::Error for SyntaxError {}

impl From<SyntaxError> for Issue {
    fn from(error: SyntaxError) -> Issue {
        let position = error.position();
        let span = Span::new(position, Position { offset: position.offset + 1, ..position });

        Issue::error(error.to_string()).with_annotation(Annotation::primary(span).with_message("Syntax error."))
    }
}

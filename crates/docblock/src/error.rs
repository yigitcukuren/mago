use serde::Deserialize;
use serde::Serialize;

use mago_span::HasSpan;
use mago_span::Span;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParseError {
    InvalidTrivia(Span),
    UnclosedInlineTag(Span),
    UnclosedInlineCode(Span),
    UnclosedCodeBlock(Span),
    InvalidTagName(Span),
    InvalidAnnotationName(Span),
    UnclosedAnnotationArguments(Span),
    MalformedCodeBlock(Span),
    InvalidComment(Span),
    InconsistentIndentation(Span, usize, usize),
    MissingAsterisk(Span),
    MissingWhitespaceAfterAsterisk(Span),
    MissingWhitespaceAfterOpeningAsterisk(Span),
    MissingWhitespaceBeforeClosingAsterisk(Span),
    ExpectedLine(Span),
}

impl HasSpan for ParseError {
    fn span(&self) -> Span {
        match self {
            ParseError::InvalidTrivia(span)
            | ParseError::UnclosedInlineTag(span)
            | ParseError::UnclosedInlineCode(span)
            | ParseError::UnclosedCodeBlock(span)
            | ParseError::InvalidTagName(span)
            | ParseError::InvalidAnnotationName(span)
            | ParseError::UnclosedAnnotationArguments(span)
            | ParseError::MalformedCodeBlock(span)
            | ParseError::InvalidComment(span)
            | ParseError::InconsistentIndentation(span, _, _)
            | ParseError::MissingAsterisk(span)
            | ParseError::MissingWhitespaceAfterAsterisk(span)
            | ParseError::MissingWhitespaceAfterOpeningAsterisk(span)
            | ParseError::MissingWhitespaceBeforeClosingAsterisk(span)
            | ParseError::ExpectedLine(span) => *span,
        }
    }
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidTrivia(_) | ParseError::InvalidComment(_) => {
                write!(f, "Invalid docblock format")
            }
            ParseError::UnclosedInlineTag(_) => write!(f, "Unclosed inline tag"),
            ParseError::UnclosedInlineCode(_) => write!(f, "Unclosed inline code"),
            ParseError::UnclosedCodeBlock(_) => write!(f, "Unclosed code block"),
            ParseError::InvalidTagName(_) => write!(f, "Invalid tag name"),
            ParseError::InvalidAnnotationName(_) => write!(f, "Invalid annotation name"),
            ParseError::UnclosedAnnotationArguments(_) => write!(f, "Unclosed annotation arguments"),
            ParseError::MalformedCodeBlock(_) => write!(f, "Malformed code block"),
            ParseError::InconsistentIndentation(_, expected, actual) => {
                write!(f, "Inconsistent indentation: expected {expected}, found {actual}")
            }
            ParseError::MissingAsterisk(_) => write!(f, "Missing leading asterisk on docblock line"),
            ParseError::MissingWhitespaceAfterAsterisk(_) => {
                write!(f, "Missing space after leading asterisk")
            }
            ParseError::MissingWhitespaceAfterOpeningAsterisk(_)
            | ParseError::MissingWhitespaceBeforeClosingAsterisk(_) => {
                write!(f, "Improperly formatted single-line docblock")
            }
            ParseError::ExpectedLine(_) => write!(f, "Unexpected end of docblock"),
        }
    }
}

impl ParseError {
    pub fn note(&self) -> String {
        match self {
            ParseError::InvalidTrivia(_) | ParseError::InvalidComment(_) => {
                "Docblocks must start with `/**` and end with `*/`.".to_string()
            }
            ParseError::UnclosedInlineTag(_) => {
                "Inline tags like `{@see}` must be closed with a matching `}`.".to_string()
            }
            ParseError::UnclosedInlineCode(_) => {
                "Inline code snippets must be enclosed in matching backticks (`).".to_string()
            }
            ParseError::UnclosedCodeBlock(_) => {
                "Multi-line code blocks must be terminated with a closing ```.".to_string()
            }
            ParseError::InvalidTagName(_) => {
                "Docblock tags like `@param` must contain only letters, numbers, and hyphens.".to_string()
            }
            ParseError::InvalidAnnotationName(_) => {
                "Annotations must start with an uppercase letter, `_`, or `\\`.".to_string()
            }
            ParseError::UnclosedAnnotationArguments(_) => {
                "Arguments for an annotation must be enclosed in parentheses `()`.".to_string()
            }
            ParseError::MalformedCodeBlock(_) => {
                "A code block must start with ``` optionally followed by a language identifier.".to_string()
            }
            ParseError::InconsistentIndentation(_, expected, actual) => {
                format!(
                    "This line has an indentation of {actual}, but {expected} was expected based on the first line."
                )
            }
            ParseError::MissingAsterisk(_) => {
                "Each line in a multi-line docblock should start with an aligned asterisk `*`.".to_string()
            }
            ParseError::MissingWhitespaceAfterAsterisk(_) => {
                "A space is required after the leading `*` to separate it from the content.".to_string()
            }
            ParseError::MissingWhitespaceAfterOpeningAsterisk(_)
            | ParseError::MissingWhitespaceBeforeClosingAsterisk(_) => {
                "Single-line docblocks should have spaces padding the content, like `/** content */`.".to_string()
            }
            ParseError::ExpectedLine(_) => {
                "A tag or description was expected here, but the docblock ended prematurely.".to_string()
            }
        }
    }

    pub fn help(&self) -> String {
        match self {
            ParseError::UnclosedInlineTag(_) => "Add a closing `}` to complete the inline tag.".to_string(),
            ParseError::UnclosedInlineCode(_) => {
                "Add a closing backtick ` ` ` to terminate the inline code.".to_string()
            }
            ParseError::UnclosedCodeBlock(_) => "Add a closing ``` to terminate the code block.".to_string(),
            ParseError::InvalidTagName(_) => {
                "Correct the tag name to use only valid characters (e.g., `@my-custom-tag`).".to_string()
            }
            ParseError::InvalidAnnotationName(_) => {
                "Correct the annotation name to follow PSR-5 standards.".to_string()
            }
            ParseError::UnclosedAnnotationArguments(_) => {
                "Add a closing `)` to complete the annotation's argument list.".to_string()
            }
            ParseError::InconsistentIndentation(_, _, _) => {
                "Adjust the indentation to be consistent across all lines in the docblock.".to_string()
            }
            ParseError::MissingAsterisk(_) => "Add a leading `*` to the beginning of this line.".to_string(),
            ParseError::MissingWhitespaceAfterAsterisk(_) => {
                "Insert a space after the `*` at the beginning of the line.".to_string()
            }
            _ => "Review the docblock syntax to ensure it is correctly formatted.".to_string(),
        }
    }
}

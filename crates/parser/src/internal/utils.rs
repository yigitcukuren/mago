use fennec_ast::ast::*;
use fennec_span::Span;
use fennec_token::Token;
use fennec_token::TokenKind;

use crate::error::ParseError;
use crate::internal::token_stream::TokenStream;

pub fn peek(stream: &mut TokenStream<'_, '_>) -> Result<Token, ParseError> {
    match stream.peek() {
        Some(Ok(token)) => Ok(token),
        Some(Err(error)) => Err(error.into()),
        None => Err(unexpected(stream, None, &[])),
    }
}

pub fn maybe_peek(stream: &mut TokenStream<'_, '_>) -> Result<Option<Token>, ParseError> {
    match stream.peek() {
        Some(Ok(token)) => Ok(Some(token)),
        Some(Err(error)) => Err(error.into()),
        None => Ok(None),
    }
}

pub fn peek_nth(stream: &mut TokenStream<'_, '_>, n: usize) -> Result<Token, ParseError> {
    match stream.peek_nth(n) {
        Some(Ok(token)) => Ok(token),
        Some(Err(error)) => Err(error.into()),
        None => Err(unexpected(stream, None, &[])),
    }
}

pub fn maybe_peek_nth(stream: &mut TokenStream<'_, '_>, n: usize) -> Result<Option<Token>, ParseError> {
    match stream.peek_nth(n) {
        Some(Ok(token)) => Ok(Some(token)),
        Some(Err(error)) => Err(error.into()),
        None => Ok(None),
    }
}

pub fn expect_any(stream: &mut TokenStream<'_, '_>) -> Result<Token, ParseError> {
    match stream.advance() {
        Some(Ok(token)) => Ok(token),
        Some(Err(error)) => Err(error.into()),
        None => Err(unexpected(stream, None, &[])),
    }
}

pub fn expect(stream: &mut TokenStream<'_, '_>, kind: TokenKind) -> Result<Token, ParseError> {
    let token = expect_any(stream)?;

    if kind == token.kind {
        Ok(token)
    } else {
        Err(unexpected(stream, Some(token), &[kind]))
    }
}

pub fn expect_one_of(stream: &mut TokenStream<'_, '_>, one_of: &[TokenKind]) -> Result<Token, ParseError> {
    let token = expect_any(stream)?;

    if one_of.contains(&token.kind) {
        Ok(token)
    } else {
        Err(unexpected(stream, Some(token), one_of))
    }
}

pub fn maybe_expect(stream: &mut TokenStream<'_, '_>, kind: TokenKind) -> Result<Option<Token>, ParseError> {
    let next = match stream.peek() {
        Some(Ok(token)) => token,
        Some(Err(error)) => return Err(error.into()),
        None => return Ok(None),
    };

    if kind == next.kind {
        let token = match stream.advance() {
            Some(result) => result?,
            None => unreachable!("the token was peeked, so it should be available"),
        };

        Ok(Some(token))
    } else {
        Ok(None)
    }
}

pub fn expect_span(stream: &mut TokenStream<'_, '_>, kind: TokenKind) -> Result<Span, ParseError> {
    expect(stream, kind).map(|token| token.span)
}

pub fn expect_one_of_keyword(stream: &mut TokenStream<'_, '_>, one_of: &[TokenKind]) -> Result<Keyword, ParseError> {
    expect_one_of(stream, one_of).map(to_keyword)
}

pub fn maybe_expect_keyword(stream: &mut TokenStream<'_, '_>, kind: TokenKind) -> Result<Option<Keyword>, ParseError> {
    maybe_expect(stream, kind).map(|maybe_token| maybe_token.map(to_keyword))
}

pub fn expect_keyword(stream: &mut TokenStream<'_, '_>, kind: TokenKind) -> Result<Keyword, ParseError> {
    expect(stream, kind).map(to_keyword)
}

pub fn expect_any_keyword(stream: &mut TokenStream<'_, '_>) -> Result<Keyword, ParseError> {
    expect_any(stream).map(to_keyword)
}

pub fn to_keyword(token: Token) -> Keyword {
    Keyword { span: token.span, value: token.value }
}

pub fn unexpected(stream: &mut TokenStream<'_, '_>, token: Option<Token>, one_of: &[TokenKind]) -> ParseError {
    if let Some(token) = token {
        ParseError::UnexpectedToken(one_of.to_vec(), token.kind, token.span)
    } else {
        ParseError::UnexpectedEndOfFile(one_of.to_vec(), stream.get_position())
    }
}

use mago_ast::ast::*;
use mago_token::T;

use crate::error::ParseError;
use crate::internal::identifier::parse_local_identifier;
use crate::internal::terminator::parse_terminator;
use crate::internal::token_stream::TokenStream;
use crate::internal::utils;

pub fn parse_goto(stream: &mut TokenStream<'_, '_>) -> Result<Goto, ParseError> {
    Ok(Goto {
        goto: utils::expect_keyword(stream, T!["goto"])?,
        label: parse_local_identifier(stream)?,
        terminator: parse_terminator(stream)?,
    })
}

pub fn parse_label(stream: &mut TokenStream<'_, '_>) -> Result<Label, ParseError> {
    Ok(Label { name: parse_local_identifier(stream)?, colon: utils::expect_span(stream, T![":"])? })
}

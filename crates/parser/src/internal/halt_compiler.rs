use mago_ast::ast::*;
use mago_token::T;

use crate::error::ParseError;
use crate::internal::terminator::parse_terminator;
use crate::internal::token_stream::TokenStream;
use crate::internal::utils;

pub fn parse_halt_compiler(stream: &mut TokenStream<'_, '_>) -> Result<HaltCompiler, ParseError> {
    Ok(HaltCompiler {
        halt_compiler: utils::expect_one_of_keyword(stream, &[T!["__halt_compiler"]])?,
        left_parenthesis: utils::expect_span(stream, T!["("])?,
        right_parenthesis: utils::expect_span(stream, T![")"])?,
        terminator: parse_terminator(stream)?,
    })
}

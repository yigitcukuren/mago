use mago_ast::ast::*;
use mago_ast::sequence::Sequence;
use mago_token::T;

use crate::error::ParseError;
use crate::internal::statement::parse_statement;
use crate::internal::token_stream::TokenStream;
use crate::internal::utils;

pub fn parse_block(stream: &mut TokenStream<'_, '_>) -> Result<Block, ParseError> {
    Ok(Block {
        left_brace: utils::expect_span(stream, T!["{"])?,
        statements: {
            let mut statements = Vec::new();
            loop {
                let next = utils::peek(stream)?;
                if matches!(next.kind, T!["}"]) {
                    break;
                }

                statements.push(parse_statement(stream)?);
            }

            Sequence::new(statements)
        },
        right_brace: utils::expect_span(stream, T!["}"])?,
    })
}

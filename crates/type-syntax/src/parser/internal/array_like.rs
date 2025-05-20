use crate::ast::*;
use crate::error::ParseError;
use crate::parser::internal::generic::parse_generic_parameters_or_none;
use crate::parser::internal::parse_type;
use crate::parser::internal::stream::TypeTokenStream;
use crate::token::TypeTokenKind;

#[inline]
pub fn parse_array_like_type<'input>(stream: &mut TypeTokenStream<'input>) -> Result<Type<'input>, ParseError> {
    let next = stream.peek()?;
    let (keyword, kind) = match next.kind {
        TypeTokenKind::Array => {
            let keyword = Keyword::from(stream.consume()?);
            if !stream.is_at(TypeTokenKind::LeftBrace)? {
                return Ok(Type::Array(ArrayType { keyword, parameters: parse_generic_parameters_or_none(stream)? }));
            }

            (keyword, ShapeTypeKind::Array)
        }
        TypeTokenKind::NonEmptyArray => {
            let keyword = Keyword::from(stream.consume()?);
            if !stream.is_at(TypeTokenKind::LeftBrace)? {
                return Ok(Type::NonEmptyArray(NonEmptyArrayType {
                    keyword,
                    parameters: parse_generic_parameters_or_none(stream)?,
                }));
            }

            (keyword, ShapeTypeKind::NonEmptyArray)
        }
        TypeTokenKind::AssociativeArray => {
            let keyword = Keyword::from(stream.consume()?);
            if !stream.is_at(TypeTokenKind::LeftBrace)? {
                return Ok(Type::AssociativeArray(AssociativeArrayType {
                    keyword,
                    parameters: parse_generic_parameters_or_none(stream)?,
                }));
            }

            (keyword, ShapeTypeKind::AssociativeArray)
        }
        TypeTokenKind::List => {
            let keyword = Keyword::from(stream.consume()?);
            if !stream.is_at(TypeTokenKind::LeftBrace)? {
                return Ok(Type::List(ListType { keyword, parameters: parse_generic_parameters_or_none(stream)? }));
            }

            (keyword, ShapeTypeKind::List)
        }
        TypeTokenKind::NonEmptyList => {
            let keyword = Keyword::from(stream.consume()?);
            if !stream.is_at(TypeTokenKind::LeftBrace)? {
                return Ok(Type::NonEmptyList(NonEmptyListType {
                    keyword,
                    parameters: parse_generic_parameters_or_none(stream)?,
                }));
            }

            (keyword, ShapeTypeKind::NonEmptyList)
        }
        _ => {
            return Err(ParseError::UnexpectedToken(
                vec![
                    TypeTokenKind::Array,
                    TypeTokenKind::NonEmptyArray,
                    TypeTokenKind::AssociativeArray,
                    TypeTokenKind::List,
                    TypeTokenKind::NonEmptyList,
                ],
                next.kind,
                next.span,
            ));
        }
    };

    Ok(Type::Shape(ShapeType {
        kind,
        keyword,
        left_brace: stream.eat(TypeTokenKind::LeftBrace)?.span,
        fields: {
            let mut fields = Vec::new();
            while !stream.is_at(TypeTokenKind::RightBrace)? && !stream.is_at(TypeTokenKind::Ellipsis)? {
                let mut has_key = false;
                if stream.is_at(TypeTokenKind::LiteralString)?
                    || stream.is_at(TypeTokenKind::LiteralInteger)?
                    || stream.is_at(TypeTokenKind::LiteralFloat)?
                    || stream.is_at(TypeTokenKind::Identifier)?
                {
                    has_key = matches!(
                        stream.lookahead(1)?.map(|t| t.kind),
                        Some(TypeTokenKind::Colon | TypeTokenKind::Question)
                    );
                } else if stream.is_at(TypeTokenKind::Minus)? || stream.is_at(TypeTokenKind::Plus)? {
                    has_key = matches!(
                        stream.lookahead(1)?.map(|t| t.kind),
                        Some(TypeTokenKind::LiteralInteger | TypeTokenKind::LiteralFloat)
                    ) && matches!(
                        stream.lookahead(2)?.map(|t| t.kind),
                        Some(TypeTokenKind::Colon | TypeTokenKind::Question)
                    );
                };

                let field = ShapeField {
                    key: if has_key {
                        Some(ShapeFieldKey {
                            name: Box::new(parse_type(stream)?),
                            question_mark: if stream.is_at(TypeTokenKind::Question)? {
                                Some(stream.consume()?.span)
                            } else {
                                None
                            },
                            colon: stream.eat(TypeTokenKind::Colon)?.span,
                        })
                    } else {
                        None
                    },
                    value: Box::new(parse_type(stream)?),
                    comma: if stream.is_at(TypeTokenKind::Comma)? { Some(stream.consume()?.span) } else { None },
                };

                if field.comma.is_none() {
                    fields.push(field);
                    break;
                }

                fields.push(field);
            }

            fields
        },
        additional_fields: {
            if !stream.is_at(TypeTokenKind::Ellipsis)? {
                None
            } else {
                Some(ShapeAdditionalFields {
                    ellipsis: stream.consume()?.span,
                    parameters: parse_generic_parameters_or_none(stream)?,
                })
            }
        },
        right_brace: stream.eat(TypeTokenKind::RightBrace)?.span,
    }))
}

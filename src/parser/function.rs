use super::{block::block_parser, token::Token};
use crate::ast::{Function, Parameter, Type};
use chumsky::{input::ValueInput, prelude::*};

pub fn function_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Function, extra::Err<Rich<'tokens, Token>>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let type_ = select! {
        Token::Identifier(name) if name == "int" => Type::Int,
        Token::Identifier(name) if name == "float" => Type::Float,
        Token::Identifier(name) if name == "void" => Type::Void,
    };

    let parameter = type_
        .clone()
        .then(select! {Token::Identifier(name) => name})
        .map(|(param_type, name)| Parameter { name, param_type });

    let param_list = parameter
        .separated_by(just(Token::Comma))
        .allow_trailing()
        .collect()
        .delimited_by(just(Token::LeftParen), just(Token::RightParen));

    type_
        .then(select! {Token::Identifier(name) => name})
        .then(param_list)
        .then(block_parser())
        .map(|(((ret_type, name), params), body)| Function {
            name,
            params,
            body,
            ret_type,
        })
}

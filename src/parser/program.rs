use super::{function::function_parser, token::Token};
use crate::ast::Ast;
use chumsky::{input::ValueInput, prelude::*};

pub fn program_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Ast, extra::Err<Rich<'tokens, Token>>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    function_parser()
        .repeated()
        .collect()
        .map(|functions| Ast { program: functions })
}

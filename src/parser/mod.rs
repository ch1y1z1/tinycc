mod block;
mod expr;
mod function;
mod program;
mod token;

use crate::ast::Ast;
use chumsky::{
    Parser,
    error::Rich,
    input::{Input, Stream},
};
use eros::{Context, IntoDynTracedError, Result};
use logos::Logos;
use program::program_parser;
use token::Token;

pub fn parse(input: &str) -> Result<Ast> {
    let tokens = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
        Ok(t) => (t, span.into()),
        Err(_) => (Token::Error, span.into()),
    });

    let token_stream =
        Stream::from_iter(tokens).map((0..input.len()).into(), |(tok, span): (_, _)| (tok, span));

    program_parser()
        .parse(token_stream)
        .into_result()
        .map_err(|e| ParseError::from_vec_rich(e))
        .traced_dyn()
        .context("ParseError")
}

#[derive(Debug, Clone)]
struct ParseError(String);

impl ParseError {
    fn from_vec_rich(errs: Vec<Rich<'_, Token>>) -> Self {
        let mut msg = String::new();
        for err in errs {
            msg.push_str(&format!("{}\n", err));
        }
        ParseError(msg)
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

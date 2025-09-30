use super::{expr::expr_parser, token::Token};
use crate::ast::{Block, BlockItem, Declaration, Statement, Type};
use chumsky::{input::ValueInput, prelude::*};

pub fn block_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Block, extra::Err<Rich<'tokens, Token>>>
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    let mut statement = Recursive::declare();
    let mut block = Recursive::declare();

    let type_ = select! {
        Token::Identifier(name) if name == "int" => Type::Int,
        Token::Identifier(name) if name == "float" => Type::Float,
        Token::Identifier(name) if name == "void" => Type::Void,
    };

    let declaration = type_
        .then(select! {Token::Identifier(name) => name})
        .then(just(Token::Assign).ignore_then(expr_parser::<I>()).or_not())
        .then_ignore(just(Token::Semicolon))
        .map(|((type_, name), expr)| Declaration {
            var_type: type_,
            name,
            init: expr,
        });

    let block_item = choice((
        statement.clone().map(|stmt| BlockItem::Stmt(stmt)),
        declaration.map(|decl| BlockItem::Decl(decl)),
    ));

    let block_impl = block_item
        .repeated()
        .collect()
        .delimited_by(just(Token::LeftBrace), just(Token::RightBrace))
        .map(|items| Block { items });

    block.define(block_impl);

    let return_stmt = just(Token::Return)
        .ignore_then(expr_parser::<I>().or_not())
        .then_ignore(just(Token::Semicolon))
        .map(|expr| Statement::Ret(expr));

    let if_stmt = just(Token::If)
        .ignore_then(
            expr_parser::<I>().delimited_by(just(Token::LeftParen), just(Token::RightParen)),
        )
        .then(statement.clone())
        .then(just(Token::Else).ignore_then(statement.clone()).or_not())
        .map(|((condition, then_branch), else_branch)| Statement::If {
            condition,
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(Box::new),
        });

    let expr_stmt = expr_parser::<I>()
        .then_ignore(just(Token::Semicolon))
        .map(Statement::Expr);

    // TODO: while stmt, empty stmt

    let statement_impl = choice((
        block.clone().map(Statement::Block),
        return_stmt,
        if_stmt,
        expr_stmt,
    ));
    statement.define(statement_impl);

    block
}

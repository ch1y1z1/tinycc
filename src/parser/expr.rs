use super::token::Token;
use crate::ast::{BinaryOperator, Expression, LValue, Literal, UnaryOperator};
use chumsky::{input::ValueInput, prelude::*};

pub fn expr_parser<'tokens, 'src: 'tokens, I>()
-> impl Parser<'tokens, I, Expression, extra::Err<Rich<'tokens, Token>>> + Clone
where
    I: ValueInput<'tokens, Token = Token, Span = SimpleSpan>,
{
    recursive(|expr| {
        let literal = select! {
            Token::IntLiteral(n) => Expression::Literal(Literal::Int(n.parse().unwrap())),
            Token::Identifier(name) => Expression::Variable(name),
        };

        let primary = expr
            .clone()
            .delimited_by(just(Token::LeftParen), just(Token::RightParen))
            .or(literal);

        let function_call = primary
            .clone()
            .then(
                expr.clone()
                    .separated_by(just(Token::Comma))
                    .allow_trailing()
                    .collect()
                    .delimited_by(just(Token::LeftParen), just(Token::RightParen)),
            )
            .map(|(p, args)| Expression::FunctionCall {
                callee: Box::new(p),
                arguments: args,
            })
            .or(primary);

        let unary = choice((
            just(Token::Plus).to(UnaryOperator::Plus),
            just(Token::Minus).to(UnaryOperator::Negate),
        ))
        .or_not()
        .then(function_call)
        .map(|(op, fc)| {
            if let Some(op) = op {
                Expression::Unary {
                    operator: op,
                    operand: Box::new(fc),
                }
            } else {
                fc
            }
        });

        let multiplicative = unary.clone().foldl(
            choice((
                just(Token::Asterisk).to(BinaryOperator::Multiply),
                just(Token::Slash).to(BinaryOperator::Divide),
                just(Token::Percent).to(BinaryOperator::Modulus),
            ))
            .then(unary)
            .repeated(),
            |l, (op, r)| Expression::Binary {
                left: Box::new(l),
                operator: op,
                right: Box::new(r),
            },
        );

        let additive = multiplicative.clone().foldl(
            choice((
                just(Token::Plus).to(BinaryOperator::Add),
                just(Token::Minus).to(BinaryOperator::Subtract),
            ))
            .then(multiplicative)
            .repeated(),
            |l, (op, r)| Expression::Binary {
                left: Box::new(l),
                operator: op,
                right: Box::new(r),
            },
        );

        let relational = additive.clone().foldl(
            choice((
                just(Token::Greater).to(BinaryOperator::Greater),
                just(Token::GreaterEqual).to(BinaryOperator::GreaterEqual),
                just(Token::Less).to(BinaryOperator::Less),
                just(Token::LessEqual).to(BinaryOperator::LessEqual),
            ))
            .then(additive)
            .repeated(),
            |l, (op, r)| Expression::Binary {
                left: Box::new(l),
                operator: op,
                right: Box::new(r),
            },
        );

        // logical_or logical_and

        let assignment = recursive(|assignment| {
            relational
                .clone()
                .then(just(Token::Assign).ignore_then(assignment))
                .map(|(var, value)| Expression::Assignment {
                    target: match var {
                        Expression::Variable(name) => LValue::Var(name),
                        _ => panic!("Invalid assignment target"),
                    },
                    value: Box::new(value),
                })
                .or(relational)
        });

        assignment
    })
}

#[test]
fn test() {
    use chumsky::input::Stream;
    use logos::Logos;
    let input = r#"1 + 2 / s * (2 - b)"#;
    let tokens = Token::lexer(&input).spanned().map(|(tok, span)| match tok {
        Ok(t) => (t, span.into()),
        Err(_) => (Token::Error, span.into()),
    });

    let token_stream =
        Stream::from_iter(tokens).map((0..input.len()).into(), |(tok, span): (_, _)| (tok, span));

    let expr = expr_parser().parse(token_stream).into_result().unwrap();
}

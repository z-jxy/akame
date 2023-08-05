use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};

use nom::sequence::{delimited, tuple, terminated};
use nom::IResult;

use crate::ast::{Expression, Statement};
use super::functions::ws;
use super::tokens::{parse_identifier, expr};

pub fn parse_print_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = ws(tag("println!"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;
    Ok((input, Statement::Print(expr)))
}

pub fn parse_let_statement(input: &str) -> IResult<&str, Statement> {
    map(
        tuple((
            delimited(opt(multispace0), 
            tag("let"),
            multispace0
            ),
            terminated(
            parse_identifier,
            opt(multispace0)
            ),
            tag("="),
            delimited(
                opt(multispace0),
            expr,
            opt(multispace0)
            ),
            tag(";"),
        )),
        |(_let,  ident,  _assignment_op, expr, _semicolon,)| match ident {
            Expression::Identifier(id) => Statement::Let(id.to_string(), expr),
            _ => panic!("Expecting identifier in let statement"),
        }
    )(input)
}

pub fn parse_return_statement(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(";")(input)?; // Expecting a semicolon after the expression
    Ok((input, Statement::Return(expr)))
}

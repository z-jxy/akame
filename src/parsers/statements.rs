use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, char, alphanumeric0, anychar, digit1};
use nom::combinator::{recognize, map, opt};
use nom::multi::{separated_list0, many0};
use nom::sequence::{delimited, tuple, terminated};
use nom::{IResult, Parser};

use crate::ast::{Expression, Statement};
use crate::types::integer::{parse_large_integer, Integer};

use super::tokens::{parse_identifier, expr};

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

    Ok((input, Statement::Return(expr)))
}
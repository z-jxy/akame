use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};

use nom::sequence::{delimited, tuple, terminated};
use nom::IResult;


use crate::llvm::ast::{Stmt, Expr};

use super::expressions::expression;
use super::tokens::parse_identifier;
/*
pub fn parse_print_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = opt(multispace0)(input)?; // Optional whitespace
    let (input, _) = tag("println!")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = delimited(tag("("), expression, tag(")"))(input)?;
    //println!("Parsed print statement: {:?}", expr);
    Ok((input, Statement::Print(expr)))
}
 */

pub fn parse_let_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = opt(multispace0)(input)?; // Optional whitespace
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
            expression,
            opt(multispace0)
            ),
            tag(";"),

        )),
        |(_let,  ident,  _assignment_op, expr, ..)| match ident {
            Expr::Ident(id) => Stmt::Assignment { ident: id, expr: expr },
            _ => panic!("Expecting identifier in let statement"),
        }
    )(input)
}

pub fn parse_return_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("return")(input)?;
    //println!("Parsing return statement");
    //println!("Input: {:?}", input);
    let (input, _) = multispace0(input)?;
    let (input, expr) = expression(input)?;
    //println!("Input2: {:?}", input);
    let (input, _) = multispace0(input)?;
    let (input, _) = tag(";")(input)?; // Expecting a semicolon after the expression
    Ok((input, Stmt::Return(expr)))
}


use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;


use nom::sequence::delimited;
use nom::{InputTakeAtPosition, AsChar, Parser};


use crate::llvm::ast::{Stmt, Expr};

use super::ParseResult;
use super::error::CustomError;
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
fn space_opt<I, O, F>(parser: F) -> impl FnMut(I) -> ParseResult<I, O>
where
    I: InputTakeAtPosition + Clone + PartialEq,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    F: Parser<I, O, CustomError<I>>,
{
    delimited(opt(multispace0), parser, opt(multispace0))
}

pub fn parse_let_statement(input: &str) -> ParseResult<&str, Stmt> {
    let (input, _) = opt(multispace0)(input)?;

    let (input, _) = space_opt(tag("let"))(input)?;
    let (input, ident) = space_opt(parse_identifier)(input)?;
    let (input, _) = space_opt(tag("="))(input)?;
    let (input, expr) = space_opt(expression)(input)?;
    let (input, _) = tag(";")(input)?;

    match ident {
        Expr::Ident(id) => Ok((input, Stmt::Assignment { ident: id, expr })),
        _ => Err(nom::Err::Error(CustomError::UnexpectedToken("Expected identifier"))),
    }
}

pub fn parse_return_statement(input: &str) -> ParseResult<&str, Stmt> {
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


use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;


use nom::sequence::delimited;
use nom::{InputTakeAtPosition, AsChar, Parser};


use crate::llvm::ast::{Stmt, Expr};
use crate::parsers::expressions::{parse_primary_expr, parse_function_call, parse_infix_expr};

use super::ParseResult;
use super::error::CustomError;
use super::expressions::expression;
use super::tokens::parse_identifier;


pub fn space_opt<I, O, F>(parser: F) -> impl FnMut(I) -> ParseResult<I, O>
where
    I: InputTakeAtPosition + Clone + PartialEq,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    F: Parser<I, O, CustomError<I>>,
{
    delimited(opt(multispace0), parser, opt(multispace0))
}

pub fn parse_let_statement(input: &str) -> ParseResult<&str, Stmt> {
    //println!("let stmnt input: {:?}", input);
    let (input, _) = space_opt(tag("let"))(input)?;
    //println!("its a let stmnt");
    let (input, ident) = space_opt(parse_identifier)(input)?;
    //println!("let after ident: {}", input);
    //println!("let stmnt IDENT: {:?}", ident);
    let (input, _) = space_opt(tag("="))(input)?;
    //println!("let after EQUALS: {}", input);
    let (input, expr) = space_opt(alt((parse_function_call, parse_infix_expr, parse_primary_expr)))(input)?;
    //println!("let after expr: {}", input);
    let (input, _) = space_opt(tag(";"))(input)?;

    //println!("stmnt PARSED: {}", input);

    match ident {
        Expr::Ident(id) => Ok((input, Stmt::Assignment { ident: id, expr })),
        _ => Err(nom::Err::Failure(CustomError::UnexpectedToken("Expected identifier"))),
    }
}

pub fn parse_return_statement(input: &str) -> ParseResult<&str, Stmt> {
    let (input, _) = space_opt(tag("return"))(input)?;
    let (input, expr) = space_opt(expression)(input)?;
    //let (input, _) = space_opt(tag(";"))(input)?;
    Ok((input, Stmt::Return(expr)))
}


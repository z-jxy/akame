use nom::combinator::opt;
use nom::{branch::alt, multi::separated_list0};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::error::context;

use nom::sequence::{delimited, tuple};



use crate::llvm::ast::Expr;
use crate::parsers::statements::space_opt;


use super::ParseResult;

use super::array::{parse_array, parse_array_indexing};
use super::tokens::{
    parse_identifier, 
    parse_number, 
    parse_string, 
    parse_char, 
    parse_variable, 
    get_identifier, parse_qualified_identifier,
};

pub fn parse_function_call(input: &str) -> ParseResult<&str, Expr> {
    //println!("parse_function_call INPUT: {}", input);
    let (input, name) = space_opt(parse_identifier)(input)?;
    //println!("parse_function_call NAME: {}", name);
    let (input, _) = multispace0(input)?;
    let (input, args) = delimited(
        tag("("), 
        separated_list0(tag(","), expression),
        tag(")")
    )(input)?;
    let id = get_identifier(name).unwrap();
    
    Ok((input, Expr::Call(id, args)))
}


pub fn parse_primary_expr(input: &str) -> ParseResult<&str, Expr> {
    alt((
        parse_array_indexing,
        parse_qualified_identifier,
        parse_identifier,
        parse_array,
        parse_number,
        parse_string,
        parse_char,
        delimited(tag("("), expression, tag(")")),
    ))(input)
}

pub fn parse_infix_expr(input: &str) -> ParseResult<&str, Expr> {
    let (input, _) = multispace0(input)?;
    let (input, left) =  parse_variable(input)?;
    let (input, _) = multispace0(input)?;
    let (input, op) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("=="),
        tag("!="),
        tag("<"),
        tag(">"),
        tag("<="),
        tag(">="),
    ))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, right) = parse_variable(input)?;
    let (input, _) = multispace0(input)?;

    let infix = Expr::Infix(Box::new(left), op.into(), Box::new(right));
    //let (input, _) = opt(tag(";"))(input)?;
    Ok((input, infix))
}


pub fn expression(input: &str) -> ParseResult<&str, Expr> {
    context(
        "expr",
        tuple((
                alt((
                    parse_function_call,
                    parse_infix_expr,
                    parse_primary_expr,
                )),
            opt(tuple((
                tag(";"),
                multispace0,
            )))
        ))
    )(input)
        .map(|(input, (expr, _))| 
        (input, expr)
    )
}
use nom::{branch::alt, multi::separated_list0};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::error::context;

use nom::sequence::delimited;
use nom::IResult;


use crate::llvm::ast::Expr;


use super::tokens::{
    parse_identifier, 
    parse_number, 
    parse_string, 
    parse_char, 
    parse_variable, 
    get_identifier,
};

pub fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = parse_identifier(input)?;
    let (input, args) = delimited(
        tag("("), 
        separated_list0(tag(","), 
        expression),
         tag(")"))(input)?;
    let id = get_identifier(name).unwrap();
    Ok((input, Expr::Call(id, args)))
}

fn parse_primary_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        delimited(tag("("), expression, tag(")")),
    ))(input)
}

fn parse_infix_expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_variable(input)?;
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


pub fn expression(input: &str) -> IResult<&str, Expr> {
    context(
        "expr", 
        alt((
            parse_function_call,
            parse_infix_expr,
            parse_primary_expr,
        ))
    )(input).map(|(input, expr)| {
        //println!("expr: {:?}", expr);
        //println!("input: {:?}", input);
        (input, expr)
    })
}
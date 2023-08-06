use nom::{branch::alt, multi::separated_list0};
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::context;

use nom::sequence::delimited;
use nom::IResult;

use crate::ast::Expression;


use super::tokens::{
    parse_identifier, 
    parse_number, 
    parse_string, 
    parse_char, 
    parse_variable, 
    get_identifier,
};

pub fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, name) = parse_identifier(input)?;
    let (input, args) = delimited(
        tag("("), 
        separated_list0(tag(","), 
        expression),
         tag(")"))(input)?;
    let id = get_identifier(name).unwrap();
    Ok((input, Expression::Call(id, args)))
}

fn parse_primary_expr(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        delimited(tag("("), expression, tag(")")),
    ))(input)
}

fn parse_infix_expr(input: &str) -> IResult<&str, Expression> {
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
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Expression::Infix(Box::new(left), op.to_string(), Box::new(right))))
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
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
use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;


use nom::character::streaming::multispace0;
use nom::multi::{separated_list0, many0};
use nom::sequence::delimited;
use nom::IResult;

use crate::ast::{Expression, Statement};
use crate::parsers::tokens::{parse_identifier, get_identifier};


use super::statements::{parse_return_statement, parse_let_statement};
use super::tokens::expr;


impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "Identifier({})", s),
            Expression::Number(n) => write!(f, "Number({})", n),
            Expression::String(s) => write!(f, "String({})", s),
            Expression::Char(c) => write!(f, "Char({})", c),
            Expression::Infix(op, left, right) => write!(f, "Infix({} {} {})", op, left, right),
            Expression::Call(ident, args) => write!(f, "Call({} {})", ident, args.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}

fn parse_function_declaration(input: &str) -> IResult<&str, Statement> {
    let (input, _) = tag("fn")(input)?;
    //println!("it's a function!");
    let (input, _) = multispace0(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, params) = delimited(
        tag("("), 
        separated_list0(tag(","), parse_identifier), 
        tag(")")
    )(input)?;
    //println!("params: {:?}", params);
    let (input, _) = multispace0(input)?;

    let (input, body) = delimited(
        tag("{"), many0(parse_statement), tag("}"))
        (input)?;

    //println!("body: {:?}", body);
    let params = params.into_iter().map(|expr| {
        if let Expression::Identifier(s) = expr {
            s
        } else {
            panic!("Expecting identifier in function parameter list")
        }
    }).collect();
    
    match get_identifier(ident) {
        Some(s) => Ok((input, Statement::Function(s, params, body))),
        None => panic!("Expecting identifier in function declaration"),
    }
}





fn parse_expr_statement(input: &str) -> IResult<&str, Statement> {
    let (input, expr) = expr(input)?;
    Ok((input, Statement::Expr(expr)))
}


pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((
        parse_let_statement,
        parse_return_statement,
        parse_function_declaration,
        parse_expr_statement,
        
    ))(input)
}

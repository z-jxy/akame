use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, multispace0, char, alphanumeric0};
use nom::combinator::recognize;
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

use crate::ast::{Expr, Stmt};


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(s) => write!(f, "Identifier({})", s),
            Expr::Number(n) => write!(f, "Number({})", n),
            Expr::String(s) => write!(f, "String({})", s),
            // Add all other variants of your Expr enum here...
            _ => write!(f, "Other Expr variant"),
        }
    }
}

fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    map(recognize(pair(alpha1, alphanumeric0)), |s: &str| {
        Expr::Identifier(s.to_string())
    })(input)
}

fn parse_function_declaration(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, params) = delimited(
        tag("("), 
        separated_list0(tag(","), parse_identifier), 
        tag(")")
    )(input)?;
    // Assuming the function's body is a single statement for simplicity
    // You'll need to modify this to parse multiple statements
    let (input, body) = delimited(tag("{"), parse_statement, tag("}"))(input)?;

    let params = params.into_iter().map(|expr| {
        if let Expr::Identifier(s) = expr {
            s
        } else {
            panic!("Expecting identifier in function parameter list")
        }
    }).collect();

    Ok((input, Stmt::Function(name.to_string(), params, vec![body])))
}


fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, num) = alphanumeric1(input)?;
    Ok((input, Expr::Number(num.parse().unwrap())))
}

fn parse_string(input: &str) -> IResult<&str, Expr> {
    let (input, s) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, Expr::String(s.to_string())))
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_number, parse_string, parse_identifier))(input)
}

fn parse_let_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("let")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, id) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;

    Ok((input, Stmt::Let(id.to_string(), expr)))
}

fn parse_return_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;

    Ok((input, Stmt::Return(expr)))
}

fn parse_expr_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, expr) = parse_expr(input)?;
    Ok((input, Stmt::Expr(expr)))
}

fn parse_statement(input: &str) -> IResult<&str, Stmt> {
    alt((
        parse_let_statement,
         parse_return_statement,
          parse_function_declaration,
           parse_expr_statement
        ))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<Stmt>> {
    separated_list0(tag(";"), parse_statement)(input)
}

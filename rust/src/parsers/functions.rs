use core::panic;
use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::tag;


use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::error::context;
use nom::multi::{separated_list0, many0};
use nom::sequence::delimited;
use nom::IResult;


use crate::llvm::ast::{Expr, Stmt};
use crate::parsers::tokens::parse_identifier;


use super::expressions::expression;

use super::statements::{parse_return_statement, parse_let_statement};



impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Ident(s) => write!(f, "Identifier({})", s),
            Expr::Num(n) => write!(f, "Number({})", n),
            Expr::Str(s) => write!(f, "String({})", s),
            Expr::Char(c) => write!(f, "Char({})", c),
            Expr::Infix(op, left, right) => write!(f, "Infix({} {} {})", op, left, right),
            Expr::Call(ident, args) => write!(f, "Call({} {})", ident, args.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}
/// parse the required function declaration ()
fn parse_function(input: &str) -> IResult<&str, Stmt> {
    //let (input, _) = ws(tag("fn"))(input)?;
    let (input, _) = multispace0(input)?;
    let (input, ident) = parse_identifier(input)
        .expect("Missing identifier in function declaration");
    let (input, _) = multispace0(input)?;
    let (input, params) = delimited(
        tag("("), 
        separated_list0(tag(","), parse_identifier), 
        tag(")")
    )(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _left_brace) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(ws(parse_statement))(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, _right_brace) = tag("}")(input)?;
    let params = params.into_iter().map(|expr| {
        if let Expr::Ident(s) = expr {
            s
        } else {
            panic!("Expecting identifier in function parameter list")
        }
    }).collect();

    if let Expr::Ident(s) = ident {
        Ok((input, Stmt::FunctionDeclaration{
            ident: s,
            params,
            body,
        }))
    } else {
        panic!("Expecting identifier in function declaration")
    }
}

fn parse_function_declaration(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = ws(tag("fn"))(input)?;
    //let function = function(input).expect("Error parsing function declaration");
    Ok(parse_function(input).expect("Error parsing function declaration"))
}


fn parse_expr_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, expr) = expression(input)?;
    Ok((input, Stmt::Expression(expr)))
}

pub fn ws<'a, T, F>(parser: F) -> impl Fn(&'a str) -> IResult<&'a str, T>
where
    F: Fn(&'a str) -> IResult<&'a str, T>,
{
    move |input: &'a str| {
        let (input, _) = opt(multispace0)(input)?;
        let (input, _) = opt(tag("\n"))(input)?;
        let (input, _) = opt(tag("\t"))(input)?;
        parser(input)
    }
}


pub fn parse_statement(input: &str) -> IResult<&str, Stmt> {
    context(
        "statement",
        alt((
            parse_function_declaration,
            parse_return_statement,
            parse_let_statement,
            parse_expr_statement,
        )),
    )(input).map(|(input, statement)| {
        (input, statement)
    })
}

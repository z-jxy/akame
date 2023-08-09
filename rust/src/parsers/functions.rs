
use std::fmt;


use nom::bytes::complete::tag;


use nom::character::complete::multispace0;
use nom::combinator::opt;

use nom::multi::{separated_list0, many0};
use nom::sequence::delimited;



use crate::llvm::ast::{Expr, Stmt};
use crate::parsers::tokens::parse_identifier;


use super::{ParseResult, parse_statement};

use super::error::CustomError;
use super::expressions::expression;

use super::statements::space_opt;

pub const USER_DEFINED_ENTRY : &str = "_main";

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Ident(s) => write!(f, "Identifier({})", s),
            Expr::Num(n) => write!(f, "Number({})", n),
            Expr::Str(s) => write!(f, "String({})", s),
            Expr::Char(c) => write!(f, "Char({})", c),
            Expr::Infix(op, left, right) => write!(f, "Infix({} {} {})", op, left, right),
            Expr::Call(ident, args) => write!(f, "Call({} {})", ident, args.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
            Expr::QualifiedIdent(idents) => write!(f, "QualifiedIdent({:#?})", idents),
            Expr::ArrayIndexing(array, index) => write!(f, "ArrayIndexing({} {})", array, index),
            Expr::Array(array) => write!(f, "Array({})", array.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}

fn parse_parameters(input: &str) -> ParseResult<&str, Vec<String>> {
    let (input, params) = space_opt(delimited(
        tag("("),
        separated_list0(space_opt(tag(",")), parse_identifier),
        tag(")")
    ))(input)?;

    let parsed_params = params.into_iter().map(|expr| {
        if let Expr::Ident(s) = expr {
            Ok(s)
        } else {
            Err(nom::Err::Failure(CustomError::UnexpectedToken("Expected identifier in function parameter list")))
        }
    }).collect::<Result<_, _>>()?;

    Ok((input, parsed_params))
}

fn parse_function_body(input: &str) -> ParseResult<&str, Vec<Stmt>> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(ws(parse_statement))(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    let (input, _) = opt(multispace0)(input)?;
    let (input, _) = tag("}")(input)?;

    Ok((input, body))
}

fn parse_function_name(input: &str) -> ParseResult<&str, String> {
    let (input, ident) = space_opt(parse_identifier)(input)?;
    match &ident {
        Expr::Ident(id) => Ok((input, id.clone())),
        _ => Err(nom::Err::Failure(CustomError::UnexpectedToken("Expected identifier in function declaration"))),
    }
}

/// parse the required function declaration ()
fn parse_function(input: &str) -> ParseResult<&str, Stmt> {
    let (input, ident) = parse_function_name(input)?;
    let (input, params) = parse_parameters(input)?;
    if ident == "main" && !params.is_empty() {
        return Err(nom::Err::Failure(CustomError::MainFunctionWithParams(input)));
    }
    
    let (input, body) = parse_function_body(input)?;

    Ok((
        input, 
        Stmt::FunctionDeclaration{ 
            ident: match ident == "main" {
                true => USER_DEFINED_ENTRY.to_string(),
                false => ident,
            }, params, body 
        }
    ))
}

pub fn parse_function_declaration(input: &str) -> ParseResult<&str, Stmt> {
    let (input, _) = ws(tag("fn"))(input)?;
    parse_function(input)
}


pub fn parse_expr_statement(input: &str) -> ParseResult<&str, Stmt> {
    let (input, expr) = expression(input)?;
    Ok((input, Stmt::Expression(expr)))
}

pub fn ws<'a, T, F>(parser: F) -> impl Fn(&'a str) -> ParseResult<&'a str, T>
where
    F: Fn(&'a str) -> ParseResult<&'a str, T>,
{
    move |input: &'a str| {
        let (input, _) = opt(multispace0)(input)?;
        let (input, _) = opt(tag("\n"))(input)?;
        let (input, _) = opt(tag("\t"))(input)?;
        parser(input)
    }
}




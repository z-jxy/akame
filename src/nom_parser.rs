use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, multispace0, char, alphanumeric0, anychar};
use nom::combinator::{recognize, map, opt};
use nom::multi::{separated_list0, many0};
use nom::sequence::{delimited, tuple, terminated};
use nom::{IResult, Parser};

use crate::ast::{Expr, Stmt};


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(s) => write!(f, "Identifier({})", s),
            Expr::Number(n) => write!(f, "Number({})", n),
            Expr::String(s) => write!(f, "String({})", s),
            Expr::Char(c) => write!(f, "Char({})", c),
            Expr::Infix(op, left, right) => write!(f, "Infix({} {} {})", op, left, right),
            Expr::Call(ident, args) => write!(f, "Call({} {})", ident, args.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join(", ")),
        }
    }
}
fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    map(
        recognize(
            tuple((
                alt((alpha1, tag("_"))),
                alphanumeric0,
            ))
        ),
        |s: &str| Expr::Identifier(s.to_string())
    )(input)
}

pub fn get_identifier(e: Expr) -> Option<String> {
    match e {
        Expr::Identifier(s) => Some(s),
        _ => None,
    }
}

fn parse_function_declaration(input: &str) -> IResult<&str, Stmt> {
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
        if let Expr::Identifier(s) = expr {
            s
        } else {
            panic!("Expecting identifier in function parameter list")
        }
    }).collect();
    
    println!("successfully parsed function declaration");
    match get_identifier(ident) {
        Some(s) => Ok((input, Stmt::Function(s, params, body))),
        None => panic!("Expecting identifier in function declaration"),
    }
}


fn parse_number(input: &str) -> IResult<&str, Expr> {
    let (input, num) = alphanumeric1(input)?;
    Ok((input, Expr::Number(num.parse().unwrap())))
}

fn parse_string(input: &str) -> IResult<&str, Expr> {
    let (input, s) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, Expr::String(s.to_string())))
}

fn parse_function_call(input: &str) -> IResult<&str, Expr> {
    let (input, name) = parse_identifier(input)?;
    println!("function name: {}", name);
    let (input, args) = delimited(tag("("), separated_list0(tag(","), expr), tag(")"))(input)?;
    println!("calling function {} with args {:?}", name, args);
    let id = get_identifier(name).unwrap();
    Ok((input, Expr::Call(id, args)))
}


fn parse_char(input: &str) -> IResult<&str, Expr> {
    delimited(tag("'"), anychar, tag("'"))
        .map(|c| Expr::Char(c))
        .parse(input)
}

//fn parse_paren_expr(input: &str) -> IResult<&str, Expr> {
//    delimited(tag("("), expr, tag(")"))(input)
//}

fn parse_infix_expr(input: &str) -> IResult<&str, Expr> {
    let (input, left) = parse_primary_expr(input)?;
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
    let (input, right) = parse_primary_expr(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(tag(";"))(input)?;
    Ok((input, Expr::Infix(Box::new(left), op.to_string(), Box::new(right))))
}

//fn parse_terminated_expr(input: &str) -> IResult<&str, Expr> {
//    let (input, expr) = parse_expr(input)?;
//    let (input, _) = tag(";")(input)?;
//    Ok((input, expr))
//}

fn parse_primary_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        delimited(tag("("), expr, tag(")")),
    ))(input)
}

fn parse_let_statement(input: &str) -> IResult<&str, Stmt> {
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
            expr,
            opt(multispace0)
            ),
            tag(";"),
        )),
        |(_let,  ident,  _assignment_op, expr, _semicolon,)| match ident {
            Expr::Identifier(id) => Stmt::Let(id.to_string(), expr),
            _ => panic!("Expecting identifier in let statement"),
        }
    )(input)
}

fn parse_return_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;

    Ok((input, Stmt::Return(expr)))
}


fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_function_call,
        parse_infix_expr,
        parse_primary_expr,
    ))(input)
}

fn parse_expr_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, expr) = expr(input)?;
    Ok((input, Stmt::Expr(expr)))
}


fn parse_statement(input: &str) -> IResult<&str, Stmt> {
    alt((
        parse_let_statement,
        parse_return_statement,
        parse_function_declaration,
        parse_expr_statement,
        
    ))(input)
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<Stmt>> {
    separated_list0(tag(";"), parse_statement)(input)
}

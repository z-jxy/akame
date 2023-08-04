use std::fmt;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, alphanumeric1, multispace0, char, alphanumeric0, anychar};
use nom::combinator::{recognize, map};
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, tuple, pair};
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
            // Add all other variants of your Expr enum here...
            _ => write!(f, "Other Expr variant"),
        }
    }
}
fn parse_identifier(input: &str) -> IResult<&str, Expr> {
    let (input, ident) = recognize(pair(alpha1, alphanumeric0))(input)?;
    Ok((input, Expr::Identifier(ident.to_string())))
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
    println!("left: {}", left);
    // trim whitespace
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
    println!("op: {}", op);
    let (input, _) = multispace0(input)?;
    let (input, right) = parse_primary_expr(input)?;
    println!("left: {}, op: {}, right: {}", left, op, right);
    //Ok((input, Expr::Infix(Box::new(left), op.to_string(), Box::new(right))))
    return Ok((input, Expr::Infix(Box::new(left), op.to_string(), Box::new(right))))
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
    let (input, _) = tag("let")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _id) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;
    
    let id = if let Expr::Identifier(s) = _id {
        s
    } else {
        panic!("Expecting identifier in let statement")
    };
    println!("id: {}, expr: {}", id, expr);

    Ok((input, Stmt::Let(id.to_string(), expr)))
}

fn parse_return_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = expr(input)?;

    Ok((input, Stmt::Return(expr)))
}


fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        parse_infix_expr,
        parse_primary_expr,
    ))(input)
}

fn parse_expr_statement(input: &str) -> IResult<&str, Stmt> {
    let (input, expr) = expr(input)?;
    //let (input, _) = tag(";")(input)?;
    println!("!expr: {}!", expr);
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

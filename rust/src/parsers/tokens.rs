use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, alphanumeric0, anychar, digit1, multispace0};
use nom::combinator::{recognize, map};

use nom::sequence::{delimited, tuple};
use nom::Parser;


use crate::llvm::ast::Expr;
use crate::types::integer::{parse_large_integer, Integer, try_parse_number};

use super::ParseResult;

pub fn parse_identifier(input: &str) -> ParseResult<&str, Expr> {
    let (input, _) = multispace0(input)?; // consume whitespace
    map(
        recognize(
            tuple((
                alt((
                    alpha1, 
                    tag("_")
                )),
                alphanumeric0,
            ))
        ),
        |s: &str| Expr::Ident(s.to_string())
    )(input)
}

pub fn get_identifier(e: Expr) -> Option<String> {
    match e {
        Expr::Ident(s) => Some(s),
        _ => None,
    }
}

pub fn parse_number(input: &str) -> ParseResult<&str, Expr> {
    let (input, num_str) = digit1(input)?;
    let _number = match num_str {
        s if try_parse_number::<i8>(s).is_some() => Integer::Int8(try_parse_number::<i8>(s).unwrap()),
        s if try_parse_number::<i32>(s).is_some() => Integer::Int(try_parse_number::<i32>(s).unwrap()),
        s if try_parse_number::<i64>(s).is_some() => Integer::Int64(try_parse_number::<i64>(s).unwrap()),
        s if try_parse_number::<i128>(s).is_some() => Integer::Int128(try_parse_number::<i128>(s).unwrap()),
        s => {
            // handle integer overflow
            match parse_large_integer(s) {
                Ok(i) => Integer::Int128(i),
                Err(_) => panic!("Integer overflow"),
            }
        }
    };
    let i_value = try_parse_number::<i32>(num_str).unwrap();
    Ok((input, Expr::Num(i_value)))
} 



pub fn parse_string(input: &str) -> ParseResult<&str, Expr> {
    let (input, s) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, Expr::Str(s.to_string())))
}


pub fn parse_char(input: &str) -> ParseResult<&str, Expr> {
    delimited(tag("'"), anychar, tag("'"))
        .map(|c| Expr::Char(c))
        .parse(input)
}

pub fn parse_variable(input: &str) -> ParseResult<&str, Expr> {
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        //delimited(tag("("), expr, tag(")")),
    ))(input)
}
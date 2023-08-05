use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, char, alphanumeric0, anychar, digit1};
use nom::combinator::{recognize, map};

use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

use crate::ast::Expression;
use crate::types::integer::{parse_large_integer, Integer, try_parse_number};

pub fn parse_identifier(input: &str) -> IResult<&str, Expression> {
    map(
        recognize(
            tuple((
                alt((alpha1, tag("_"))),
                alphanumeric0,
            ))
        ),
        |s: &str| Expression::Identifier(s.to_string())
    )(input)
}

pub fn get_identifier(e: Expression) -> Option<String> {
    match e {
        Expression::Identifier(s) => Some(s),
        _ => None,
    }
}

pub fn parse_number(input: &str) -> IResult<&str, Expression> {
    let (input, num_str) = digit1(input)?;
    let number = match num_str {
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
    Ok((input, Expression::Number(number)))
}



pub fn parse_string(input: &str) -> IResult<&str, Expression> {
    let (input, s) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, Expression::String(s.to_string())))
}


pub fn parse_char(input: &str) -> IResult<&str, Expression> {
    delimited(tag("'"), anychar, tag("'"))
        .map(|c| Expression::Char(c))
        .parse(input)
}

pub fn parse_variable(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        //delimited(tag("("), expr, tag(")")),
    ))(input)
}
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{alpha1, multispace0, char, alphanumeric0, anychar, digit1};
use nom::combinator::{recognize, map, opt};
use nom::multi::separated_list0;
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

fn parse_number(input: &str) -> IResult<&str, Expression> {
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



fn parse_string(input: &str) -> IResult<&str, Expression> {
    let (input, s) = delimited(char('"'), take_until("\""), char('"'))(input)?;
    Ok((input, Expression::String(s.to_string())))
}

fn parse_function_call(input: &str) -> IResult<&str, Expression> {
    let (input, name) = parse_identifier(input)?;
    let (input, args) = delimited(tag("("), separated_list0(tag(","), expr), tag(")"))(input)?;
    let id = get_identifier(name).unwrap();
    Ok((input, Expression::Call(id, args)))
}


fn parse_char(input: &str) -> IResult<&str, Expression> {
    delimited(tag("'"), anychar, tag("'"))
        .map(|c| Expression::Char(c))
        .parse(input)
}


fn parse_infix_expr(input: &str) -> IResult<&str, Expression> {
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
    Ok((input, Expression::Infix(Box::new(left), op.to_string(), Box::new(right))))
}

fn parse_primary_expr(input: &str) -> IResult<&str, Expression> {
    //let _ = peek(is_too_large_number)(input);
    alt((
        parse_identifier,
        parse_number,
        parse_string,
        parse_char,
        delimited(tag("("), expr, tag(")")),
    ))(input)
}



pub fn expr(input: &str) -> IResult<&str, Expression> {
    alt((
        parse_function_call,
        parse_infix_expr,
        parse_primary_expr,
    ))(input)
}
use nom::{IResult, multi::separated_list0, bytes::complete::tag, branch::alt, character::complete::multispace1};

use crate::ast::Statement;

use self::functions::{parse_statement, ws};

mod functions;
mod statements;
mod tokens;

fn consume_whitespace(input: &str) -> IResult<&str, &str> {
    multispace1(input)
}

fn statement_delimiter(input: &str) -> IResult<&str, &str> {
    let (input, _) =alt((
        tag(";"),
        tag("\n"),
        tag("\r\n"),
        tag("\r"),
        tag("\t"),
        multispace1
    ))(input)?;
    let (input, _) = consume_whitespace(input)?;
    Ok((input, ""))
}

pub fn parse_program(input: &str) -> IResult<&str, Vec<Statement>> {
    separated_list0(statement_delimiter, parse_statement)(input)
}
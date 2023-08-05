use nom::{IResult, multi::separated_list0, bytes::complete::tag};

use crate::ast::Statement;

use self::functions::parse_statement;

mod functions;
mod statements;
mod tokens;

pub fn parse_program(input: &str) -> IResult<&str, Vec<Statement>> {
    separated_list0(tag(";"), parse_statement)(input)
}
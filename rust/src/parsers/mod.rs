

use nom::{IResult, multi::separated_list0, bytes::complete::tag, branch::alt, character::complete::multispace1, error::context, Finish};

use crate::llvm::ast::Stmt;

use self::{functions::parse_statement, error::CustomError};

mod functions;
mod statements;
mod tokens;
mod expressions;
mod error;

pub type ParseResult<I, O> = IResult<I, O, CustomError<I>>;


fn consume_whitespace(input: &str) -> ParseResult<&str, &str> {
    multispace1(input)
}

fn statement_delimiter(input: &str) -> ParseResult<&str, &str> {
    let (input, _) = context(
        "stripper", alt((
        tag(";"),
        tag("\n"),
        tag("\r\n"),
        tag("\r"),
        tag("\t"),
        multispace1
    )))(input)?;
    let (input, _) = consume_whitespace(input)?;
    Ok((input, ""))
}

pub fn parse_program(input: &str) -> anyhow::Result<Vec<Stmt>> {
    let result = separated_list0(statement_delimiter, parse_statement)(input)
        .finish()
        .map(|(_, stmts)| stmts);

    match result {
        Ok(stmts) => Ok(stmts),
        Err(err) => Err(anyhow::anyhow!("Error parsing source: {}", err))
    }
        
}
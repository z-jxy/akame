

use nom::{IResult, multi::separated_list0, bytes::complete::tag, branch::alt, character::complete::multispace1, error::context, Finish};

use crate::llvm::ast::Stmt;

use self::{ error::CustomError, functions::{parse_function_declaration, parse_expr_statement}, statements::{parse_return_statement, parse_let_statement}};

mod functions;
mod statements;
mod tokens;
mod expressions;
mod error;
mod full_test;

pub type ParseResult<I, O> = IResult<I, O, CustomError<I>>;

fn statement_delimiter(input: &str) -> ParseResult<&str, &str> {
    let (input, _) = context(
        "stripper", alt((
            tag(";"),
            multispace1
        ))
    )(input)?;
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

pub fn parse_statement(input: &str) -> ParseResult<&str, Stmt> {
    context(
        "statement",
        alt((
            parse_function_declaration,
            parse_return_statement,
            parse_let_statement,
            parse_expr_statement,
        )),
    )(input)
    .map(|(input, statement)| (input, statement))
}


use nom::{multi::separated_list0, sequence::delimited, bytes::complete::tag, branch::alt};

use crate::llvm::ast::Expr;

use super::{statements::space_opt, ParseResult, expressions::expression, tokens::{parse_identifier, parse_qualified_identifier}};

pub fn parse_array(input: &str) -> ParseResult<&str, Expr> {
    delimited(
        tag("["),
        separated_list0(space_opt(tag(",")), expression),
        tag("]")
    )(input)
    .map(|(next_input, vec)| (next_input, Expr::Array(vec)))
}

pub fn parse_array_indexing(input: &str) -> ParseResult<&str, Expr> {
    let (input, array) = alt((parse_qualified_identifier, parse_identifier))(input)?;
    let (input, index) = delimited(tag("["), expression, tag("]"))(input)?;
    Ok((input, Expr::ArrayIndexing(Box::new(array), Box::new(index))))
}
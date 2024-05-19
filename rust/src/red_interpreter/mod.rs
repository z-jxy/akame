use std::path::PathBuf;

use crate::{llvm::ast::Ast, parsers::parse_program};

pub mod interpreter;
pub mod repl;

/// Takes a path to a source file and executes it in the interpreter
pub fn run_script(script: &PathBuf) {
    let mut interpreter = interpreter::Interpreter::new();
    let src = std::fs::read_to_string(script).expect("Could not read file");
    interpreter.eval_source(&src);
}

pub fn parse(script: &str) -> anyhow::Result<Ast> {
    Ok(parse_program(&script)?)
}

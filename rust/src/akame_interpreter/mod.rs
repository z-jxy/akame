use crate::{ parsers::parse_program, llvm::ast::Stmt};

pub mod interpreter;
pub mod repl;

#[allow(dead_code)]
pub fn run_script(script: &str) {
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval_source(script);
}

pub fn parse(script: &str) -> anyhow::Result<Vec<Stmt>> {
    Ok(parse_program(&script)?)
}
use crate::{ parsers::parse_program, llvm::ast::Stmt};

pub mod interpreter;
pub mod repl;

pub fn run_script(script: &str) {
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval_source(script);
}

pub fn parse(script: &str) -> anyhow::Result<Vec<Stmt>> {
    match parse_program(&script) {
        Ok((_, parsed_program)) => {
            Ok(parsed_program)
        },
        Err(err) => {
            Err(anyhow::anyhow!("Parser error: {}", err))
        },
    }
}
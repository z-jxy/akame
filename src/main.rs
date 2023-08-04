use crate::{parser::Parser, interpreter::Interpreter};

mod lexer;
mod parser;
mod tokens;
mod ast;
mod interpreter;
fn main() {
    let code = r#"
        let x = 5;
        let y = 10;
        x + y;
    "#;
    let mut parser = Parser::new(code);
    match parser.parse() {
        Ok(ast) => {
            println!("Successfully parsed AST: {:?}", ast);
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(ast) {
                Ok(result) => {
                    println!("Final state of the symbol table: {:?}", interpreter.symbol_table);
                    println!("Final result: {}", result);
                },
                Err(error) => {
                    eprintln!("Interpreter error: {}", error);
                    return;
                }
            }
        },
        Err(error) => {
            eprintln!("Failed to parse code: {}", error);
            std::process::exit(1);
        }
    }
}
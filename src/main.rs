use crate::{parser::Parser, interpreter::Interpreter};

mod lexer;
mod parser;
mod tokens;
mod ast;
mod interpreter;
fn main() {
    let input = "(+ (* 2 3) (- 4 5))";
    let mut parser = Parser::new(input);
    let ast = parser.parse().unwrap();
    let interpreter = Interpreter;
    let result = interpreter.visit(ast);
    println!("Result: {}", result);
}

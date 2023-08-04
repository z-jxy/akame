

mod lexer;
mod parser;
mod tokens;
mod ast;
mod interpreter;
mod repl;
mod nom_parser;
fn main() {
    if let Err(err) = repl::interactive() {
        eprintln!("Error in main: {:?}", err);
    }
}

//mod tests {
//    use crate::ast::{Stmt, Expr};
//
//    use super::*;
//
//    #[test]
//    fn test_interpreter() {
//        let code = r#"
//            let x = 5;
//            let y = 10;
//            x + y;
//        "#;
//        let mut parser = Parser::new(code);
//        match parser.parse() {
//            Ok(ast) => {
//                println!("Successfully parsed AST: {:?}", ast);
//                let mut interpreter = Interpreter::new();
//                match interpreter.interpret(ast) {
//                    Ok(result) => {
//                        println!("Final state of the symbol table: {:?}", interpreter.symbol_table);
//                        println!("Final result: {}", result);
//                        assert_eq!(result, "15");
//                    },
//                    Err(error) => {
//                        eprintln!("Interpreter error: {}", error);
//                        return;
//                    }
//                }
//            },
//            Err(error) => {
//                eprintln!("Failed to parse code: {}", error);
//                std::process::exit(1);
//            }
//        }
//        //let mut interpreter = Interpreter::new();
//        //let program = vec![
//        //    Stmt::Let("x".to_string(), Expr::Number(5)),
//        //    Stmt::Let("y".to_string(), Expr::Number(10)),
//        //    Stmt::Expr(Expr::Infix(
//        //        Box::new(Expr::Identifier("x".to_string())),
//        //        "+".to_string(),
//        //        Box::new(Expr::Identifier("y".to_string())),
//        //    )),
//        //];
//        //let result = interpreter.interpret(program);
//        //assert_eq!(result, Ok(15));
//    }
//}
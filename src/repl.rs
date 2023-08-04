use std::io;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::interpreter::{Interpreter, Interpreter2};
use crate::nom_parser;
use crate::parser::Parser;

pub fn interactive2() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter2::new();

    if rl.load_history(".history.txt").is_err() {
        println!("Lisper-lang: v0.0.1");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match nom_parser::parse_program(&line) {
                    Ok((_, parsed_program)) => {
                        for stmt in parsed_program {
                            match interpreter.visit_stmt(stmt) {
                                Ok(result) => println!("=> {}", result),
                                Err(err) => eprintln!("Interpreter error: {}", err),
                            }
                        }
                    },
                    Err(err) => {
                        println!("{}", "=".repeat(80));
                        eprintln!("[Parser::parse] error: {}", err);
                        println!("{}", "=".repeat(80));
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".history.txt")?;
    Ok(())
}


pub fn interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();

    if rl.load_history(".history.txt").is_err() {
        println!("Lisper-lang: v0.0.1");
    }

    loop {
        let readline = rl.readline(">> ");
        //let mut stack_trace = Vec::new();
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let mut parser = Parser::new(&line);
                match parser.parse() {
                    Ok(ast) => {
                        match interpreter.interpret(ast) {
                            Ok(result) => println!("=> {}", result),
                            Err(err) => eprintln!("Interpreter error: {}", err),
                        }
                    },
                    Err(_) => {
                        println!("{}", "=".repeat(80));
                        let mut parser = Parser::new(&line);
                        eprintln!("[Parser::parse] error: {}", parser.dbg_parse().err().unwrap());
                        println!("{}", "=".repeat(80));
                    },
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            },
        }
    }
    rl.save_history(".history.txt")?;
    Ok(())
}

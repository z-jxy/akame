use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::parsers::parse_program;

use super::interpreter::Interpreter;

pub fn interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();

    if rl.load_history(".history.txt").is_err() {
        println!("Lisper-lang: v0.0.1");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match parse_program(&line) {
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
use rustyline::error::ReadlineError;
use rustyline::Editor;

use crate::interpreter::Interpreter;
use crate::parser::Parser;

pub fn interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();

    loop {
        let readline = rl.readline(">> ");
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
                    Err(err) => eprintln!("Parse error: {}", err),
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
    Ok(())
}


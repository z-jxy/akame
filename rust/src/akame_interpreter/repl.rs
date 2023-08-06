use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                print!("{}", interpreter.eval_str(&line)?);
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
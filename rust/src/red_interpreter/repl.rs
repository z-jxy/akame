use std::io::Write;

use rustyline::error::ReadlineError;
use rustyline::Editor;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::interpreter::Interpreter;

#[allow(dead_code)]
pub fn interactive() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = Editor::<()>::new();
    let mut interpreter = Interpreter::new();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stderr = StandardStream::stderr(ColorChoice::Always);
    stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    println!("[*] Red: v0.0.1");
    if rl.load_history(".history.txt").is_ok() {}

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match interpreter.eval_str(&line) {
                    Ok(res) => {
                        write!(&mut stdout, "{}", res)?;
                        // print!("{}", res);
                    }
                    Err(err) => {
                        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
                        write!(&mut stderr, "{:?}\n", err)?;
                        stderr.reset()?;
                    }
                }
                // print!("{}", interpreter.eval_str(&line)?);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
            }
        }
    }
    rl.save_history(".history.txt")?;
    Ok(())
}

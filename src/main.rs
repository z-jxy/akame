mod ast;
mod akame_interpreter;
pub mod types;
mod parsers;

fn main() {
    if let Err(err) = akame_interpreter::repl::interactive() {
        eprintln!("Error in main: {:?}", err);
    }
}

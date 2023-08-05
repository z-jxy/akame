mod ast;
mod akame_interpreter;
pub mod types;
mod parsers;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            if let Err(err) = akame_interpreter::repl::interactive() {
                eprintln!("Error in main: {:?}", err);
            }
        },
        2 => {
            let filename = &args[1];
            let script = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
            akame_interpreter::run_script(&script);
        },
        _ => {
            println!("Usage: {} <filename>.ak", args[0]);
            return;
        }
    }
}

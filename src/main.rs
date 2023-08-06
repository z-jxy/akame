mod akame_interpreter;
mod ast;
mod llvm;
mod parsers;
pub mod types;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} debug || llvm", args[0]);
        return;
    }

    match args[1].as_str() {
        "llvm" => llvm::llvm_codegen(),
        "debug" => llvm::llvm_debug(),
        _ => {
            let filename = &args[1];
            let script = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
            akame_interpreter::run_script(&script);
        }
    }

    //let args: Vec<String> = std::env::args().collect();
    //match args.len() {
    //    1 => {
    //        if let Err(err) = akame_interpreter::repl::interactive() {
    //            eprintln!("Error in main: {:?}", err);
    //        }
    //    },
    //    2 => {
    //        let filename = &args[1];
    //        let script = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    //        akame_interpreter::run_script(&script);
    //    },
    //    _ => {
    //        println!("Usage: {} <filename>.ak", args[0]);
    //        return;
    //    }
    //}
}

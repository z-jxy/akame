mod akame_interpreter;
mod ast;
mod llvm;
mod parsers;
pub mod types;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mode
    mode: Option<String>,

    /// Source to emit LLVM IR from
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Emit {
        /// Source to emit LLVM IR from
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Compile {
        /// Source to compile
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Parse {
        /// Source to parse
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    }
}


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(file) = cli.file {
        println!("Value for file: {:?}", file);
    }
    match cli.command {
        Some(Commands::Emit { file }) => {
            println!("[*] Emitting IR for file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path()).expect("Something went wrong reading the file");
            match akame_interpreter::parse(&script) {
                Ok(ast) => {
                    if cli.debug > 0 {
                        println!("AST: {:?}", ast);
                    }
                    llvm::emit(ast)
                },
                Err(err) => {
                    println!("{:?}", err);
                    //std::process::exit(1);
                }
            }
        },
        Some(Commands::Parse { file }) => {
            println!("[*] Parsing file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path()).expect("Something went wrong reading the file");
            match akame_interpreter::parse(&script) {
                Ok(ast) => {
                    println!("AST: {:?}", ast);
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        },
        Some(Commands::Compile { file }) => {
            println!("[*] Compiling: {:?}", file);
            let script = std::fs::read_to_string(file.as_path()).expect("Something went wrong reading the file");
            match akame_interpreter::parse(&script) {
                Ok(ast) => {
                    llvm::compile_ast(ast)?;
                },
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        },
        None => {
            println!("No subcommand was used");
        }
    }

    /*

    match args[1].as_str() {
        "llvm" => llvm::llvm_codegen(),
        //"debug" => llvm::llvm_debug(),
        _ => {
            let filename = &args[1];
            let script = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
            akame_interpreter::run_script(&script);
        }
    }
 */
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
    Ok(())
}

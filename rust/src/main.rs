mod llvm;
mod parsers;
mod red_interpreter;
pub mod types;
mod utils;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::utils::which_bin;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    /// Emit LLVM IR
    Emit {
        /// Source to emit LLVM IR from
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Compile {
        /// Source to compile
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,

        #[arg(short, long, value_name = "bin/main")]
        out_dir: Option<PathBuf>,
    },
    /// Parse source code file
    Parse {
        /// Source to parse
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Test {
        /// Source to parse
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Emit { file }) => {
            println!("[*] Emitting IR for file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path())
                .expect("Something went wrong reading the input file");
            match red_interpreter::parse(&script) {
                Ok(ast) => {
                    if args.debug > 0 {
                        println!("AST: {:?}", ast);
                    }
                    llvm::emit(ast)?
                }
                Err(err) => {
                    println!("{:?}", err);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Parse { file }) => {
            println!("[*] Parsing file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path())
                .expect("Something went wrong reading the file");
            match red_interpreter::parse(&script) {
                Ok(ast) => {
                    println!("AST: {:#?}", ast);
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        Some(Commands::Compile { file, out_dir }) => {
            println!("[*] Compiling: {:?}", file);
            let script = std::fs::read_to_string(file.as_path())
                .expect("Something went wrong reading the file");
            match red_interpreter::parse(&script) {
                Ok(ast) => {
                    llvm::compile_ast(ast, out_dir)?;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        Some(Commands::Test { file }) => {
            println!("[*] Testing: {:?}", file);
            let res = which_bin("clang");
            println!("clang: {:?}", res);
        }
        None => {
            println!("No subcommand was used");
        }
    }
    Ok(())
}

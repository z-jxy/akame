mod akame_interpreter;
mod ast;
mod llvm;
mod parsers;
pub mod types;

use std::{path::{PathBuf, Path}, env};

use clap::{Parser, Subcommand};

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
    },
    Test {
        /// Source to parse
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    }
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Some(Commands::Emit { file }) => {
            println!("[*] Emitting IR for file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path())
                .expect("Something went wrong reading the input file");
            match akame_interpreter::parse(&script) {
                Ok(ast) => {
                    if args.debug > 0 {
                        println!("AST: {:?}", ast);
                    }
                    llvm::emit(ast)?
                },
                Err(err) => {
                    println!("{:?}", err);
                    std::process::exit(1);
                }
            }
        },
        Some(Commands::Parse { file }) => {
            println!("[*] Parsing file: {}", file.display());
            let script = std::fs::read_to_string(file.as_path()).expect("Something went wrong reading the file");
            match akame_interpreter::parse(&script) {
                Ok(ast) => {
                    println!("AST: {:#?}", ast);
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
        Some(Commands::Test { file }) => {
            println!("[*] Testing: {:?}", file);
            //let script = std::fs::read_to_string(file.as_path()).expect("Something went wrong reading the file");
            let res = find_it("clang");
            println!("clang: {:?}", res);
        },
        None => {
            println!("No subcommand was used");
        }
    }
    Ok(())
}

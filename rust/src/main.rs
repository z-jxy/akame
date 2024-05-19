mod llvm;
mod parsers;
mod red_interpreter;
pub mod types;
mod utils;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

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
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
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
    Run {
        /// Source to parse
        //#[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Repl,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let tracing_level = match args.debug {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2 => Level::TRACE,
        _ => Level::TRACE,
    };

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing_level)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

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
        Some(Commands::Run { file }) => {
            red_interpreter::run_script(&file);
        }
        Some(Commands::Repl) => {
            if let Err(e) = red_interpreter::repl::interactive() {
                eprintln!("{:?}", e);
            }
        }
        None => {
            println!("No subcommand was used");
        }
    }
    Ok(())
}

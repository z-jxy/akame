use std::{process::Command, path::{PathBuf, Path}};

use inkwell::context::Context;

use crate::find_it;

use self::{compiler::Compiler, ast::Ast};

pub mod codegen;
mod compiler;
pub mod ast;

/* we're creating a wrapper around the user's main function so we can initalize the runtime */
pub const GLOBAL_ENTRY : &str = "main";
pub const USER_DEFINED_ENTRY : &str = "_main";
pub const BUILD_DIR : &str = "_build";

pub fn emit(stmts: Vec<ast::Stmt>) -> anyhow::Result<()> {
	let llvm_ir = codegen::emit_from_statements(stmts)?;
	println!("{}", llvm_ir.to_string());
	Ok(())
}

pub fn compile_ast(ast: Ast, out_dir: Option<PathBuf>)-> anyhow::Result<()> {
	// create build directory if it doesn't exist
	match std::path::Path::new(BUILD_DIR).try_exists() {
		Ok(bool) => {
			if bool == false {
				std::fs::create_dir(BUILD_DIR).expect("failed to create build directory");
			}
		},
		Err(e) => {
			println!("Error: {}", e);
			std::fs::create_dir(BUILD_DIR).expect("failed to check if directory exists");
		}
	}
	// emit the LLVM IR to a file
	let context = Context::create();
    let mut compiler = Compiler::new(&context);
	compiler.compile_ir(ast)?;
	let ir_out: String = format!("{}/{}", BUILD_DIR, "main.ll");
	if compiler.module.print_to_file(std::path::Path::new(&ir_out)).is_err() {
		println!("[-] Failed to write LLVM IR to file");
		std::process::exit(1);
	}
	println!("[*] Emitted to LLVM IR 1/2");
	
	// compile the LLVM IR to a binary
	match find_it("clang") {
		Some(clang) => {
			println!("[*] Found a clang at: {}", clang.display());
			let out_file = match out_dir {
				Some(dir) => dir,
				None => Path::new("main.out").to_path_buf(),
			};
			
			let output = Command::new(clang)
			.arg("-o").arg(&out_file) // set the output file
			.arg(ir_out) // set the input file
			.output()
			.expect("Failed to compile to binary from LLVM IR");
	
			match output.status.success() {
				true => {
					println!("[+] [2/2] Successfully compiled to {}", out_file.display());
					Ok(())
				},
				false => {
					println!("[-] Failed to compile to {} 2/2", out_file.display());
					println!("[-] Error: {}", String::from_utf8_lossy(&output.stderr));
					Err(anyhow::anyhow!("[-] Error: {}", String::from_utf8_lossy(&output.stderr)))
				},
			}
		},
		None => {
			println!("[-] clang not found in PATH");
			std::process::exit(1);
		}
	}
}
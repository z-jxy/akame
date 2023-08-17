use std::{process::Command, path::{PathBuf, Path}, env};

use inkwell::context::Context;

use self::compiler::Compiler;

pub mod codegen;
mod compiler;
pub mod ast;

/* we're creating a wrapper around the user's main function so we can initalize the runtime */
pub const GLOBAL_ENTRY : &str = "main";
pub const USER_DEFINED_ENTRY : &str = "_main";

pub fn emit(stmts: Vec<ast::Stmt>) -> anyhow::Result<()> {
	codegen::emit_from_statements(stmts)
}

/// Find the first executable with the given name in the PATH. Used for validating dependencies.
/// clang is required to compile the LLVM IR to a binary. or we can also use gcc if it's available.
/// TODO: gcc will also require `llc` to be able to compile the LLVM IR to assembly. at this point, there should be clang anyway most likely
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

pub fn compile_ast(ast: Vec<ast::Stmt>)-> anyhow::Result<()> {
	let context = Context::create();
    let mut compiler = Compiler::new(&context);
	
	// add the standard library to the compiler
	compiler.add_stdlib();
	// create the real entry point for the program
	compiler.emit_main_function(); 
	// compile the user's ast
    compiler.compile(&ast)?;
	// wrap the user's main function the real entry point created in `emit_main_function`
	compiler.link_user_main_to_entry(); 

	println!("[*] Emitted to LLVM IR 1/2");

	let build_dir = "bin";
	let ir_file_name = "main.ll";
	let out_file = format!("{}/main", build_dir);

	let ir_out: String = format!("{}/{}", build_dir, ir_file_name);

	// create build directory if it doesn't exist
	match std::path::Path::new(build_dir).try_exists() {
		Ok(bool) => {
			if bool == false {
				std::fs::create_dir(build_dir).expect("failed to create build directory");
			}
		},
		Err(e) => {
			println!("Error: {}", e);
			std::fs::create_dir(build_dir).expect("failed to check if directory exists");
		}
	}
    
	if compiler.module.print_to_file(std::path::Path::new(&ir_out)).is_err() {
		println!("[-] Failed to write LLVM IR to file");
		std::process::exit(1);
	}

	match find_it("clang") {
		Some(clang_path) => {
			println!("[*] Found a clang at: {}", clang_path.display());
			let output = Command::new(clang_path)
			.arg("-o").arg(&out_file) // set the output file
			.arg(ir_out) // set the input file
			.output()
			.expect("Failed to compile to binary from LLVM IR");
	
			match output.status.success() {
				true => {
					println!("[+] [2/2] Successfully compiled to {}", out_file);
					Ok(())
				},
				false => {
					println!("[-] Failed to compile to {} 2/2", out_file);
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
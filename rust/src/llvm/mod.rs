use std::process::Command;

use inkwell::context::Context;

use self::compiler::Compiler;

pub mod codegen;
mod compiler;
pub mod ast;
/*
pub fn llvm_codegen() {
	codegen::codegen();
}
 */

//pub fn llvm_debug() {
//	codegen::debug();
//}

pub fn emit(stmts: Vec<ast::Stmt>) {
	codegen::emit_from_statements(stmts);
}

pub fn compile_ast(ast: Vec<ast::Stmt>)-> anyhow::Result<()> {
	let context = Context::create();
    let mut compiler = Compiler::new(&context);
	
	// add the standard library to the compiler
	compiler.add_stdlib();

	// compile the ast
    compiler.compile(&ast);
	// wrap the users main function in a function called __entry, this is the entry point for the program
	compiler.emit_main_function();


	println!("[*] Emitted to LLVM IR 1/2");

	let build_out = "akame-build";
	let ir_file_name = "main.ll";

	let ir_out: String = format!("{}/{}", build_out, ir_file_name);

	// create build directory if it doesn't exist
	match std::path::Path::new(build_out).try_exists() {
		Ok(bool) => {
			if bool == false {
				std::fs::create_dir(build_out).expect("failed to create build directory");
			}
		},
		Err(e) => {
			println!("Error: {}", e);
			std::fs::create_dir(build_out).expect("failed to check if directory exists");
		}
	}
    
	compiler.module.print_to_file(std::path::Path::new(&ir_out)).unwrap();


	let output = Command::new("clang")
		// set the entry point to our _entry function which wraps the users main function
		.arg("-Wl,-e,__entry")
		// set the output file to build/main 
		.arg("-o")
		.arg(&format!("{}/main", build_out))
		// set the input file to build/main.ll
		.arg(ir_out)
		.output()
		.expect("failed to execute process");

	match output.status.success() {
		true => {
			println!("[+] [2/2] Successfully compiled to build/main");
			Ok(())
		},
		false => {
			println!("[-] Failed to compile to build/main 2/2");
			println!("[-] Error: {}", String::from_utf8_lossy(&output.stderr));
			Err(anyhow::anyhow!("[-] Error: {}", String::from_utf8_lossy(&output.stderr)))
		},
	}
	

}
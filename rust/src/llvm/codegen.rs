
use inkwell::context::Context;
use crate::llvm::ast::Stmt;
use crate::llvm::compiler::Compiler;

#[no_mangle]
pub extern "C" fn printd(x: i32) -> i32 {
    println!("{}", x);
    x
}

pub fn emit_from_statements(ast: Vec<Stmt>) -> anyhow::Result<()> {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);
    compiler.add_stdlib();
    compiler.emit_main_function(); // create the real entry point
    compiler.compile(&ast)?; // compile users code
    compiler.link_user_main_to_entry(); // link users main to the real entry point
    println!("{}", compiler.module.print_to_string().to_string());
    Ok(())
}

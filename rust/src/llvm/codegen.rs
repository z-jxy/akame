
use inkwell::context::Context;
use inkwell::support::LLVMString;
use crate::llvm::compiler::Compiler;

use super::ast::Ast;

#[no_mangle]
pub extern "C" fn printd(x: i32) -> i32 {
    println!("{}", x);
    x
}

pub fn emit_from_statements(ast: Ast) -> anyhow::Result<LLVMString> {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);
    compiler.compile_ir(ast)?;
    Ok(compiler.module.print_to_string())
}
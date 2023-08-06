pub mod codegen;
mod compiler;
mod ast;

pub fn llvm_codegen() {
	codegen::codegen();
}

//pub fn llvm_debug() {
//	codegen::debug();
//}
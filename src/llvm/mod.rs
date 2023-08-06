pub mod codegen;

pub fn llvm_codegen() {
	codegen::codegen();
}

pub fn llvm_debug() {
	codegen::debug();
}
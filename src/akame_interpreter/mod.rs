pub mod interpreter;
pub mod repl;

pub fn run_script(script: &str) {
    let mut interpreter = interpreter::Interpreter::new();
    interpreter.eval_source(script);
}
use crate::types::integer::Integer;

// ignore dead code
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}


#[derive(Debug, Clone)]
pub enum Expression {
    Number(Integer),
    Char(char),
    String(String),
    Identifier(String),
    Infix(Box<Expression>, String, Box<Expression>),
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expression),
    Expr(Expression),
    Return(Expression),  
    Function(String, Vec<String>, Vec<Statement>),
    Print(Expression),
}


impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let (ident, expr) => write!(f, "let {} = {}", ident, expr),
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
            Statement::Function(
                ident,
                params,
                body,
            ) => write!(f, "fn {ident}({}) {{\n{}\n}}", params.join(", "), body.iter().map(|stmt| {
                if let Statement::Expr(expr) = stmt {
                    format!("{}", &expr)
                } else {
                    format!("{:?}", &stmt)
                }
            }).collect::<Vec<String>>().join("\n")),
            Statement::Print(expr) => write!(f, "{}", expr),
        }
    }
}

// Assuming an AST enum (a very basic version)
enum ASTNode {
    Function(String, Vec<String>, Box<ASTNode>), // name, params, body
    Expression(Box<ASTNode>),
    Add(Box<ASTNode>, Box<ASTNode>),
    Number(i32),
    Call(String, Vec<ASTNode>),  // function name, arguments
}

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine, JitFunction};
use inkwell::module::Module;
use std::error::Error;

/// Convenience type alias for the `sum` function.
///
/// Calling this is innately `unsafe` because there's no guarantee it doesn't
/// do `unsafe` operations internally.
type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    fn jit_compile_sum(&self) -> Option<JitFunction<SumFunc>> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum");
        let sum = self.builder.build_int_add(sum, z, "sum");

        self.builder.build_return(Some(&sum));

        unsafe { self.execution_engine.get_function("sum").ok() }
    }
}


pub fn go_llvm() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("sum");
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None)?;
    let codegen = CodeGen {
        context: &context,
        module,
        builder: context.create_builder(),
        execution_engine,
    };

    let sum = codegen.jit_compile_sum().ok_or("Unable to JIT compile `sum`")?;

    let x = 1u64;
    let y = 2u64;
    let z = 3u64;

    unsafe {
        println!("{} + {} + {} = {}", x, y, z, sum.call(x, y, z));
        assert_eq!(sum.call(x, y, z), x + y + z);
    }

    Ok(())
}
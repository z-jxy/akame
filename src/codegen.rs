use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::OptimizationLevel;

enum Expr {
    Num(u64),
    Add(Box<Expr>, Box<Expr>),
    Call(String, Box<Expr>),
    Var(String), 
}

enum Stmt {
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Expr,
    },
    Assignment {
        var_name: String,
        expr: Expr,
    },
    Print(String),
}

struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: inkwell::builder::Builder<'ctx>,
    module: inkwell::module::Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    variables: std::collections::HashMap<String, inkwell::values::IntValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    fn compile_expr(&self, expr: &Expr) -> inkwell::values::IntValue<'ctx> {
        match expr {
            Expr::Num(n) => self.context.i64_type().const_int(*n, false),
            Expr::Add(left, right) => {
                let left_value = self.compile_expr(left);
                let right_value = self.compile_expr(right);
                self.builder.build_int_add(left_value, right_value, "addtmp")
            },
            Expr::Call(func_name, arg) => {
                let function = self.module.get_function(func_name).unwrap();
                let arg_value = self.compile_expr(arg);
                let result = self.builder.build_call(function, &[arg_value.into()], "calltmp");
                match result.try_as_basic_value().left() {
                    Some(value) => value.into_int_value(),
                    None => panic!("Expected integer value from function call."),
                }
            },
            Expr::Var(var_name) => {
                if let Some(value) = self.variables.get(var_name) {
                    *value
                } else {
                    // This will print the name of the missing variable
                    println!("Variable {} not found in symbol table", var_name);
                    panic!("Variable not found in symbol table");
                }
            },
        }
    }

    fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::FunctionDeclaration { name, params, body } => {
                    let param_types = vec![self.context.i64_type().into(); params.len()];
                    let fn_type = self.context.i64_type().fn_type(&param_types, false);
                    let function = self.module.add_function(name, fn_type, None);

                    for (i, param) in params.iter().enumerate() {
                        let value = function.get_nth_param(i as u32).unwrap().into_int_value();
                        self.variables.insert(param.clone(), value);
                    }

                    let entry = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(entry);

                    let result = self.compile_expr(body);
                    self.builder.build_return(Some(&result));
                    //unsafe { self.execution_engine.get_function("sum").ok() }
                },
                Stmt::Assignment { var_name, expr } => {
                    //let value = self.compile_expr(expr);
                    //self.variables.insert(var_name.clone(), value);
                },
                Stmt::Print(var_name) => {
                    // For simplicity, just evaluate the function and print the result

                    unsafe {
                        let hello_function = self.execution_engine.get_function::<unsafe extern "C" fn(i32) -> i32>
                        (var_name).unwrap();
                        let result = hello_function.call(5);
                        println!("{}", result);
                    }

                },
            }
        }
    }
}

pub fn codegen() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
    let variables = std::collections::HashMap::new();
    let mut compiler = Compiler {
        context: &context,
        builder,
        module,
        execution_engine,
        variables,
    };

    let ast = vec![
        Stmt::FunctionDeclaration {
            name: "hello".to_string(),
            params: vec!["num".to_string()],
            body: Expr::Add(Box::new(Expr::Num(5)), Box::new(Expr::Var("num".to_string()))), // num + 5
        },
        Stmt::Assignment {
            var_name: "result".to_string(),
            expr: Expr::Call("hello".to_string(), Box::new(Expr::Num(3))),
        },
        Stmt::Print("hello".to_string()),
    ];

    compiler.compile(&ast);
}

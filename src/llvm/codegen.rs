use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::values::AnyValue;
use inkwell::OptimizationLevel;

#[derive(Debug, Clone)]
enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Call(String, Box<Expr>),
    Var(String),
    Print(Box<Expr>), 
}

enum Stmt {
    FunctionDeclaration {
        name: String,
        params: Vec<String>,
        body: Vec<Expr>,
    },
    Assignment {
        var_name: String,
        expr: Expr,
    },
    Print(String),
}

#[derive(Debug, Clone)]
enum VariableValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
    Str(String),
}

struct Compiler<'ctx> {
    context: &'ctx Context,
    builder: inkwell::builder::Builder<'ctx>,
    module: inkwell::module::Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    variables: std::collections::HashMap<String, VariableValue<'ctx>>,
}

#[no_mangle]
pub extern "C" fn printd(x: i32) -> i32 {
    println!("{}", x);
    x
}

//#[no_mangle]
//pub extern "C" fn printf(s: *const u8) {
//    let c_str: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(s) };
//    let str_slice: &std::str::String = c_str.to_str().unwrap();
//    println!("{}", str_slice);
//}

impl<'ctx> Compiler<'ctx> {
    fn compile_expr(&self, expr: &Expr) -> inkwell::values::IntValue<'ctx> {
        match expr {
            Expr::Num(n) => self.context.i32_type().const_int(*n as u64, false),
            Expr::Add(left, right) => {
                let left_value = self.compile_expr(left);
                let right_value = self.compile_expr(right);
                self.builder
                    .build_int_add(left_value, right_value, "addtmp")
            }
            Expr::Call(func_name, arg) => {
                let function = self.module.get_function(func_name).unwrap();
                let arg_value = self.compile_expr(arg);
                let result = self
                    .builder
                    .build_call(function, &[arg_value.into()], "calltmp");
                match result.try_as_basic_value().left() {
                    Some(value) => value.into_int_value(),
                    None => panic!("Expected integer value from function call."),
                }
            }
            Expr::Var(var_name) => match self.variables.get(var_name) {
                Some(VariableValue::Int(value)) => *value,
                Some(VariableValue::Str(referenced_var)) => {
                    match self.variables.get(referenced_var) {
                        Some(VariableValue::Int(referenced_value)) => *referenced_value,
                        _ => panic!("Dereferencing non-int variable or undefined variable"),
                    }
                }
                None => {
                    println!("Variable {} not found in symbol table", var_name);
                    panic!("Variable not found in symbol table");
                }
            },
            Expr::Print(expr) => {
                let value = self.compile_expr(expr);
                let function = self.module.get_function("printd").unwrap();
                self.builder.build_call(function, &[value.into()], "calltmp");
                value
            }
        }
    }

    fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::FunctionDeclaration { name, params, body } => {
                    let param_types = vec![self.context.i32_type().into(); params.len()];
                    let fn_type = self.context.i32_type().fn_type(&param_types, false);
                    let function = self.module.add_function(name, fn_type, None);
                    let entry = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(entry);

                    for (i, param) in params.iter().enumerate() {
                        let value = function.get_nth_param(i as u32).unwrap().into_int_value();
                        self.variables
                            .insert(param.clone(), VariableValue::Int(value));
                    }
                    


                    let mut last_value = None;

                    for (i, expr) in body.iter().enumerate() {
                        let is_last_expr = i == body.len() - 1;

                        match expr {
                            Expr::Call(func_name, arg) => {
                                let function = self.module.get_function(func_name).unwrap();
                                let arg_value = self.compile_expr(arg);
                                let call_result = self.builder.build_call(function, &[arg_value.into()],                    "calltmp");

                                if is_last_expr {
                                    last_value = call_result.try_as_basic_value().left();
                                }
                            }
                            _ => {
                                let value = self.compile_expr(expr);
                                if is_last_expr {
                                    last_value = Some(value.into());
                                }
                            }
                        }
                    }

                    // Return the value of the last expression:
                    if let Some(value) = last_value {
                        self.builder.build_return(Some(&value));
                    } else {
                        // Alternatively, you can have a default return value or handle this case differently
                        self.builder.build_return(Some(&self.context.i64_type().const_int(0, false)));
                    }

            
                    
                    //self.builder.build_return(Some(&result));
                    //unsafe { self.execution_engine.get_function("sum").ok() }
                }
                Stmt::Assignment { var_name, expr } => {
                    match expr {
                        Expr::Var(string_value) => {
                            // If it's a Var expression, assume it's a string value
                            self.variables
                                .insert(var_name.clone(), VariableValue::Str(string_value.clone()));
                        }
                        _ => {
                            let value = self.compile_expr(expr);
                            self.variables
                                .insert(var_name.clone(), VariableValue::Int(value));
                        }
                    }
                }
                Stmt::Print(var_name) => {
                    if let Some(value) = self.variables.get(var_name) {
                        let i32_type = self.context.i32_type();
                        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
                        let printd_func = self.module.add_function("printd", fn_type, None);
                        // Print variable value if it exists
                        match value {
                            VariableValue::Int(value) => {
                                let value = value.clone();
                                self.builder
                                    .build_call(
                                        printd_func, 
                                        &[value.into()], 
                                        "printd_call");

                            },
                            VariableValue::Str(value) => println!("{} => {}", var_name, value),
                        }
                    } else {
                        // Try to execute it as a function
                        let i32_type = self.context.i32_type();
                        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
                        let printd_func = self.module.add_function("printd", fn_type, None);
                        unsafe {
                            if let Ok(hello_function) =
                                self.execution_engine
                                    .get_function::<unsafe extern "C" fn(i32) -> i32>(var_name)
                            {
                                let result = hello_function.call(5);
                                //assert_eq!(result, 10);
                                // print the output in stdout from final executable
                                let v = self.context.i32_type().const_int(result.try_into().unwrap(), false);
                                self.builder
                                .build_call(
                                    printd_func, 
                                    &[v.into()], 
                                    "printd_call");


                                //println!("{} => {}", var_name, result);
                            } else {
                                //println!("Error: Cannot print '{}'", var_name);
                            }
                        }
                    }
                    //println!("variables: {:#?}", self.variables);
                }
            }
        }
    }
}

pub fn codegen() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
    let variables = std::collections::HashMap::new();
    let mut compiler = Compiler {
        context: &context,
        builder,
        module,
        execution_engine,
        variables,
    };

    let i32_type = compiler.context.i32_type();
    let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    compiler.module.add_function("printd", fn_type, None);

    let ast = vec![
        // define hello function
        Stmt::FunctionDeclaration {
            name: "hello".to_string(),
            params: vec!["num".to_string()],
            body: vec![Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Var("num".to_string())),
            )], // num + 5
        },
        // define main function
        Stmt::FunctionDeclaration {
            name: "main".to_string(),
            params: vec!["argc".to_string()],
            body: vec![
                Expr::Call("hello".to_string(), Box::new(Expr::Num(5))),
                Expr::Call("printd".to_string(), Box::new(Expr::Num(5)))
            ],
        },
        Stmt::Assignment {
            var_name: "x".to_string(),
            expr: Expr::Var("hello-world".to_string()),
        },
    ];
    compiler.compile(&ast);
    println!("{}", compiler.module.print_to_string().to_string());
}

pub fn debug() {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let execution_engine = module
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();
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
            body: vec![Expr::Add(
                Box::new(Expr::Num(5)),
                Box::new(Expr::Var("num".to_string())),
            )], // num + 5
        },
        Stmt::FunctionDeclaration {
            name: "main".to_string(),
            params: vec!["argc".to_string()],
            body: vec![Expr::Call("hello".to_string(), Box::new(Expr::Num(5)))],
        },
        Stmt::Assignment {
            var_name: "x".to_string(),
            expr: Expr::Var("hello-world".to_string()),
        },
        Stmt::Print("hello".to_string()),
    ];
    compiler.compile(&ast);
    //println!("{}", compiler.module.print_to_string().to_string());
}
use inkwell::{execution_engine::ExecutionEngine, context::Context, AddressSpace, values::BasicValue, OptimizationLevel};

use super::ast::{Stmt, Expr, VariableValue};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    builder: inkwell::builder::Builder<'ctx>,
    pub  module: inkwell::module::Module<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
    variables: std::collections::HashMap<String, VariableValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(ctx: &'ctx Context) -> Compiler<'ctx> {
        let module = ctx.create_module("main");
        let builder = ctx.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let variables = std::collections::HashMap::new();
        Compiler {
            context: &ctx,
            builder,
            module,
            execution_engine,
            variables,
        }
    }


    fn compile_expr(&self, expr: &Expr) -> inkwell::values::BasicValueEnum<'ctx> {
        match expr {
            Expr::Str(s) => {
                let string_val = self.context.const_string(s.as_bytes(), false);
                let global_str = self.module.add_global(string_val.get_type(), Some             (AddressSpace::default()), "global_string");
                global_str.set_initializer(&string_val);
                global_str.as_pointer_value().into()
            },
            Expr::Num(n) => self.context.i32_type().const_int(*n as u64, false).into(),
            Expr::Add(left, right) => {
                let left_value = self.compile_expr(left).into_int_value();
                let right_value = self.compile_expr(right).into_int_value();
                self.builder.build_int_add(left_value, right_value, "addtmp").into()
            }
            Expr::Call(func_name, arg) => {
                let function = self.module.get_function(func_name).unwrap();
                let arg_value = self.compile_expr(arg);
                let result = self
                    .builder
                    .build_call(function, &[arg_value.into()], "calltmp");
                match result.try_as_basic_value().left() {
                    Some(value) => value.into(),
                    None => panic!("Expected integer value from function call."),
                }
            }
            Expr::Var(var_name) => match self.variables.get(var_name) {
                Some(VariableValue::Int(value)) => inkwell::values::BasicValueEnum::IntValue(*value),
                Some(VariableValue::Str(referenced_var)) => {
                    match self.variables.get(referenced_var) {
                        Some(VariableValue::Int(referenced_value)) => inkwell::values::BasicValueEnum::IntValue(*referenced_value),
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

    pub fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::FunctionDeclaration { ident: name, params, body } => {
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

                    for (i, stmt) in body.iter().enumerate() {
                        let is_last_expr = i == body.len() - 1;

                        match stmt {
                            Stmt::Assignment { ident: var_name, expr } => {
                                let value = self.compile_expr(expr);
                                self.variables
                                    .insert(var_name.clone(), VariableValue::Int(value.into_int_value()));
                                if is_last_expr {
                                    last_value = Some(value.into());
                                }
                            },
                            x => {
                                    if let Stmt::Expression(expr) = x {
                                    let value = self.compile_expr(expr);
                                    if is_last_expr {
                                        last_value = Some(value.as_basic_value_enum());
                                    }
                                }
                            },
                            //Expr::Call(func_name, arg) => {
                            //    let function = self.module.get_function(func_name).unwrap();
                            //    let arg_value = self.compile_expr(arg);
                            //    let call_result = self.builder.build_call(function, &[arg_value.into()],                    "calltmp");

                            //    if is_last_expr {
                            //        last_value = call_result.try_as_basic_value().left();
                            //    }
                            //}
                            //_ => {
                            //    let value = self.compile_expr(expr);
                            //    if is_last_expr {
                            //        last_value = Some(value.into());
                            //    }
                            //}
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
                Stmt::Assignment { ident: var_name, expr } => {
                    match expr {
                        Expr::Var(string_value) => {
                            // If it's a Var expression, assume it's a string value
                            self.variables
                                .insert(var_name.clone(), VariableValue::Str(string_value.clone()));
                        }
                        _ => {
                            let value = self.compile_expr(expr);
                            self.variables
                                .insert(var_name.clone(), VariableValue::Int(value.into_int_value()));
                        }
                    }
                },
                Stmt::Expression(expr) => {
                    self.compile_expr(expr);
                },


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
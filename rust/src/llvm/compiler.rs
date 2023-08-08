use inkwell::{execution_engine::ExecutionEngine, context::Context, AddressSpace, values::{BasicValue, BasicValueEnum}, OptimizationLevel};

use super::ast::{Stmt, Expr, VariableValue, BinaryOp};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: inkwell::builder::Builder<'ctx>,
    pub  module: inkwell::module::Module<'ctx>,
    #[allow(dead_code)]
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

    // implementations for some stdlib functions we're going to make available
    pub fn add_stdlib(&self) {
        // printf
        let printf_type = self.context.i8_type().fn_type(
            &[self.context.i8_type().ptr_type(AddressSpace::default()).into()],
            true
        );
        self.module.add_function("printf", printf_type, None);

        // Define printd
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let printd_fn = self.module.add_function("printd", fn_type, None);
            
        let basic_block = self.context.append_basic_block(printd_fn, "entry");
            self.builder.position_at_end(basic_block);
        let param = printd_fn.get_nth_param(0).unwrap().into_int_value();
        
        let format_str = self.builder.build_global_string_ptr("%d\n\00", "format_str");
            self.builder
                .build_call(
                    self.module.get_function("printf").unwrap(),
                    &[inkwell::values::BasicMetadataValueEnum::PointerValue(format_str.as_pointer_value()), param.into()], "calltmp");
        self.builder.build_return(Some(&param));
    }

    pub fn add_string_format_global(&self) -> inkwell::values::GlobalValue<'ctx> {
        let str_format = self.builder.build_global_string_ptr("%s\n", "str_format");
        str_format
    }

    pub fn emit_main_function(&self) {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into(), i8_ptr_type.ptr_type(AddressSpace::default()).into()], false);
        
        let main_fn = self.module.add_function("_entry", fn_type, None);
        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);
    
        // Retrieve argv[0]
        let argv = main_fn.get_nth_param(1).unwrap().into_pointer_value();
        let argv_1_ptr = unsafe {
            self.builder.build_gep(
                argv.get_type().get_context().i8_type().ptr_type(AddressSpace::default()),
                argv, 
                &[i32_type.const_int(1, false)], 
                "argv_0_ptr")
        };
        let argv_0 = self.builder.build_load(
            self.context.i8_type().ptr_type(AddressSpace::default()),
            argv_1_ptr, 
            "argv_0");
    
        // Call printf
        let printf_fn = self.module.get_function("printf").expect("printf function not found");
        let format_str = self.add_string_format_global();
        self.builder.build_call(
            printf_fn, 
            &[format_str.as_basic_value_enum().into(), argv_0.into()], 
            "printf_call");

        let user_defined_main = self.module.get_function("main")
            .expect("main function not found");

        // Call user-defined main with no args
        self.builder.build_call(
            user_defined_main, 
            &[], 
            "user_main_call");


        // Create a new stack frame for the function

        // Create a new basic block for the function


        // Call the user-defined main function

        // Return the value from the user-defined main function


    
        // Return 0 from main
        self.builder.build_return(Some(&i32_type.const_int(0, false)));
    }

    fn compile_expr(&self, expr: &Expr) -> inkwell::values::BasicValueEnum<'ctx> {
        match expr {
            //Expr::Str(s) => {
            //    let string_val = self.context.const_string(s.as_bytes(), false);
            //    let global_str = self.module.add_global(string_val.get_type(), Some             (AddressSpace::default()), "global_string");
            //    global_str.set_initializer(&string_val);
            //    global_str.as_pointer_value().into()
            //},
            Expr::Str(s) => {
                // Create a global constant for the string with a null terminator
                let global_string = self.module.add_global(
                    self.context.i8_type().array_type((s.len() + 1) as u32),
                    None,
                    &format!("str_{}", s)
                );
                global_string.set_initializer(&self.context.const_string(s.as_bytes(), true));

                let gep_indices = [
                    self.context.i32_type().const_zero(), // For the first dimension (since it's a global array)
                    self.context.i32_type().const_zero()  // For the first character of the string
                ];
        
                // Return a pointer to the start of the string
                unsafe {
                    inkwell::values::BasicValueEnum::PointerValue(
                        self.builder
                        .build_gep(
                            self.context.i8_type().array_type((s.len() + 1) as u32),
                            global_string.as_pointer_value(), 
                            &gep_indices, 
                            "str_ptr")
                    )   
                }
            }
            Expr::Num(n) => self.context
                .i32_type()
                .const_int(*n as u64, false)
                .into(),
            Expr::Call(func_name, arg) => {
                let function = self.module.get_function(func_name).unwrap();
                let args = arg.iter().map(|arg| self.compile_expr(arg).into()).collect::<Vec<inkwell::values::BasicMetadataValueEnum>>();
                let result = self
                    .builder
                    .build_call(function, args.as_slice(), "calltmp");
                match result.try_as_basic_value().left() {
                    Some(value) => value.into(),
                    None => panic!("Expected integer value from function call."),
                }
            }
            Expr::Ident(var_name) => match self.variables.get(var_name) {
                Some(VariableValue::Int(value)) => BasicValueEnum::IntValue(*value),
                Some(VariableValue::Str(referenced_var)) => {
                    match self.variables.get(referenced_var) {
                        Some(VariableValue::Int(referenced_value)) => BasicValueEnum::IntValue(*referenced_value),
                        Some(VariableValue::Str(s)) => {
                            let gep_indices = [
                                self.context.i32_type().const_zero(), // For the first dimension (since it's a global array)
                                self.context.i32_type().const_zero()  // For the first character of the string
                            ];
                            
                            // Return a pointer to the start of the string

                            unsafe {
                                inkwell::values::BasicValueEnum::PointerValue(
                                    self.builder
                                    .build_gep(
                                        self.context.i8_type().array_type((s.len() + 1) as u32),
                                        self.module.get_global(&format!("str_{}", s)).unwrap().as_pointer_value(),
                                        &gep_indices, 
                                        "str_ptr")
                                )   
                            }

                        }
                        _ => panic!("Dereferencing non-int variable or undefined variable"),
                    }
                }
                None => {
                    println!("Variable {} not found in symbol table", var_name);
                    panic!("Variable not found in symbol table");
                }
            },
            Expr::Char(c) => {
                let char_val = self.context.i8_type().const_int(*c as u64, false);
                char_val.into()
            },
            Expr::Infix(left, op, right) => {
                let left_value = self.compile_expr(left).into_int_value();
                let right_value = self.compile_expr(right).into_int_value();
                match op {
                    BinaryOp::Add => self.builder.build_int_add(left_value, right_value, "addtmp").into(),
                    BinaryOp::Subtract => self.builder.build_int_sub(left_value, right_value, "subtmp").into(),
                    BinaryOp::Multiply => self.builder.build_int_mul(left_value, right_value, "multmp").into(),
                    BinaryOp::Divide => self.builder.build_int_signed_div(left_value, right_value, "divtmp").into(),
                    //"%" => self.builder.build_int_signed_rem(left_value, right_value, "modtmp").into(),
                }
            }
            //Expr::Print(expr) => {
            //    let value = self.compile_expr(expr);
            //    let function = self.module.get_function("printd").unwrap();
            //    self.builder.build_call(function, &[value.into()], "calltmp");
            //    value
            //}
        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Return(expr) => {
                    let value = self.compile_expr(expr);
                    self.builder.build_return(Some(&value));
                },
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
                                //if is_last_expr {
                                //    last_value = Some(value.into());
                                //}
                            },
                            Stmt::Return(expr) => {
                                let value = self.compile_expr(expr);
                                self.builder.build_return(Some(&value));
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

                    // check if there's a return value already, if not, return 0
                    if body.iter().any(|stmt| {
                        if let Stmt::Return(_) = stmt {
                            true
                        } else {
                            false
                        }
                    }) {
                    } else {
                        // Alternatively, you can have a default return value or handle this case differently
                        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
                    }

                    // Return the value of the last expression:
                    /*
                    if let Some(value) = last_value {
                        self.builder.build_return(Some(&value));
                    } else {
                        // Alternatively, you can have a default return value or handle this case differently
                        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
                    }
                     */
            
                    
                    //self.builder.build_return(Some(&result));
                    //unsafe { self.execution_engine.get_function("sum").ok() }
                }
                Stmt::Assignment { ident: var_name, expr } => {
                    match expr {
                        Expr::Ident(string_value) => {
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


                //Stmt::Print(var_name) => {
                //    if let Some(value) = self.variables.get(var_name) {
                //        let i32_type = self.context.i32_type();
                //        let fn_type = i32_type.fn_type(&[i32_type.into()], //false);
                //        let printd_func = self.module.add_function("printd", //fn_type, None);
                //        // Print variable value if it exists
                //        match value {
                //            VariableValue::Int(value) => {
                //                let value = value.clone();
                //                self.builder
                //                    .build_call(
                //                        printd_func, 
                //                        &[value.into()], 
                //                        "printd_call");
//
                //            },
                //            VariableValue::Str(value) => println!("{} => {}", var_name, value),
                //        }
                //    } else {
                //        // Try to execute it as a function
                //        let i32_type = self.context.i32_type();
                //        let fn_type = i32_type.fn_type(&[i32_type.into()], //false);
                //        let printd_func = self.module.add_function("printd", //fn_type, None);
                //        unsafe {
                //            if let Ok(hello_function) =
                //                self.execution_engine
                //                    .get_function::<unsafe extern "C" fn(i32) -> i32>(var_name)
                //            {
                //                let result = hello_function.call(5);
                //                //assert_eq!(result, 10);
                //                // print the output in stdout from final executable
                //                let v = self.context.i32_type().const_int(result.//try_into().unwrap(), false);
                //                self.builder
                //                .build_call(
                //                    printd_func, 
                //                    &[v.into()], 
                //                    "printd_call");
//
//
                //                //println!("{} => {}", var_name, result);
                //            } else {
                //                //println!("Error: Cannot print '{}'", var_name);
                //            }
                //        }
                //    }
                //    //println!("variables: {:#?}", self.variables);
                //}
            }
        }
    }

}
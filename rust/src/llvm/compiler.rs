use inkwell::{execution_engine::ExecutionEngine, context::Context, AddressSpace, values::{BasicValue, BasicValueEnum, AnyValueEnum, AnyValue}, OptimizationLevel, types::{BasicType, VoidType, AnyType, BasicTypeEnum}};

use super::ast::{Stmt, Expr, VariableValue, BinaryOp};

pub struct Compiler<'ctx> {
    pub context: &'ctx Context,
    pub builder: inkwell::builder::Builder<'ctx>,
    pub  module: inkwell::module::Module<'ctx>,
    #[allow(dead_code)]
    execution_engine: ExecutionEngine<'ctx>,
    variables: std::collections::HashMap<String, VariableValue<'ctx>>,
}   

const GLOBAL_ENTRY : &str = "_entry";

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
        self.add_printf();
        self.add_print_string_fn();
        self.add_printd();
    }


    pub fn get_runtime_args(&self) {
        // lookup the entry point and get the args
        let i32_type = self.context.i32_type();
        let main_fn = self.module.get_function(GLOBAL_ENTRY).unwrap();
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
    }

    fn add_print_string_fn(&self) {
        let i8_ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let void_type = self.context.void_type();
        
        // Define function type for our wrapper
        let fn_type = void_type.fn_type(&[i8_ptr_type.into()], false);
    
        // Add function to the module
        let function = self.module.add_function("print", fn_type, None);
    
        // Create the entry block
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
    
        // Load our format string
        let format_str_s = self.builder.build_global_string_ptr("%s\n\00", "format_str_s_");
    
        // Get the first parameter (the string to print)
        let arg = function.get_nth_param(0).unwrap().into_pointer_value();
    
        // Call printf with our format string and the argument
        let printf_fn = self.module.get_function("printf").expect("printf function not found");
        self.builder.build_call(
            printf_fn, 
            &[format_str_s.as_basic_value_enum().into(), 
            arg.into()], "printf_call");
    
        // Return
        self.builder.build_return(None);
    }


    fn add_printf(&self) {
        // printf
        let printf_type = self.context.i8_type().fn_type(
            &[self.context.i8_type().ptr_type(AddressSpace::default()).into()],
            true
        );
        
        self.module.add_function("printf", printf_type, None);
    }

    fn add_printd(&self) {
        // Define printd
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[i32_type.into()], false);
        let printd_fn = self.module.add_function("printd", fn_type, None);
            
        let basic_block = self.context.append_basic_block(printd_fn, "entry");
            self.builder.position_at_end(basic_block);
        let param = printd_fn.get_nth_param(0).unwrap().into_int_value();
        
        let format_str = self.builder.build_global_string_ptr("%d\n\00", "format_str_d");
        self.builder.build_global_string_ptr("%s\n\00", "format_str_s");
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
        
        let main_fn = self.module.add_function(GLOBAL_ENTRY, fn_type, None);


        let entry = self.context.append_basic_block(main_fn, "entry");
        self.builder.position_at_end(entry);

            // Store argc and argv as global variables
        let argc_global = self.module.add_global(i32_type, Some(AddressSpace::default()),   "argc_global");
        let argv_global = self.module.add_global(i8_ptr_type.ptr_type(AddressSpace::default()),     Some(AddressSpace::default()), "argv_global");

        argc_global.set_initializer(&i32_type.const_zero()); // initialize with 0
        argv_global.set_initializer(&i8_ptr_type.const_null()); // initialize with null

        let argc = main_fn.get_nth_param(0).unwrap();
        let argv = main_fn.get_nth_param(1).unwrap().into_pointer_value();

        self.builder.build_store(argc_global.as_pointer_value(), argc);
        self.builder.build_store(argv_global.as_pointer_value(), argv);

    }

    pub fn link_user_main_to_entry(&self) {
        let user_defined_main = self.module.get_function("main")
        .expect("main function not found");

        let real_entry = self.module.get_function(GLOBAL_ENTRY).expect("_entry function not found");
        // add a new basic block to the entry function

        let blocks = real_entry.get_basic_blocks();

        if let Some(block) = blocks.first()  {
            self.builder.position_at_end(*block);
            // Call user-defined main with no args
            self.builder.build_call(
                user_defined_main, 
                &[], 
                "user_main_call");
            // Return 0 from main
            println!("[*] linked user-defined main to _entry");
            self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
        }
        /*
        let entry = self.context.append_basic_block(real_entry, "entry");
        self.builder.position_at_end(entry);
        // Call user-defined main with no args

        self.builder.build_call(
            user_defined_main, 
            &[], 
            "user_main_call");

        // Return 0 from main
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false)));
         */
    }

    fn compile_expr(&self, expr: &Expr) -> AnyValueEnum<'ctx> {
        match expr {
            Expr::Str(s) => {
                let string_val = self.context.const_string(s.as_bytes(), false);
                let global_str = self.module.add_global(string_val.get_type(), Some             (AddressSpace::default()), "global_string");
                global_str.set_initializer(&string_val);
                global_str.as_pointer_value().into()
            },
            Expr::Array(array) => {
                // Here, we're assuming all elements of your array are of the same type.
                // We'll compile the first expression to get its type, 
                // then use it to create the LLVM array type.
                let element = self.compile_expr(&array[0]);
                let array_type = element.get_type().into_array_type().array_type(array.len() as u32);

                //let array_vals: Vec<_> = array.iter().map(|e| self.compile_expr(e)).collect();
                
                
                let array_vals: Vec<_> = array.iter().map(|e| self.compile_expr(e).into_array_value()).collect();
                
                // Create a constant array with the given values.
               // let const_array = array_type.const_array(&array_values);
                
                // Create a global instance of the array and set its initializer.
                let global_array = self.module.add_global(array_type, Some(AddressSpace::default()), "global_array");
                global_array.set_initializer(&array_vals[0]);
                
                // Return the global array's pointer.
                global_array.as_pointer_value().into()
            },
            Expr::ArrayIndexing(array, index) => {
                let array_val_pointer = self.compile_expr(array).into_pointer_value();
                let index_val = self.compile_expr(index).into_int_value();
            
                // You might have a function like this:
                let element_type = self.context.i32_type().ptr_type(AddressSpace::default()); // This function returns the BasicType of the element.
                
                let gep = unsafe {
                    self.builder.build_gep(
                        element_type,
                        array_val_pointer,
                        &[index_val],
                        "array_indexing"
                    )
                };
                self.builder.build_load(
                    element_type,
                    gep, 
                    "array_indexing_load"
                ).into()
            }
            Expr::QualifiedIdent(idents) => {
                // Assuming idents is a Vec<String> or similar
                if let Some(first_ident) = idents.first() {
                    match first_ident.as_str() {
                        "std" => self.stdlib(idents).expect("Unable to link to stdlib").into(),
                        _ => panic!("Unknown qualified identifier: {}", first_ident),
                    }
                } else {
                    panic!("Empty qualified identifier");
                }
            },
            Expr::Num(n) => self.context
                .i32_type()
                .const_int(*n as u64, false)
                .into(),
            Expr::Call(func_name, arg) => {
                let function = self.module.get_function(func_name).unwrap();
                let args = arg.iter().map(|arg| self.compile_expr(arg).try_into().expect("Unable to try into basic BasicMetadataValueEnum for call")).collect::<Vec<inkwell::values::BasicMetadataValueEnum>>();
                let result = self
                    .builder
                    .build_call(function, args.as_slice(), "calltmp");
                //let ret_type = function.get_type()
                 //   .get_return_type().expect("No return type found");
                match result.try_as_basic_value().left() {
                    Some(value) => value.into(),
                    None => {
                        //panic!("Expected integer value from function call.: {:#?}", result);
                        // it's a void reutrn so we shou;dn't return anything
                 
                        AnyValueEnum::PointerValue(self.context.i32_type().ptr_type(AddressSpace::default()).const_zero())
                    },
                }
            }
            Expr::Ident(var_name) => match self.variables.get(var_name) {
                Some(VariableValue::Int(value)) => AnyValueEnum::IntValue(*value),
                Some(VariableValue::Ptr(value)) => AnyValueEnum::PointerValue(*value),
                Some(VariableValue::Str(referenced_var)) => {
                    match self.variables.get(referenced_var) {
                        Some(VariableValue::Int(referenced_value)) => AnyValueEnum::IntValue(*referenced_value),
                        Some(VariableValue::Str(s)) => {
                            let gep_indices = [
                                self.context.i32_type().const_zero(), // For the first dimension (since it's a global array)
                                self.context.i32_type().const_zero()  // For the first character of the string
                            ];
                            
                            // Return a pointer to the start of the string

                            unsafe {
                                inkwell::values::AnyValueEnum::PointerValue(
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
                    panic!("Variable: {var_name} not found in symbol table");
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

        }
    }

    pub fn compile(&mut self, stmts: &[Stmt]) {
        for stmt in stmts {
            match stmt {
                Stmt::Return(expr) => {
                    let value = self.compile_expr(expr);
                    if value.get_type().is_void_type() {
                        self.builder.build_return(None);
                    } else {
                        match value {
                            AnyValueEnum::IntValue(i) => {
                                self.builder.build_return(Some(&i));
                            },
                            AnyValueEnum::PointerValue(p) => {
                                self.builder.build_return(Some(&p));
                            },
                            _ => panic!("Unknown return type: {:#?}", value),
                        }
                    };
                },
                Stmt::FunctionDeclaration { ident: name, params, body } => {
                    let param_types: Vec<inkwell::types::BasicMetadataTypeEnum<'_>> = vec![self.context.i32_type().into(); params.len()];
                    let fn_type = self.context.i32_type().fn_type(&param_types, false);
                    let function = self.module.add_function(name, fn_type, None);
                    let entry = self.context.append_basic_block(function, "entry");
                    self.builder.position_at_end(entry);

                    for (i, param) in params.iter().enumerate() {
                        let value = function.get_nth_param(i as u32).unwrap().into_int_value();
                        self.variables
                            .insert(param.clone(), VariableValue::Int(value));
                    }
                    


                    let mut _last_value = None;

                    for (i, stmt) in body.iter().enumerate() {
                        let is_last_expr = i == body.len() - 1;

                        match stmt {
                            Stmt::Assignment { ident: var_name, expr } => {
                                let value = self.compile_expr(expr);
                                match value {
                                    AnyValueEnum::IntValue(int_val) => {
                                        self.variables
                                            .insert(var_name.clone(), VariableValue::Int(int_val));
                                    },
                                    AnyValueEnum::PointerValue(ptr_val) => {
                                        // If you want to refine further, you might check the type of the pointer
                                        // but for now, we'll assume any pointer is a string
                                        self.variables
                                            .insert(var_name.clone(), VariableValue::Ptr(ptr_val));
                                    },
                                    // Add other types as necessary
                                    _ => panic!("Unsupported assignment type for variable {}", var_name),
                                }
                            },
                            Stmt::Return(expr) => {
                                let value = self.compile_expr(expr);
                                if value.get_type().is_void_type() {
                                    self.builder.build_return(None);
                                } else {
                                    match value {
                                        AnyValueEnum::IntValue(i) => {
                                            self.builder.build_return(Some(&i));
                                        },
                                        AnyValueEnum::PointerValue(p) => {
                                            self.builder.build_return(Some(&p));
                                        },
                                        _ => panic!("Unknown return type: {:#?}", value),
                                    }
                                };
                            },
                            x => {
                                    if let Stmt::Expression(expr) = x {
                                    let value = self.compile_expr(expr);
                                    if is_last_expr {
                                        _last_value = Some(value.as_any_value_enum());
                                    }
                                }
                            },
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

            }
        }
    }

    pub fn stdlib(&self, idents: &Vec<String>) -> anyhow::Result<BasicValueEnum<'ctx>> {
                if let Some(second_ident) = idents.get(1) {
                    match second_ident.as_str() {
                        "printf" => {
                            let printf_fn = self.module.get_function("printf").expect("printf function not found");
                            let format_str = self.add_string_format_global();
                            self.builder.build_call(
                                printf_fn, 
                                &[format_str.as_basic_value_enum().into()], 
                                "printf_call");
                            return Ok(self.context.i32_type().const_int(0, false).into());
                        },
                        "args" => {
                            let argv_global = self.module.get_global("argv_global").expect("argv_global not found");

                            let argv_type = self.context.i8_type().ptr_type(AddressSpace::default()).ptr_type(AddressSpace::default());
                            let argv = self.builder.build_load(argv_type, argv_global.as_pointer_value(), "argv_val").into_pointer_value();

                            // return argv
                            return Ok(argv.into());
                        }
                        _ => panic!("Unknown std function: {}", second_ident),
                    } 
                }   else {
                    // Handle error: no identifier
                    panic!("No identifier found");
                } 
            
         
    }

}

use inkwell::context::Context;
use crate::llvm::ast::Stmt;
use crate::llvm::compiler::Compiler;

#[no_mangle]
pub extern "C" fn printd(x: i32) -> i32 {
    println!("{}", x);
    x
}

pub fn emit_from_statements(ast: Vec<Stmt>) {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    compiler.add_stdlib();
    
    compiler.emit_main_function(); // create the real entry point
    compiler.compile(&ast); // compile users code
    compiler.link_user_main_to_entry(); // link users main to the real entry point
    //compiler.module.print_to_file(std::path::Path::new("main.ll")).unwrap();
    println!("{}", compiler.module.print_to_string().to_string());
}
/*
pub fn codegen() {
    let context = Context::create();
    let mut compiler = Compiler::new(&context);

    //let i32_type = compiler.context.i32_type();
    //let fn_type = i32_type.fn_type(&[i32_type.into()], false);
    //compiler.module.add_function("printd", fn_type, None)

    // we get this from libc 
    let printf_type = compiler.context.i8_type().fn_type(
        &[compiler.context.i8_type().ptr_type(AddressSpace::default()).into()],
        true
    );
    compiler.module.add_function("printf", printf_type, None);

    let ast = vec![
        // define hello function
        Stmt::FunctionDeclaration {
            ident: "hello".to_string(),
            params: vec!["num".to_string()],
            body: vec![
                Stmt::Expression(Expr::Infix(
                    Box::new(Expr::Num(5)),
                    "+".into(),
                    Box::new(Expr::Ident("num".to_string()))
                )),
            ], // num + 5
        },
        // define main function
        Stmt::FunctionDeclaration {
            ident: "main".to_string(),
            params: vec!["argc".to_string(), "argv".to_string()],
            body: vec![
                Stmt::Expression(Expr::Call("hello".to_string(), vec![Expr::Num(5)])),
                Stmt::Expression(Expr::Call("printf".to_string(), vec![Expr::Str("helloWorld".to_string())])),
                Stmt::Return(Expr::Num(0)),
            ],
        },
        Stmt::Assignment {
            ident: "x".to_string(),
            expr: Expr::Ident("hello-world".to_string()),
        },
    ];
    compiler.compile(&ast);
    compiler.module.print_to_file(std::path::Path::new("main.ll")).unwrap();
    println!("{}", compiler.module.print_to_string().to_string());
}
 */
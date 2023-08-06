

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    Add(Box<Expr>, Box<Expr>),
    Call(String, Box<Expr>),
    Var(String),
    Str(String),
    Print(Box<Expr>), 
}

pub enum Stmt {
    FunctionDeclaration {
        ident: String,
        params: Vec<String>,
        body: Vec<Stmt>,
    },
    Assignment {
        ident: String,
        expr: Expr,
    },
    Print(String),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub enum VariableValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
    Str(String),
}
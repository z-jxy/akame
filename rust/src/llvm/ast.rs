

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i32),
    //Add(Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>), // should this be a box?
    //Var(String),
    Str(String),
    Ident(String),
    Char(char),
    Infix(Box<Expr>, BinaryOp, Box<Expr>),
    //Print(Box<Expr>), 
}

#[derive(Debug, Clone)]
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
    //Print(String),
    Expression(Expr),
    Return(Expr),
}

#[derive(Debug, Clone)]
pub enum VariableValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
    Str(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Into<BinaryOp> for &str {
    fn into(self) -> BinaryOp {
        match self {
            "+" => BinaryOp::Add,
            "-" => BinaryOp::Subtract,
            "*" => BinaryOp::Multiply,
            "/" => BinaryOp::Divide,
            _ => panic!("Unknown binary operator: {}", self),
        }
    }
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinaryOp::Add => write!(f, "+"),
            BinaryOp::Subtract => write!(f, "-"),
            BinaryOp::Multiply => write!(f, "*"),
            BinaryOp::Divide => write!(f, "/"),
        }
    }
}
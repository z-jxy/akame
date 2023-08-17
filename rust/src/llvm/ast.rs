pub type Ast = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(i32),
    Call(String, Vec<Expr>), // should this be a box?
    Str(String),
    Ident(String),
    QualifiedIdent(Vec<String>),
    Char(char),
    Infix(Box<Expr>, BinaryOp, Box<Expr>),

    Array(Vec<Expr>),             // Represents an array literal, e.g., [1, "hello", 'c']
    ArrayIndexing(Box<Expr>, Box<Expr>), // Represents array indexing, e.g., arr[2]

}

#[derive(Debug, Clone, PartialEq)]
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
    Expression(Expr),
    Return(Expr),
}



#[derive(Debug, Clone)]
pub enum VariableValue<'ctx> {
    Int(inkwell::values::IntValue<'ctx>),
    Str(String),
    Ptr(inkwell::values::PointerValue<'ctx>),
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

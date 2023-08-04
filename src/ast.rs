pub enum ASTNode {
    NumberNode { value: i32 },
    IdentifierNode { value: String },
    StringNode { value: String },
    ProgramNode { statements: Vec<StatementNode> },
    StatementNode { statement: Statement },
    ExpressionNode { expression: Expression },
    BinaryOpNode { op: BinaryOp, left: Box<ASTNode>, right: Box<ASTNode> },
    CharacterNode { value: char },
}

pub struct StatementNode {
    pub statement: Statement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Return { value: Expression },
    Expression { value: Expression },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier { value: String },
    Number { value: i32 },
    Prefix { operator: String, right: Box<Expression> },
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionLiteral {
    parameters: Vec<String>,
    body: BlockStatement,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Char(char),
    String(String),
    Identifier(String),
    Infix(Box<Expr>, String, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(String, Expr),
    Expr(Expr),
    Return(Expr),  
    Function(String, Vec<String>, Vec<Stmt>),
}

use crate::types::integer::Integer;

// ignore dead code
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}


#[derive(Debug, Clone)]
pub enum Expression {
    Number(Integer),
    Char(char),
    String(String),
    Identifier(String),
    Infix(Box<Expression>, String, Box<Expression>),
    Call(String, Vec<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let(String, Expression),
    Expr(Expression),
    Return(Expression),  
    Function(String, Vec<String>, Vec<Statement>),
}

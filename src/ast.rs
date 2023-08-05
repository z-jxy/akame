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


impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let (ident, expr) => write!(f, "let {} = {}", ident, expr),
            Statement::Expr(expr) => write!(f, "{}", expr),
            Statement::Return(expr) => write!(f, "return {}", expr),
            Statement::Function(
                ident,
                params,
                body,
            ) => write!(f, "fn {ident}({}) {{\n{}\n}}", params.join(", "), body.iter().map(|stmt| {
                if let Statement::Expr(expr) = stmt {
                    format!("{}", &expr)
                } else {
                    format!("{:?}", &stmt)
                }
            }).collect::<Vec<String>>().join("\n")),
        }
    }
}

use std::collections::HashMap;

use crate::ast::{ASTNode, BinaryOp, Stmt, Expr};

pub struct Interpreter {
    pub symbol_table: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Str(String),
    // You can add more types here in future.
}

// implement display for Value
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
        }
    }
}


impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Vec<Stmt>) -> anyhow::Result<String> {
        let mut last_result = 0;
        let mut last_statement = String::new();
        for stmt in program {
            match stmt {
                Stmt::Expr(expr) => {
                    let result = self.visit_expr(expr)?;
                    match result {
                        Value::Number(n) => last_result = n,
                        Value::Str(s) => last_statement = s,
                    }
                    return Ok(last_result.to_string())
                },
                _ => { 
                    last_statement.push_str(&self.visit_stmt(stmt)?);
                    return Ok(last_statement)
                }
            }
        }
        Ok(last_result.to_string())
    }

    pub fn visit_stmt(&mut self, stmt: Stmt) -> anyhow::Result<String> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let variable = ident.clone();
                let value = self.visit_expr(expr)?;
                self.symbol_table.insert(ident, value.clone());
                match value {
                    Value::Number(n) => Ok(format!("{}: Int = {}", variable, n)),
                    Value::Str(s) => Ok(format!("{}: String = {}", variable, s)),
                }
            },
            Stmt::Expr(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("=> {}", value))
            },
            Stmt::Return(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("=> {}", value))
            },
        }
    }

    pub fn visit_expr(&mut self, expr: Expr) -> anyhow::Result<Value> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::Identifier(ident) => {
                match self.symbol_table.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None => Err(anyhow::anyhow!("Variable {} not found", ident)),
                }
            },
            Expr::Infix(left, op, right) => {
                let left_value = self.visit_expr(*left)?;
                let right_value = self.visit_expr(*right)?;

                match (&left_value, op.as_str(), &right_value) {
                    (Value::Number(left_value), "+", Value::Number(right_value)) => Ok(Value::Number(left_value + right_value)),
                    //"+" => Ok(left_value + right_value),
                    //"-" => Ok(left_value - right_value),
                    //"*" => Ok(left_value * right_value),
                    //"/" => Ok(left_value / right_value),
                    _ => Err(anyhow::anyhow!("Unexpected operator: {}", op)),
                }
            },
            // TODO: fix this return
            Expr::String(s) => Ok(Value::Str(s)),
        }
    }
}

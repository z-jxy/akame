use std::collections::HashMap;

use crate::ast::{ASTNode, BinaryOp, Stmt, Expr};

pub struct Interpreter {
    pub symbol_table: HashMap<String, i32>,
}


impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Vec<Stmt>) -> Result<i32, String> {
        let mut last_result = 0;
        for stmt in program {
            match stmt {
                Stmt::Expr(expr) => last_result = self.visit_expr(expr)?,
                _ => { self.visit_stmt(stmt)?; }
            }
        }
        Ok(last_result)
    }

    pub fn visit_stmt(&mut self, stmt: Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let value = self.visit_expr(expr)?;
                self.symbol_table.insert(ident, value);
                Ok(())
            },
            Stmt::Expr(expr) => {
                let _value = self.visit_expr(expr)?;
                Ok(())
            },
            Stmt::Return(expr) => {
                let _value = self.visit_expr(expr)?;
                Ok(())
            },
        }
    }

    pub fn visit_expr(&mut self, expr: Expr) -> Result<i32, String> {
        match expr {
            Expr::Number(n) => Ok(n),
            Expr::Identifier(ident) => {
                match self.symbol_table.get(&ident) {
                    Some(&value) => Ok(value),
                    None => Err(format!("Variable {} not found", ident)),
                }
            },
            Expr::Infix(left, op, right) => {
                let left_value = self.visit_expr(*left)?;
                let right_value = self.visit_expr(*right)?;

                match op.as_str() {
                    "+" => Ok(left_value + right_value),
                    "-" => Ok(left_value - right_value),
                    "*" => Ok(left_value * right_value),
                    "/" => Ok(left_value / right_value),
                    _ => Err(format!("Unexpected operator: {}", op)),
                }
            }
        }
    }
}

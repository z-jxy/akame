use std::collections::HashMap;

use crate::{ast::{Statement, Expression}, types::integer::Integer};

pub struct Interpreter {
    symbol_table: HashMap<String, Value>,  // assuming all variables are i32 for simplicity
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn visit_expr(&mut self, expr: Expression) -> anyhow::Result<Value> {
        //println!("Environment: {:?}", self.environment);
        match expr {
            Expression::Number(n) => Ok(Value::Number(n)),
            Expression::Identifier(ident) => {
                match self.symbol_table.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None =>{
                        println!("Symbol table: {:?}", self.symbol_table);
                        Err(anyhow::anyhow!("Undefined variable: {}", ident))
                    },
                }
            },
            Expression::Infix(left, op, right) => {
                let left_value = self.visit_expr(*left)?;
                let right_value = self.visit_expr(*right)?;

                match (&left_value, &right_value) {
                    (Value::Number(left), Value::Number(right)) => {
                        let (left, right) = (left.clone(), right.clone());
                        return match op.as_str() {
                            "+" => Ok(Value::Number(left + right)),
                            "-" => Ok(Value::Number(left - right)),
                            "*" => Ok(Value::Number(left * right)),
                            "/" => Ok(Value::Number(left / right)),
                            _ => Err(anyhow::anyhow!("Unexpected operator: {}", op)),
                        };
                    },
                    (Value::Str(left), Value::Str(right)) => {
                        return match op.as_str() {
                            "+" => Ok(Value::Str(format!("{}{}", left, right))),
                            _ => Err(anyhow::anyhow!("Unexpected operator: {}", op)),
                        };
                    },
                    _ => return Err(anyhow::anyhow!("Invalid operation: {} {} {}", left_value, op, right_value)),
                }
            },
            Expression::Char(c) => Ok(Value::Char(c)),
            Expression::String(s) => Ok(Value::Str(s)),
            Expression::Call(name, args) => {
                let function = match self.symbol_table.get(&name) {
                    Some(value) => value,
                    None => return Err(anyhow::anyhow!("Undefined function: {}", name)),
                };
                match function.clone() {
                    Value::Function(params, body) => {
                        if params.len() != args.len() {
                            panic!("Function {} expects {} parameters, but got {}.", name, params.len(), args.len());
                        }
            
                        // Create a new scope
                        let old_env = self.symbol_table.clone();
                        
                        // Bind arguments to parameters
                        for (param, arg) in params.iter().zip(args.iter()) {
                            let arg_value = self.visit_expr(arg.clone())?;
                            self.symbol_table.insert(param.clone(), arg_value);
                        }
            
                        // Execute the body
                        let mut last_value = Value::Number(0.into()); // Default value
                        for stmt in &body {
                            let res = self.visit_stmt(stmt.clone())?;
                            last_value = Value::Str(res);
                        }
            
                        // Restore the old scope
                        self.symbol_table = old_env;
            
                        Ok(last_value) 
                    },
                    _ => panic!("Not a function: {}", name),
                }
            },
        }
    }

    pub fn visit_stmt(&mut self, stmt: Statement) -> anyhow::Result<String> {
        match stmt {
            Statement::Let(ident, expr) => {
                let value = self.visit_expr(expr)?;
                self.symbol_table.insert(ident.clone(), value.clone());
                Ok(format!("{}: {} = {}", ident, value.type_name(), value))
            },
            Statement::Expr(expr) => {
                Ok(format!("{}", self.visit_expr(expr)?))
            },
            Statement::Return(expr) => {
                Ok(format!("{}",  self.visit_expr(expr)?))
            },
            Statement::Function(name, params, body) => {
                let output = format!("fn {name}({params}) {{\n  {body}\n  }}", 
                params=params.join(", "), 
                body=body.iter().map(|stmt| {
                    format!("   {}", stmt)
                }).collect::<Vec<String>>().join("\n"));
                self.symbol_table.insert(name.clone(), Value::Function(params, body));
                Ok(output)
            },
        }
}

}




#[derive(Debug, Clone)]
pub enum Value {
    Number(Integer),
    Str(String),
    Char(char),
    Function(Vec<String>, Vec<Statement>),
    // You can add more types here in future.
}

// implement display for Value
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Function(
                params,
                body,
            ) => write!(f, "fn({}) {{\n{}\n}}", params.join(", "), body.iter().map(|stmt| {
                if let Statement::Expr(expr) = stmt {
                    format!("{}", expr)
                } else {
                    format!("{:?}", stmt)
                }
            }).collect::<Vec<String>>().join("\n")),
        }
    }
}

impl Value {
    pub fn type_name(&self) -> &str {
        match self {
            Value::Number(_) => "number",
            Value::Str(_) => "string",
            Value::Char(_) => "char",
            Value::Function(..) => "function",
        }
    }
}

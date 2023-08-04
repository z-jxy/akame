use std::collections::HashMap;

use crate::ast::{Stmt, Expr};

pub struct Interpreter {
    symbol_table: HashMap<String, Value>,  // assuming all variables are i32 for simplicity
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn visit_expr(&mut self, expr: Expr) -> anyhow::Result<Value> {
        //println!("Environment: {:?}", self.environment);
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::Identifier(ident) => {
                match self.symbol_table.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None =>{
                        println!("Symbol table: {:?}", self.symbol_table);
                        Err(anyhow::anyhow!("Undefined variable: {}", ident))
                    },
                }
            },
            Expr::Infix(left, op, right) => {
                let left_value = self.visit_expr(*left)?;
                let right_value = self.visit_expr(*right)?;
    
                // For numerical operations
                if let (Value::Number(left), Value::Number(right)) = (&left_value, &right_value) {
                    return match op.as_str() {
                        "+" => Ok(Value::Number(left + right)),
                        "-" => Ok(Value::Number(left - right)),
                        "*" => Ok(Value::Number(left * right)),
                        "/" => Ok(Value::Number(left / right)),
                        _ => Err(anyhow::anyhow!("Unexpected operator: {}", op)),
                    };
                }
    
                // For string concatenation
                if let (Value::Str(left), Value::Str(right)) = (&left_value, &right_value) {
                    if op == "+" {
                        return Ok(Value::Str(format!("{}{}", left, right)));
                    }
                }

    
                Err(anyhow::anyhow!("Invalid operation: {} {} {}", left_value, op, right_value))
            },
            Expr::Char(c) => Ok(Value::Char(c)),
            Expr::String(s) => Ok(Value::Str(s)),
            Expr::Call(name, args) => {
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
                        let mut last_value = Value::Number(0); // Default value
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
            //Expr::Call(ident, args) => {
            //    let func = self.symbol_table.get(&ident).ok_or(anyhow::anyhow!("Undefined function: {}", ident))?;
            //    if let Value::Function(params, body) = func {
            //        if params.len() != args.len() {
            //            return Err(anyhow::anyhow!("Function {} expects {} arguments, but {} arguments were given", ident, params.len(), args.len()));
            //        }
            //        let mut new_env = self.symbol_table.clone();
            //        for (param, arg) in params.iter().zip(args.iter()) {
            //            new_env.insert(param.clone(), self.visit_expr(arg.clone())?);
            //        }
            //        let mut interpreter = Interpreter {
            //            symbol_table: new_env,
            //        };
            //        let mut result = Value::Number(0);
            //        for stmt in body {
            //            result = Value::Str(interpreter.visit_stmt(stmt.clone())?);
            //        }
            //        return Ok(result);
            //    }
            //    Err(anyhow::anyhow!("Expected function, found {}", func.type_name()))
            //},
        }
    }

    pub fn visit_stmt(&mut self, stmt: Stmt) -> anyhow::Result<String> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let value = self.visit_expr(expr)?;
                self.symbol_table.insert(ident.clone(), value.clone());
                Ok(format!("{}: {} = {}", ident, value.type_name(), value))
            },
            Stmt::Expr(expr) => {
                Ok(format!("{}", self.visit_expr(expr)?))
            },
            Stmt::Return(expr) => {
                Ok(format!("{}",  self.visit_expr(expr)?))
            },
            Stmt::Function(name, params, body) => {
                self.symbol_table.insert(name.clone(), Value::Function(params, body));
                Ok(format!("Function defined: {}", name))
            },
        }
}

}




#[derive(Debug, Clone)]
pub enum Value {
    Number(i32),
    Str(String),
    Char(char),
    Function(Vec<String>, Vec<Stmt>),
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
                if let Stmt::Expr(expr) = stmt {
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

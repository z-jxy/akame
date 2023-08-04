use std::collections::HashMap;

use crate::{ast::{Stmt, Expr}, tokens::Token};

pub struct Interpreter {
    pub symbol_table: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Interpreter2 {
    environment: HashMap<String, Value>,  // assuming all variables are i32 for simplicity
}

impl Interpreter2 {
    pub fn new() -> Self {
        Self {
            environment: HashMap::new(),
        }
    }

    pub fn visit_expr(&mut self, expr: Expr) -> anyhow::Result<Value> {
        //println!("Environment: {:?}", self.environment);
        match expr {
            Expr::Number(n) => Ok(Value::Number(n)),
            Expr::Identifier(ident) => {
                println!("Looking up variable: {}", ident);
                println!("Environment: {:?}", self.environment);
                match self.environment.get(&ident) {
                    Some(value) => Ok(value.clone()),
                    None => Err(anyhow::anyhow!("Undefined variable: {}", ident)),
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
                if op == "+" {
                    return Ok(Value::Str(format!("{}{}", left_value, right_value)));
                }
    
                Err(anyhow::anyhow!("Invalid operation: {} {} {}", left_value, op, right_value))
            },
            Expr::Char(c) => Ok(Value::Char(c)),
            Expr::String(s) => Ok(Value::Str(s)),
            _ => Err(anyhow::anyhow!("Unsupported expression: {:?}", expr)),
        }
    }

    pub fn visit_stmt(&mut self, stmt: Stmt) -> anyhow::Result<String> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let variable = ident.clone();
                let id = if let Token::Identifier(s) = Token::Identifier(ident.clone()) {
                    println!("idd: {:?}", s);
                    s.clone()
                } else {
                    panic!("Expected identifier, got {:?}", ident);
                };
                println!("id: {:?}", id);
                println!("expr: {:?}", expr);
                let value = self.visit_expr(expr)?;
                println!("value: {:?}", value);
                self.environment.insert(ident, value.clone());
                Ok(format!("{}: {} = {}", variable, value.type_name(), value))
            },
            Stmt::Expr(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("=> {}", value))
            },
            Stmt::Return(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("Return: {}", value))
            },
            Stmt::Function(name, params, body) => {
                // Implement function declaration logic here
                // Here's a placeholder implementation
                self.environment.insert(name.clone(), Value::Function(params.clone(), body.clone()));
                Ok(format!("Function defined: {}", name))
            },
            _ => Err(anyhow::anyhow!("Unsupported statement: {:?}", stmt)),
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
            Value::Function(..) => write!(f, "<function>"),
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


impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn get_stacktrace(&self) -> String {
        self.symbol_table.iter().map(|(k, v)| format!("{}: {} = {}", k, v.type_name(), v)).collect::<Vec<String>>().join("\n")
    }

    pub fn interpret(&mut self, program: Vec<Stmt>) -> anyhow::Result<String> {
        let mut output = String::new();
        for stmt in program {
            output.push_str(&self.visit_stmt(stmt)?);
            output.push('\n');
        }
        Ok(output)
    }

    pub fn visit_function_def(&mut self, name: String, params: Vec<String>, body: Vec<Stmt>) -> anyhow::Result<String> {
        // Store the function in the symbol table
        self.symbol_table.insert(name.clone(), Value::Function(params, body));
        Ok(format!("Defined function {}", name))
    }

    pub fn visit_function_call(&mut self, name: String, args: Vec<Expr>) -> anyhow::Result<String> {
        // Get a clone of the function
        let func = match self.symbol_table.get(&name) {
            Some(Value::Function(params, body)) => Ok((params.clone(), body.clone())),
            Some(_) => Err(anyhow::anyhow!("{} is not a function", name)),
            None => Err(anyhow::anyhow!("Undefined function {}", name)),
        }?;
    
        // Now that we have the function, we can map over the args
        let arg_values: Result<Vec<_>, _> = args.clone().into_iter().map(|arg| self.visit_expr(arg)).collect();
        let arg_values = arg_values?;
    
        let (params, body) = func; // Extract values from tuple
    
        // Check argument count
        if params.len() != args.len() {
            return Err(anyhow::anyhow!("Incorrect argument count for function {}", name));
        }
        // Create a new scope and add the parameters to it
        let mut new_scope = self.symbol_table.clone();
        for (param, arg_value) in params.iter().zip(arg_values.iter()) {
            new_scope.insert(param.clone(), arg_value.clone());
        }
        // Execute the function in the new scope
        let old_symbol_table = std::mem::replace(&mut self.symbol_table, new_scope);
        let result = self.interpret(body);
        self.symbol_table = old_symbol_table;
        result
    }
    

    pub fn visit_stmt(&mut self, stmt: Stmt) -> anyhow::Result<String> {
        match stmt {
            Stmt::Let(ident, expr) => {
                let variable = ident.clone();
                let value = self.visit_expr(expr)?;
                self.symbol_table.insert(ident, value.clone());
               Ok(format!("{}: {} = {}", variable, value.type_name(), value))
            },
            Stmt::Expr(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("=> {}", value))
            },
            Stmt::Return(expr) => {
                let value = self.visit_expr(expr)?;
                Ok(format!("=> {}", value))
            },
            Stmt::Function(name, params, body) => 
            {  
                let value = self.visit_function_def(name, params, body)?;
                Ok(format!("=> {}", value))}
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
                if op == "+" {
                    return Ok(Value::Str(format!("{}{}", left_value, right_value)));
                }
    
                Err(anyhow::anyhow!("Invalid operation: {} {} {}", left_value, op, right_value))
            },
            Expr::Char(c) => Ok(Value::Char(c)),
            Expr::String(s) => Ok(Value::Str(s)),
        }
    }
}

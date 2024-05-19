use std::collections::HashMap;

use crate::{
    llvm::ast::{BinaryOp, Expr, Stmt},
    parsers::parse_program,
    types::integer::Integer,
};

pub struct Interpreter {
    symbol_table: HashMap<String, Value>, // assuming all variables are i32 for simplicity
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
        }
    }

    pub fn eval_source(&mut self, script: &str) {
        match parse_program(&script) {
            Ok(parsed_program) => {
                parsed_program.iter().for_each(|stmt| {
                    match self.visit_stmt(stmt) {
                        // we don't print anything since
                        Ok(_) => (),
                        Err(err) => eprintln!("Interpreter error: {}", err),
                    }
                });
            }
            Err(err) => {
                println!("{}", "=".repeat(80));
                eprintln!("[Parser::parse] error: {}", err);
                println!("{}", "=".repeat(80));
            }
        }
    }

    #[allow(dead_code)]
    pub fn eval_str(&mut self, input: &str) -> anyhow::Result<String> {
        match parse_program(&input) {
            Ok(parsed_program) => {
                let mut result = String::new();
                for stmt in parsed_program {
                    match self.visit_stmt(&stmt) {
                        Ok(value) => result.push_str(&format!("=> {}\n", value)),
                        Err(err) => return Err(anyhow::anyhow!("Interpreter error: {}", err)),
                    }
                }
                Ok(result)
            }
            Err(err) => {
                println!("{}", "=".repeat(80));
                eprintln!("[Parser::parse] error: {}", err);
                println!("{}", "=".repeat(80));
                Err(anyhow::anyhow!("Parser error: {}", err))
            }
        }
    }

    fn visit_expr(&mut self, expr: &Expr) -> anyhow::Result<Value> {
        match expr {
            Expr::Num(n) => Ok(Value::Number(Integer::Int(n.clone()))),
            Expr::Ident(ident) => {
                let value = self.symbol_table.get(ident);
                match value {
                    Some(value) => Ok(value.to_owned()),
                    None => {
                        println!("Symbol table: {:?}", self.symbol_table);
                        Err(anyhow::anyhow!("Undefined variable: {}", ident))
                    }
                }
            }
            Expr::Array(array) => {
                let mut values = Vec::new();
                for expr in array {
                    values.push(self.visit_expr(expr)?);
                }
                Ok(Value::Array(values))
            }
            Expr::ArrayIndexing(array, index) => {
                let array = self.visit_expr(array)?;
                let index = self.visit_expr(index)?;
                match (&array, &index) {
                    (Value::Array(array), Value::Number(Integer::Int(index))) => {
                        let index = *index as usize;
                        if index >= array.len() {
                            return Err(anyhow::anyhow!("Index out of bounds: {}", index));
                        }
                        Ok(array[index].to_owned())
                    }
                    _ => Err(anyhow::anyhow!(
                        "Invalid array indexing: {}[{}]",
                        array,
                        index
                    )),
                }
            }
            Expr::QualifiedIdent(_) => todo!(),

            Expr::Infix(left, op, right) => {
                let left_value;
                let right_value;
                {
                    left_value = self.visit_expr(left)?;
                    right_value = self.visit_expr(right)?;
                    match (&left_value, &right_value) {
                        (Value::Number(left), Value::Number(right)) => {
                            let (left, right) = (left.clone(), right.clone());
                            return match op {
                                BinaryOp::Add => Ok(Value::Number(left + right)),
                                BinaryOp::Subtract => Ok(Value::Number(left - right)),
                                BinaryOp::Multiply => Ok(Value::Number(left * right)),
                                BinaryOp::Divide => Ok(Value::Number(left / right)),
                            };
                        }
                        (Value::Str(left), Value::Str(right)) => {
                            return match op {
                                BinaryOp::Add => {
                                    Ok(Value::Str(format!("{}{}", left, right).into()))
                                }
                                _ => Err(anyhow::anyhow!("Unexpected operator: {}", op)),
                            };
                        }
                        _ => {
                            return Err(anyhow::anyhow!(
                                "Invalid operation: {} {} {}",
                                left_value,
                                op,
                                right_value
                            ))
                        }
                    }
                }
            }

            Expr::Char(c) => Ok(Value::Char(*c)),
            Expr::Str(s) => Ok(Value::Str(s.clone().into())),
            Expr::Call(name, args) => match self.call_function(name.as_str(), args) {
                Ok(value) => Ok(value),
                Err(err) => Err(err),
            },
        }
    }

    fn call_function(&mut self, name: &str, args: &[Expr]) -> anyhow::Result<Value> {
        //println!("calling function: {}", name);
        match name {
            "println!" => {
                //println!("calling println!");
                if let Some(arg) = args.get(0) {
                    let value = self.visit_expr(arg)?;
                    println!("about to print: {:?}", value);
                    match value {
                        Value::Str(s) => {
                            println!("{:?}", s);
                            Ok(Value::None)
                        }
                        Value::Number(n) => {
                            // Handle numbers
                            println!("{}", n);
                            Ok(Value::None)
                        }
                        x => Err(anyhow::anyhow!(
                            "Expected string argument to print. Got: {:?}",
                            x
                        )),
                    }
                } else {
                    Err(anyhow::anyhow!(
                        "print function expects at least one argument"
                    ))
                }
            }
            _ => {
                let symbol = self.symbol_table.get(name);
                match symbol {
                    Some(value) => {
                        match value.clone() {
                            Value::Function(params, body) => {
                                if params.len() != args.len() {
                                    return Err(anyhow::anyhow!(
                                        "Expected {} arguments, got {}",
                                        params.len(),
                                        args.len()
                                    ));
                                }
                                // Create a new scope
                                let old_env = self.symbol_table.clone();

                                let ret = {
                                    // Bind arguments to parameters
                                    params.iter().zip(args.iter().cloned()).for_each(|x| {
                                        let (param, arg) = x;
                                        let arg_value = self.visit_expr(&arg);
                                        match arg_value {
                                            Ok(value) => {
                                                self.symbol_table.insert(param.to_owned(), value);
                                            }
                                            Err(err) => {
                                                println!("Error: {}", err);
                                            }
                                        }
                                    });
                                    // Execute the body
                                    let mut value = Value::Number(0.into()); // Default value
                                    body.iter().cloned().for_each(|stmt| {
                                        value = self.visit_stmt(&stmt).unwrap();
                                    });
                                    // Return the last value
                                    value
                                };
                                //println!("ret: {:?} ({})", ret, ret.type_name());

                                self.symbol_table = old_env;

                                return Ok(Value::Return(Box::new(ret)));
                            }
                            Value::None => Ok(Value::None),
                            _ => todo!("Not a function: {}", name),
                        }
                    }
                    None => Err(anyhow::anyhow!("Undefined function: {}", name)),
                }
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> anyhow::Result<Value> {
        match stmt {
            Stmt::Assignment { ident, expr } => {
                let value = self.visit_expr(&expr)?;
                self.symbol_table.insert(ident.clone(), value.clone());
                Ok(value)
            }
            Stmt::Expression(expr) | Stmt::Return(expr) => Ok(self.visit_expr(&expr)?),
            Stmt::FunctionDeclaration {
                ident,
                params,
                body,
            } => {
                let value: Value = Value::Function(params.to_owned(), body.to_owned());
                self.symbol_table.insert(ident.to_owned(), value.to_owned());
                Ok(Value::None)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(Integer),
    Str(Box<str>),
    Char(char),
    Function(Vec<String>, Vec<Stmt>),
    Return(Box<Value>),
    Array(Vec<Value>),
    None,
    // You can add more types here in future.
}

// implement display for Value
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Str(s) => write!(f, "{}", s),
            Value::Char(c) => write!(f, "{}", c),
            Value::Function(params, body) => write!(
                f,
                "fn({}) {{\n{}\n}}",
                params.join(", "),
                body.iter()
                    .map(|stmt| {
                        if let Stmt::Expression(expr) = stmt {
                            format!("{:?}", expr)
                        } else {
                            format!("{:?}", stmt)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            Value::None => Ok(()),
            Value::Return(boxed_value) => write!(f, "{}", boxed_value),
            Value::Array(values) => {
                write!(f, "[")?;
                for (i, value) in values.iter().enumerate() {
                    if i != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
        }
    }
}

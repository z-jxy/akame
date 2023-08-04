use crate::{tokens::Token, lexer::Lexer, ast::{Expr, Stmt}};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
    next_token: Option<Token>,
}


impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = Some(lexer.get_next_token());
        let next_token = Some(lexer.peek_next_token());
        Self { lexer, current_token, next_token }
    }



    pub fn parse(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while let Some(token) = &self.current_token {
            if matches!(token, Token::EOF) {
                break;
            }
            let statement = match token {
                Token::Let => self.parse_let_statement(),
                Token::Fn => self.parse_function_statement(),
                _ => self.parse_expression_statement(),
            };
            match statement {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    return Err(anyhow::anyhow!("Failed to parse statement: {}", err));
                },
            }
        }
        Ok(statements)
    }

    pub fn dbg_parse(&mut self) -> anyhow::Result<Vec<Stmt>> {
        let mut statements = Vec::new();
        while let Some(token) = &self.current_token {
            if matches!(token, Token::EOF) {
                break;
            }
            let statement = match token {
                Token::Let => self.dbg_parse_let_statement(),
                _ => self.parse_expression_statement(),
            };
            match statement {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    return Err(anyhow::anyhow!("Failed to parse statement: {}", err));
                },
            }
        }
        Ok(statements)
    }

    fn eat2(&mut self, token: Token) -> Result<Token, String> {
        if Some(&token) == self.current_token.as_ref() {
            //println!("Matched token: {:?}", token);
            self.current_token = Some(self.lexer.get_next_token());
            Ok(token)
        } else {
            println!("Failed to match: {:?}, found {:?}", token, self.current_token);
            Err(format!("Unexpected token: {:?}", self.current_token))
        }
    }

    fn dbg_eat(&mut self, token: Token) -> Result<Token, String> {
        if Some(&token) == self.current_token.as_ref() {
            println!("Matched token: {:?}", token);
            self.current_token = Some(self.lexer.get_next_token());
            Ok(token)
        } else {
            println!("Failed to match: {:?}, found {:?}", token, self.current_token);
            Err(format!("Unexpected token: {:?}", self.current_token))
        }
    }

    //fn factor(&mut self) -> Result<ASTNode, String> {
    //    let result = match &self.current_token {
    //        Some(Token::Number(n)) => {
    //            let node = ASTNode::NumberNode { value: *n };
    //            self.current_token = Some(self.lexer.get_next_token());
    //            Ok(node)
    //        },
    //        Some(Token::LeftParen) => {
    //            self.eat(Token::LeftParen)?;
    //            let node = self.expr()?;
    //            self.eat(Token::RightParen)?;
    //            Ok(node)
    //        },
    //        Some(Token::String(s)) => {
    //            let node = ASTNode::StringNode { value: s.clone() };
    //            self.current_token = Some(self.lexer.get_next_token());
    //            Ok(node)
    //        },
    //        _ => Err(format!("Unexpected token: {:?}", self.current_token)),
    //    };
    //
    //    result
    //}
    //
    //fn expr(&mut self) -> Result<ASTNode, String> {
    //    let mut node = self.factor()?;
    //
    //    while let Some(token) = self.current_token.clone() {
    //        match token {
    //            Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
    //                let op = match token {
    //                    Token::Plus => BinaryOp::Add,
    //                    Token::Minus => BinaryOp::Subtract,
    //                    Token::Multiply => BinaryOp::Multiply,
    //                    Token::Divide => BinaryOp::Divide,
    //                    _ => unreachable!(), // We already checked the token type above
    //                };
    //                self.eat(token)?;
    //                let right = self.factor()?;
    //                node = ASTNode::BinaryOpNode {
    //                    op,
    //                    left: Box::new(node),
    //                    right: Box::new(right),
    //                };
    //            },
    //            _ => break,
    //        }
    //    }
    //
    //    Ok(node)
    //}
    


    fn parse_expression(&mut self) -> Result<Expr, String> {
        let left = match self.current_token.clone() {
            Some(Token::Number(n)) => {
                self.eat(Token::Number(n))?;
                Expr::Number(n)
            },
            Some(Token::String(s)) => {
                self.eat(Token::String(s.clone()))?;
                Expr::String(s)
            },
            Some(Token::Identifier(s)) => {
                self.eat(Token::Identifier(s.clone()))?;
                Expr::Identifier(s)
            },
            Some(Token::LeftParen) => {
                self.eat(Token::LeftParen)?;
                let inner_expr = self.parse_expression()?;
                self.eat(Token::RightParen)?;
                inner_expr
            },
            Some(Token::Char(value)) => {
                self.eat(Token::Char(value))?;
                Expr::Char(value)
            },
            
            _ => return Err(format!("Unexpected expr: {:?}", self.current_token)),
        };
    
        if let Some(token) = self.current_token.clone() {
            match token {
                Token::Plus | Token::Minus | Token::Multiply | Token::Divide => {
                    let op = match token {
                        Token::Plus => "+",
                        Token::Minus => "-",
                        Token::Multiply => "*",
                        Token::Divide => "/",
                        _ => unreachable!(), // We already checked the token type above
                    };
                    self.eat(token)?;
                    let right = self.parse_expression()?;
                    return Ok(Expr::Infix(Box::new(left), op.to_string(), Box::new(right)));
                },
                _ => (),
            }
        }
    
        Ok(left)
    }

    

        // Parse a comma-separated list of identifiers.


    //pub fn eat(&mut self, token: Token) -> Result<(), String> {
    //    if self.current_token == Some(token.clone()) {
    //        self.current_token = self.next_token.clone();
    //        self.next_token = Some(self.lexer.get_next_token());
    //        Ok(())
    //    } else {
    //        Err(format!("Expected {:?}, found {:?}", token, self.current_token))
    //    }
    //}

    fn eat(&mut self, token: Token) -> Result<Token, String> {
        if Some(&token) == self.current_token.as_ref() {
            //println!("Matched token: {:?}", token);
            self.current_token = Some(self.lexer.get_next_token());
            Ok(token)
        } else {
            println!("Failed to match: {:?}, found {:?}", token, self.current_token);
            Err(format!("Unexpected token: {:?}", self.current_token))
        }
    }

    // Parse a block of statements enclosed in braces.
    fn parse_block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = vec![];
        while let Some(token) = &self.next_token {
            if let Token::RightBrace = token {
                break;
            }
            stmts.push(self.parse_statement()?);
        }
        self.eat(Token::RightBrace)?; // Consume the `RightBrace`
        Ok(stmts)
    }


    
    fn parse_function_statement(&mut self) -> Result<Stmt, String> {
        self.eat(Token::Fn)?;
        let id = if let Some(Token::Identifier(s)) = &self.current_token {
            s.clone()
        } else {
            return Err(format!("Expected identifier, found {:?}", self.current_token));
        };
        self.eat(Token::Identifier(id.clone()))?;
        self.eat(Token::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.eat(Token::RightParen)?;
        self.eat(Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.eat(Token::RightBrace)?;
        Ok(Stmt::Function(id, params, body))
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<String>, String> {
        let mut params = vec![];
        while let Some(Token::Identifier(param)) = &self.current_token {
            params.push(param.to_string());
            match &self.current_token {
                Some(Token::Comma) => {
                    self.eat(Token::Comma)?;
                }
                Some(Token::RightParen) => {
                    break;
                }
                _ => {
                    return Err(format!("Expected ',' or ')', found {:?}", self.current_token));
                }
            }
        }
        Ok(params)
    }
    
    


    fn dbg_parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.dbg_eat(Token::Let)?;
        let id = if let Some(Token::Identifier(s)) = &self.current_token {
            s.clone()
        } else {
            return Err(format!("Expected identifier, found {:?}", self.current_token));
        };
        self.dbg_eat(Token::Identifier(id.clone()))?;
        self.dbg_eat(Token::Equal)?;
        let expr = self.parse_expression()?;
        self.dbg_eat(Token::Semicolon)?;
        Ok(Stmt::Let(id, expr))
    }

    //fn parse_let_statement_x(&mut self, consumer: fn(token: Token) -> Result<Token, String>) -> Result<Stmt, String> {
    //    consumer(Token::Let)?;
    //    let id = if let Some(Token::Identifier(s)) = &self.current_token {
    //        s.clone()
    //    } else {
    //        return Err(format!("Expected identifier, found {:?}", self.current_token));
    //    };
    //    consumer(Token::Identifier(id.clone()))?;
    //    consumer(Token::Equal)?;
    //    let expr = self.parse_expression()?;
    //    consumer(Token::Semicolon)?;
    //    Ok(Stmt::Let(id, expr))
    //}
    

    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        self.eat(Token::Let)?;
        let id = if let Some(Token::Identifier(s)) = &self.current_token {
            s.clone()
        } else {
            return Err(format!("Expected identifier, found {:?}", self.current_token));
        };
        self.eat(Token::Identifier(id.clone()))?;
        self.eat(Token::Equal)?;
        let expr = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Stmt::Let(id, expr))
    }

    fn parse_expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn parse_return_statement(&mut self) -> Result<Stmt, String> {
        self.eat(Token::Return)?;
        let expr = self.parse_expression()?;
        self.eat(Token::Semicolon)?;
        Ok(Stmt::Return(expr))
    }

    fn parse_statement(&mut self) -> Result<Stmt, String> {
        match &self.current_token {
            Some(Token::Let) => self.parse_let_statement(),
            Some(Token::Fn) => self.parse_function_statement(),
            Some(Token::Return) => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }
    

    
    
    
}

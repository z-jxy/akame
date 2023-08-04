use crate::{tokens::Token, lexer::Lexer, ast::{ASTNode, BinaryOp}};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current_token = lexer.get_next_token();
        Self { lexer, current_token }
    }

    fn eat(&mut self, token: Token) -> Result<(), String> {
        if Some(token) == self.current_token {
            self.current_token = self.lexer.get_next_token();
            Ok(())
        } else {
            Err(format!("Unexpected token: {:?}", self.current_token))
        }
    }

    fn factor(&mut self) -> Result<ASTNode, String> {
        let result = match &self.current_token {
            Some(Token::Number(n)) => {
                let node = ASTNode::NumberNode { value: *n };
                self.current_token = self.lexer.get_next_token();
                Ok(node)
            },
            Some(Token::LeftParen) => {
                self.eat(Token::LeftParen)?;
                let node = self.expr()?;
                self.eat(Token::RightParen)?;
                Ok(node)
            },
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        };
    
        result
    }
    
    fn expr(&mut self) -> Result<ASTNode, String> {
        match &self.current_token {
            Some(Token::Number(n)) => {
                let node = ASTNode::NumberNode { value: *n };
                self.current_token = self.lexer.get_next_token();
                Ok(node)
            },
            Some(Token::LeftParen) => {
                self.eat(Token::LeftParen)?;
                
                let op = match self.current_token.take() {
                    Some(Token::Plus) => BinaryOp::Add,
                    Some(Token::Minus) => BinaryOp::Subtract,
                    Some(Token::Multiply) => BinaryOp::Multiply,
                    Some(Token::Divide) => BinaryOp::Divide,
                    _ => return Err(format!("Unexpected token: {:?}", self.current_token)),
                };
                self.current_token = self.lexer.get_next_token();
    
                let mut node = self.expr()?;
    
                while !matches!(self.current_token, Some(Token::RightParen)) {
                    node = ASTNode::BinaryOpNode {
                        op: op.clone(),
                        left: Box::new(node),
                        right: Box::new(self.expr()?),
                    };
                }
    
                self.eat(Token::RightParen)?;
                Ok(node)
            },
            _ => Err(format!("Unexpected token: {:?}", self.current_token)),
        }
    }
    
    
    

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        self.expr()
    }
}

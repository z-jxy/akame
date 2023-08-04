use crate::tokens::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.chars().nth(self.pos) {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }
    

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }
    
        let c = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;
    
        match c {
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            ';' => Token::Semicolon,
            '=' => Token::Equal,
            ',' => Token::Comma,
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut id = c.to_string();
                while let Some(c) = self.input.chars().nth(self.pos) {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            id.push(c);
                            self.pos += 1;
                        }
                        _ => break,
                    }
                }
    
                match id.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    _ => Token::Identifier(id),
                }
            }
            '0'..='9' => {
                let mut num_str = c.to_string();
                while let Some(c) = self.input.chars().nth(self.pos) {
                    match c {
                        '0'..='9' => {
                            num_str.push(c);
                            self.pos += 1;
                        }
                        _ => break,
                    }
                }
                Token::Number(num_str.parse().unwrap())
            }
            _ => panic!("Unexpected character: {}", c),
        }
    }

    fn identifier(&mut self) -> Token {
        let start = self.pos;
        while let Some(ch) = self.input.chars().nth(self.pos) {
            if ch.is_alphanumeric() || ch == '_' {
                self.pos += 1;
            } else {
                break;
            }
        }
    
        let value: String = self.input.chars().skip(start).take(self.pos - start).collect();
        match value.as_str() {
            "let" => Token::Let,
            "return" => Token::Return,
            _ => Token::Identifier(value),
        }
    }
}

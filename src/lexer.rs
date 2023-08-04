use crate::tokens::Token;

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    pub fn get_next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.input.chars().nth(self.pos) {
            self.pos += 1;
    
            return match c {
                '(' => Some(Token::LeftParen),
                ')' => Some(Token::RightParen),
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '*' => Some(Token::Multiply),
                '/' => Some(Token::Divide),
                '0'..='9' => {
                    let mut num_str = c.to_string();
                    while let Some('0'..='9') = self.input.chars().nth(self.pos) {
                        num_str.push(self.input.chars().nth(self.pos).unwrap());
                        self.pos += 1;
                    }
                    let num: i32 = num_str.parse().unwrap();
                    Some(Token::Number(num))
                }
                ' ' | '\t' | '\n' | '\r' => continue,
                _ => None,
            };
        }
    
        None
    }
    
}

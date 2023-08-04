use crate::tokens::Token;
use anyhow::anyhow;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    current_char: Option<char>,
}

pub enum LexerError {
    UnexpectedCharacter(char),
    UnexpectedEndOfInput,
    IllegalToken,
}

impl<'a> Lexer<'a> {
    pub fn peek_next_token(&mut self) -> Token {
        let mut temp_lexer = self.clone();
        temp_lexer.get_next_token()
    }

    pub fn new(input: &'a str) -> Self {
        let current_char = input.chars().next();
        Self { input, pos: 0, current_char }
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

    fn advance(&mut self) {
        self.current_char = self.input.chars().nth(self.pos);
        self.pos += 1;
    }

    fn string(&mut self) -> anyhow::Result<Token> {
        let mut string = String::new();
        while let Some(c) = self.input.chars().nth(self.pos) {
            if c == '"' {
                // End of the string, increment pos to skip the ending quote and break
                self.pos += 1;
                return Ok(Token::String(string));
            } else {
                // Add character to the string and increment pos
                string.push(c);
                self.pos += 1;
            }
        }
        Err(anyhow!("Unexpected end of input while parsing string"))
    }
    

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return Token::EOF;
        }

        self.advance();
    
        let c = self.current_char.unwrap();
    
        match self.current_char.unwrap() {
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
            '"' => {
                match self.string() {
                    Ok(token) => token,
                    Err(e) => {
                        println!("{}", e);
                        Token::ILLEGAL
                    }
                }
            },
            // support for chars
            '\'' => match self.character() {
                Ok(token) => token,
                Err(e) => {
                    println!("{}", e);
                    Token::ILLEGAL
                }
            }
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

    fn character(&mut self) -> Result<Token, anyhow::Error> {
        self.advance(); // Skip the starting quote

        // Get the character
        let mut c = self.current_char.ok_or(anyhow::anyhow!("Unexpected end of input"))?;
        if c == '\'' {
            return Err(anyhow::anyhow!("Character literal must contain a character. To represent a single quote, escape it using a backslash: \\'"));
        }

        if c == '\\' {
                self.advance(); // Skip the backslash
                let escape_char = self.current_char.ok_or(anyhow::anyhow!("Unexpected end of input"))?;
                let final_char = match escape_char {
                    '\'' => '\'',
                    '\\' => '\\',
                    _ => return Err(anyhow::anyhow!("Invalid escape sequence")),
                };
                c = final_char;
        }
        // Check for end quote
        self.advance(); // Move to the end quote
        if self.current_char != Some('\'') {
            println!("Unexpected value at end of character literal. '{}'", self.current_char.unwrap());
            return Err(anyhow::anyhow!("Character literal must be closed"));
        }

        Ok(Token::Char(c))
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
            "fn" => Token::Fn,
            _ => Token::Identifier(value),
        }
    }


}


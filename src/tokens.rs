#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Fn,
    Return,
    Identifier(String),
    String(String),
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Semicolon,
    Equal,
    EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftParen,
    RightParen,
    Plus,
    Minus,
    Multiply,
    Divide,
    Number(i32),
}

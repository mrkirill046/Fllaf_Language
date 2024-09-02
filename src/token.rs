#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Number(i64),
    StringLiteral(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    Semicolon,
    Dot,
    Comment(String),
    BlockComment(String),
    System,
    Log,
    EndOfFile,
}

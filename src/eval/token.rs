#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Asterisk,
    Eof,
    Illegal,
    Int,
    LParen,
    Minus,
    Plus,
    RParen,
    Slash,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenType,
    pub spelling: String,
}

impl Token {
    pub fn new(kind: TokenType, spelling: String) -> Self {
        Token { kind, spelling }
    }
}

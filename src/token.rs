use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    // single char tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // one or two char tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // literals
    String,
    Number,
    Identifier,

    // keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: TokenLiteral,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}, {}, {})",
            self.token_type, self.lexeme, self.literal
        )
    }
}

#[derive(Debug, Clone)]
pub enum TokenLiteral {
    String(String),
    Number(f64),
    Boolean(bool),
    Empty,
}

impl fmt::Display for TokenLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenLiteral::String(value) => write!(f, "{}", value),
            TokenLiteral::Number(value) => write!(f, "{}", value),
            TokenLiteral::Boolean(value) => write!(f, "{}", value),
            TokenLiteral::Empty => write!(f, ""),
        }
    }
}

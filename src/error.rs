use std::{
    error::Error,
    fmt::{self},
};

use crate::token::{Token, TokenType};

pub type Result<T> = std::result::Result<T, LoxiteError>;

#[derive(Debug)]
pub struct ParserError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug)]
pub struct LexerError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum LoxiteError {
    Lexer(LexerError),
    Parser(ParserError),
}

impl fmt::Display for LoxiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxiteError::Lexer(err) => write!(
                f,
                "lexer error: [line {}] Error{}: {}",
                err.line, "", err.message
            ),
            LoxiteError::Parser(err) => {
                let location = match err.token.token_type {
                    TokenType::EOF => " at end".to_string(),
                    _ => format!(" at '{}'", err.token.lexeme),
                };

                write!(
                    f,
                    "parser error: [line {}] Error{}: {}",
                    err.token.line, location, err.message
                )
            }
        }
    }
}

impl LoxiteError {
    pub fn print(&self) {
        eprintln!("{}", self);
    }
}

impl Error for LoxiteError {
    fn description(&self) -> &str {
        match self {
            LoxiteError::Lexer(_) => "lexer error",
            LoxiteError::Parser(_) => "parser error",
        }
    }
}

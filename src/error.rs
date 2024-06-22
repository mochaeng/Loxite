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
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn number_operands(token: &Token) -> Self {
        Self {
            token: token.clone(),
            message: String::from("Operands must be numbers."),
        }
    }

    pub fn number_or_string_operands(token: &Token) -> Self {
        Self {
            token: token.clone(),
            message: String::from("Operands must be two integers or two strings."),
        }
    }

    pub fn error(token: &Token, msg: &str) -> Self {
        Self {
            token: token.clone(),
            message: msg.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum LoxiteError {
    Lexer(LexerError),
    Parser(ParserError),
    Runtime(RuntimeError),
}

impl fmt::Display for LoxiteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxiteError::Lexer(err) => write!(
                f,
                "Lexer error: [line {}] Error{}: {}",
                err.line, "", err.message
            ),
            LoxiteError::Parser(err) => {
                let location = match err.token.token_type {
                    TokenType::EOF => " at end".to_string(),
                    _ => format!(" at '{}'", err.token.lexeme),
                };

                write!(
                    f,
                    "Parser error: [line {}] Error{}: {}",
                    err.token.line, location, err.message
                )
            }
            LoxiteError::Runtime(err) => write!(
                f,
                "Runtime Error: [line {}]: {}",
                err.token.line, err.message
            ),
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
            LoxiteError::Lexer(_) => "an error during lexical phase",
            LoxiteError::Parser(_) => "an error during parser phase",
            LoxiteError::Runtime(_) => "an error during runtime",
        }
    }
}

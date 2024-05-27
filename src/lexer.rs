use std::str::Chars;

use crate::error;
use crate::token::Token;
use crate::token::TokenType;
use crate::token::TokenType::*;

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    pub fn new(source: &String) -> Self {
        Lexer {
            source: source.chars().collect(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.tokenize();
        }
        self.tokens.push(Token {
            type_of: EOF,
            lexeme: "".to_string(),
            line: self.line,
        });
        &self.tokens
    }

    fn tokenize(&mut self) {
        let mut had_error = false;

        match self.advance() {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightBrace),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => match self.match_next_char('=') {
                true => self.add_token(BangEqual),
                false => self.add_token(Bang),
            },
            '=' => match self.match_next_char('=') {
                true => self.add_token(EqualEqual),
                false => self.add_token(Equal),
            },
            '<' => match self.match_next_char('=') {
                true => self.add_token(LessEqual),
                false => self.add_token(Less),
            },
            '>' => match self.match_next_char('=') {
                true => self.add_token(GreaterEqual),
                false => self.add_token(Greater),
            },
            '/' => match self.match_next_char('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(Slash),
            },
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => {
                error::error(self.line, "Unexpected character.");
                had_error = true;
            }
        }
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, type_of: TokenType) {
        let text = self.source[self.start..self.current]
            .iter()
            .collect::<String>();

        self.tokens.push(Token {
            type_of,
            lexeme: text,
            line: self.line,
        });
    }

    fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

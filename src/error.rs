pub mod parser {
    use crate::token::{Token, TokenType};
    use std::{
        error::Error,
        fmt::{self},
    };

    #[derive(Debug)]
    pub struct ParserError {
        pub token: Token,
        pub message: String,
    }

    impl fmt::Display for ParserError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let location = match self.token.token_type {
                TokenType::EOF => " at end".to_string(),
                _ => format!(" at '{}'", self.token.lexeme),
            };

            write!(
                f,
                "[line {}] Error{}: {}",
                self.token.line, location, self.message
            )
        }
    }

    impl Error for ParserError {}

    pub type Result<T> = std::result::Result<T, ParserError>;

    impl ParserError {
        pub fn print(&self) {
            eprintln!("{}", self);
        }
    }
}

fn report(line: usize, where_in: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, where_in, message);
}
pub fn lexer_error(line: usize, message: &str) {
    report(line, "", message)
}

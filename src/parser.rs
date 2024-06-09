use crate::{
    error::{LoxiteError, ParserError},
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{Token, TokenLiteral, TokenType},
};

// --------------------- GRAMMAR -------------------------------
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
// --------------------------------------------------------------

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parser(&mut self) -> Option<Expr> {
        match self.expression() {
            Ok(expr) => Some(expr),
            Err(err) => {
                err.print();
                None
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, LoxiteError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxiteError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxiteError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxiteError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxiteError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(BinaryExpr {
                left: expr,
                operator,
                right,
            }))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxiteError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary(Box::new(UnaryExpr { operator, right })));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxiteError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: TokenLiteral::Boolean(false),
            }));
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: TokenLiteral::Boolean(true),
            }));
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: TokenLiteral::Empty,
            }));
        }

        if self.match_token(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal.clone(),
            }));
        }

        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(GroupingExpr { expression: expr })));
        }

        // let err = LoxiteError::Parser(ParserError {
        //     token: self.peek().clone(),
        //     message: "Expected expression.".to_string(),
        // });
        // err.print();
        // return Err(err);

        return Err(LoxiteError::Parser(ParserError {
            token: self.peek().clone(),
            message: "Expected expression.".to_string(),
        }));
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, LoxiteError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        // let err = LoxiteError::Parser(ParserError {
        //     token: self.peek().clone(),
        //     message: message.to_string(),
        // });
        // err.print();
        // Err(err)
        Err(LoxiteError::Parser(ParserError {
            token: self.peek().clone(),
            message: message.to_string(),
        }))
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}

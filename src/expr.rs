use std::fmt;

use crate::token::Token;

pub enum Expr {
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Grouping(Box<GroupingExpr>),
    Literal(LiteralExpr),
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Expr,
}

pub struct BinaryExpr {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

pub struct GroupingExpr {
    pub expression: Expr,
}

pub struct LiteralExpr {
    pub value: LiteralToken,
}

#[derive(Debug)]
pub enum LiteralToken {
    String(String),
    Number(f64),
    Empty,
}

impl fmt::Display for LiteralToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralToken::String(value) => write!(f, "{}", value),
            LiteralToken::Number(value) => write!(f, "{}", value),
            LiteralToken::Empty => write!(f, ""),
        }
    }
}

use crate::token::{Token, TokenLiteral};

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
    pub value: TokenLiteral,
}

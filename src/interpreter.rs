use crate::{
    expr::{Expr, GroupingExpr, LiteralExpr},
    token::TokenLiteral,
};

pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}
pub struct Interpreter {}

impl Interpreter {
    fn evaluate(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(literal_expr) => match literal_expr {
                LiteralExpr { value } => match value {
                    TokenLiteral::Number(value) => Value::Number(*value),
                    TokenLiteral::String(value) => Value::String(value.clone()),
                    TokenLiteral::Boolean(value) => Value::Boolean(*value),
                    TokenLiteral::Empty => Value::Nil,
                },
            },
            Expr::Grouping(grouping_expr) => match grouping_expr.as_ref() {
                GroupingExpr { expression } => self.evaluate(expression),
            },
            _ => todo!(),
        }
    }
}

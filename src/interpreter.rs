use core::{num, prelude::v1};
use std::mem;

use crate::{
    expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
    token::{TokenLiteral, TokenType},
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
            Expr::Literal(literal_expr) => match &literal_expr.value {
                TokenLiteral::Number(number) => Value::Number(*number),
                TokenLiteral::String(string) => Value::String(string.clone()),
                TokenLiteral::Boolean(boolean) => Value::Boolean(*boolean),
                TokenLiteral::Empty => Value::Nil,
            },
            Expr::Grouping(grouping_expr) => self.evaluate(&grouping_expr.expression),
            Expr::Unary(unary_expr) => {
                let right = self.evaluate(&unary_expr.right);

                match right {
                    Value::Number(value) => Value::Number(-value),
                    value => Value::Boolean(self.is_truthy(&value)),
                }
            }
            Expr::Binary(binary_expr) => self.evaluate_binary(binary_expr),
        }
    }

    fn evaluate_binary(&self, binary_expr: &BinaryExpr) -> Value {
        let left = self.evaluate(&binary_expr.left);
        let right = self.evaluate(&binary_expr.right);

        match binary_expr.operator.token_type {
            TokenType::Minus => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Number(val_1 - val_2)
                }
                _ => todo!(),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Number(val_1 / val_2)
                }
                _ => todo!(),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Number(val_1 / val_2)
                }
                _ => todo!(),
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Number(val_1 + val_2)
                }
                (Value::String(str_1), Value::String(str_2)) => {
                    return Value::String(str_1 + &str_2)
                }
                _ => todo!(),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Boolean(val_1 > val_2)
                }
                _ => todo!(),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Boolean(val_1 >= val_2)
                }
                _ => todo!(),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Boolean(val_1 < val_2)
                }
                _ => todo!(),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => {
                    return Value::Boolean(val_1 <= val_2)
                }
                _ => todo!(),
            },
            TokenType::BangEqual => return Value::Boolean(self.is_equal(&left, &right)),
            _ => todo!(),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Boolean(val) => *val,
            Value::Nil => false,
            _ => true,
        }
    }

    fn is_equal(&self, value_1: &Value, value_2: &Value) -> bool {
        match (value_1, value_2) {
            (Value::Number(num_1), Value::Number(num_2)) => return num_1 == num_2,
            (Value::String(str_1), Value::String(str_2)) => return str_1 == str_2,
            (Value::Boolean(bool_1), Value::Boolean(bool_2)) => return bool_1 == bool_2,
            (Value::Nil, Value::Nil) => return true,
            _ => return false,
        }
    }
}

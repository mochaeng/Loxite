use std::fmt::{self};

use crate::{
    error::{LoxiteError, RuntimeError},
    expr::{BinaryExpr, Expr},
    token::{TokenLiteral, TokenType},
};

pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(val) => write!(f, "{}", val),
            Value::Number(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn interpreter(&self, expr: &Expr) {
        let evaluation = self.evaluate(expr);
        match evaluation {
            Ok(value) => println!("{}", value),
            Err(err) => println!("{}", err),
        }
    }

    fn evaluate(&self, expr: &Expr) -> Result<Value, LoxiteError> {
        match expr {
            Expr::Literal(literal_expr) => match &literal_expr.value {
                TokenLiteral::Number(number) => Ok(Value::Number(*number)),
                TokenLiteral::String(string) => Ok(Value::String(string.clone())),
                TokenLiteral::Boolean(boolean) => Ok(Value::Boolean(*boolean)),
                TokenLiteral::Empty => Ok(Value::Nil),
            },
            Expr::Grouping(grouping_expr) => self.evaluate(&grouping_expr.expression),
            Expr::Unary(unary_expr) => {
                let right = self.evaluate(&unary_expr.right)?;
                match right {
                    Value::Number(value) => Ok(Value::Number(-value)),
                    value => Ok(Value::Boolean(!self.is_truthy(&value))),
                }
            }
            Expr::Binary(binary_expr) => self.evaluate_binary(binary_expr),
        }
    }

    fn evaluate_binary(&self, binary_expr: &BinaryExpr) -> Result<Value, LoxiteError> {
        let left = self.evaluate(&binary_expr.left)?;
        let right = self.evaluate(&binary_expr.right)?;
        let token = &binary_expr.operator;

        match token.token_type {
            TokenType::Minus => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Number(val_1 - val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Number(val_1 / val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Number(val_1 * val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Number(val_1 + val_2)),
                (Value::String(str_1), Value::String(str_2)) => Ok(Value::String(str_1 + &str_2)),
                _ => Err(LoxiteError::Runtime(
                    RuntimeError::number_or_string_operands(token),
                )),
            },
            TokenType::Greater => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Boolean(val_1 > val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Boolean(val_1 >= val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Boolean(val_1 < val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(val_1), Value::Number(val_2)) => Ok(Value::Boolean(val_1 <= val_2)),
                _ => Err(LoxiteError::Runtime(RuntimeError::number_operands(token))),
            },
            TokenType::BangEqual => Ok(Value::Boolean(!self.is_equal(&left, &right))),
            TokenType::EqualEqual => Ok(Value::Boolean(self.is_equal(&left, &right))),
            _ => Err(LoxiteError::Runtime(RuntimeError::error(
                token,
                "I don't thing this will ever be called",
            ))),
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

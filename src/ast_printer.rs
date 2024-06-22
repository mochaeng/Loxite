use crate::{expr::Expr, token::TokenLiteral};

pub struct AstPrinter;

impl AstPrinter {
    pub fn get_expr_as_str(&self, expr: &Expr) -> String {
        expr.print_expr()
    }
}

impl Expr {
    fn print_expr(&self) -> String {
        match self {
            Expr::Binary(expr) => {
                parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
            }
            Expr::Grouping(expr) => parenthesize("group", vec![&expr.expression]),
            Expr::Unary(expr) => parenthesize(&expr.operator.lexeme, vec![&expr.right]),
            Expr::Literal(expr) => match &expr.value {
                TokenLiteral::Empty => "nil".to_string(),
                _ => expr.value.to_string(),
            },
        }
    }
}

fn parenthesize(name: &str, exprs: Vec<&Expr>) -> String {
    let mut result = String::new();
    result.push_str("(");
    result.push_str(name);

    exprs.iter().for_each(|expr| {
        result.push_str(" ");
        result.push_str(expr.print_expr().as_str())
    });

    result.push_str(")");
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        ast_printer::AstPrinter,
        expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, UnaryExpr},
        token::{Token, TokenLiteral, TokenType},
    };

    #[test]
    fn test_small_expressions() {
        let expr1 = Expr::Unary(Box::new(UnaryExpr {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: TokenLiteral::Empty,
                line: 1,
            },
            right: Expr::Literal(LiteralExpr {
                value: TokenLiteral::Number(123 as f64),
            }),
        }));

        let expr2 = Expr::Grouping(Box::new(GroupingExpr {
            expression: Expr::Literal(LiteralExpr {
                value: TokenLiteral::Number(45.67),
            }),
        }));

        let ast_printer = AstPrinter;
        assert_eq!(ast_printer.get_expr_as_str(&expr1), "(- 123)");
        assert_eq!(ast_printer.get_expr_as_str(&expr2), "(group 45.67)");
    }

    #[test]
    fn test_complete_expr() {
        let expr = Expr::Binary(Box::new(BinaryExpr {
            left: Expr::Unary(Box::new(UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: TokenLiteral::Empty,
                    line: 1,
                },
                right: Expr::Literal(LiteralExpr {
                    value: TokenLiteral::Number(123 as f64),
                }),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: TokenLiteral::Empty,
                line: 1,
            },
            right: Expr::Grouping(Box::new(GroupingExpr {
                expression: Expr::Literal(LiteralExpr {
                    value: TokenLiteral::Number(45.67),
                }),
            })),
        }));

        let ast_printer = AstPrinter;
        assert_eq!(
            ast_printer.get_expr_as_str(&expr),
            "(* (- 123) (group 45.67))"
        );
    }
}

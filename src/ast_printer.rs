use crate::expr::{Expr, LiteralToken};
pub struct AstPrinter;

impl AstPrinter {
    fn print_expr(&self, expr: Expr) -> String {
        expr.visit_expr()
    }
}

impl Expr {
    fn visit_expr(&self) -> String {
        match self {
            Expr::Binary(expr) => {
                parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
            }
            Expr::Grouping(expr) => parenthesize("group", vec![&expr.expression]),
            Expr::Unary(expr) => parenthesize(&expr.operator.lexeme, vec![&expr.right]),
            Expr::Literal(expr) => match &expr.value {
                LiteralToken::Empty => "nil".to_string(),
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
        result.push_str(expr.visit_expr().as_str())
    });

    result.push_str(")");
    result
}

#[cfg(test)]
mod tests {
    use crate::{
        ast_printer::{self, AstPrinter},
        expr::{BinaryExpr, Expr, GroupingExpr, LiteralExpr, LiteralToken, UnaryExpr},
        token::{Token, TokenType},
    };

    #[test]
    fn test_small_expressions() {
        let expr1 = Expr::Unary(Box::new(UnaryExpr {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: LiteralToken::Empty,
                line: 1,
            },
            right: Expr::Literal(LiteralExpr {
                value: LiteralToken::Number(123 as f64),
            }),
        }));

        let expr2 = Expr::Grouping(Box::new(GroupingExpr {
            expression: Expr::Literal(LiteralExpr {
                value: LiteralToken::Number(45.67),
            }),
        }));

        let ast_printer = AstPrinter;
        assert_eq!(ast_printer.print_expr(expr1), "(- 123)");
        assert_eq!(ast_printer.print_expr(expr2), "(group 45.67)");
    }

    #[test]
    fn test_complete_expr() {
        let expr = Expr::Binary(Box::new(BinaryExpr {
            left: Expr::Unary(Box::new(UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: "-".to_string(),
                    literal: LiteralToken::Empty,
                    line: 1,
                },
                right: Expr::Literal(LiteralExpr {
                    value: LiteralToken::Number(123 as f64),
                }),
            })),
            operator: Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: LiteralToken::Empty,
                line: 1,
            },
            right: Expr::Grouping(Box::new(GroupingExpr {
                expression: Expr::Literal(LiteralExpr {
                    value: LiteralToken::Number(45.67),
                }),
            })),
        }));

        let ast_printer = AstPrinter;
        assert_eq!(ast_printer.print_expr(expr), "(* (- 123) (group 45.67))");
    }
}

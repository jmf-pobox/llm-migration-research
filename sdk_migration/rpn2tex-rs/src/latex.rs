//! LaTeX generator for converting AST expressions to LaTeX format.
//!
//! This module provides the `LaTeXGenerator` struct which converts an Abstract
//! Syntax Tree (AST) into properly formatted LaTeX output with correct operator
//! precedence and parenthesization.

use crate::ast::Expr;

/// Generator for converting AST to LaTeX format.
///
/// Handles operator precedence and parenthesization to produce correct LaTeX
/// output. All expressions are wrapped in `$...$` for inline math mode.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Expr;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let expr = Expr::Number {
///     line: 1,
///     column: 1,
///     value: "42".to_string(),
/// };
///
/// let generator = LaTeXGenerator;
/// assert_eq!(generator.generate(&expr), "$42$");
/// ```
#[derive(Debug, Clone, Copy)]
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    /// Generates LaTeX output from an AST expression.
    ///
    /// The output is wrapped in `$...$` for LaTeX inline math mode.
    /// Operators are converted to their LaTeX equivalents and proper
    /// parenthesization is applied based on precedence rules.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// // Simple addition: 5 + 3
    /// let left = Box::new(Expr::Number {
    ///     line: 1,
    ///     column: 1,
    ///     value: "5".to_string(),
    /// });
    /// let right = Box::new(Expr::Number {
    ///     line: 1,
    ///     column: 3,
    ///     value: "3".to_string(),
    /// });
    /// let expr = Expr::BinaryOp {
    ///     line: 1,
    ///     column: 5,
    ///     operator: "+".to_string(),
    ///     left,
    ///     right,
    /// };
    ///
    /// let generator = LaTeXGenerator;
    /// assert_eq!(generator.generate(&expr), "$5 + 3$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = Self::visit(ast);
        format!("${content}$")
    }

    /// Visits an AST node and generates its LaTeX representation.
    ///
    /// This is a recursive visitor function that handles both number literals
    /// and binary operations.
    fn visit(node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let my_prec = precedence(operator);
                let latex_op = operator_mapping(operator);

                let mut left_str = Self::visit(left);
                if needs_parens(left, my_prec, false) {
                    left_str = format!("( {left_str} )");
                }

                let mut right_str = Self::visit(right);
                if needs_parens(right, my_prec, true) {
                    right_str = format!("( {right_str} )");
                }

                format!("{left_str} {latex_op} {right_str}")
            }
        }
    }
}

/// Returns the precedence level of an operator.
///
/// Precedence levels:
/// - Level 1 (low): `+`, `-`
/// - Level 2 (high): `*`, `/`
fn precedence(operator: &str) -> u32 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

/// Maps RPN operators to their LaTeX equivalents.
///
/// Mappings:
/// - `+` → `+`
/// - `-` → `-`
/// - `*` → `\times`
/// - `/` → `\div`
fn operator_mapping(operator: &str) -> &str {
    match operator {
        "+" => "+",
        "-" => "-",
        "*" => r"\times",
        "/" => r"\div",
        _ => operator,
    }
}

/// Determines if a child expression needs parentheses.
///
/// Parentheses are required when:
/// 1. Child has lower precedence than parent (always)
/// 2. Child has equal precedence and appears on RIGHT side of `-` or `/`
///    (to maintain left-associativity)
///
/// # Arguments
///
/// * `child` - The child expression to check
/// * `parent_precedence` - The precedence level of the parent operator
/// * `is_right` - Whether the child is the right operand
fn needs_parens(child: &Expr, parent_precedence: u32, is_right: bool) -> bool {
    match child {
        Expr::BinaryOp { operator, .. } => {
            let child_prec = precedence(operator);

            // Lower precedence always needs parens
            if child_prec < parent_precedence {
                return true;
            }

            // Equal precedence on right of - or / needs parens
            if child_prec == parent_precedence && is_right {
                return matches!(operator.as_str(), "-" | "/");
            }

            false
        }
        Expr::Number { .. } => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let expr = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), "$42$");
    }

    #[test]
    fn test_decimal_number() {
        let expr = Expr::Number {
            line: 1,
            column: 1,
            value: "3.14".to_string(),
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), "$3.14$");
    }

    #[test]
    fn test_simple_addition() {
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "5".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 3,
            value: "3".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "+".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), "$5 + 3$");
    }

    #[test]
    fn test_simple_subtraction() {
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "5".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 3,
            value: "3".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "-".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), "$5 - 3$");
    }

    #[test]
    fn test_multiplication() {
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "4".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 3,
            value: "7".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "*".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$4 \times 7$");
    }

    #[test]
    fn test_division() {
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "10".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 4,
            value: "2".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 6,
            operator: "/".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$10 \div 2$");
    }

    #[test]
    fn test_addition_then_multiply() {
        // (5 + 3) * 2
        let left = Box::new(Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "5".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 7,
            value: "2".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "*".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_multiply_then_addition() {
        // 5 * 3 + 2
        let left = Box::new(Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "*".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "5".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 7,
            value: "2".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "+".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_subtraction_chain() {
        // 5 - 3 - 2
        let left = Box::new(Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "-".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "5".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 7,
            value: "2".to_string(),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "-".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), "$5 - 3 - 2$");
    }

    #[test]
    fn test_addition_on_right_of_multiply() {
        // 2 * (3 + 4)
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "2".to_string(),
        });
        let right = Box::new(Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "4".to_string(),
            }),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "*".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_complex_expression() {
        // (1 + 2) * (3 + 4)
        let left = Box::new(Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "1".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "2".to_string(),
            }),
        });
        let right = Box::new(Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "3".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 7,
                value: "4".to_string(),
            }),
        });
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "*".to_string(),
            left,
            right,
        };

        let generator = LaTeXGenerator;
        assert_eq!(generator.generate(&expr), r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_functions() {
        assert_eq!(precedence("+"), 1);
        assert_eq!(precedence("-"), 1);
        assert_eq!(precedence("*"), 2);
        assert_eq!(precedence("/"), 2);
    }

    #[test]
    fn test_operator_mapping() {
        assert_eq!(operator_mapping("+"), "+");
        assert_eq!(operator_mapping("-"), "-");
        assert_eq!(operator_mapping("*"), r"\times");
        assert_eq!(operator_mapping("/"), r"\div");
    }

    #[test]
    fn test_needs_parens_lower_precedence() {
        let child = Expr::BinaryOp {
            line: 1,
            column: 1,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "5".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        };

        // Addition (prec 1) needs parens when child of multiplication (prec 2)
        assert!(needs_parens(&child, 2, false));
        assert!(needs_parens(&child, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_side() {
        let child = Expr::BinaryOp {
            line: 1,
            column: 1,
            operator: "-".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "5".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        };

        // Subtraction on right of subtraction needs parens
        assert!(needs_parens(&child, 1, true));
        // But not on left
        assert!(!needs_parens(&child, 1, false));
    }

    #[test]
    fn test_needs_parens_number() {
        let child = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        // Numbers never need parens
        assert!(!needs_parens(&child, 1, false));
        assert!(!needs_parens(&child, 1, true));
        assert!(!needs_parens(&child, 2, false));
        assert!(!needs_parens(&child, 2, true));
    }
}

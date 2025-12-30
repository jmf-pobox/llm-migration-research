//! LaTeX code generation from AST.
//!
//! This module converts parsed AST expressions into LaTeX source code using
//! a visitor pattern. It handles operator precedence intelligently to insert
//! parentheses only where needed to preserve mathematical meaning.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ast::Expr;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! let generator = LaTeXGenerator::new();
//!
//! // Simple addition: 5 + 3
//! let expr = Expr::BinaryOp {
//!     line: 1,
//!     column: 5,
//!     operator: "+".to_string(),
//!     left: Box::new(Expr::Number {
//!         line: 1,
//!         column: 1,
//!         value: "5".to_string(),
//!     }),
//!     right: Box::new(Expr::Number {
//!         line: 1,
//!         column: 3,
//!         value: "3".to_string(),
//!     }),
//! };
//!
//! assert_eq!(generator.generate(&expr), "$5 + 3$");
//! ```

use crate::ast::Expr;

/// Converts AST expressions to LaTeX source code.
///
/// Uses a visitor pattern to traverse the AST and generate LaTeX code
/// with proper operator precedence handling and parenthesization.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Expr;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let gen = LaTeXGenerator::new();
///
/// // Multiplication has higher precedence than addition
/// let expr = Expr::BinaryOp {
///     line: 1,
///     column: 7,
///     operator: "+".to_string(),
///     left: Box::new(Expr::BinaryOp {
///         line: 1,
///         column: 3,
///         operator: "*".to_string(),
///         left: Box::new(Expr::Number {
///             line: 1,
///             column: 1,
///             value: "5".to_string(),
///         }),
///         right: Box::new(Expr::Number {
///             line: 1,
///             column: 3,
///             value: "3".to_string(),
///         }),
///     }),
///     right: Box::new(Expr::Number {
///         line: 1,
///         column: 5,
///         value: "2".to_string(),
///     }),
/// };
///
/// assert_eq!(gen.generate(&expr), "$5 \\times 3 + 2$");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    /// Creates a new LaTeX generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let generator = LaTeXGenerator::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates LaTeX code from an AST expression.
    ///
    /// Returns the LaTeX code wrapped in math delimiters `$...$`.
    ///
    /// # Arguments
    ///
    /// * `ast` - The root AST expression to convert
    ///
    /// # Returns
    ///
    /// A string containing the LaTeX representation with math delimiters
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let gen = LaTeXGenerator::new();
    ///
    /// let expr = Expr::Number {
    ///     line: 1,
    ///     column: 1,
    ///     value: "42".to_string(),
    /// };
    ///
    /// assert_eq!(gen.generate(&expr), "$42$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let latex = Self::visit(ast, None, false);
        format!("${latex}$")
    }

    /// Returns the precedence level for an operator.
    ///
    /// Higher numbers indicate tighter binding.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator symbol
    ///
    /// # Returns
    ///
    /// The precedence level (1 for +/-, 2 for */÷)
    #[must_use]
    fn precedence(operator: &str) -> u8 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Converts an operator to its LaTeX representation.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator symbol
    ///
    /// # Returns
    ///
    /// The LaTeX representation of the operator
    #[must_use]
    fn operator_to_latex(operator: &str) -> &str {
        match operator {
            "+" => "+",
            "-" => "-",
            "*" => r"\times",
            "/" => r"\div",
            _ => operator,
        }
    }

    /// Visits an AST node and generates LaTeX code.
    ///
    /// This is the core visitor method that recursively processes the AST
    /// and handles parenthesization based on operator precedence.
    ///
    /// # Arguments
    ///
    /// * `node` - The AST node to visit
    /// * `parent_precedence` - The precedence of the parent operator (if any)
    /// * `is_right` - Whether this node is the right operand of its parent
    ///
    /// # Returns
    ///
    /// The LaTeX representation of the node and its subtree
    #[must_use]
    fn visit(node: &Expr, parent_precedence: Option<u8>, is_right: bool) -> String {
        match node {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let op_precedence = Self::precedence(operator);
                let op_latex = Self::operator_to_latex(operator);

                // Visit left and right children
                let left_latex = Self::visit(left, Some(op_precedence), false);
                let right_latex = Self::visit(right, Some(op_precedence), true);

                // Generate the expression
                let expr = format!("{left_latex} {op_latex} {right_latex}");

                // Determine if we need parentheses
                let needs_parens = if let Some(parent_prec) = parent_precedence {
                    op_precedence < parent_prec
                        || (op_precedence == parent_prec
                            && is_right
                            && matches!(operator.as_str(), "-" | "/"))
                } else {
                    false
                };

                if needs_parens {
                    format!("( {expr} )")
                } else {
                    expr
                }
            }
        }
    }
}

impl Default for LaTeXGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_number() {
        let gen = LaTeXGenerator::new();
        let expr = Expr::Number {
            line: 1,
            column: 1,
            value: "5".to_string(),
        };
        assert_eq!(gen.generate(&expr), "$5$");
    }

    #[test]
    fn test_simple_addition() {
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 5,
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
        assert_eq!(gen.generate(&expr), "$5 + 3$");
    }

    #[test]
    fn test_addition_then_multiply() {
        // (5 + 3) * 2
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "*".to_string(),
            left: Box::new(Expr::BinaryOp {
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
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "2".to_string(),
            }),
        };
        assert_eq!(gen.generate(&expr), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_multiply_then_addition() {
        // 5 * 3 + 2
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "+".to_string(),
            left: Box::new(Expr::BinaryOp {
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
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "2".to_string(),
            }),
        };
        assert_eq!(gen.generate(&expr), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_division_then_multiply() {
        // 10 / 2 * 5
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "*".to_string(),
            left: Box::new(Expr::BinaryOp {
                line: 1,
                column: 3,
                operator: "/".to_string(),
                left: Box::new(Expr::Number {
                    line: 1,
                    column: 1,
                    value: "10".to_string(),
                }),
                right: Box::new(Expr::Number {
                    line: 1,
                    column: 3,
                    value: "2".to_string(),
                }),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "5".to_string(),
            }),
        };
        assert_eq!(gen.generate(&expr), r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_left_associative_subtraction() {
        // 5 - 3 - 2 (should be ((5 - 3) - 2), no parens needed)
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "-".to_string(),
            left: Box::new(Expr::BinaryOp {
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
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 5,
                value: "2".to_string(),
            }),
        };
        assert_eq!(gen.generate(&expr), "$5 - 3 - 2$");
    }

    #[test]
    fn test_multiply_with_addition_on_right() {
        // 2 * (3 + 4)
        let gen = LaTeXGenerator::new();
        let expr = Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "*".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "2".to_string(),
            }),
            right: Box::new(Expr::BinaryOp {
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
            }),
        };
        assert_eq!(gen.generate(&expr), r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_default_trait() {
        let gen1 = LaTeXGenerator::new();
        let gen2 = LaTeXGenerator::default();
        assert_eq!(gen1, gen2);
    }

    #[test]
    fn test_clone() {
        let gen1 = LaTeXGenerator::new();
        let gen2 = gen1.clone();
        assert_eq!(gen1, gen2);
    }

    #[test]
    fn test_debug() {
        let gen = LaTeXGenerator::new();
        let debug_str = format!("{gen:?}");
        assert!(debug_str.contains("LaTeXGenerator"));
    }
}

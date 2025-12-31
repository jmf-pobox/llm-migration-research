//! Abstract Syntax Tree node definitions for representing parsed RPN expressions.
//!
//! This module defines the AST structure used to represent mathematical expressions
//! after parsing from RPN (Reverse Polish Notation). The AST preserves position
//! information for error reporting and uses recursive structures to represent
//! arbitrarily complex expressions.

/// An expression node in the Abstract Syntax Tree.
///
/// Each variant includes position information (line and column, both 1-based)
/// for precise error reporting. Numbers are stored as strings to preserve
/// exact representation for LaTeX output (e.g., "3.14" remains "3.14").
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Expr;
///
/// // A simple number
/// let num = Expr::Number {
///     line: 1,
///     column: 1,
///     value: "42".to_string(),
/// };
///
/// // A binary operation: 3 + 5
/// let add = Expr::BinaryOp {
///     line: 1,
///     column: 5,
///     operator: "+".to_string(),
///     left: Box::new(Expr::Number {
///         line: 1,
///         column: 1,
///         value: "3".to_string(),
///     }),
///     right: Box::new(Expr::Number {
///         line: 1,
///         column: 3,
///         value: "5".to_string(),
///     }),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal.
    ///
    /// The value is stored as a string to preserve exact representation
    /// for LaTeX output without floating-point precision issues.
    Number {
        /// 1-based line number where this number appears
        line: u32,
        /// 1-based column number where this number starts
        column: u32,
        /// String representation of the number (e.g., "3.14", "42")
        value: String,
    },

    /// A binary operation with left and right operands.
    ///
    /// The operator is one of: "+", "-", "*", "/"
    /// Left and right operands are boxed to allow recursive expression trees.
    BinaryOp {
        /// 1-based line number where this operator appears
        line: u32,
        /// 1-based column number where this operator is located
        column: u32,
        /// The operator symbol ("+", "-", "*", "/")
        operator: String,
        /// Left operand (evaluated first in RPN)
        left: Box<Expr>,
        /// Right operand (evaluated second in RPN)
        right: Box<Expr>,
    },
}

impl Expr {
    /// Returns the line number where this expression node is located.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::Number {
    ///     line: 5,
    ///     column: 10,
    ///     value: "42".to_string(),
    /// };
    /// assert_eq!(num.line(), 5);
    /// ```
    #[must_use]
    pub fn line(&self) -> u32 {
        match self {
            Self::Number { line, .. } | Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number where this expression node is located.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::Number {
    ///     line: 5,
    ///     column: 10,
    ///     value: "42".to_string(),
    /// };
    /// assert_eq!(num.column(), 10);
    /// ```
    #[must_use]
    pub fn column(&self) -> u32 {
        match self {
            Self::Number { column, .. } | Self::BinaryOp { column, .. } => *column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let num = Expr::Number {
            line: 1,
            column: 5,
            value: "42".to_string(),
        };

        assert_eq!(num.line(), 1);
        assert_eq!(num.column(), 5);

        if let Expr::Number { value, .. } = num {
            assert_eq!(value, "42");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_number_with_decimal() {
        let num = Expr::Number {
            line: 2,
            column: 10,
            value: "3.14159".to_string(),
        };

        if let Expr::Number { value, .. } = num {
            assert_eq!(value, "3.14159");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_binary_op_creation() {
        let left = Expr::Number {
            line: 1,
            column: 1,
            value: "5".to_string(),
        };

        let right = Expr::Number {
            line: 1,
            column: 3,
            value: "3".to_string(),
        };

        let add = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "+".to_string(),
            left: Box::new(left),
            right: Box::new(right),
        };

        assert_eq!(add.line(), 1);
        assert_eq!(add.column(), 5);

        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = add
        {
            assert_eq!(operator, "+");
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "5");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "3");
            }
        } else {
            panic!("Expected BinaryOp variant");
        }
    }

    #[test]
    fn test_nested_binary_ops() {
        // Build: (5 + 3) * 2
        let add = Expr::BinaryOp {
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

        let mult = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "*".to_string(),
            left: Box::new(add),
            right: Box::new(Expr::Number {
                line: 1,
                column: 9,
                value: "2".to_string(),
            }),
        };

        assert_eq!(mult.line(), 1);
        assert_eq!(mult.column(), 7);

        if let Expr::BinaryOp { operator, .. } = mult {
            assert_eq!(operator, "*");
        } else {
            panic!("Expected BinaryOp variant");
        }
    }

    #[test]
    fn test_all_operators() {
        let operators = vec!["+", "-", "*", "/"];

        for op in operators {
            let expr = Expr::BinaryOp {
                line: 1,
                column: 1,
                operator: op.to_string(),
                left: Box::new(Expr::Number {
                    line: 1,
                    column: 1,
                    value: "1".to_string(),
                }),
                right: Box::new(Expr::Number {
                    line: 1,
                    column: 1,
                    value: "2".to_string(),
                }),
            };

            if let Expr::BinaryOp { operator, .. } = expr {
                assert_eq!(operator, op);
            } else {
                panic!("Expected BinaryOp variant");
            }
        }
    }

    #[test]
    fn test_expr_equality() {
        let expr1 = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let expr2 = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_expr_inequality() {
        let expr1 = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let expr2 = Expr::Number {
            line: 1,
            column: 1,
            value: "43".to_string(),
        };

        assert_ne!(expr1, expr2);
    }

    #[test]
    fn test_clone() {
        let original = Expr::Number {
            line: 1,
            column: 5,
            value: "3.14".to_string(),
        };

        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_negative_numbers() {
        let num = Expr::Number {
            line: 1,
            column: 1,
            value: "-42".to_string(),
        };

        if let Expr::Number { value, .. } = num {
            assert_eq!(value, "-42");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_position_tracking() {
        let num1 = Expr::Number {
            line: 3,
            column: 7,
            value: "100".to_string(),
        };

        let num2 = Expr::Number {
            line: 5,
            column: 12,
            value: "200".to_string(),
        };

        assert_eq!(num1.line(), 3);
        assert_eq!(num1.column(), 7);
        assert_eq!(num2.line(), 5);
        assert_eq!(num2.column(), 12);
    }
}

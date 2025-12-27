//! Abstract Syntax Tree (AST) node types for representing parsed expressions.
//!
//! This module defines the `Expr` enum which represents expressions in the AST.
//! Each expression node tracks its position in the source (line and column) for
//! error reporting purposes.

/// Expression node in the Abstract Syntax Tree.
///
/// Represents either a numeric literal or a binary operation with two operands.
/// All nodes include position information (line and column) for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Expr;
///
/// // Create a number node
/// let num = Expr::Number {
///     line: 1,
///     column: 1,
///     value: "42".to_string(),
/// };
///
/// // Create a binary operation
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
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal.
    ///
    /// The value is stored as a string to preserve the original formatting
    /// (e.g., "3.14" vs "3.140").
    Number {
        /// Line number (1-based) where this number appears in the source
        line: u32,
        /// Column number (1-based) where this number appears in the source
        column: u32,
        /// String representation of the number (e.g., "42", "3.14", "-5")
        value: String,
    },

    /// A binary operation (e.g., addition, multiplication).
    ///
    /// Represents an operation with two operands (left and right).
    /// The operands are boxed to allow recursive tree structures.
    BinaryOp {
        /// Line number (1-based) where this operator appears in the source
        line: u32,
        /// Column number (1-based) where this operator appears in the source
        column: u32,
        /// The operator symbol: "+", "-", "*", or "/"
        operator: String,
        /// Left operand (boxed for recursion)
        left: Box<Expr>,
        /// Right operand (boxed for recursion)
        right: Box<Expr>,
    },
}

impl Expr {
    /// Returns the line number where this expression appears in the source.
    ///
    /// All expression nodes track their position for error reporting.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let expr = Expr::Number {
    ///     line: 5,
    ///     column: 10,
    ///     value: "42".to_string(),
    /// };
    /// assert_eq!(expr.line(), 5);
    /// ```
    #[must_use]
    pub fn line(&self) -> u32 {
        match self {
            Self::Number { line, .. } | Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number where this expression appears in the source.
    ///
    /// All expression nodes track their position for error reporting.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let expr = Expr::Number {
    ///     line: 5,
    ///     column: 10,
    ///     value: "42".to_string(),
    /// };
    /// assert_eq!(expr.column(), 10);
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
    fn test_number_construction() {
        let num = Expr::Number {
            line: 1,
            column: 5,
            value: "42".to_string(),
        };

        assert_eq!(num.line(), 1);
        assert_eq!(num.column(), 5);

        if let Expr::Number { value, .. } = &num {
            assert_eq!(value, "42");
        } else {
            panic!("Expected Number variant");
        }
    }

    #[test]
    fn test_binary_op_construction() {
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

        assert_eq!(expr.line(), 1);
        assert_eq!(expr.column(), 5);

        if let Expr::BinaryOp { operator, .. } = &expr {
            assert_eq!(operator, "+");
        } else {
            panic!("Expected BinaryOp variant");
        }
    }

    #[test]
    fn test_position_accessors() {
        let num = Expr::Number {
            line: 10,
            column: 20,
            value: "3.14".to_string(),
        };
        assert_eq!(num.line(), 10);
        assert_eq!(num.column(), 20);

        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "7".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 3,
            value: "2".to_string(),
        });
        let op = Expr::BinaryOp {
            line: 2,
            column: 15,
            operator: "*".to_string(),
            left,
            right,
        };
        assert_eq!(op.line(), 2);
        assert_eq!(op.column(), 15);
    }

    #[test]
    fn test_nested_binary_ops() {
        // Create: (5 + 3) * 2
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

        assert_eq!(expr.line(), 1);
        assert_eq!(expr.column(), 5);
    }

    #[test]
    fn test_clone() {
        let expr = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_debug_format() {
        let num = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let debug_str = format!("{:?}", num);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
    }
}

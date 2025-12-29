//! Abstract Syntax Tree node types for RPN expressions.
//!
//! This module defines the AST structure used by the parser to represent
//! parsed RPN expressions as an immutable tree structure. Each node tracks
//! its position in the source text for error reporting.
//!
//! # Structure
//!
//! The AST is represented by the [`Expr`] enum, which can be:
//! - A numeric literal ([`Expr::Number`])
//! - A binary operation ([`Expr::BinaryOp`]) with left and right operands
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ast::Expr;
//!
//! // Create a number node
//! let num = Expr::Number {
//!     line: 1,
//!     column: 1,
//!     value: "42".to_string(),
//! };
//!
//! // Create a binary operation: 2 + 3
//! let left = Box::new(Expr::Number {
//!     line: 1,
//!     column: 1,
//!     value: "2".to_string(),
//! });
//! let right = Box::new(Expr::Number {
//!     line: 1,
//!     column: 3,
//!     value: "3".to_string(),
//! });
//! let add_expr = Expr::BinaryOp {
//!     line: 1,
//!     column: 5,
//!     operator: "+".to_string(),
//!     left,
//!     right,
//! };
//! ```

/// An expression node in the abstract syntax tree.
///
/// Represents either a numeric literal or a binary operation. Each variant
/// includes position information (line and column) for error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal value.
    ///
    /// # Fields
    ///
    /// * `line` - The line number where this number appears (1-indexed)
    /// * `column` - The column number where this number appears (1-indexed)
    /// * `value` - The string representation of the numeric value
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::Number {
    ///     line: 1,
    ///     column: 1,
    ///     value: "3.14".to_string(),
    /// };
    /// ```
    Number {
        line: u32,
        column: u32,
        value: String,
    },

    /// A binary operation with two operands.
    ///
    /// # Fields
    ///
    /// * `line` - The line number where this operator appears (1-indexed)
    /// * `column` - The column number where this operator appears (1-indexed)
    /// * `operator` - The operator symbol (e.g., "+", "-", "*", "/")
    /// * `left` - The left operand expression
    /// * `right` - The right operand expression
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// // Represents: 5 + 3
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
    BinaryOp {
        line: u32,
        column: u32,
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    /// Returns the position (line, column) of this expression node.
    ///
    /// # Returns
    ///
    /// A tuple of `(line, column)` where both are 1-indexed.
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
    ///
    /// assert_eq!(num.position(), (5, 10));
    /// ```
    #[must_use]
    pub const fn position(&self) -> (u32, u32) {
        match self {
            Self::Number { line, column, .. } | Self::BinaryOp { line, column, .. } => {
                (*line, *column)
            }
        }
    }

    /// Returns the line number where this expression appears.
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
    ///
    /// assert_eq!(num.line(), 5);
    /// ```
    #[must_use]
    pub const fn line(&self) -> u32 {
        match self {
            Self::Number { line, .. } | Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number where this expression appears.
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
    ///
    /// assert_eq!(num.column(), 10);
    /// ```
    #[must_use]
    pub const fn column(&self) -> u32 {
        match self {
            Self::Number { column, .. } | Self::BinaryOp { column, .. } => *column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_position() {
        let num = Expr::Number {
            line: 5,
            column: 10,
            value: "42".to_string(),
        };

        assert_eq!(num.position(), (5, 10));
        assert_eq!(num.line(), 5);
        assert_eq!(num.column(), 10);
    }

    #[test]
    fn test_binary_op_position() {
        let left = Box::new(Expr::Number {
            line: 1,
            column: 1,
            value: "2".to_string(),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 3,
            value: "3".to_string(),
        });
        let op = Expr::BinaryOp {
            line: 1,
            column: 5,
            operator: "+".to_string(),
            left,
            right,
        };

        assert_eq!(op.position(), (1, 5));
        assert_eq!(op.line(), 1);
        assert_eq!(op.column(), 5);
    }

    #[test]
    fn test_clone() {
        let num = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let cloned = num.clone();
        assert_eq!(num, cloned);
    }

    #[test]
    fn test_debug() {
        let num = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };

        let debug_str = format!("{num:?}");
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_equality() {
        let num1 = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };
        let num2 = Expr::Number {
            line: 1,
            column: 1,
            value: "42".to_string(),
        };
        let num3 = Expr::Number {
            line: 1,
            column: 1,
            value: "43".to_string(),
        };

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
    }

    #[test]
    fn test_nested_binary_op() {
        // Represents: (2 + 3) * 4
        let left = Box::new(Expr::BinaryOp {
            line: 1,
            column: 3,
            operator: "+".to_string(),
            left: Box::new(Expr::Number {
                line: 1,
                column: 1,
                value: "2".to_string(),
            }),
            right: Box::new(Expr::Number {
                line: 1,
                column: 3,
                value: "3".to_string(),
            }),
        });
        let right = Box::new(Expr::Number {
            line: 1,
            column: 5,
            value: "4".to_string(),
        });
        let mul = Expr::BinaryOp {
            line: 1,
            column: 7,
            operator: "*".to_string(),
            left,
            right,
        };

        assert_eq!(mul.position(), (1, 7));
    }
}

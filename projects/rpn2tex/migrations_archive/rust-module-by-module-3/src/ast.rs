//! Abstract Syntax Tree (AST) node definitions.
//!
//! This module defines the AST structure for representing mathematical expressions
//! parsed from RPN notation. The AST uses an enum-based approach with two variants:
//! - `Number`: Represents numeric literals
//! - `BinaryOp`: Represents binary operations (+, -, *, /)
//!
//! Each node carries position information (line and column) for error reporting.
//! Position tracking uses 1-based indexing.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ast::Expr;
//!
//! // Create a number node
//! let num = Expr::number("42", 1, 1);
//!
//! // Create a binary operation node
//! let left = Expr::number("5", 1, 1);
//! let right = Expr::number("3", 1, 3);
//! let add = Expr::binary_op("+", left, right, 1, 2);
//! ```

/// Represents an expression in the abstract syntax tree.
///
/// Each variant carries position information (line and column) for error reporting.
/// This enum replaces Python's class inheritance hierarchy with Rust's enum-based
/// algebraic data types.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// A numeric literal.
    ///
    /// The value is stored as a string to preserve the exact input representation
    /// (e.g., "3.14" vs "3.140").
    Number {
        /// The numeric value as a string
        value: String,
        /// Line number in the source (1-based)
        line: usize,
        /// Column number in the source (1-based)
        column: usize,
    },
    /// A binary operation (e.g., addition, multiplication).
    ///
    /// The left and right operands are boxed to allow recursive tree structure.
    /// Boxing is required because `Expr` has an indeterminate size at compile time
    /// due to recursion.
    BinaryOp {
        /// The operator symbol ("+", "-", "*", "/")
        operator: String,
        /// Left operand expression
        left: Box<Expr>,
        /// Right operand expression
        right: Box<Expr>,
        /// Line number in the source (1-based)
        line: usize,
        /// Column number in the source (1-based)
        column: usize,
    },
}

impl Expr {
    /// Creates a new number expression.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value as a string
    /// * `line` - Line number in the source (1-based)
    /// * `column` - Column number in the source (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::number("42", 1, 1);
    /// assert_eq!(num.line(), 1);
    /// assert_eq!(num.column(), 1);
    /// ```
    #[must_use]
    pub fn number(value: impl Into<String>, line: usize, column: usize) -> Self {
        Self::Number {
            value: value.into(),
            line,
            column,
        }
    }

    /// Creates a new binary operation expression.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator symbol ("+", "-", "*", "/")
    /// * `left` - Left operand expression
    /// * `right` - Right operand expression
    /// * `line` - Line number in the source (1-based)
    /// * `column` - Column number in the source (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let left = Expr::number("5", 1, 1);
    /// let right = Expr::number("3", 1, 3);
    /// let add = Expr::binary_op("+", left, right, 1, 2);
    /// assert_eq!(add.line(), 1);
    /// assert_eq!(add.column(), 2);
    /// ```
    #[must_use]
    pub fn binary_op(
        operator: impl Into<String>,
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    ) -> Self {
        Self::BinaryOp {
            operator: operator.into(),
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }

    /// Returns the line number where this expression starts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::number("42", 5, 10);
    /// assert_eq!(num.line(), 5);
    /// ```
    #[must_use]
    pub fn line(&self) -> usize {
        match self {
            Self::Number { line, .. } => *line,
            Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number where this expression starts.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::number("42", 5, 10);
    /// assert_eq!(num.column(), 10);
    /// ```
    #[must_use]
    pub fn column(&self) -> usize {
        match self {
            Self::Number { column, .. } => *column,
            Self::BinaryOp { column, .. } => *column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let num = Expr::number("42", 1, 1);
        assert_eq!(num.line(), 1);
        assert_eq!(num.column(), 1);
        match num {
            Expr::Number { value, .. } => assert_eq!(value, "42"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_number_with_string() {
        let num = Expr::number("3.14".to_string(), 2, 5);
        assert_eq!(num.line(), 2);
        assert_eq!(num.column(), 5);
        match num {
            Expr::Number { value, .. } => assert_eq!(value, "3.14"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_binary_op_creation() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let op = Expr::binary_op("+", left, right, 1, 2);

        assert_eq!(op.line(), 1);
        assert_eq!(op.column(), 2);
        match op {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_binary_op_nested() {
        let num1 = Expr::number("1", 1, 1);
        let num2 = Expr::number("2", 1, 3);
        let add = Expr::binary_op("+", num1, num2, 1, 2);

        let num3 = Expr::number("3", 1, 5);
        let mult = Expr::binary_op("*", add, num3, 1, 4);

        assert_eq!(mult.line(), 1);
        assert_eq!(mult.column(), 4);
        match mult {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                match *left {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
                    _ => panic!("Expected nested BinaryOp"),
                }
                match *right {
                    Expr::Number { value, .. } => assert_eq!(value, "3"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_position_accessors() {
        let num = Expr::number("42", 5, 10);
        assert_eq!(num.line(), 5);
        assert_eq!(num.column(), 10);

        let left = Expr::number("1", 2, 3);
        let right = Expr::number("2", 2, 5);
        let op = Expr::binary_op("-", left, right, 2, 4);
        assert_eq!(op.line(), 2);
        assert_eq!(op.column(), 4);
    }

    #[test]
    fn test_expr_clone() {
        let num = Expr::number("42", 1, 1);
        let num_clone = num.clone();
        assert_eq!(num, num_clone);
    }

    #[test]
    fn test_expr_equality() {
        let num1 = Expr::number("42", 1, 1);
        let num2 = Expr::number("42", 1, 1);
        let num3 = Expr::number("43", 1, 1);

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
    }

    #[test]
    fn test_binary_op_with_all_operators() {
        let operators = vec!["+", "-", "*", "/"];
        for op in operators {
            let left = Expr::number("5", 1, 1);
            let right = Expr::number("3", 1, 3);
            let expr = Expr::binary_op(op, left, right, 1, 2);
            match expr {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, op),
                _ => panic!("Expected BinaryOp"),
            }
        }
    }

    #[test]
    fn test_pattern_matching() {
        let num = Expr::number("42", 1, 1);
        assert!(matches!(num, Expr::Number { .. }));

        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let op = Expr::binary_op("+", left, right, 1, 2);
        assert!(matches!(op, Expr::BinaryOp { .. }));
    }

    #[test]
    fn test_recursive_structure() {
        // Test deeply nested structure: ((1 + 2) * 3) - 4
        let num1 = Expr::number("1", 1, 1);
        let num2 = Expr::number("2", 1, 3);
        let add = Expr::binary_op("+", num1, num2, 1, 2);

        let num3 = Expr::number("3", 1, 5);
        let mult = Expr::binary_op("*", add, num3, 1, 4);

        let num4 = Expr::number("4", 1, 7);
        let sub = Expr::binary_op("-", mult, num4, 1, 6);

        assert_eq!(sub.line(), 1);
        assert_eq!(sub.column(), 6);
    }

    #[test]
    fn test_impl_into_string() {
        // Test that we can pass both &str and String
        let num1 = Expr::number("42", 1, 1);
        let num2 = Expr::number(String::from("42"), 1, 1);
        assert_eq!(num1, num2);

        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let op1 = Expr::binary_op("+", left.clone(), right.clone(), 1, 2);
        let op2 = Expr::binary_op(String::from("+"), left, right, 1, 2);
        assert_eq!(op1, op2);
    }
}

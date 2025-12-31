//! Abstract Syntax Tree node definitions.
//!
//! This module defines the AST node types that represent parsed RPN expressions.

/// An expression in the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal.
    Number(Number),
    /// A binary operation (e.g., addition).
    BinaryOp(BinaryOp),
}

/// A numeric literal node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// The string representation of the number (preserved from input).
    pub value: String,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
}

impl Number {
    /// Creates a new number node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Number;
    ///
    /// let num = Number::new("42".to_string(), 1, 1);
    /// assert_eq!(num.value, "42");
    /// ```
    #[must_use]
    pub fn new(value: String, line: usize, column: usize) -> Self {
        Self {
            value,
            line,
            column,
        }
    }
}

/// A binary operation node (e.g., addition, subtraction).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// The operator (e.g., "+", "-", "*", "/").
    pub operator: String,
    /// The left operand.
    pub left: Box<Expr>,
    /// The right operand.
    pub right: Box<Expr>,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
}

impl BinaryOp {
    /// Creates a new binary operation node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{BinaryOp, Number, Expr};
    ///
    /// let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
    /// let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
    /// let binop = BinaryOp::new("+".to_string(), left, right, 1, 5);
    /// assert_eq!(binop.operator, "+");
    /// ```
    #[must_use]
    pub fn new(
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            operator,
            left,
            right,
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let num = Number::new("5".to_string(), 1, 1);
        assert_eq!(num.value, "5");
        assert_eq!(num.line, 1);
        assert_eq!(num.column, 1);
    }

    #[test]
    fn test_number_preserves_string() {
        let num = Number::new("3.14".to_string(), 1, 1);
        assert_eq!(num.value, "3.14");
    }

    #[test]
    fn test_expr_number_variant() {
        let num = Number::new("42".to_string(), 1, 1);
        let expr = Expr::Number(num);
        match expr {
            Expr::Number(n) => assert_eq!(n.value, "42"),
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_binary_op_creation() {
        let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let binop = BinaryOp::new("+".to_string(), left, right, 1, 5);
        assert_eq!(binop.operator, "+");
        assert_eq!(binop.line, 1);
        assert_eq!(binop.column, 5);
    }

    #[test]
    fn test_expr_binary_op_variant() {
        let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let binop = BinaryOp::new("+".to_string(), left, right, 1, 5);
        let expr = Expr::BinaryOp(binop);
        match expr {
            Expr::BinaryOp(op) => assert_eq!(op.operator, "+"),
            _ => panic!("Expected BinaryOp variant"),
        }
    }
}

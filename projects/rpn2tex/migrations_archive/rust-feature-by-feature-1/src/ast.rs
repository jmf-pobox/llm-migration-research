//! AST node definitions for rpn2tex parser.
//!
//! This module defines the Abstract Syntax Tree (AST) nodes used to represent
//! parsed RPN expressions.

/// Expression node types.
///
/// Represents the structure of mathematical expressions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Numeric literal
    Number(Number),
    /// Binary operation (e.g., addition, subtraction)
    BinaryOp(BinaryOp),
}

/// Numeric literal node.
///
/// Represents integer and decimal numbers in expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Number;
///
/// let num = Number::new("42".to_string(), 1, 1);
/// assert_eq!(num.value, "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// The string representation of the number
    pub value: String,
    /// Line number (1-based) where node appears
    pub line: usize,
    /// Column number (1-based) where node starts
    pub column: usize,
}

impl Number {
    /// Create a new Number node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Number;
    ///
    /// let num = Number::new("3.14".to_string(), 1, 1);
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

/// Binary operation node.
///
/// Represents operations with two operands (e.g., addition, subtraction).
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{BinaryOp, Expr, Number};
///
/// let left = Expr::Number(Number::new("5".to_string(), 1, 1));
/// let right = Expr::Number(Number::new("3".to_string(), 1, 3));
/// let add = BinaryOp::new("+".to_string(), left, right, 1, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// The operator string (e.g., "+", "-", "*", "/")
    pub operator: String,
    /// Left operand
    pub left: Box<Expr>,
    /// Right operand
    pub right: Box<Expr>,
    /// Line number (1-based) where node appears
    pub line: usize,
    /// Column number (1-based) where node starts
    pub column: usize,
}

impl BinaryOp {
    /// Create a new BinaryOp node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{BinaryOp, Expr, Number};
    ///
    /// let left = Expr::Number(Number::new("5".to_string(), 1, 1));
    /// let right = Expr::Number(Number::new("3".to_string(), 1, 3));
    /// let add = BinaryOp::new("+".to_string(), left, right, 1, 5);
    /// ```
    #[must_use]
    pub fn new(operator: String, left: Expr, right: Expr, line: usize, column: usize) -> Self {
        Self {
            operator,
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }
}

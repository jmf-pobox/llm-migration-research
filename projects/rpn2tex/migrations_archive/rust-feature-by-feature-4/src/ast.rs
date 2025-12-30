//! AST node definitions for rpn2tex parser.
//!
//! This module defines the Abstract Syntax Tree (AST) nodes used to represent
//! parsed RPN expressions.

/// A numeric literal node.
///
/// Represents integer and decimal numbers in expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Number;
///
/// let num = Number::new(1, 1, "42".to_string());
/// assert_eq!(num.value, "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// Line number (1-based) where node appears
    pub line: usize,
    /// Column number (1-based) where node starts
    pub column: usize,
    /// The string representation of the number
    pub value: String,
}

impl Number {
    /// Create a new number node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Number;
    ///
    /// let num = Number::new(1, 1, "42".to_string());
    /// ```
    #[must_use]
    pub fn new(line: usize, column: usize, value: String) -> Self {
        Self {
            line,
            column,
            value,
        }
    }
}

/// A binary operation node.
///
/// Represents operations like addition, subtraction, etc.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{BinaryOp, Expr, Number};
///
/// let left = Expr::Number(Number::new(1, 1, "5".to_string()));
/// let right = Expr::Number(Number::new(1, 3, "3".to_string()));
/// let op = BinaryOp::new(1, 5, "+".to_string(), left, right);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// Line number (1-based) where node appears
    pub line: usize,
    /// Column number (1-based) where node starts
    pub column: usize,
    /// The operator symbol (+, -, *, /, etc.)
    pub operator: String,
    /// Left operand expression
    pub left: Box<Expr>,
    /// Right operand expression
    pub right: Box<Expr>,
}

impl BinaryOp {
    /// Create a new binary operation node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{BinaryOp, Expr, Number};
    ///
    /// let left = Expr::Number(Number::new(1, 1, "5".to_string()));
    /// let right = Expr::Number(Number::new(1, 3, "3".to_string()));
    /// let op = BinaryOp::new(1, 5, "+".to_string(), left, right);
    /// ```
    #[must_use]
    pub fn new(line: usize, column: usize, operator: String, left: Expr, right: Expr) -> Self {
        Self {
            line,
            column,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Expression types in the AST.
///
/// Represents all possible expression node types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal
    Number(Number),
    /// A binary operation
    BinaryOp(BinaryOp),
}

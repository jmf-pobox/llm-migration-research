//! Abstract Syntax Tree (AST) node types for RPN expressions.

/// Represents a numeric literal in the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// The numeric value as a string (not parsed to int/float).
    pub value: String,
    /// Line number where this number appears (1-indexed).
    pub line: usize,
    /// Column number where this number starts (1-indexed).
    pub column: usize,
}

/// Represents a binary operation (e.g., addition, subtraction) in the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// Line number where this operation appears (1-indexed).
    pub line: usize,
    /// Column number where this operation starts (1-indexed).
    pub column: usize,
    /// The operator symbol (e.g., "+", "-").
    pub operator: String,
    /// The left operand expression.
    pub left: Box<Expr>,
    /// The right operand expression.
    pub right: Box<Expr>,
}

impl Number {
    /// Creates a new Number node.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::ast::Number;
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

impl BinaryOp {
    /// Creates a new BinaryOp node.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::ast::{BinaryOp, Expr, Number};
    /// let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
    /// let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
    /// let op = BinaryOp::new("+".to_string(), left, right, 1, 5);
    /// assert_eq!(op.operator, "+");
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
            line,
            column,
            operator,
            left,
            right,
        }
    }
}

/// Expression types in the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal.
    Number(Number),
    /// A binary operation (e.g., addition, subtraction).
    BinaryOp(BinaryOp),
}

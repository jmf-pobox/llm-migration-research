//! AST node definitions for rpn2tex parser.
//!
//! This module defines the Abstract Syntax Tree (AST) nodes used to represent
//! parsed RPN expressions.
//!
//! AST nodes are structures that represent the structure of mathematical expressions.
//! Each node carries position information for error reporting.
//!
//! # Node Types
//!
//! * `Number` - Numeric literals (5, 3.14, -2)
//! * `BinaryOp` - Binary operations (a + b, x * y)

/// Base position information for AST nodes.
///
/// All AST nodes track their position in source for error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    /// Line number (1-based) where node appears
    pub line: usize,
    /// Column number (1-based) where node starts
    pub column: usize,
}

impl Position {
    /// Creates a new position.
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

/// Numeric literal node.
///
/// Represents integer and decimal numbers in expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{Number, Position};
///
/// let num = Number::new(Position::new(1, 1), "42".to_string());
/// assert_eq!(num.value, "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// Position in source
    pub pos: Position,
    /// The string representation of the number
    pub value: String,
}

impl Number {
    /// Creates a new number node.
    #[must_use]
    pub fn new(pos: Position, value: String) -> Self {
        Self { pos, value }
    }
}

/// Binary operation node.
///
/// Represents operations with two operands: +, -, *, /
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{BinaryOp, Expr, Number, Position};
///
/// // Represents "5 + 3"
/// let left = Expr::Number(Number::new(Position::new(1, 1), "5".to_string()));
/// let right = Expr::Number(Number::new(Position::new(1, 3), "3".to_string()));
/// let add = BinaryOp::new(Position::new(1, 3), "+".to_string(), left, right);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// Position in source
    pub pos: Position,
    /// The operator string ("+", "-", "*", "/")
    pub operator: String,
    /// The left operand expression
    pub left: Box<Expr>,
    /// The right operand expression
    pub right: Box<Expr>,
}

impl BinaryOp {
    /// Creates a new binary operation node.
    #[must_use]
    pub fn new(pos: Position, operator: String, left: Expr, right: Expr) -> Self {
        Self {
            pos,
            operator,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

/// Expression type representing all possible AST nodes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Number literal
    Number(Number),
    /// Binary operation
    BinaryOp(BinaryOp),
}

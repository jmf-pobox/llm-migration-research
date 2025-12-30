//! Abstract Syntax Tree node definitions.
//!
//! This module defines the AST nodes used to represent parsed expressions.

/// An expression node in the AST.
///
/// Supports numeric literals and binary operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Expr {
    /// A numeric literal.
    Number(Number),
    /// A binary operation (addition, etc.).
    BinaryOp(BinaryOp),
}

/// A numeric literal node.
///
/// Stores the numeric value as a string to preserve original formatting.
///
/// # Examples
///
/// ```
/// use rpn2tex::Number;
///
/// let num = Number::new("3.14", 1, 1);
/// assert_eq!(num.value(), "3.14");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Number {
    value: String,
    line: usize,
    column: usize,
}

impl Number {
    /// Creates a new number node.
    ///
    /// # Arguments
    ///
    /// * `value` - The string representation of the number
    /// * `line` - The line number (1-based)
    /// * `column` - The column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Number;
    ///
    /// let num = Number::new("42", 1, 5);
    /// assert_eq!(num.value(), "42");
    /// assert_eq!(num.line(), 1);
    /// assert_eq!(num.column(), 5);
    /// ```
    pub fn new(value: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            value: value.into(),
            line,
            column,
        }
    }

    /// Returns the string value of the number.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the line number (1-based).
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number (1-based).
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

/// A binary operation node.
///
/// Represents binary operations like addition, subtraction, multiplication, and division.
///
/// # Examples
///
/// ```
/// use rpn2tex::{BinaryOp, Number, Expr};
///
/// let left = Number::new("5", 1, 1);
/// let right = Number::new("3", 1, 3);
/// let binop = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);
/// assert_eq!(binop.operator(), "+");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct BinaryOp {
    operator: String,
    left: Box<Expr>,
    right: Box<Expr>,
    line: usize,
    column: usize,
}

impl BinaryOp {
    /// Creates a new binary operation node.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator string ("+", "-", "*", "/")
    /// * `left` - The left operand expression
    /// * `right` - The right operand expression
    /// * `line` - The line number (1-based)
    /// * `column` - The column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{BinaryOp, Number, Expr};
    ///
    /// let left = Number::new("2", 1, 1);
    /// let right = Number::new("3", 1, 3);
    /// let add = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);
    /// assert_eq!(add.operator(), "+");
    /// ```
    pub fn new(
        operator: impl Into<String>,
        left: Expr,
        right: Expr,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            operator: operator.into(),
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }

    /// Returns the operator string.
    #[must_use]
    pub fn operator(&self) -> &str {
        &self.operator
    }

    /// Returns a reference to the left operand.
    pub fn left(&self) -> &Expr {
        &self.left
    }

    /// Returns a reference to the right operand.
    pub fn right(&self) -> &Expr {
        &self.right
    }

    /// Returns the line number (1-based).
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number (1-based).
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let num = Number::new("42", 1, 1);
        assert_eq!(num.value(), "42");
        assert_eq!(num.line(), 1);
        assert_eq!(num.column(), 1);
    }

    #[test]
    fn test_number_with_decimal() {
        let num = Number::new("3.14", 2, 5);
        assert_eq!(num.value(), "3.14");
        assert_eq!(num.line(), 2);
        assert_eq!(num.column(), 5);
    }

    #[test]
    fn test_number_negative() {
        let num = Number::new("-5", 1, 1);
        assert_eq!(num.value(), "-5");
    }

    #[test]
    fn test_expr_number() {
        let num = Number::new("100", 1, 1);
        let expr = Expr::Number(num.clone());
        match expr {
            Expr::Number(n) => assert_eq!(n.value(), "100"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_number_clone() {
        let num1 = Number::new("42", 1, 1);
        let num2 = num1.clone();
        assert_eq!(num1, num2);
    }

    #[test]
    fn test_binop_creation() {
        let left = Number::new("5", 1, 1);
        let right = Number::new("3", 1, 3);
        let binop = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);
        assert_eq!(binop.operator(), "+");
        assert_eq!(binop.line(), 1);
        assert_eq!(binop.column(), 5);
    }

    #[test]
    fn test_binop_operands() {
        let left = Number::new("2", 1, 1);
        let right = Number::new("3", 1, 3);
        let binop = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);

        match binop.left() {
            Expr::Number(n) => assert_eq!(n.value(), "2"),
            _ => panic!("Expected Number"),
        }

        match binop.right() {
            Expr::Number(n) => assert_eq!(n.value(), "3"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_expr_binop() {
        let left = Number::new("1", 1, 1);
        let right = Number::new("2", 1, 3);
        let binop = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);
        let expr = Expr::BinaryOp(binop.clone());

        match expr {
            Expr::BinaryOp(b) => assert_eq!(b.operator(), "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_binop_clone() {
        let left = Number::new("5", 1, 1);
        let right = Number::new("3", 1, 3);
        let binop1 = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 5);
        let binop2 = binop1.clone();
        assert_eq!(binop1, binop2);
    }
}

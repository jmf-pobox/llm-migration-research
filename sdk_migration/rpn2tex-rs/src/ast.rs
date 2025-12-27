//! Abstract Syntax Tree (AST) node definitions.
//!
//! This module defines the AST structures used to represent parsed RPN expressions,
//! including numeric literals and binary operations.

use std::fmt;

/// Represents a numeric literal in the AST.
///
/// Contains the string representation of the number along with position information
/// for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::Number;
///
/// let num = Number::new("42", 1, 1);
/// assert_eq!(num.value, "42");
/// assert_eq!(num.line, 1);
/// assert_eq!(num.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number {
    /// String representation of the number
    pub value: String,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

impl Number {
    /// Creates a new Number node.
    ///
    /// # Arguments
    ///
    /// * `value` - The string representation of the number
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Number;
    ///
    /// let num = Number::new("3.14", 2, 5);
    /// assert_eq!(num.value, "3.14");
    /// ```
    #[must_use]
    pub fn new(value: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            value: value.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Number('{}', {}:{})", self.value, self.line, self.column)
    }
}

/// Represents a binary operation in the AST.
///
/// Contains the operator symbol, left and right operands (as boxed expressions
/// for recursive structure), and position information.
///
/// # Examples
///
/// ```
/// use rpn2tex::{BinaryOp, Expr, Number};
///
/// let left = Expr::Number(Number::new("3", 1, 1));
/// let right = Expr::Number(Number::new("4", 1, 3));
/// let binop = BinaryOp::new("+", left, right, 1, 5);
/// assert_eq!(binop.operator, "+");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    /// Operator symbol ("+", "-", "*", "/")
    pub operator: String,
    /// Left operand
    pub left: Box<Expr>,
    /// Right operand
    pub right: Box<Expr>,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

impl BinaryOp {
    /// Creates a new `BinaryOp` node.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator symbol
    /// * `left` - The left operand expression
    /// * `right` - The right operand expression
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{BinaryOp, Expr, Number};
    ///
    /// let left = Expr::Number(Number::new("5", 1, 1));
    /// let right = Expr::Number(Number::new("2", 1, 3));
    /// let binop = BinaryOp::new("*", left, right, 1, 5);
    /// assert_eq!(binop.operator, "*");
    /// ```
    #[must_use]
    pub fn new(
        operator: impl Into<String>,
        left: Expr,
        right: Expr,
        line: u32,
        column: u32,
    ) -> Self {
        Self {
            operator: operator.into(),
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BinaryOp('{}', {}, {}, {}:{})",
            self.operator, self.left, self.right, self.line, self.column
        )
    }
}

/// Expression type for the AST.
///
/// Represents any expression in the AST, either a numeric literal or a binary operation.
/// This enum is used throughout the codebase for pattern matching on expression types.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Expr, Number, BinaryOp};
///
/// let num_expr = Expr::Number(Number::new("42", 1, 1));
/// match num_expr {
///     Expr::Number(n) => assert_eq!(n.value, "42"),
///     Expr::BinaryOp(_) => panic!("Expected number"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Numeric literal
    Number(Number),
    /// Binary operation
    BinaryOp(BinaryOp),
}

impl Expr {
    /// Returns the line number of this expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Expr, Number};
    ///
    /// let expr = Expr::Number(Number::new("42", 3, 5));
    /// assert_eq!(expr.line(), 3);
    /// ```
    #[must_use]
    pub fn line(&self) -> u32 {
        match self {
            Self::Number(n) => n.line,
            Self::BinaryOp(b) => b.line,
        }
    }

    /// Returns the column number of this expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Expr, Number};
    ///
    /// let expr = Expr::Number(Number::new("42", 3, 5));
    /// assert_eq!(expr.column(), 5);
    /// ```
    #[must_use]
    pub fn column(&self) -> u32 {
        match self {
            Self::Number(n) => n.column,
            Self::BinaryOp(b) => b.column,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::BinaryOp(b) => write!(f, "{b}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_construction() {
        let num = Number::new("42", 1, 1);
        assert_eq!(num.value, "42");
        assert_eq!(num.line, 1);
        assert_eq!(num.column, 1);
    }

    #[test]
    fn test_number_construction_with_string() {
        let value = String::from("3.14");
        let num = Number::new(value, 2, 5);
        assert_eq!(num.value, "3.14");
        assert_eq!(num.line, 2);
        assert_eq!(num.column, 5);
    }

    #[test]
    fn test_number_display() {
        let num = Number::new("42", 1, 1);
        assert_eq!(num.to_string(), "Number('42', 1:1)");

        let num = Number::new("3.14", 2, 5);
        assert_eq!(num.to_string(), "Number('3.14', 2:5)");
    }

    #[test]
    fn test_number_equality() {
        let num1 = Number::new("42", 1, 1);
        let num2 = Number::new("42", 1, 1);
        let num3 = Number::new("43", 1, 1);
        let num4 = Number::new("42", 2, 1);

        assert_eq!(num1, num2);
        assert_ne!(num1, num3);
        assert_ne!(num1, num4);
    }

    #[test]
    fn test_number_clone() {
        let num = Number::new("42", 1, 1);
        let cloned = num.clone();
        assert_eq!(num, cloned);
    }

    #[test]
    fn test_binaryop_construction() {
        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let binop = BinaryOp::new("+", left, right, 1, 5);

        assert_eq!(binop.operator, "+");
        assert_eq!(binop.line, 1);
        assert_eq!(binop.column, 5);
    }

    #[test]
    fn test_binaryop_with_string_operator() {
        let left = Expr::Number(Number::new("5", 1, 1));
        let right = Expr::Number(Number::new("2", 1, 3));
        let operator = String::from("*");
        let binop = BinaryOp::new(operator, left, right, 1, 5);

        assert_eq!(binop.operator, "*");
    }

    #[test]
    fn test_binaryop_recursive_structure() {
        // Represents: (3 + 4) * 5
        let left_inner = Expr::Number(Number::new("3", 1, 1));
        let right_inner = Expr::Number(Number::new("4", 1, 3));
        let left = Expr::BinaryOp(BinaryOp::new("+", left_inner, right_inner, 1, 5));
        let right = Expr::Number(Number::new("5", 1, 7));
        let binop = BinaryOp::new("*", left, right, 1, 9);

        assert_eq!(binop.operator, "*");
        if let Expr::BinaryOp(inner) = binop.left.as_ref() {
            assert_eq!(inner.operator, "+");
        } else {
            panic!("Expected BinaryOp in left");
        }
    }

    #[test]
    fn test_binaryop_equality() {
        let left1 = Expr::Number(Number::new("3", 1, 1));
        let right1 = Expr::Number(Number::new("4", 1, 3));
        let binop1 = BinaryOp::new("+", left1.clone(), right1.clone(), 1, 5);

        let left2 = Expr::Number(Number::new("3", 1, 1));
        let right2 = Expr::Number(Number::new("4", 1, 3));
        let binop2 = BinaryOp::new("+", left2, right2, 1, 5);

        let left3 = Expr::Number(Number::new("3", 1, 1));
        let right3 = Expr::Number(Number::new("5", 1, 3));
        let binop3 = BinaryOp::new("+", left3, right3, 1, 5);

        assert_eq!(binop1, binop2);
        assert_ne!(binop1, binop3);
    }

    #[test]
    fn test_binaryop_clone() {
        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let binop = BinaryOp::new("+", left, right, 1, 5);
        let cloned = binop.clone();
        assert_eq!(binop, cloned);
    }

    #[test]
    fn test_expr_number_variant() {
        let expr = Expr::Number(Number::new("42", 1, 1));
        match expr {
            Expr::Number(n) => {
                assert_eq!(n.value, "42");
                assert_eq!(n.line, 1);
                assert_eq!(n.column, 1);
            }
            Expr::BinaryOp(_) => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_expr_binaryop_variant() {
        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let expr = Expr::BinaryOp(BinaryOp::new("+", left, right, 1, 5));

        match expr {
            Expr::Number(_) => panic!("Expected BinaryOp variant"),
            Expr::BinaryOp(b) => {
                assert_eq!(b.operator, "+");
                assert_eq!(b.line, 1);
                assert_eq!(b.column, 5);
            }
        }
    }

    #[test]
    fn test_expr_line_method() {
        let num_expr = Expr::Number(Number::new("42", 3, 5));
        assert_eq!(num_expr.line(), 3);

        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let binop_expr = Expr::BinaryOp(BinaryOp::new("+", left, right, 2, 7));
        assert_eq!(binop_expr.line(), 2);
    }

    #[test]
    fn test_expr_column_method() {
        let num_expr = Expr::Number(Number::new("42", 3, 5));
        assert_eq!(num_expr.column(), 5);

        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let binop_expr = Expr::BinaryOp(BinaryOp::new("+", left, right, 2, 7));
        assert_eq!(binop_expr.column(), 7);
    }

    #[test]
    fn test_expr_equality() {
        let expr1 = Expr::Number(Number::new("42", 1, 1));
        let expr2 = Expr::Number(Number::new("42", 1, 1));
        let expr3 = Expr::Number(Number::new("43", 1, 1));

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);

        let left1 = Expr::Number(Number::new("3", 1, 1));
        let right1 = Expr::Number(Number::new("4", 1, 3));
        let expr4 = Expr::BinaryOp(BinaryOp::new("+", left1, right1, 1, 5));

        let left2 = Expr::Number(Number::new("3", 1, 1));
        let right2 = Expr::Number(Number::new("4", 1, 3));
        let expr5 = Expr::BinaryOp(BinaryOp::new("+", left2, right2, 1, 5));

        assert_eq!(expr4, expr5);
        assert_ne!(expr1, expr4);
    }

    #[test]
    fn test_expr_clone() {
        let expr = Expr::Number(Number::new("42", 1, 1));
        let cloned = expr.clone();
        assert_eq!(expr, cloned);

        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let expr = Expr::BinaryOp(BinaryOp::new("+", left, right, 1, 5));
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_expr_display() {
        let expr = Expr::Number(Number::new("42", 1, 1));
        assert_eq!(expr.to_string(), "Number('42', 1:1)");

        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let expr = Expr::BinaryOp(BinaryOp::new("+", left, right, 1, 5));
        assert_eq!(
            expr.to_string(),
            "BinaryOp('+', Number('3', 1:1), Number('4', 1:3), 1:5)"
        );
    }

    #[test]
    fn test_position_tracking() {
        // Test that all nodes preserve position information
        let num = Number::new("42", 5, 10);
        assert_eq!(num.line, 5);
        assert_eq!(num.column, 10);

        let left = Expr::Number(Number::new("3", 1, 1));
        let right = Expr::Number(Number::new("4", 1, 3));
        let binop = BinaryOp::new("+", left, right, 2, 8);
        assert_eq!(binop.line, 2);
        assert_eq!(binop.column, 8);

        let expr = Expr::BinaryOp(binop);
        assert_eq!(expr.line(), 2);
        assert_eq!(expr.column(), 8);
    }

    #[test]
    fn test_deeply_nested_structure() {
        // Test: ((1 + 2) * 3) / 4
        let one = Expr::Number(Number::new("1", 1, 1));
        let two = Expr::Number(Number::new("2", 1, 3));
        let add = Expr::BinaryOp(BinaryOp::new("+", one, two, 1, 5));

        let three = Expr::Number(Number::new("3", 1, 7));
        let mult = Expr::BinaryOp(BinaryOp::new("*", add, three, 1, 9));

        let four = Expr::Number(Number::new("4", 1, 11));
        let div = BinaryOp::new("/", mult, four, 1, 13);

        assert_eq!(div.operator, "/");
        assert_eq!(div.line, 1);
        assert_eq!(div.column, 13);

        // Verify the nested structure
        if let Expr::BinaryOp(mult_op) = div.left.as_ref() {
            assert_eq!(mult_op.operator, "*");
            if let Expr::BinaryOp(add_op) = mult_op.left.as_ref() {
                assert_eq!(add_op.operator, "+");
            } else {
                panic!("Expected nested addition");
            }
        } else {
            panic!("Expected nested multiplication");
        }
    }

    #[test]
    fn test_all_operators() {
        let operators = vec!["+", "-", "*", "/"];
        for op in operators {
            let left = Expr::Number(Number::new("1", 1, 1));
            let right = Expr::Number(Number::new("2", 1, 3));
            let binop = BinaryOp::new(op, left, right, 1, 5);
            assert_eq!(binop.operator, op);
        }
    }
}

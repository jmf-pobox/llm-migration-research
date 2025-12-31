//! Abstract syntax tree node types.

/// An expression in the AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal.
    Number {
        value: String,
        line: usize,
        column: usize,
    },
    /// A binary operation (e.g., addition, subtraction, multiplication, division).
    BinaryOp {
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
        line: usize,
        column: usize,
    },
}

impl Expr {
    /// Creates a new number expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let expr = Expr::number("42", 1, 1);
    /// if let Expr::Number { value, .. } = expr {
    ///     assert_eq!(value, "42");
    /// }
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
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let left = Expr::number("5", 1, 1);
    /// let right = Expr::number("3", 1, 3);
    /// let expr = Expr::binary_op("+", left, right, 1, 5);
    /// if let Expr::BinaryOp { operator, .. } = expr {
    ///     assert_eq!(operator, "+");
    /// }
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

    /// Returns the line number of this expression.
    #[must_use]
    pub fn line(&self) -> usize {
        match self {
            Self::Number { line, .. } | Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number of this expression.
    #[must_use]
    pub fn column(&self) -> usize {
        match self {
            Self::Number { column, .. } | Self::BinaryOp { column, .. } => *column,
        }
    }

    /// Returns the precedence level of this expression.
    ///
    /// Precedence levels:
    /// - 0: Numbers (no operator)
    /// - 1: Addition (+) and Subtraction (-)
    /// - 2: Multiplication (*) and Division (/)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    ///
    /// let num = Expr::number("5", 1, 1);
    /// assert_eq!(num.precedence(), 0);
    ///
    /// let left = Expr::number("5", 1, 1);
    /// let right = Expr::number("3", 1, 3);
    /// let add = Expr::binary_op("+", left.clone(), right.clone(), 1, 5);
    /// assert_eq!(add.precedence(), 1);
    ///
    /// let mul = Expr::binary_op("*", left, right, 1, 5);
    /// assert_eq!(mul.precedence(), 2);
    /// ```
    #[must_use]
    pub fn precedence(&self) -> u32 {
        match self {
            Self::Number { .. } => 0,
            Self::BinaryOp { operator, .. } => match operator.as_str() {
                "+" | "-" => 1,
                "*" | "/" => 2,
                _ => 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_creation() {
        let expr = Expr::number("42", 1, 5);
        assert!(matches!(expr, Expr::Number { .. }));
        if let Expr::Number {
            value,
            line,
            column,
        } = expr
        {
            assert_eq!(value, "42");
            assert_eq!(line, 1);
            assert_eq!(column, 5);
        }
    }

    #[test]
    fn test_decimal_number() {
        let expr = Expr::number("3.14", 2, 10);
        if let Expr::Number { value, .. } = expr {
            assert_eq!(value, "3.14");
        }
    }

    #[test]
    fn test_expr_line() {
        let expr = Expr::number("5", 3, 7);
        assert_eq!(expr.line(), 3);
    }

    #[test]
    fn test_expr_column() {
        let expr = Expr::number("5", 3, 7);
        assert_eq!(expr.column(), 7);
    }

    #[test]
    fn test_binary_op_creation() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("+", left, right, 1, 5);
        assert!(matches!(expr, Expr::BinaryOp { .. }));
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            line,
            column,
        } = expr
        {
            assert_eq!(operator, "+");
            assert_eq!(line, 1);
            assert_eq!(column, 5);
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "5");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "3");
            }
        }
    }

    #[test]
    fn test_binary_op_line() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("+", left, right, 2, 5);
        assert_eq!(expr.line(), 2);
    }

    #[test]
    fn test_binary_op_column() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("+", left, right, 1, 10);
        assert_eq!(expr.column(), 10);
    }

    #[test]
    fn test_number_precedence() {
        let expr = Expr::number("5", 1, 1);
        assert_eq!(expr.precedence(), 0);
    }

    #[test]
    fn test_addition_precedence() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("+", left, right, 1, 5);
        assert_eq!(expr.precedence(), 1);
    }

    #[test]
    fn test_subtraction_precedence() {
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("-", left, right, 1, 5);
        assert_eq!(expr.precedence(), 1);
    }

    #[test]
    fn test_multiplication_precedence() {
        let left = Expr::number("4", 1, 1);
        let right = Expr::number("7", 1, 3);
        let expr = Expr::binary_op("*", left, right, 1, 5);
        assert_eq!(expr.precedence(), 2);
    }

    #[test]
    fn test_division_precedence() {
        let left = Expr::number("10", 1, 1);
        let right = Expr::number("2", 1, 4);
        let expr = Expr::binary_op("/", left, right, 1, 6);
        assert_eq!(expr.precedence(), 2);
    }
}

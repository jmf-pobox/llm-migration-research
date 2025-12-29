//! Abstract Syntax Tree (AST) node types.

/// A node in the abstract syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    /// A numeric literal.
    Number(Number),
    /// A binary operation (e.g., addition).
    BinaryOp(BinaryOp),
}

/// A numeric literal node.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Number;
///
/// let num = Number::new("42", 1, 1);
/// assert_eq!(num.value(), "42");
/// assert_eq!(num.line(), 1);
/// assert_eq!(num.column(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    #[must_use]
    pub fn new(value: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            value: value.into(),
            line,
            column,
        }
    }

    /// Returns the string value of this number.
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
/// # Examples
///
/// ```
/// use rpn2tex::ast::{BinaryOp, Number, ASTNode};
///
/// let left = ASTNode::Number(Number::new("5", 1, 1));
/// let right = ASTNode::Number(Number::new("3", 1, 3));
/// let binop = BinaryOp::new("+", left, right, 1, 5);
/// assert_eq!(binop.operator(), "+");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOp {
    operator: String,
    left: Box<ASTNode>,
    right: Box<ASTNode>,
    line: usize,
    column: usize,
}

impl BinaryOp {
    /// Creates a new binary operation node.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator string (e.g., "+")
    /// * `left` - The left operand
    /// * `right` - The right operand
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    #[must_use]
    pub fn new(
        operator: impl Into<String>,
        left: ASTNode,
        right: ASTNode,
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
    #[must_use]
    pub fn left(&self) -> &ASTNode {
        &self.left
    }

    /// Returns a reference to the right operand.
    #[must_use]
    pub fn right(&self) -> &ASTNode {
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

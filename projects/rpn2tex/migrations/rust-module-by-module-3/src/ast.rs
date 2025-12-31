//! Abstract Syntax Tree node types for parsed RPN expressions.
//!
//! This module defines the AST structure that represents the parsed RPN expression.
//! The design uses an enum-based approach where each variant carries position information
//! for error reporting.

/// Represents any expression node in the AST.
///
/// The AST is a tree of expressions where:
/// - `Number` represents numeric literals (leaf nodes)
/// - `BinaryOp` represents binary operations with two child expressions (branch nodes)
///
/// # Position Tracking
///
/// All variants include position information (line and column) for error reporting.
/// Position numbers are 1-based to match standard text editor conventions.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::ASTNode;
///
/// // A simple number node
/// let num = ASTNode::number("42", 1, 1);
/// assert_eq!(num.line(), 1);
/// assert_eq!(num.column(), 1);
///
/// // A binary operation: 5 + 3
/// let five = ASTNode::number("5", 1, 1);
/// let three = ASTNode::number("3", 1, 3);
/// let expr = ASTNode::binary_op("+", five, three, 1, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ASTNode {
    /// Numeric literal node.
    ///
    /// # Fields
    ///
    /// * `value` - The string representation of the number (e.g., "42", "3.14", "-5")
    /// * `line` - 1-based line number where the number appears
    /// * `column` - 1-based column number where the number starts
    Number {
        /// The string representation of the number
        value: String,
        /// 1-based line number
        line: u32,
        /// 1-based column number
        column: u32,
    },

    /// Binary operation node.
    ///
    /// # Fields
    ///
    /// * `operator` - The operator as a string ("+", "-", "*", or "/")
    /// * `left` - The left operand (boxed for heap allocation)
    /// * `right` - The right operand (boxed for heap allocation)
    /// * `line` - 1-based line number where the operator appears
    /// * `column` - 1-based column number where the operator starts
    BinaryOp {
        /// The operator string
        operator: String,
        /// Left operand (boxed for recursive types)
        left: Box<ASTNode>,
        /// Right operand (boxed for recursive types)
        right: Box<ASTNode>,
        /// 1-based line number
        line: u32,
        /// 1-based column number
        column: u32,
    },
}

impl ASTNode {
    /// Creates a new Number node.
    ///
    /// # Arguments
    ///
    /// * `value` - String representation of the number
    /// * `line` - 1-based line number
    /// * `column` - 1-based column number
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let num = ASTNode::number("42", 1, 1);
    /// if let ASTNode::Number { value, .. } = num {
    ///     assert_eq!(value, "42");
    /// }
    /// ```
    #[must_use]
    pub fn number(value: impl Into<String>, line: u32, column: u32) -> Self {
        Self::Number {
            value: value.into(),
            line,
            column,
        }
    }

    /// Creates a new BinaryOp node.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator string ("+", "-", "*", or "/")
    /// * `left` - The left operand node
    /// * `right` - The right operand node
    /// * `line` - 1-based line number
    /// * `column` - 1-based column number
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let left = ASTNode::number("5", 1, 1);
    /// let right = ASTNode::number("3", 1, 3);
    /// let expr = ASTNode::binary_op("+", left, right, 1, 5);
    /// ```
    #[must_use]
    pub fn binary_op(
        operator: impl Into<String>,
        left: ASTNode,
        right: ASTNode,
        line: u32,
        column: u32,
    ) -> Self {
        Self::BinaryOp {
            operator: operator.into(),
            left: Box::new(left),
            right: Box::new(right),
            line,
            column,
        }
    }

    /// Returns the line number of this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let num = ASTNode::number("42", 5, 10);
    /// assert_eq!(num.line(), 5);
    /// ```
    #[must_use]
    pub const fn line(&self) -> u32 {
        match self {
            Self::Number { line, .. } | Self::BinaryOp { line, .. } => *line,
        }
    }

    /// Returns the column number of this node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let num = ASTNode::number("42", 5, 10);
    /// assert_eq!(num.column(), 10);
    /// ```
    #[must_use]
    pub const fn column(&self) -> u32 {
        match self {
            Self::Number { column, .. } | Self::BinaryOp { column, .. } => *column,
        }
    }

    /// Returns the value if this is a Number node, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let num = ASTNode::number("42", 1, 1);
    /// assert_eq!(num.as_number(), Some("42"));
    ///
    /// let left = ASTNode::number("5", 1, 1);
    /// let right = ASTNode::number("3", 1, 3);
    /// let expr = ASTNode::binary_op("+", left, right, 1, 5);
    /// assert_eq!(expr.as_number(), None);
    /// ```
    #[must_use]
    pub fn as_number(&self) -> Option<&str> {
        match self {
            Self::Number { value, .. } => Some(value),
            Self::BinaryOp { .. } => None,
        }
    }

    /// Returns the operator if this is a BinaryOp node, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let left = ASTNode::number("5", 1, 1);
    /// let right = ASTNode::number("3", 1, 3);
    /// let expr = ASTNode::binary_op("+", left, right, 1, 5);
    /// assert_eq!(expr.as_operator(), Some("+"));
    ///
    /// let num = ASTNode::number("42", 1, 1);
    /// assert_eq!(num.as_operator(), None);
    /// ```
    #[must_use]
    pub fn as_operator(&self) -> Option<&str> {
        match self {
            Self::BinaryOp { operator, .. } => Some(operator),
            Self::Number { .. } => None,
        }
    }

    /// Returns a reference to the left operand if this is a BinaryOp node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let left = ASTNode::number("5", 1, 1);
    /// let right = ASTNode::number("3", 1, 3);
    /// let expr = ASTNode::binary_op("+", left.clone(), right, 1, 5);
    ///
    /// let left_ref = expr.left().unwrap();
    /// assert_eq!(left_ref.as_number(), Some("5"));
    /// ```
    #[must_use]
    pub const fn left(&self) -> Option<&ASTNode> {
        match self {
            Self::BinaryOp { left, .. } => Some(left),
            Self::Number { .. } => None,
        }
    }

    /// Returns a reference to the right operand if this is a BinaryOp node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    ///
    /// let left = ASTNode::number("5", 1, 1);
    /// let right = ASTNode::number("3", 1, 3);
    /// let expr = ASTNode::binary_op("+", left, right.clone(), 1, 5);
    ///
    /// let right_ref = expr.right().unwrap();
    /// assert_eq!(right_ref.as_number(), Some("3"));
    /// ```
    #[must_use]
    pub const fn right(&self) -> Option<&ASTNode> {
        match self {
            Self::BinaryOp { right, .. } => Some(right),
            Self::Number { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_node_creation() {
        let node = ASTNode::number("42", 1, 1);
        assert_eq!(node.line(), 1);
        assert_eq!(node.column(), 1);
        assert_eq!(node.as_number(), Some("42"));
        assert_eq!(node.as_operator(), None);
    }

    #[test]
    fn test_number_node_with_decimal() {
        let node = ASTNode::number("3.14", 2, 5);
        assert_eq!(node.line(), 2);
        assert_eq!(node.column(), 5);
        assert_eq!(node.as_number(), Some("3.14"));
    }

    #[test]
    fn test_number_node_with_negative() {
        let node = ASTNode::number("-42", 1, 1);
        assert_eq!(node.as_number(), Some("-42"));
    }

    #[test]
    fn test_binary_op_creation() {
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr = ASTNode::binary_op("+", left, right, 1, 5);

        assert_eq!(expr.line(), 1);
        assert_eq!(expr.column(), 5);
        assert_eq!(expr.as_operator(), Some("+"));
        assert_eq!(expr.as_number(), None);
    }

    #[test]
    fn test_binary_op_operands() {
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr = ASTNode::binary_op("+", left, right, 1, 5);

        let left_node = expr.left().unwrap();
        let right_node = expr.right().unwrap();

        assert_eq!(left_node.as_number(), Some("5"));
        assert_eq!(right_node.as_number(), Some("3"));
    }

    #[test]
    fn test_nested_binary_op() {
        // (5 + 3) * 2
        let five = ASTNode::number("5", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let sum = ASTNode::binary_op("+", five, three, 1, 5);
        let two = ASTNode::number("2", 1, 7);
        let product = ASTNode::binary_op("*", sum, two, 1, 9);

        assert_eq!(product.as_operator(), Some("*"));

        let left_child = product.left().unwrap();
        assert_eq!(left_child.as_operator(), Some("+"));

        let right_child = product.right().unwrap();
        assert_eq!(right_child.as_number(), Some("2"));
    }

    #[test]
    fn test_all_operators() {
        let operators = ["+", "-", "*", "/"];
        for op in operators {
            let left = ASTNode::number("5", 1, 1);
            let right = ASTNode::number("3", 1, 3);
            let expr = ASTNode::binary_op(op, left, right, 1, 5);
            assert_eq!(expr.as_operator(), Some(op));
        }
    }

    #[test]
    fn test_node_equality() {
        let node1 = ASTNode::number("42", 1, 1);
        let node2 = ASTNode::number("42", 1, 1);
        let node3 = ASTNode::number("43", 1, 1);

        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_binary_op_equality() {
        let left1 = ASTNode::number("5", 1, 1);
        let right1 = ASTNode::number("3", 1, 3);
        let expr1 = ASTNode::binary_op("+", left1, right1, 1, 5);

        let left2 = ASTNode::number("5", 1, 1);
        let right2 = ASTNode::number("3", 1, 3);
        let expr2 = ASTNode::binary_op("+", left2, right2, 1, 5);

        let left3 = ASTNode::number("5", 1, 1);
        let right3 = ASTNode::number("4", 1, 3);
        let expr3 = ASTNode::binary_op("+", left3, right3, 1, 5);

        assert_eq!(expr1, expr2);
        assert_ne!(expr1, expr3);
    }

    #[test]
    fn test_clone_number_node() {
        let node = ASTNode::number("42", 1, 1);
        let cloned = node.clone();
        assert_eq!(node, cloned);
    }

    #[test]
    fn test_clone_binary_op() {
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr = ASTNode::binary_op("+", left, right, 1, 5);
        let cloned = expr.clone();
        assert_eq!(expr, cloned);
    }

    #[test]
    fn test_position_tracking_in_complex_tree() {
        // 5 - 3 - 2 => (5 - 3) - 2
        let five = ASTNode::number("5", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let first_sub = ASTNode::binary_op("-", five, three, 1, 5);
        let two = ASTNode::number("2", 1, 7);
        let second_sub = ASTNode::binary_op("-", first_sub, two, 1, 9);

        assert_eq!(second_sub.line(), 1);
        assert_eq!(second_sub.column(), 9);

        let left_child = second_sub.left().unwrap();
        assert_eq!(left_child.line(), 1);
        assert_eq!(left_child.column(), 5);
    }

    #[test]
    fn test_number_node_returns_none_for_left_right() {
        let num = ASTNode::number("42", 1, 1);
        assert!(num.left().is_none());
        assert!(num.right().is_none());
    }

    #[test]
    fn test_string_conversion_in_constructor() {
        // Test that impl Into<String> works with both &str and String
        let node1 = ASTNode::number("42", 1, 1);
        let node2 = ASTNode::number(String::from("42"), 1, 1);
        assert_eq!(node1, node2);

        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr1 = ASTNode::binary_op("+", left.clone(), right.clone(), 1, 5);
        let expr2 = ASTNode::binary_op(String::from("+"), left, right, 1, 5);
        assert_eq!(expr1, expr2);
    }

    #[test]
    fn test_multi_level_nesting() {
        // ((1 + 2) + 3) + 4
        let one = ASTNode::number("1", 1, 1);
        let two = ASTNode::number("2", 1, 3);
        let add1 = ASTNode::binary_op("+", one, two, 1, 5);

        let three = ASTNode::number("3", 1, 7);
        let add2 = ASTNode::binary_op("+", add1, three, 1, 9);

        let four = ASTNode::number("4", 1, 11);
        let add3 = ASTNode::binary_op("+", add2, four, 1, 13);

        assert_eq!(add3.as_operator(), Some("+"));

        let left_level2 = add3.left().unwrap();
        assert_eq!(left_level2.as_operator(), Some("+"));

        let left_level1 = left_level2.left().unwrap();
        assert_eq!(left_level1.as_operator(), Some("+"));
    }

    #[test]
    fn test_floating_point_numbers() {
        let nodes = [
            ASTNode::number("3.14", 1, 1),
            ASTNode::number("1.5", 1, 5),
            ASTNode::number("0.5", 1, 9),
        ];

        assert_eq!(nodes[0].as_number(), Some("3.14"));
        assert_eq!(nodes[1].as_number(), Some("1.5"));
        assert_eq!(nodes[2].as_number(), Some("0.5"));
    }
}

//! Abstract Syntax Tree node types for mathematical expressions.
//!
//! This module defines the core AST structures used to represent parsed
//! mathematical expressions in RPN (Reverse Polish Notation) format.

/// Represents an operator in a binary operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    /// Addition operator (+)
    Add,
    /// Subtraction operator (-)
    Subtract,
    /// Multiplication operator (*)
    Multiply,
    /// Division operator (/)
    Divide,
    /// Exponentiation operator (^)
    Power,
}

impl Operator {
    /// Returns the precedence level of the operator.
    ///
    /// Higher values indicate higher precedence. Multiplication and division
    /// have precedence 2, while addition and subtraction have precedence 1.
    /// Power has the highest precedence of 3.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Operator;
    ///
    /// assert_eq!(Operator::Multiply.precedence(), 2);
    /// assert_eq!(Operator::Add.precedence(), 1);
    /// assert_eq!(Operator::Power.precedence(), 3);
    /// ```
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::Add | Self::Subtract => 1,
            Self::Multiply | Self::Divide => 2,
            Self::Power => 3,
        }
    }

    /// Returns the LaTeX representation of the operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Operator;
    ///
    /// assert_eq!(Operator::Add.to_latex(), "+");
    /// assert_eq!(Operator::Multiply.to_latex(), "\\times");
    /// assert_eq!(Operator::Divide.to_latex(), "\\div");
    /// ```
    #[must_use]
    pub const fn to_latex(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "\\times",
            Self::Divide => "\\div",
            Self::Power => "^",
        }
    }
}

/// Represents a node in the Abstract Syntax Tree.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// A numeric literal value
    Number(f64),
    /// A binary operation with left operand, operator, and right operand
    BinaryOp {
        /// Left operand
        left: Box<AstNode>,
        /// The operator
        operator: Operator,
        /// Right operand
        right: Box<AstNode>,
    },
}

impl AstNode {
    /// Creates a new number node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::AstNode;
    ///
    /// let node = AstNode::number(42.0);
    /// assert!(matches!(node, AstNode::Number(x) if x == 42.0));
    /// ```
    #[must_use]
    pub const fn number(value: f64) -> Self {
        Self::Number(value)
    }

    /// Creates a new binary operation node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{AstNode, Operator};
    ///
    /// let left = AstNode::number(5.0);
    /// let right = AstNode::number(3.0);
    /// let node = AstNode::binary_op(left, Operator::Add, right);
    /// ```
    #[must_use]
    pub fn binary_op(left: Self, operator: Operator, right: Self) -> Self {
        Self::BinaryOp {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    /// Returns true if this node is a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::AstNode;
    ///
    /// let node = AstNode::number(42.0);
    /// assert!(node.is_number());
    /// ```
    #[must_use]
    pub const fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Returns true if this node is a binary operation.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{AstNode, Operator};
    ///
    /// let node = AstNode::binary_op(
    ///     AstNode::number(5.0),
    ///     Operator::Add,
    ///     AstNode::number(3.0),
    /// );
    /// assert!(node.is_binary_op());
    /// ```
    #[must_use]
    pub const fn is_binary_op(&self) -> bool {
        matches!(self, Self::BinaryOp { .. })
    }

    /// Returns the operator if this is a binary operation node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{AstNode, Operator};
    ///
    /// let node = AstNode::binary_op(
    ///     AstNode::number(5.0),
    ///     Operator::Multiply,
    ///     AstNode::number(3.0),
    /// );
    /// assert_eq!(node.operator(), Some(Operator::Multiply));
    /// ```
    #[must_use]
    pub const fn operator(&self) -> Option<Operator> {
        match self {
            Self::BinaryOp { operator, .. } => Some(*operator),
            Self::Number(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_precedence() {
        assert_eq!(Operator::Add.precedence(), 1);
        assert_eq!(Operator::Subtract.precedence(), 1);
        assert_eq!(Operator::Multiply.precedence(), 2);
        assert_eq!(Operator::Divide.precedence(), 2);
        assert_eq!(Operator::Power.precedence(), 3);
    }

    #[test]
    fn test_operator_precedence_ordering() {
        assert!(Operator::Multiply.precedence() > Operator::Add.precedence());
        assert!(Operator::Divide.precedence() > Operator::Subtract.precedence());
        assert!(Operator::Power.precedence() > Operator::Multiply.precedence());
    }

    #[test]
    fn test_operator_to_latex() {
        assert_eq!(Operator::Add.to_latex(), "+");
        assert_eq!(Operator::Subtract.to_latex(), "-");
        assert_eq!(Operator::Multiply.to_latex(), "\\times");
        assert_eq!(Operator::Divide.to_latex(), "\\div");
        assert_eq!(Operator::Power.to_latex(), "^");
    }

    #[test]
    fn test_ast_node_number() {
        let node = AstNode::number(42.0);
        assert!(matches!(node, AstNode::Number(x) if x == 42.0));
        assert!(node.is_number());
        assert!(!node.is_binary_op());
        assert_eq!(node.operator(), None);
    }

    #[test]
    fn test_ast_node_binary_op() {
        let left = AstNode::number(5.0);
        let right = AstNode::number(3.0);
        let node = AstNode::binary_op(left, Operator::Add, right);

        assert!(node.is_binary_op());
        assert!(!node.is_number());
        assert_eq!(node.operator(), Some(Operator::Add));
    }

    #[test]
    fn test_ast_node_nested_binary_op() {
        // (5 + 3) * 2
        let inner = AstNode::binary_op(AstNode::number(5.0), Operator::Add, AstNode::number(3.0));
        let outer = AstNode::binary_op(inner, Operator::Multiply, AstNode::number(2.0));

        assert!(outer.is_binary_op());
        assert_eq!(outer.operator(), Some(Operator::Multiply));
    }

    #[test]
    fn test_ast_node_equality() {
        let node1 = AstNode::number(42.0);
        let node2 = AstNode::number(42.0);
        let node3 = AstNode::number(43.0);

        assert_eq!(node1, node2);
        assert_ne!(node1, node3);
    }

    #[test]
    fn test_ast_node_clone() {
        let node = AstNode::binary_op(
            AstNode::number(5.0),
            Operator::Multiply,
            AstNode::number(3.0),
        );
        let cloned = node.clone();

        assert_eq!(node, cloned);
    }

    #[test]
    fn test_complex_expression() {
        // ((2 + 3) * 4) - 5
        let add = AstNode::binary_op(AstNode::number(2.0), Operator::Add, AstNode::number(3.0));
        let mult = AstNode::binary_op(add, Operator::Multiply, AstNode::number(4.0));
        let sub = AstNode::binary_op(mult, Operator::Subtract, AstNode::number(5.0));

        assert!(sub.is_binary_op());
        assert_eq!(sub.operator(), Some(Operator::Subtract));
    }

    #[test]
    fn test_floating_point_numbers() {
        let node = AstNode::number(3.14159);
        assert!(matches!(node, AstNode::Number(x) if (x - 3.14159).abs() < 1e-10));
    }

    #[test]
    fn test_negative_numbers() {
        let node = AstNode::number(-42.0);
        assert!(matches!(node, AstNode::Number(x) if x == -42.0));
    }
}

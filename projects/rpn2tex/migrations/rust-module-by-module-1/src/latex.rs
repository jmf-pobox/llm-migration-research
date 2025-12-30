//! LaTeX generation from Abstract Syntax Trees.
//!
//! This module converts AST nodes into LaTeX mathematical notation,
//! handling operator precedence and automatic parenthesization.

use crate::ast::{AstNode, Operator};

/// Generates LaTeX output from an Abstract Syntax Tree.
///
/// The `LatexGenerator` traverses an AST and produces properly formatted
/// LaTeX output with appropriate parenthesization based on operator precedence.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{AstNode, Operator};
/// use rpn2tex::latex::LatexGenerator;
///
/// let ast = AstNode::binary_op(
///     AstNode::number(5.0),
///     Operator::Add,
///     AstNode::number(3.0),
/// );
/// let generator = LatexGenerator::new();
/// let latex = generator.generate(&ast);
/// assert_eq!(latex, "$5 + 3$");
/// ```
#[derive(Debug)]
pub struct LatexGenerator;

impl LatexGenerator {
    /// Creates a new LaTeX generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LatexGenerator;
    ///
    /// let generator = LatexGenerator::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generates LaTeX output from an AST.
    ///
    /// The output is wrapped in dollar signs (`$...$`) and uses proper LaTeX
    /// operators with correct spacing. Parentheses are added automatically
    /// based on operator precedence.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{AstNode, Operator};
    /// use rpn2tex::latex::LatexGenerator;
    ///
    /// // Simple addition: 5 + 3
    /// let ast = AstNode::binary_op(
    ///     AstNode::number(5.0),
    ///     Operator::Add,
    ///     AstNode::number(3.0),
    /// );
    /// let generator = LatexGenerator::new();
    /// assert_eq!(generator.generate(&ast), "$5 + 3$");
    ///
    /// // With precedence: (5 + 3) * 2
    /// let inner = AstNode::binary_op(
    ///     AstNode::number(5.0),
    ///     Operator::Add,
    ///     AstNode::number(3.0),
    /// );
    /// let outer = AstNode::binary_op(inner, Operator::Multiply, AstNode::number(2.0));
    /// assert_eq!(generator.generate(&outer), "$( 5 + 3 ) \\times 2$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &AstNode) -> String {
        let inner = self.visit(ast);
        format!("${}$", inner)
    }

    /// Visits a node and generates its LaTeX representation.
    fn visit(&self, node: &AstNode) -> String {
        match node {
            AstNode::Number(value) => self.visit_number(*value),
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => self.visit_binary_op(left, *operator, right),
        }
    }

    /// Visits a number node and returns its string representation.
    fn visit_number(&self, value: f64) -> String {
        // Format the number, removing unnecessary trailing zeros and decimal point
        let formatted = format!("{}", value);

        // If it's a whole number, ensure no decimal point
        if value.fract() == 0.0 && !formatted.contains('e') {
            format!("{}", value as i64)
        } else {
            formatted
        }
    }

    /// Visits a binary operation node and generates LaTeX with proper precedence.
    fn visit_binary_op(&self, left: &AstNode, operator: Operator, right: &AstNode) -> String {
        let op_latex = operator.to_latex();
        let parent_precedence = operator.precedence();

        // Generate left operand with parentheses if needed
        let left_text = self.visit(left);
        let left_text = if self.needs_parens(left, parent_precedence, false) {
            format!("( {} )", left_text)
        } else {
            left_text
        };

        // Generate right operand with parentheses if needed
        let right_text = self.visit(right);
        let right_text = if self.needs_parens(right, parent_precedence, true) {
            format!("( {} )", right_text)
        } else {
            right_text
        };

        format!("{} {} {}", left_text, op_latex, right_text)
    }

    /// Determines if a child node needs parentheses.
    ///
    /// Parentheses are needed when:
    /// - The child is a binary operation with lower precedence than the parent
    /// - The child is on the right side with equal precedence and non-commutative operator
    fn needs_parens(&self, child: &AstNode, parent_precedence: u8, is_right: bool) -> bool {
        match child {
            AstNode::Number(_) => false,
            AstNode::BinaryOp {
                operator: child_op, ..
            } => {
                let child_precedence = child_op.precedence();

                // Lower precedence always needs parentheses
                if child_precedence < parent_precedence {
                    return true;
                }

                // Equal precedence on right side: check for non-commutative operators
                // This handles left-associativity for subtraction and division
                if child_precedence == parent_precedence && is_right {
                    // For right associative operators (like Power), we need special handling
                    // But for now, we keep the rule simple for left-associative operators
                    matches!(*child_op, Operator::Subtract | Operator::Divide)
                } else {
                    false
                }
            }
        }
    }
}

impl Default for LatexGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_simple_number() {
        let generator = LatexGenerator::new();
        let ast = AstNode::number(5.0);
        assert_eq!(generator.generate(&ast), "$5$");
    }

    #[test]
    fn test_generate_floating_point_number() {
        let generator = LatexGenerator::new();
        let ast = AstNode::number(3.14);
        assert_eq!(generator.generate(&ast), "$3.14$");
    }

    #[test]
    fn test_generate_simple_addition() {
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(AstNode::number(5.0), Operator::Add, AstNode::number(3.0));
        assert_eq!(generator.generate(&ast), "$5 + 3$");
    }

    #[test]
    fn test_generate_simple_subtraction() {
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(
            AstNode::number(5.0),
            Operator::Subtract,
            AstNode::number(3.0),
        );
        assert_eq!(generator.generate(&ast), "$5 - 3$");
    }

    #[test]
    fn test_generate_simple_multiplication() {
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(
            AstNode::number(4.0),
            Operator::Multiply,
            AstNode::number(7.0),
        );
        assert_eq!(generator.generate(&ast), "$4 \\times 7$");
    }

    #[test]
    fn test_generate_simple_division() {
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(
            AstNode::number(10.0),
            Operator::Divide,
            AstNode::number(2.0),
        );
        assert_eq!(generator.generate(&ast), "$10 \\div 2$");
    }

    #[test]
    fn test_precedence_addition_wrapped_by_multiplication() {
        // (5 + 3) * 2
        let generator = LatexGenerator::new();
        let inner = AstNode::binary_op(AstNode::number(5.0), Operator::Add, AstNode::number(3.0));
        let outer = AstNode::binary_op(inner, Operator::Multiply, AstNode::number(2.0));
        assert_eq!(generator.generate(&outer), "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_precedence_no_parens_needed() {
        // 5 * 3 + 2
        let generator = LatexGenerator::new();
        let inner = AstNode::binary_op(
            AstNode::number(5.0),
            Operator::Multiply,
            AstNode::number(3.0),
        );
        let outer = AstNode::binary_op(inner, Operator::Add, AstNode::number(2.0));
        assert_eq!(generator.generate(&outer), "$5 \\times 3 + 2$");
    }

    #[test]
    fn test_precedence_multiplication_higher_than_addition() {
        // 2 + 3 * 4
        let generator = LatexGenerator::new();
        let mult = AstNode::binary_op(
            AstNode::number(3.0),
            Operator::Multiply,
            AstNode::number(4.0),
        );
        let add = AstNode::binary_op(AstNode::number(2.0), Operator::Add, mult);
        assert_eq!(generator.generate(&add), "$2 + 3 \\times 4$");
    }

    #[test]
    fn test_precedence_right_operand_needs_parens() {
        // 2 * (3 + 4)
        let generator = LatexGenerator::new();
        let add = AstNode::binary_op(AstNode::number(3.0), Operator::Add, AstNode::number(4.0));
        let mult = AstNode::binary_op(AstNode::number(2.0), Operator::Multiply, add);
        assert_eq!(generator.generate(&mult), "$2 \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_left_associativity_subtraction() {
        // 5 - 3 - 2
        let generator = LatexGenerator::new();
        let inner = AstNode::binary_op(
            AstNode::number(5.0),
            Operator::Subtract,
            AstNode::number(3.0),
        );
        let outer = AstNode::binary_op(inner, Operator::Subtract, AstNode::number(2.0));
        assert_eq!(generator.generate(&outer), "$5 - 3 - 2$");
    }

    #[test]
    fn test_left_associativity_division() {
        // 100 / 10 / 5 / 2
        let generator = LatexGenerator::new();
        let div1 = AstNode::binary_op(
            AstNode::number(100.0),
            Operator::Divide,
            AstNode::number(10.0),
        );
        let div2 = AstNode::binary_op(div1, Operator::Divide, AstNode::number(5.0));
        let div3 = AstNode::binary_op(div2, Operator::Divide, AstNode::number(2.0));
        assert_eq!(generator.generate(&div3), "$100 \\div 10 \\div 5 \\div 2$");
    }

    #[test]
    fn test_multiple_additions() {
        // 1 + 2 + 3 + 4
        let generator = LatexGenerator::new();
        let add1 = AstNode::binary_op(AstNode::number(1.0), Operator::Add, AstNode::number(2.0));
        let add2 = AstNode::binary_op(add1, Operator::Add, AstNode::number(3.0));
        let add3 = AstNode::binary_op(add2, Operator::Add, AstNode::number(4.0));
        assert_eq!(generator.generate(&add3), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_both_operands_need_parens() {
        // (1 + 2) * (3 + 4)
        let generator = LatexGenerator::new();
        let left = AstNode::binary_op(AstNode::number(1.0), Operator::Add, AstNode::number(2.0));
        let right = AstNode::binary_op(AstNode::number(3.0), Operator::Add, AstNode::number(4.0));
        let mult = AstNode::binary_op(left, Operator::Multiply, right);
        assert_eq!(generator.generate(&mult), "$( 1 + 2 ) \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_complex_precedence() {
        // (10 / 2 + 3) * 4
        let generator = LatexGenerator::new();
        let div = AstNode::binary_op(
            AstNode::number(10.0),
            Operator::Divide,
            AstNode::number(2.0),
        );
        let add = AstNode::binary_op(div, Operator::Add, AstNode::number(3.0));
        let mult = AstNode::binary_op(add, Operator::Multiply, AstNode::number(4.0));
        assert_eq!(generator.generate(&mult), "$( 10 \\div 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_division_and_multiplication_chain() {
        // 10 / 2 * 5
        let generator = LatexGenerator::new();
        let div = AstNode::binary_op(
            AstNode::number(10.0),
            Operator::Divide,
            AstNode::number(2.0),
        );
        let mult = AstNode::binary_op(div, Operator::Multiply, AstNode::number(5.0));
        assert_eq!(generator.generate(&mult), "$10 \\div 2 \\times 5$");
    }

    #[test]
    fn test_floating_point_in_expression() {
        // 3.14 * 2
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(
            AstNode::number(3.14),
            Operator::Multiply,
            AstNode::number(2.0),
        );
        assert_eq!(generator.generate(&ast), "$3.14 \\times 2$");
    }

    #[test]
    fn test_floating_point_addition() {
        // 1.5 + 0.5
        let generator = LatexGenerator::new();
        let ast = AstNode::binary_op(AstNode::number(1.5), Operator::Add, AstNode::number(0.5));
        assert_eq!(generator.generate(&ast), "$1.5 + 0.5$");
    }

    #[test]
    fn test_multiplication_then_addition() {
        // 2 * 3 + 4
        let generator = LatexGenerator::new();
        let mult = AstNode::binary_op(
            AstNode::number(2.0),
            Operator::Multiply,
            AstNode::number(3.0),
        );
        let add = AstNode::binary_op(mult, Operator::Add, AstNode::number(4.0));
        assert_eq!(generator.generate(&add), "$2 \\times 3 + 4$");
    }

    #[test]
    fn test_addition_then_multiplication() {
        // (2 + 3) * 4
        let generator = LatexGenerator::new();
        let add = AstNode::binary_op(AstNode::number(2.0), Operator::Add, AstNode::number(3.0));
        let mult = AstNode::binary_op(add, Operator::Multiply, AstNode::number(4.0));
        assert_eq!(generator.generate(&mult), "$( 2 + 3 ) \\times 4$");
    }
}

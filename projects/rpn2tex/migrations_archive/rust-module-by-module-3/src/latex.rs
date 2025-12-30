//! LaTeX code generation from AST.
//!
//! This module converts abstract syntax trees (AST) to LaTeX mathematical expressions
//! with proper operator precedence handling and parenthesization.
//!
//! The generator uses a visitor pattern to traverse the AST and produce LaTeX output
//! wrapped in `$...$` delimiters. It handles:
//! - Operator precedence (multiplication/division before addition/subtraction)
//! - Left-associativity for non-commutative operators (-, /)
//! - Proper spacing around operators and parentheses
//! - LaTeX-specific operator symbols (\times, \div)
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ast::Expr;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! // Simple addition: "5 3 +" → "$5 + 3$"
//! let ast = Expr::binary_op(
//!     "+",
//!     Expr::number("5", 1, 1),
//!     Expr::number("3", 1, 3),
//!     1, 2
//! );
//! let generator = LaTeXGenerator::new();
//! assert_eq!(generator.generate(&ast), "$5 + 3$");
//!
//! // With parentheses: "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
//! let add = Expr::binary_op(
//!     "+",
//!     Expr::number("5", 1, 1),
//!     Expr::number("3", 1, 3),
//!     1, 2
//! );
//! let mult = Expr::binary_op("*", add, Expr::number("2", 1, 5), 1, 4);
//! assert_eq!(generator.generate(&mult), "$( 5 + 3 ) \\times 2$");
//! ```

use crate::ast::Expr;

/// Generator for converting AST to LaTeX format.
///
/// The generator is stateless and can be reused for multiple conversions.
/// It implements a visitor pattern to traverse the AST and produce properly
/// formatted LaTeX output with correct operator precedence and parenthesization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    /// Creates a new LaTeX generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let generator = LaTeXGenerator::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Generates LaTeX code from an AST.
    ///
    /// The output is wrapped in `$...$` delimiters and includes proper spacing
    /// and parenthesization according to operator precedence rules.
    ///
    /// # Arguments
    ///
    /// * `ast` - The root expression node to convert
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let ast = Expr::binary_op(
    ///     "+",
    ///     Expr::number("5", 1, 1),
    ///     Expr::number("3", 1, 3),
    ///     1, 2
    /// );
    /// let generator = LaTeXGenerator::new();
    /// assert_eq!(generator.generate(&ast), "$5 + 3$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${}$", content)
    }

    /// Visits an AST node and generates LaTeX code.
    ///
    /// This is the main visitor method that dispatches to specific handlers
    /// based on the node type.
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp { .. } => self.visit_binary_op(node),
        }
    }

    /// Visits a binary operation node and generates LaTeX code.
    ///
    /// Handles operator conversion, precedence-based parenthesization,
    /// and proper spacing.
    fn visit_binary_op(&self, node: &Expr) -> String {
        match node {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let op_latex = self.get_latex_operator(operator);
                let my_precedence = self.get_precedence(operator);

                // Generate left operand with parentheses if needed
                let mut left_str = self.visit(left);
                if self.needs_parens(left, my_precedence, false) {
                    left_str = format!("( {} )", left_str);
                }

                // Generate right operand with parentheses if needed
                let mut right_str = self.visit(right);
                if self.needs_parens(right, my_precedence, true) {
                    right_str = format!("( {} )", right_str);
                }

                format!("{} {} {}", left_str, op_latex, right_str)
            }
            _ => unreachable!("visit_binary_op called on non-BinaryOp node"),
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are added when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence, is right operand, and parent is non-commutative (-, /)
    ///
    /// # Arguments
    ///
    /// * `child` - The child expression to check
    /// * `parent_precedence` - Precedence level of the parent operator
    /// * `is_right` - Whether the child is the right operand
    fn needs_parens(&self, child: &Expr, parent_precedence: u8, is_right: bool) -> bool {
        match child {
            Expr::Number { .. } => false,
            Expr::BinaryOp { operator, .. } => {
                let child_prec = self.get_precedence(operator);

                // Lower precedence always needs parens
                if child_prec < parent_precedence {
                    return true;
                }

                // Equal precedence on right side for non-commutative ops
                child_prec == parent_precedence && is_right && (operator == "-" || operator == "/")
            }
        }
    }

    /// Maps an operator to its LaTeX representation.
    ///
    /// # Arguments
    ///
    /// * `op` - The operator symbol ("+", "-", "*", "/")
    fn get_latex_operator(&self, op: &str) -> String {
        match op {
            "+" => "+".to_string(),
            "-" => "-".to_string(),
            "*" => r"\times".to_string(),
            "/" => r"\div".to_string(),
            _ => op.to_string(),
        }
    }

    /// Returns the precedence level for an operator.
    ///
    /// Higher numbers indicate higher precedence.
    /// - Addition and subtraction: 1
    /// - Multiplication and division: 2
    ///
    /// # Arguments
    ///
    /// * `op` - The operator symbol ("+", "-", "*", "/")
    fn get_precedence(&self, op: &str) -> u8 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }
}

impl Default for LaTeXGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        let ast = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&ast), "$5 + 3$");
    }

    #[test]
    fn test_simple_multiplication() {
        let ast = Expr::binary_op("*", Expr::number("4", 1, 1), Expr::number("7", 1, 3), 1, 2);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&ast), r"$4 \times 7$");
    }

    #[test]
    fn test_simple_division() {
        let ast = Expr::binary_op("/", Expr::number("10", 1, 1), Expr::number("2", 1, 4), 1, 3);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&ast), r"$10 \div 2$");
    }

    #[test]
    fn test_addition_times_number() {
        // "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
        let add = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        let mult = Expr::binary_op("*", add, Expr::number("2", 1, 5), 1, 4);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&mult), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_multiplication_plus_number() {
        // "5 3 * 2 +" → "$5 \times 3 + 2$"
        let mult = Expr::binary_op("*", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        let add = Expr::binary_op("+", mult, Expr::number("2", 1, 5), 1, 4);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&add), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_left_associative_subtraction() {
        // "5 3 - 2 -" → "$5 - 3 - 2$"
        let sub1 = Expr::binary_op("-", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        let sub2 = Expr::binary_op("-", sub1, Expr::number("2", 1, 5), 1, 4);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&sub2), "$5 - 3 - 2$");
    }

    #[test]
    fn test_left_associative_division() {
        // "100 10 / 5 / 2 /" → "$100 \div 10 \div 5 \div 2$"
        let div1 = Expr::binary_op(
            "/",
            Expr::number("100", 1, 1),
            Expr::number("10", 1, 5),
            1,
            4,
        );
        let div2 = Expr::binary_op("/", div1, Expr::number("5", 1, 8), 1, 7);
        let div3 = Expr::binary_op("/", div2, Expr::number("2", 1, 11), 1, 10);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&div3), r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_both_operands_need_parens() {
        // "1 2 + 3 4 + *" → "$( 1 + 2 ) \times ( 3 + 4 )$"
        let add1 = Expr::binary_op("+", Expr::number("1", 1, 1), Expr::number("2", 1, 3), 1, 2);
        let add2 = Expr::binary_op("+", Expr::number("3", 1, 5), Expr::number("4", 1, 7), 1, 6);
        let mult = Expr::binary_op("*", add1, add2, 1, 8);
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&mult), r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_decimal_number() {
        // "3.14 2 *" → "$3.14 \times 2$"
        let mult = Expr::binary_op(
            "*",
            Expr::number("3.14", 1, 1),
            Expr::number("2", 1, 6),
            1,
            5,
        );
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.generate(&mult), r"$3.14 \times 2$");
    }

    #[test]
    fn test_generator_reuse() {
        let generator = LaTeXGenerator::new();
        let ast1 = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        let ast2 = Expr::binary_op("*", Expr::number("4", 1, 1), Expr::number("7", 1, 3), 1, 2);

        assert_eq!(generator.generate(&ast1), "$5 + 3$");
        assert_eq!(generator.generate(&ast2), r"$4 \times 7$");
    }

    #[test]
    fn test_default_trait() {
        let generator1 = LaTeXGenerator::new();
        let generator2 = LaTeXGenerator::default();
        assert_eq!(generator1, generator2);
    }

    #[test]
    fn test_precedence_values() {
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.get_precedence("+"), 1);
        assert_eq!(generator.get_precedence("-"), 1);
        assert_eq!(generator.get_precedence("*"), 2);
        assert_eq!(generator.get_precedence("/"), 2);
    }

    #[test]
    fn test_latex_operators() {
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.get_latex_operator("+"), "+");
        assert_eq!(generator.get_latex_operator("-"), "-");
        assert_eq!(generator.get_latex_operator("*"), r"\times");
        assert_eq!(generator.get_latex_operator("/"), r"\div");
    }

    #[test]
    fn test_needs_parens_number() {
        let generator = LaTeXGenerator::new();
        let num = Expr::number("5", 1, 1);
        assert!(!generator.needs_parens(&num, 2, false));
        assert!(!generator.needs_parens(&num, 2, true));
    }

    #[test]
    fn test_needs_parens_lower_precedence() {
        let generator = LaTeXGenerator::new();
        let add = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        assert!(generator.needs_parens(&add, 2, false)); // Left side
        assert!(generator.needs_parens(&add, 2, true)); // Right side
    }

    #[test]
    fn test_needs_parens_equal_precedence_commutative() {
        let generator = LaTeXGenerator::new();
        let add = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        assert!(!generator.needs_parens(&add, 1, false)); // Left: no parens
        assert!(!generator.needs_parens(&add, 1, true)); // Right: no parens (+ is commutative)
    }

    #[test]
    fn test_needs_parens_equal_precedence_non_commutative() {
        let generator = LaTeXGenerator::new();
        let sub = Expr::binary_op("-", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        assert!(!generator.needs_parens(&sub, 1, false)); // Left: no parens
        assert!(generator.needs_parens(&sub, 1, true)); // Right: needs parens (- is non-commutative)

        let div = Expr::binary_op("/", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
        assert!(!generator.needs_parens(&div, 2, false)); // Left: no parens
        assert!(generator.needs_parens(&div, 2, true)); // Right: needs parens (/ is non-commutative)
    }

    #[test]
    fn test_clone_and_equality() {
        let gen1 = LaTeXGenerator::new();
        let gen2 = gen1.clone();
        assert_eq!(gen1, gen2);
    }
}

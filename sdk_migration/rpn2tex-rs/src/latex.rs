//! LaTeX code generation from AST.
//!
//! This module provides functionality to convert an abstract syntax tree (AST)
//! into LaTeX mathematical notation with proper operator precedence and parenthesization.

use crate::ast::{BinaryOp, Expr, Number};

/// LaTeX code generator.
///
/// Converts an AST expression tree into LaTeX notation with proper operator
/// precedence handling and parenthesization rules.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Expr, Number, BinaryOp, LaTeXGenerator};
///
/// let generator = LaTeXGenerator::new();
/// let num = Expr::Number(Number::new("42", 1, 1));
/// let latex = generator.generate(&num);
/// assert_eq!(latex, "$42$");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    /// Creates a new LaTeX generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::LaTeXGenerator;
    ///
    /// let generator = LaTeXGenerator::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Generates LaTeX code from an AST expression.
    ///
    /// Returns a LaTeX string wrapped in `$...$` delimiters.
    ///
    /// # Arguments
    ///
    /// * `ast` - The root expression of the AST to convert
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Expr, Number, BinaryOp, LaTeXGenerator};
    ///
    /// let generator = LaTeXGenerator::new();
    ///
    /// // Simple number
    /// let num = Expr::Number(Number::new("42", 1, 1));
    /// assert_eq!(generator.generate(&num), "$42$");
    ///
    /// // Binary operation: 3 + 4
    /// let left = Expr::Number(Number::new("3", 1, 1));
    /// let right = Expr::Number(Number::new("4", 1, 3));
    /// let add = Expr::BinaryOp(BinaryOp::new("+", left, right, 1, 5));
    /// assert_eq!(generator.generate(&add), "$3 + 4$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${content}$")
    }

    /// Visits an expression node and generates its LaTeX representation.
    ///
    /// This is the main recursive visitor method that handles different expression types.
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number(n) => Self::visit_number(n),
            Expr::BinaryOp(b) => self.visit_binary_op(b),
        }
    }

    /// Visits a number node and returns its string value.
    fn visit_number(node: &Number) -> String {
        node.value.clone()
    }

    /// Visits a binary operation node and generates LaTeX with proper parenthesization.
    ///
    /// Handles operator precedence and associativity rules to determine when
    /// parentheses are needed around operands.
    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        // Get LaTeX representation of operator
        let op_latex = Self::operator_to_latex(&node.operator);

        // Get precedence of this operation
        let my_precedence = Self::operator_precedence(&node.operator);

        // Visit left operand
        let mut left = self.visit(&node.left);
        if Self::needs_parens(&node.left, my_precedence, false) {
            left = format!("( {left} )");
        }

        // Visit right operand
        let mut right = self.visit(&node.right);
        if Self::needs_parens(&node.right, my_precedence, true) {
            right = format!("( {right} )");
        }

        format!("{left} {op_latex} {right}")
    }

    /// Converts an operator symbol to its LaTeX representation.
    fn operator_to_latex(operator: &str) -> String {
        match operator {
            "+" => "+".to_string(),
            "-" => "-".to_string(),
            "*" => r"\times".to_string(),
            "/" => r"\div".to_string(),
            _ => operator.to_string(),
        }
    }

    /// Returns the precedence level of an operator.
    ///
    /// Higher numbers indicate higher precedence.
    /// Addition and subtraction have precedence 1.
    /// Multiplication and division have precedence 2.
    fn operator_precedence(operator: &str) -> i32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// # Arguments
    ///
    /// * `child` - The child expression to check
    /// * `parent_precedence` - The precedence of the parent operator
    /// * `is_right` - Whether this is the right operand (affects associativity)
    ///
    /// # Returns
    ///
    /// `true` if parentheses are needed, `false` otherwise.
    ///
    /// # Parenthesization Rules
    ///
    /// 1. Numbers never need parentheses
    /// 2. Lower precedence operators need parentheses (e.g., `(a + b) * c`)
    /// 3. Same precedence operations:
    ///    - Left operands always need parentheses when they are binary ops
    ///    - Right operands of non-commutative operators need parentheses
    ///    - Right operands of commutative operators don't need parentheses
    fn needs_parens(child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        // Numbers never need parens
        if let Expr::BinaryOp(child_op) = child {
            let child_precedence = Self::operator_precedence(&child_op.operator);

            // Lower precedence always needs parens
            if child_precedence < parent_precedence {
                return true;
            }

            // Same precedence
            if child_precedence == parent_precedence {
                // Left operands always need parens to show explicit left-associativity
                if !is_right {
                    return true;
                }

                // Right operands: only non-commutative operators need parens
                if matches!(child_op.operator.as_str(), "-" | "/") {
                    return true;
                }
            }

            false
        } else {
            false
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

    fn make_number(value: &str) -> Expr {
        Expr::Number(Number::new(value, 1, 1))
    }

    fn make_binop(op: &str, left: Expr, right: Expr) -> Expr {
        Expr::BinaryOp(BinaryOp::new(op, left, right, 1, 1))
    }

    #[test]
    fn test_generator_creation() {
        let generator = LaTeXGenerator::new();
        assert_eq!(generator, LaTeXGenerator);
    }

    #[test]
    fn test_generator_default() {
        let generator = LaTeXGenerator::default();
        assert_eq!(generator, LaTeXGenerator::new());
    }

    #[test]
    fn test_single_positive_integer() {
        let generator = LaTeXGenerator::new();
        let ast = make_number("42");
        assert_eq!(generator.generate(&ast), "$42$");
    }

    #[test]
    fn test_single_decimal() {
        let generator = LaTeXGenerator::new();
        let ast = make_number("3.14");
        assert_eq!(generator.generate(&ast), "$3.14$");
    }

    #[test]
    fn test_single_negative_number() {
        let generator = LaTeXGenerator::new();
        let ast = make_number("-5");
        assert_eq!(generator.generate(&ast), "$-5$");
    }

    #[test]
    fn test_addition() {
        let generator = LaTeXGenerator::new();
        let ast = make_binop("+", make_number("3"), make_number("4"));
        assert_eq!(generator.generate(&ast), "$3 + 4$");
    }

    #[test]
    fn test_subtraction() {
        let generator = LaTeXGenerator::new();
        let ast = make_binop("-", make_number("5"), make_number("2"));
        assert_eq!(generator.generate(&ast), "$5 - 2$");
    }

    #[test]
    fn test_multiplication() {
        let generator = LaTeXGenerator::new();
        let ast = make_binop("*", make_number("3"), make_number("4"));
        assert_eq!(generator.generate(&ast), r"$3 \times 4$");
    }

    #[test]
    fn test_division() {
        let generator = LaTeXGenerator::new();
        let ast = make_binop("/", make_number("8"), make_number("2"));
        assert_eq!(generator.generate(&ast), r"$8 \div 2$");
    }

    #[test]
    fn test_precedence_addition_then_multiply() {
        // RPN: 3 4 + 2 *
        // AST: (3 + 4) * 2
        // LaTeX: ( 3 + 4 ) \times 2
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("3"), make_number("4"));
        let mult = make_binop("*", add, make_number("2"));
        assert_eq!(generator.generate(&mult), r"$( 3 + 4 ) \times 2$");
    }

    #[test]
    fn test_precedence_multiply_then_add() {
        // RPN: 3 4 * 2 +
        // AST: (3 * 4) + 2
        // LaTeX: 3 \times 4 + 2 (no parens needed)
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("3"), make_number("4"));
        let add = make_binop("+", mult, make_number("2"));
        assert_eq!(generator.generate(&add), r"$3 \times 4 + 2$");
    }

    #[test]
    fn test_precedence_multiply_on_right() {
        // RPN: 2 3 4 * +
        // AST: 2 + (3 * 4)
        // LaTeX: 2 + 3 \times 4 (no parens needed)
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("3"), make_number("4"));
        let add = make_binop("+", make_number("2"), mult);
        assert_eq!(generator.generate(&add), r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_left_associative_subtraction() {
        // RPN: 5 3 - 2 -
        // AST: (5 - 3) - 2
        // LaTeX: ( 5 - 3 ) - 2
        let generator = LaTeXGenerator::new();
        let sub1 = make_binop("-", make_number("5"), make_number("3"));
        let sub2 = make_binop("-", sub1, make_number("2"));
        assert_eq!(generator.generate(&sub2), "$( 5 - 3 ) - 2$");
    }

    #[test]
    fn test_right_associative_subtraction() {
        // RPN: 5 3 2 - -
        // AST: 5 - (3 - 2)
        // LaTeX: 5 - ( 3 - 2 ) (parens needed on right)
        let generator = LaTeXGenerator::new();
        let sub1 = make_binop("-", make_number("3"), make_number("2"));
        let sub2 = make_binop("-", make_number("5"), sub1);
        assert_eq!(generator.generate(&sub2), "$5 - ( 3 - 2 )$");
    }

    #[test]
    fn test_left_associative_division() {
        // RPN: 8 4 / 2 /
        // AST: (8 / 4) / 2
        // LaTeX: ( 8 \div 4 ) \div 2
        let generator = LaTeXGenerator::new();
        let div1 = make_binop("/", make_number("8"), make_number("4"));
        let div2 = make_binop("/", div1, make_number("2"));
        assert_eq!(generator.generate(&div2), r"$( 8 \div 4 ) \div 2$");
    }

    #[test]
    fn test_right_associative_division() {
        // RPN: 8 4 2 / /
        // AST: 8 / (4 / 2)
        // LaTeX: 8 \div ( 4 \div 2 ) (parens needed on right)
        let generator = LaTeXGenerator::new();
        let div1 = make_binop("/", make_number("4"), make_number("2"));
        let div2 = make_binop("/", make_number("8"), div1);
        assert_eq!(generator.generate(&div2), r"$8 \div ( 4 \div 2 )$");
    }

    #[test]
    fn test_addition_on_right_of_addition() {
        // RPN: 1 2 3 + +
        // AST: 1 + (2 + 3)
        // LaTeX: 1 + 2 + 3 (no parens needed, commutative)
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("2"), make_number("3"));
        let add2 = make_binop("+", make_number("1"), add1);
        assert_eq!(generator.generate(&add2), "$1 + 2 + 3$");
    }

    #[test]
    fn test_multiplication_on_right_of_multiplication() {
        // RPN: 2 3 4 * *
        // AST: 2 * (3 * 4)
        // LaTeX: 2 \times 3 \times 4 (no parens needed, commutative)
        let generator = LaTeXGenerator::new();
        let mult1 = make_binop("*", make_number("3"), make_number("4"));
        let mult2 = make_binop("*", make_number("2"), mult1);
        assert_eq!(generator.generate(&mult2), r"$2 \times 3 \times 4$");
    }

    #[test]
    fn test_complex_nested_expression() {
        // RPN: 3 4 + 2 * 5 -
        // AST: ((3 + 4) * 2) - 5
        // LaTeX: ( 3 + 4 ) \times 2 - 5
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("3"), make_number("4"));
        let mult = make_binop("*", add, make_number("2"));
        let sub = make_binop("-", mult, make_number("5"));
        assert_eq!(generator.generate(&sub), r"$( 3 + 4 ) \times 2 - 5$");
    }

    #[test]
    fn test_deeply_nested_expression() {
        // RPN: 1 2 + 3 4 + *
        // AST: (1 + 2) * (3 + 4)
        // LaTeX: ( 1 + 2 ) \times ( 3 + 4 )
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("1"), make_number("2"));
        let add2 = make_binop("+", make_number("3"), make_number("4"));
        let mult = make_binop("*", add1, add2);
        assert_eq!(generator.generate(&mult), r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_division_with_subtraction() {
        // RPN: 10 2 3 - /
        // AST: 10 / (2 - 3)
        // LaTeX: 10 \div ( 2 - 3 ) (parens needed, lower precedence on right)
        let generator = LaTeXGenerator::new();
        let sub = make_binop("-", make_number("2"), make_number("3"));
        let div = make_binop("/", make_number("10"), sub);
        assert_eq!(generator.generate(&div), r"$10 \div ( 2 - 3 )$");
    }

    #[test]
    fn test_subtraction_with_multiplication() {
        // RPN: 10 2 3 * -
        // AST: 10 - (2 * 3)
        // LaTeX: 10 - 2 \times 3 (no parens needed, higher precedence on right)
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("2"), make_number("3"));
        let sub = make_binop("-", make_number("10"), mult);
        assert_eq!(generator.generate(&sub), r"$10 - 2 \times 3$");
    }

    #[test]
    fn test_four_level_nesting() {
        // RPN: 1 2 + 3 + 4 + 5 +
        // AST: ((((1 + 2) + 3) + 4) + 5)
        // LaTeX: ( ( ( 1 + 2 ) + 3 ) + 4 ) + 5
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("1"), make_number("2"));
        let add2 = make_binop("+", add1, make_number("3"));
        let add3 = make_binop("+", add2, make_number("4"));
        let add4 = make_binop("+", add3, make_number("5"));
        assert_eq!(generator.generate(&add4), "$( ( ( 1 + 2 ) + 3 ) + 4 ) + 5$");
    }

    #[test]
    fn test_mixed_operations() {
        // RPN: 2 3 + 4 5 + *
        // AST: (2 + 3) * (4 + 5)
        // LaTeX: ( 2 + 3 ) \times ( 4 + 5 )
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("2"), make_number("3"));
        let add2 = make_binop("+", make_number("4"), make_number("5"));
        let mult = make_binop("*", add1, add2);
        assert_eq!(generator.generate(&mult), r"$( 2 + 3 ) \times ( 4 + 5 )$");
    }

    #[test]
    fn test_output_format_with_delimiters() {
        let generator = LaTeXGenerator::new();
        let ast = make_number("1");
        let output = generator.generate(&ast);
        assert!(output.starts_with('$'));
        assert!(output.ends_with('$'));
    }

    #[test]
    fn test_operator_to_latex() {
        assert_eq!(LaTeXGenerator::operator_to_latex("+"), "+");
        assert_eq!(LaTeXGenerator::operator_to_latex("-"), "-");
        assert_eq!(LaTeXGenerator::operator_to_latex("*"), r"\times");
        assert_eq!(LaTeXGenerator::operator_to_latex("/"), r"\div");
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(LaTeXGenerator::operator_precedence("+"), 1);
        assert_eq!(LaTeXGenerator::operator_precedence("-"), 1);
        assert_eq!(LaTeXGenerator::operator_precedence("*"), 2);
        assert_eq!(LaTeXGenerator::operator_precedence("/"), 2);
    }
}

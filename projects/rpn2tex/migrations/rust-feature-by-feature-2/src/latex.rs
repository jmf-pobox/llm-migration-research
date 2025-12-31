//! LaTeX code generator for AST nodes.
//!
//! This module provides a generator that converts AST nodes into LaTeX output.

use crate::ast::{BinaryOp, Expr};

/// A LaTeX code generator.
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
    pub fn new() -> Self {
        Self
    }

    /// Generates LaTeX code for an expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LatexGenerator;
    /// use rpn2tex::ast::{Expr, Number};
    ///
    /// let generator = LatexGenerator::new();
    /// let num = Number::new("5".to_string(), 1, 1);
    /// let latex = generator.generate(&Expr::Number(num));
    /// assert_eq!(latex, "$5$");
    /// ```
    #[must_use]
    pub fn generate(&self, expr: &Expr) -> String {
        let inner = self.visit(expr);
        format!("${}$", inner)
    }

    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(num) => num.value.clone(),
            Expr::BinaryOp(op) => self.visit_binary_op(op),
        }
    }

    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        let op_latex = self.get_operator_latex(&node.operator);
        let my_precedence = self.get_precedence(&node.operator);

        // Generate left operand, adding parens if needed
        let mut left = self.visit(&node.left);
        if self.needs_parens(&node.left, my_precedence, false) {
            left = format!("( {} )", left);
        }

        // Generate right operand, adding parens if needed
        let mut right = self.visit(&node.right);
        if self.needs_parens(&node.right, my_precedence, true) {
            right = format!("( {} )", right);
        }

        format!("{} {} {}", left, op_latex, right)
    }

    fn get_operator_latex(&self, operator: &str) -> String {
        match operator {
            "+" => "+".to_string(),
            "-" => "-".to_string(),
            "*" => r"\times".to_string(),
            "/" => r"\div".to_string(),
            _ => operator.to_string(),
        }
    }

    fn get_precedence(&self, operator: &str) -> u32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    fn needs_parens(&self, child: &Expr, parent_precedence: u32, is_right: bool) -> bool {
        // Only BinaryOp children may need parentheses
        if let Expr::BinaryOp(child_op) = child {
            let child_precedence = self.get_precedence(&child_op.operator);

            // Rule 1: Lower precedence always needs parens
            if child_precedence < parent_precedence {
                return true;
            }

            // Rule 2: Equal precedence on right side needs parens for non-commutative operators
            // (handles left-associativity of - and /)
            if child_precedence == parent_precedence
                && is_right
                && (child_op.operator == "-" || child_op.operator == "/")
            {
                return true;
            }
        }

        // Numbers never need parens
        false
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
    use crate::ast::Number;

    #[test]
    fn test_generate_integer() {
        let generator = LatexGenerator::new();
        let num = Number::new("5".to_string(), 1, 1);
        let latex = generator.generate(&Expr::Number(num));
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_generate_float() {
        let generator = LatexGenerator::new();
        let num = Number::new("3.14".to_string(), 1, 1);
        let latex = generator.generate(&Expr::Number(num));
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_negative() {
        let generator = LatexGenerator::new();
        let num = Number::new("-5".to_string(), 1, 1);
        let latex = generator.generate(&Expr::Number(num));
        assert_eq!(latex, "$-5$");
    }

    #[test]
    fn test_preserves_exact_string() {
        let generator = LatexGenerator::new();
        let num = Number::new("3.14".to_string(), 1, 1);
        let latex = generator.generate(&Expr::Number(num));
        // Verify exact preservation - not "3.1400000"
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_addition() {
        let generator = LatexGenerator::new();
        let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let binop = BinaryOp::new("+".to_string(), left, right, 1, 5);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_generate_chained_addition() {
        let generator = LatexGenerator::new();
        // Build: (1 + 2) + 3
        let left_inner = Box::new(Expr::Number(Number::new("1".to_string(), 1, 1)));
        let right_inner = Box::new(Expr::Number(Number::new("2".to_string(), 1, 3)));
        let left = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            left_inner,
            right_inner,
            1,
            5,
        )));
        let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 7)));
        let binop = BinaryOp::new("+".to_string(), left, right, 1, 9);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, "$1 + 2 + 3$");
    }

    #[test]
    fn test_generate_subtraction() {
        let generator = LatexGenerator::new();
        let left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let binop = BinaryOp::new("-".to_string(), left, right, 1, 5);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, "$5 - 3$");
    }

    #[test]
    fn test_generate_chained_subtraction() {
        let generator = LatexGenerator::new();
        // Build: (5 - 3) - 2
        let left_inner = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let right_inner = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let left = Box::new(Expr::BinaryOp(BinaryOp::new(
            "-".to_string(),
            left_inner,
            right_inner,
            1,
            5,
        )));
        let right = Box::new(Expr::Number(Number::new("2".to_string(), 1, 7)));
        let binop = BinaryOp::new("-".to_string(), left, right, 1, 9);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, "$5 - 3 - 2$");
    }

    #[test]
    fn test_generate_multiplication() {
        let generator = LatexGenerator::new();
        let left = Box::new(Expr::Number(Number::new("4".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("7".to_string(), 1, 3)));
        let binop = BinaryOp::new("*".to_string(), left, right, 1, 5);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, r"$4 \times 7$");
    }

    #[test]
    fn test_generate_float_multiplication() {
        let generator = LatexGenerator::new();
        let left = Box::new(Expr::Number(Number::new("3.14".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("2".to_string(), 1, 6)));
        let binop = BinaryOp::new("*".to_string(), left, right, 1, 8);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, r"$3.14 \times 2$");
    }

    #[test]
    fn test_generate_mixed_operations() {
        let generator = LatexGenerator::new();
        // Build: 2 + (3 * 4)
        let mult_left = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let mult_right = Box::new(Expr::Number(Number::new("4".to_string(), 1, 5)));
        let mult = Box::new(Expr::BinaryOp(BinaryOp::new(
            "*".to_string(),
            mult_left,
            mult_right,
            1,
            7,
        )));
        let left = Box::new(Expr::Number(Number::new("2".to_string(), 1, 1)));
        let binop = BinaryOp::new("+".to_string(), left, mult, 1, 9);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_generate_division() {
        let generator = LatexGenerator::new();
        let left = Box::new(Expr::Number(Number::new("10".to_string(), 1, 1)));
        let right = Box::new(Expr::Number(Number::new("2".to_string(), 1, 4)));
        let binop = BinaryOp::new("/".to_string(), left, right, 1, 6);
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, r"$10 \div 2$");
    }

    #[test]
    fn test_generate_chained_division() {
        let generator = LatexGenerator::new();
        // Build: (100 / 10) / 5 / 2
        // First: 100 / 10
        let inner_left = Box::new(Expr::Number(Number::new("100".to_string(), 1, 1)));
        let inner_right = Box::new(Expr::Number(Number::new("10".to_string(), 1, 5)));
        let inner_div = Box::new(Expr::BinaryOp(BinaryOp::new(
            "/".to_string(),
            inner_left,
            inner_right,
            1,
            8,
        )));
        // Second: (100 / 10) / 5
        let second_div = Box::new(Expr::BinaryOp(BinaryOp::new(
            "/".to_string(),
            inner_div,
            Box::new(Expr::Number(Number::new("5".to_string(), 1, 10))),
            1,
            12,
        )));
        // Third: ((100 / 10) / 5) / 2
        let binop = BinaryOp::new(
            "/".to_string(),
            second_div,
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 14))),
            1,
            16,
        );
        let latex = generator.generate(&Expr::BinaryOp(binop));
        assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_generate_mixed_division_multiplication() {
        let generator = LatexGenerator::new();
        // Build: (10 / 2) * 5
        let div_left = Box::new(Expr::Number(Number::new("10".to_string(), 1, 1)));
        let div_right = Box::new(Expr::Number(Number::new("2".to_string(), 1, 4)));
        let div = Box::new(Expr::BinaryOp(BinaryOp::new(
            "/".to_string(),
            div_left,
            div_right,
            1,
            6,
        )));
        let mult = BinaryOp::new(
            "*".to_string(),
            div,
            Box::new(Expr::Number(Number::new("5".to_string(), 1, 8))),
            1,
            10,
        );
        let latex = generator.generate(&Expr::BinaryOp(mult));
        assert_eq!(latex, r"$10 \div 2 \times 5$");
    }

    // Feature 6: Precedence tests

    #[test]
    fn test_precedence_addition_under_multiplication_left() {
        let generator = LatexGenerator::new();
        // Build: (5 + 3) * 2
        // "5 3 + 2 *"
        let add_left = Box::new(Expr::Number(Number::new("5".to_string(), 1, 1)));
        let add_right = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let add = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            add_left,
            add_right,
            1,
            5,
        )));
        let mult = BinaryOp::new(
            "*".to_string(),
            add,
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 7))),
            1,
            9,
        );
        let latex = generator.generate(&Expr::BinaryOp(mult));
        assert_eq!(latex, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_addition_under_multiplication_right() {
        let generator = LatexGenerator::new();
        // Build: 2 * (3 + 4)
        // "2 3 4 + *"
        let add_left = Box::new(Expr::Number(Number::new("3".to_string(), 1, 3)));
        let add_right = Box::new(Expr::Number(Number::new("4".to_string(), 1, 5)));
        let add = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            add_left,
            add_right,
            1,
            7,
        )));
        let mult = BinaryOp::new(
            "*".to_string(),
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 1))),
            add,
            1,
            9,
        );
        let latex = generator.generate(&Expr::BinaryOp(mult));
        assert_eq!(latex, r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_both_sides() {
        let generator = LatexGenerator::new();
        // Build: (1 + 2) * (3 + 4)
        // "1 2 + 3 4 + *"
        let left_add = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            Box::new(Expr::Number(Number::new("1".to_string(), 1, 1))),
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 3))),
            1,
            5,
        )));
        let right_add = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            Box::new(Expr::Number(Number::new("3".to_string(), 1, 7))),
            Box::new(Expr::Number(Number::new("4".to_string(), 1, 9))),
            1,
            11,
        )));
        let mult = BinaryOp::new("*".to_string(), left_add, right_add, 1, 13);
        let latex = generator.generate(&Expr::BinaryOp(mult));
        assert_eq!(latex, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex_mixed() {
        let generator = LatexGenerator::new();
        // Build: ((10 / 2) + 3) * 4
        // "10 2 / 3 + 4 *"
        // First: 10 / 2
        let div = Box::new(Expr::BinaryOp(BinaryOp::new(
            "/".to_string(),
            Box::new(Expr::Number(Number::new("10".to_string(), 1, 1))),
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 4))),
            1,
            6,
        )));
        // Second: (10 / 2) + 3
        let add = Box::new(Expr::BinaryOp(BinaryOp::new(
            "+".to_string(),
            div,
            Box::new(Expr::Number(Number::new("3".to_string(), 1, 8))),
            1,
            10,
        )));
        // Third: ((10 / 2) + 3) * 4
        let mult = BinaryOp::new(
            "*".to_string(),
            add,
            Box::new(Expr::Number(Number::new("4".to_string(), 1, 12))),
            1,
            14,
        );
        let latex = generator.generate(&Expr::BinaryOp(mult));
        assert_eq!(latex, r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_no_parens_for_higher_precedence() {
        let generator = LatexGenerator::new();
        // Build: 2 + (3 * 4) - multiplication has higher precedence, no parens needed
        // "2 3 4 * +"
        let mult = Box::new(Expr::BinaryOp(BinaryOp::new(
            "*".to_string(),
            Box::new(Expr::Number(Number::new("3".to_string(), 1, 3))),
            Box::new(Expr::Number(Number::new("4".to_string(), 1, 5))),
            1,
            7,
        )));
        let add = BinaryOp::new(
            "+".to_string(),
            Box::new(Expr::Number(Number::new("2".to_string(), 1, 1))),
            mult,
            1,
            9,
        );
        let latex = generator.generate(&Expr::BinaryOp(add));
        assert_eq!(latex, r"$2 + 3 \times 4$");
    }
}

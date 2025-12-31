//! LaTeX code generation.

use crate::ast::Expr;

/// A generator that converts AST expressions to LaTeX.
#[derive(Debug)]
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

    /// Generates LaTeX code from an expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LaTeXGenerator;
    /// use rpn2tex::ast::Expr;
    ///
    /// let generator = LaTeXGenerator::new();
    /// let expr = Expr::number("5", 1, 1);
    /// let latex = generator.generate(&expr);
    /// assert_eq!(latex, "$5$");
    /// ```
    #[must_use]
    pub fn generate(&self, expr: &Expr) -> String {
        let content = self.visit(expr);
        format!("${content}$")
    }

    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let my_precedence = expr.precedence();

                // Generate left operand, adding parens if needed
                let mut left_str = self.visit(left);
                if self.needs_parens(left, my_precedence, false) {
                    left_str = format!("( {left_str} )");
                }

                // Generate right operand, adding parens if needed
                let mut right_str = self.visit(right);
                if self.needs_parens(right, my_precedence, true) {
                    right_str = format!("( {right_str} )");
                }

                let op_latex = self.operator_to_latex(operator);
                format!("{left_str} {op_latex} {right_str}")
            }
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence and is on the right side
    ///    (for non-commutative operators like - and /)
    fn needs_parens(&self, child: &Expr, parent_precedence: u32, is_right: bool) -> bool {
        match child {
            Expr::Number { .. } => false,
            Expr::BinaryOp { operator, .. } => {
                let child_precedence = child.precedence();

                // Lower precedence always needs parens
                if child_precedence < parent_precedence {
                    return true;
                }

                // Equal precedence on right side needs parens for non-commutative operators
                child_precedence == parent_precedence
                    && is_right
                    && matches!(operator.as_str(), "-" | "/")
            }
        }
    }

    fn operator_to_latex(&self, operator: &str) -> String {
        match operator {
            "+" => "+".to_string(),
            "-" => "-".to_string(),
            "*" => r"\times".to_string(),
            "/" => r"\div".to_string(),
            _ => operator.to_string(),
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
    fn test_generate_number() {
        let generator = LaTeXGenerator::new();
        let expr = Expr::number("5", 1, 1);
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_generate_decimal() {
        let generator = LaTeXGenerator::new();
        let expr = Expr::number("3.14", 1, 1);
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_visit_number() {
        let generator = LaTeXGenerator::new();
        let expr = Expr::number("42", 1, 1);
        let result = generator.visit(&expr);
        assert_eq!(result, "42");
    }

    #[test]
    fn test_simple_addition() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("+", left, right, 1, 5);
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_chained_addition() {
        let generator = LaTeXGenerator::new();
        // Build: (((1+2)+3)+4)
        let one = Expr::number("1", 1, 1);
        let two = Expr::number("2", 1, 3);
        let expr1 = Expr::binary_op("+", one, two, 1, 5);

        let three = Expr::number("3", 1, 7);
        let expr2 = Expr::binary_op("+", expr1, three, 1, 9);

        let four = Expr::number("4", 1, 11);
        let expr3 = Expr::binary_op("+", expr2, four, 1, 13);

        let latex = generator.generate(&expr3);
        assert_eq!(latex, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_visit_binary_op() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("10", 1, 1);
        let right = Expr::number("20", 1, 4);
        let expr = Expr::binary_op("+", left, right, 1, 7);
        let result = generator.visit(&expr);
        assert_eq!(result, "10 + 20");
    }

    #[test]
    fn test_simple_subtraction() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let expr = Expr::binary_op("-", left, right, 1, 5);
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$5 - 3$");
    }

    #[test]
    fn test_chained_subtraction() {
        let generator = LaTeXGenerator::new();
        // Build: ((5-3)-2)
        let five = Expr::number("5", 1, 1);
        let three = Expr::number("3", 1, 3);
        let expr1 = Expr::binary_op("-", five, three, 1, 5);

        let two = Expr::number("2", 1, 7);
        let expr2 = Expr::binary_op("-", expr1, two, 1, 9);

        let latex = generator.generate(&expr2);
        assert_eq!(latex, "$5 - 3 - 2$");
    }

    #[test]
    fn test_negative_number() {
        let generator = LaTeXGenerator::new();
        let expr = Expr::number("-5", 1, 1);
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$-5$");
    }

    #[test]
    fn test_visit_subtraction() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("10", 1, 1);
        let right = Expr::number("5", 1, 4);
        let expr = Expr::binary_op("-", left, right, 1, 7);
        let result = generator.visit(&expr);
        assert_eq!(result, "10 - 5");
    }

    #[test]
    fn test_simple_multiplication() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("4", 1, 1);
        let right = Expr::number("7", 1, 3);
        let expr = Expr::binary_op("*", left, right, 1, 5);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$4 \times 7$");
    }

    #[test]
    fn test_multiplication_with_addition() {
        let generator = LaTeXGenerator::new();
        // Build: 2 + (3*4)
        let two = Expr::number("2", 1, 1);
        let three = Expr::number("3", 1, 3);
        let four = Expr::number("4", 1, 5);
        let mult = Expr::binary_op("*", three, four, 1, 7);
        let expr = Expr::binary_op("+", two, mult, 1, 9);

        let latex = generator.generate(&expr);
        // Note: No precedence handling yet, so no parentheses
        assert_eq!(latex, r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_visit_multiplication() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("6", 1, 1);
        let right = Expr::number("9", 1, 4);
        let expr = Expr::binary_op("*", left, right, 1, 7);
        let result = generator.visit(&expr);
        assert_eq!(result, r"6 \times 9");
    }

    #[test]
    fn test_operator_to_latex() {
        let generator = LaTeXGenerator::new();
        assert_eq!(generator.operator_to_latex("+"), "+".to_string());
        assert_eq!(generator.operator_to_latex("-"), "-".to_string());
        assert_eq!(generator.operator_to_latex("*"), r"\times".to_string());
        assert_eq!(generator.operator_to_latex("/"), r"\div".to_string());
    }

    #[test]
    fn test_simple_division() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("10", 1, 1);
        let right = Expr::number("2", 1, 4);
        let expr = Expr::binary_op("/", left, right, 1, 6);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$10 \div 2$");
    }

    #[test]
    fn test_chained_division() {
        let generator = LaTeXGenerator::new();
        // Build: (((100/10)/5)/2)
        let hundred = Expr::number("100", 1, 1);
        let ten = Expr::number("10", 1, 5);
        let expr1 = Expr::binary_op("/", hundred, ten, 1, 8);

        let five = Expr::number("5", 1, 10);
        let expr2 = Expr::binary_op("/", expr1, five, 1, 12);

        let two = Expr::number("2", 1, 14);
        let expr3 = Expr::binary_op("/", expr2, two, 1, 16);

        let latex = generator.generate(&expr3);
        assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_visit_division() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("20", 1, 1);
        let right = Expr::number("4", 1, 4);
        let expr = Expr::binary_op("/", left, right, 1, 7);
        let result = generator.visit(&expr);
        assert_eq!(result, r"20 \div 4");
    }

    // Precedence tests
    #[test]
    fn test_precedence_addition_times_number_left() {
        let generator = LaTeXGenerator::new();
        // 5 3 + 2 * => (5 + 3) * 2
        let five = Expr::number("5", 1, 1);
        let three = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", five, three, 1, 5);
        let two = Expr::number("2", 1, 7);
        let expr = Expr::binary_op("*", add, two, 1, 9);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_addition_times_number_right() {
        let generator = LaTeXGenerator::new();
        // 2 3 4 + * => 2 * (3 + 4)
        let two = Expr::number("2", 1, 1);
        let three = Expr::number("3", 1, 3);
        let four = Expr::number("4", 1, 5);
        let add = Expr::binary_op("+", three, four, 1, 7);
        let expr = Expr::binary_op("*", two, add, 1, 9);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_both_sides() {
        let generator = LaTeXGenerator::new();
        // 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
        let one = Expr::number("1", 1, 1);
        let two = Expr::number("2", 1, 3);
        let add1 = Expr::binary_op("+", one, two, 1, 5);
        let three = Expr::number("3", 1, 7);
        let four = Expr::number("4", 1, 9);
        let add2 = Expr::binary_op("+", three, four, 1, 11);
        let expr = Expr::binary_op("*", add1, add2, 1, 13);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex_expression() {
        let generator = LaTeXGenerator::new();
        // 10 2 / 3 + 4 * => (10 / 2 + 3) * 4
        let ten = Expr::number("10", 1, 1);
        let two = Expr::number("2", 1, 4);
        let div = Expr::binary_op("/", ten, two, 1, 6);
        let three = Expr::number("3", 1, 8);
        let add = Expr::binary_op("+", div, three, 1, 10);
        let four = Expr::number("4", 1, 12);
        let expr = Expr::binary_op("*", add, four, 1, 14);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_no_parens_needed() {
        let generator = LaTeXGenerator::new();
        // 2 3 4 * + => 2 + 3 * 4
        let two = Expr::number("2", 1, 1);
        let three = Expr::number("3", 1, 3);
        let four = Expr::number("4", 1, 5);
        let mult = Expr::binary_op("*", three, four, 1, 7);
        let expr = Expr::binary_op("+", two, mult, 1, 9);
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$2 + 3 \times 4$");
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
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", left, right, 1, 5);
        // Addition (precedence 1) in context of multiplication (precedence 2)
        assert!(generator.needs_parens(&add, 2, false));
        assert!(generator.needs_parens(&add, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_left() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let sub = Expr::binary_op("-", left, right, 1, 5);
        // Subtraction on left side doesn't need parens
        assert!(!generator.needs_parens(&sub, 1, false));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_non_commutative() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let sub = Expr::binary_op("-", left, right, 1, 5);
        // Subtraction on right side needs parens
        assert!(generator.needs_parens(&sub, 1, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_commutative() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", left, right, 1, 5);
        // Addition on right side doesn't need parens (commutative)
        assert!(!generator.needs_parens(&add, 1, true));
    }
}

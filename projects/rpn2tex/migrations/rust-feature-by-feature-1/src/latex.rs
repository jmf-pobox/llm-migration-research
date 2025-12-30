//! LaTeX code generator.
//!
//! This module converts AST nodes into LaTeX mathematical notation.

use crate::{BinaryOp, Expr, Number};

/// A LaTeX code generator for RPN expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::{LaTeXGenerator, Lexer, Parser};
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
///
/// let generator = LaTeXGenerator::new();
/// let latex = generator.generate(&ast);
/// assert_eq!(latex, "$5$");
/// ```
#[derive(Debug, Default)]
#[must_use]
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
    pub fn new() -> Self {
        Self
    }

    /// Generates LaTeX code for the given AST.
    ///
    /// The output is wrapped in dollar signs ($...$) for inline math mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{LaTeXGenerator, Number, Expr};
    ///
    /// let num = Number::new("42", 1, 1);
    /// let expr = Expr::Number(num);
    ///
    /// let generator = LaTeXGenerator::new();
    /// let latex = generator.generate(&expr);
    /// assert_eq!(latex, "$42$");
    /// ```
    #[must_use]
    pub fn generate(&self, expr: &Expr) -> String {
        let inner = self.visit(expr);
        format!("${}$", inner)
    }

    fn visit(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(num) => self.visit_number(num),
            Expr::BinaryOp(binop) => self.visit_binary_op(binop),
        }
    }

    fn visit_number(&self, node: &Number) -> String {
        node.value().to_string()
    }

    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        // Map operator to LaTeX representation
        let op_latex = match node.operator() {
            "+" => "+",
            "-" => "-",
            "*" => r"\times",
            "/" => r"\div",
            _ => node.operator(), // Fallback for future operators
        };

        let my_precedence = self.precedence(node.operator());

        // Generate left operand, adding parens if needed
        let mut left = self.visit(node.left());
        if self.needs_parens(node.left(), my_precedence, false) {
            left = format!("( {} )", left);
        }

        // Generate right operand, adding parens if needed
        let mut right = self.visit(node.right());
        if self.needs_parens(node.right(), my_precedence, true) {
            right = format!("( {} )", right);
        }

        format!("{} {} {}", left, op_latex, right)
    }

    /// Returns the precedence level for an operator.
    ///
    /// Higher numbers indicate tighter binding (evaluated first).
    /// - Level 1: Addition, Subtraction
    /// - Level 2: Multiplication, Division
    fn precedence(&self, operator: &str) -> i32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0, // Unknown operators have lowest precedence
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence, is on the right side,
    ///    and is a non-commutative operator (- or /)
    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        // Only BinaryOp nodes can have precedence issues
        let child_binop = match child {
            Expr::BinaryOp(binop) => binop,
            Expr::Number(_) => return false,
        };

        let child_precedence = self.precedence(child_binop.operator());

        // Lower precedence always needs parens
        if child_precedence < parent_precedence {
            return true;
        }

        // Equal precedence on right side needs parens for non-commutative operators
        child_precedence == parent_precedence
            && is_right
            && matches!(child_binop.operator(), "-" | "/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinaryOp, Lexer, Parser};

    #[test]
    fn test_generate_single_digit() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_generate_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_negative() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$-5$");
    }

    #[test]
    fn test_generate_multi_digit() {
        let lexer = Lexer::new("12345");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$12345$");
    }

    #[test]
    fn test_generate_leading_zero() {
        let lexer = Lexer::new("01");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$01$");
    }

    #[test]
    fn test_generate_trailing_decimal() {
        let lexer = Lexer::new("5.");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5.$");
    }

    #[test]
    fn test_generate_very_long_decimal() {
        let lexer = Lexer::new("3.14159265358979");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$3.14159265358979$");
    }

    #[test]
    fn test_direct_number_generation() {
        let num = Number::new("42", 1, 1);
        let expr = Expr::Number(num);

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$42$");
    }

    #[test]
    fn test_generate_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_generate_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_generate_addition_with_floats() {
        let lexer = Lexer::new("1.5 0.5 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$1.5 + 0.5$");
    }

    #[test]
    fn test_direct_binop_generation() {
        let left = Number::new("10", 1, 1);
        let right = Number::new("20", 1, 4);
        let binop = BinaryOp::new("+", Expr::Number(left), Expr::Number(right), 1, 7);
        let expr = Expr::BinaryOp(binop);

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$10 + 20$");
    }

    #[test]
    fn test_generate_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 - 3$");
    }

    #[test]
    fn test_generate_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 - 3 - 2$");
    }

    #[test]
    fn test_generate_subtraction_with_floats() {
        let lexer = Lexer::new("5.5 2.3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5.5 - 2.3$");
    }

    #[test]
    fn test_direct_subtraction_generation() {
        let left = Number::new("10", 1, 1);
        let right = Number::new("3", 1, 4);
        let binop = BinaryOp::new("-", Expr::Number(left), Expr::Number(right), 1, 7);
        let expr = Expr::BinaryOp(binop);

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$10 - 3$");
    }

    #[test]
    fn test_generate_multiplication() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$4 \times 7$");
    }

    #[test]
    fn test_generate_multiplication_with_floats() {
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$3.14 \times 2$");
    }

    #[test]
    fn test_generate_multiplication_with_addition() {
        // "2 3 4 * +" should output "$2 + 3 \times 4$" (no parens yet)
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_direct_multiplication_generation() {
        let left = Number::new("5", 1, 1);
        let right = Number::new("3", 1, 4);
        let binop = BinaryOp::new("*", Expr::Number(left), Expr::Number(right), 1, 7);
        let expr = Expr::BinaryOp(binop);

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$5 \times 3$");
    }

    #[test]
    fn test_generate_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$10 \div 2$");
    }

    #[test]
    fn test_generate_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_generate_division_with_floats() {
        let lexer = Lexer::new("1.5 0.5 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$1.5 \div 0.5$");
    }

    #[test]
    fn test_generate_division_with_multiplication() {
        // "10 2 / 5 *" should output "$10 \div 2 \times 5$" (no parens yet)
        let lexer = Lexer::new("10 2 / 5 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_direct_division_generation() {
        let left = Number::new("20", 1, 1);
        let right = Number::new("4", 1, 4);
        let binop = BinaryOp::new("/", Expr::Number(left), Expr::Number(right), 1, 7);
        let expr = Expr::BinaryOp(binop);

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&expr);
        assert_eq!(latex, r"$20 \div 4$");
    }

    // Precedence Feature Tests

    #[test]
    fn test_precedence_addition_under_multiplication_left() {
        // "5 3 + 2 *" should output "$( 5 + 3 ) \times 2$"
        let lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_addition_under_multiplication_right() {
        // "2 3 4 + *" should output "$2 \times ( 3 + 4 )$"
        let lexer = Lexer::new("2 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_addition_under_multiplication_both() {
        // "1 2 + 3 4 + *" should output "$( 1 + 2 ) \times ( 3 + 4 )$"
        let lexer = Lexer::new("1 2 + 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex_nested() {
        // "10 2 / 3 + 4 *" should output "$( 10 \div 2 + 3 ) \times 4$"
        let lexer = Lexer::new("10 2 / 3 + 4 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_multiplication_over_addition_left() {
        // "5 3 * 2 +" should output "$5 \times 3 + 2$" (no parens)
        let lexer = Lexer::new("5 3 * 2 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_precedence_chained_addition_no_parens() {
        // "1 2 + 3 + 4 +" should output "$1 + 2 + 3 + 4$"
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_precedence_subtraction_on_right() {
        // "5 3 2 - -" should output "$5 - ( 3 - 2 )$"
        let lexer = Lexer::new("5 3 2 - -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 - ( 3 - 2 )$");
    }

    #[test]
    fn test_precedence_subtraction_under_multiplication() {
        // "5 3 - 2 *" should output "$( 5 - 3 ) \times 2$"
        let lexer = Lexer::new("5 3 - 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 5 - 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_division_multiplication_same_level() {
        // "10 2 / 5 *" should output "$10 \div 2 \times 5$" (no parens)
        let lexer = Lexer::new("10 2 / 5 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_precedence_case_2() {
        // I/O contract test case 2: "2 3 + 4 *" → "$( 2 + 3 ) \times 4$"
        let lexer = Lexer::new("2 3 + 4 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 2 + 3 ) \times 4$");
    }
}

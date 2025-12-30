//! LaTeX generator for rpn2tex - converts AST to LaTeX.
//!
//! This module converts the AST into LaTeX math mode output.

use crate::ast::{BinaryOp, Expr, Number};

/// Operator precedence levels for parenthesization.
///
/// Higher values indicate tighter binding:
/// - Addition/Subtraction: 1
/// - Multiplication/Division: 2
const PRECEDENCE: &[(&str, u8)] = &[("+", 1), ("-", 1), ("*", 2), ("/", 2)];

/// Get precedence level for an operator.
///
/// # Examples
///
/// ```
/// # use rpn2tex::latex::get_precedence;
/// assert_eq!(get_precedence("+"), 1);
/// assert_eq!(get_precedence("*"), 2);
/// ```
#[must_use]
pub fn get_precedence(operator: &str) -> u8 {
    PRECEDENCE
        .iter()
        .find(|(op, _)| *op == operator)
        .map(|(_, prec)| *prec)
        .unwrap_or(0)
}

/// Converts rpn2tex AST to LaTeX source code.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// let generator = LaTeXGenerator::new();
/// let latex = generator.generate(&ast);
/// assert_eq!(latex, "$5$");
/// ```
pub struct LaTeXGenerator;

impl LaTeXGenerator {
    /// Create a new LaTeX generator.
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

    /// Generate LaTeX from AST.
    ///
    /// Returns LaTeX string wrapped in math delimiters ($...$).
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// let generator = LaTeXGenerator::new();
    /// let latex = generator.generate(&ast);
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${content}$")
    }

    /// Visit an AST node and generate LaTeX.
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number(num) => self.visit_number(num),
            Expr::BinaryOp(op) => self.visit_binary_op(op),
        }
    }

    /// Generate LaTeX for a number literal.
    fn visit_number(&self, node: &Number) -> String {
        node.value.clone()
    }

    /// Generate LaTeX for a binary operation.
    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        let my_precedence = get_precedence(&node.operator);

        // Generate left operand with parens if needed
        let mut left = self.visit(&node.left);
        if self.needs_parens(&node.left, my_precedence, false) {
            left = format!("( {left} )");
        }

        // Generate right operand with parens if needed
        let mut right = self.visit(&node.right);
        if self.needs_parens(&node.right, my_precedence, true) {
            right = format!("( {right} )");
        }

        let op_latex = self.get_operator_latex(&node.operator);
        format!("{left}{op_latex}{right}")
    }

    /// Determine if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence, is on right side, and is non-commutative (-, /)
    fn needs_parens(&self, child: &Expr, parent_precedence: u8, is_right: bool) -> bool {
        if let Expr::BinaryOp(child_op) = child {
            let child_precedence = get_precedence(&child_op.operator);

            // Rule 1: Lower precedence always needs parens
            if child_precedence < parent_precedence {
                return true;
            }

            // Rule 2: Equal precedence on right side needs parens for non-commutative
            if child_precedence == parent_precedence
                && is_right
                && matches!(child_op.operator.as_str(), "-" | "/")
            {
                return true;
            }
        }

        false
    }

    /// Get LaTeX representation of an operator.
    fn get_operator_latex(&self, operator: &str) -> String {
        match operator {
            "+" => " + ".to_string(),
            "-" => " - ".to_string(),
            "*" => r" \times ".to_string(),
            "/" => r" \div ".to_string(),
            _ => format!(" {operator} "),
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
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_integer() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 - 3$");
    }

    #[test]
    fn test_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, "$5 - 3 - 2$");
    }

    #[test]
    fn test_multiplication() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$4 \times 7$");
    }

    #[test]
    fn test_mixed_mult_add() {
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$10 \div 2$");
    }

    #[test]
    fn test_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_precedence_add_mult_left() {
        let lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_add_mult_left_2() {
        let lexer = Lexer::new("2 3 + 4 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_add_mult_right() {
        let lexer = Lexer::new("2 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_both_sides() {
        let lexer = Lexer::new("1 2 + 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex() {
        let lexer = Lexer::new("10 2 / 3 + 4 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let generator = LaTeXGenerator::new();
        let latex = generator.generate(&ast);
        assert_eq!(latex, r"$( 10 \div 2 + 3 ) \times 4$");
    }
}

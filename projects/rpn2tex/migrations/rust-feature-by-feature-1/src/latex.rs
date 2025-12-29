//! LaTeX generator for rpn2tex - converts AST to LaTeX.
//!
//! This module converts the AST into LaTeX math mode output.

use crate::ast::{BinaryOp, Expr, Number};

/// Converts rpn2tex AST to LaTeX source code.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let mut lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let ast = Parser::new(tokens).parse().unwrap();
/// let latex = LaTeXGenerator::new().generate(&ast);
/// assert_eq!(latex, "$5$");
/// ```
#[derive(Debug, Default)]
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
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{Expr, Number};
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let ast = Expr::Number(Number::new("42".to_string(), 1, 1));
    /// let latex = LaTeXGenerator::new().generate(&ast);
    /// assert_eq!(latex, "$42$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${content}$")
    }

    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number(n) => self.visit_number(n),
            Expr::BinaryOp(op) => self.visit_binary_op(op),
        }
    }

    fn visit_number(&self, node: &Number) -> String {
        node.value.clone()
    }

    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        let op_latex = Self::operator_to_latex(&node.operator);
        let my_precedence = Self::precedence(&node.operator);

        // Generate left operand, adding parens if needed
        let mut left = self.visit(&node.left);
        if Self::needs_parens(&node.left, my_precedence, false) {
            left = format!("( {left} )");
        }

        // Generate right operand, adding parens if needed
        let mut right = self.visit(&node.right);
        if Self::needs_parens(&node.right, my_precedence, true) {
            right = format!("( {right} )");
        }

        format!("{left} {op_latex} {right}")
    }

    /// Get the precedence level for an operator.
    ///
    /// Higher values indicate tighter binding.
    /// Addition and subtraction have precedence 1.
    /// Multiplication and division have precedence 2.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// assert_eq!(LaTeXGenerator::precedence("+"), 1);
    /// assert_eq!(LaTeXGenerator::precedence("-"), 1);
    /// assert_eq!(LaTeXGenerator::precedence("*"), 2);
    /// assert_eq!(LaTeXGenerator::precedence("/"), 2);
    /// ```
    #[must_use]
    pub fn precedence(op: &str) -> u8 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Determine if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence, is on the right side, and is a
    ///    non-commutative operator (- or /) to handle left-associativity
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::{Expr, Number, BinaryOp};
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// // Lower precedence needs parens: (5 + 3) * 2
    /// let add = Expr::BinaryOp(BinaryOp::new(
    ///     "+".to_string(),
    ///     Expr::Number(Number::new("5".to_string(), 1, 1)),
    ///     Expr::Number(Number::new("3".to_string(), 1, 3)),
    ///     1, 5
    /// ));
    /// assert!(LaTeXGenerator::needs_parens(&add, 2, false));
    ///
    /// // Number never needs parens
    /// let num = Expr::Number(Number::new("5".to_string(), 1, 1));
    /// assert!(!LaTeXGenerator::needs_parens(&num, 2, false));
    /// ```
    #[must_use]
    pub fn needs_parens(child: &Expr, parent_precedence: u8, is_right: bool) -> bool {
        match child {
            Expr::Number(_) => false,
            Expr::BinaryOp(op) => {
                let child_precedence = Self::precedence(&op.operator);

                // Rule 1: Lower precedence always needs parens
                if child_precedence < parent_precedence {
                    return true;
                }

                // Rule 2: Equal precedence on right side needs parens
                // for non-commutative operators (handles left-associativity)
                child_precedence == parent_precedence
                    && is_right
                    && (op.operator == "-" || op.operator == "/")
            }
        }
    }

    /// Convert an operator string to its LaTeX representation.
    ///
    /// Maps operator symbols to their LaTeX equivalents:
    /// - `"+"` → `"+"`
    /// - `"-"` → `"-"`
    /// - `"*"` → `"\\times"`
    /// - `"/"` → `"\\div"`
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// assert_eq!(LaTeXGenerator::operator_to_latex("*"), "\\times");
    /// assert_eq!(LaTeXGenerator::operator_to_latex("/"), "\\div");
    /// assert_eq!(LaTeXGenerator::operator_to_latex("+"), "+");
    /// ```
    #[must_use]
    pub fn operator_to_latex(op: &str) -> &str {
        match op {
            "+" => "+",
            "-" => "-",
            "*" => "\\times",
            "/" => "\\div",
            _ => op,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Number;

    #[test]
    fn test_integer() {
        let ast = Expr::Number(Number::new("5".to_string(), 1, 1));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_decimal() {
        let ast = Expr::Number(Number::new("3.14".to_string(), 1, 1));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_addition() {
        let left = Expr::Number(Number::new("5".to_string(), 1, 1));
        let right = Expr::Number(Number::new("3".to_string(), 1, 3));
        let ast = Expr::BinaryOp(BinaryOp::new("+".to_string(), left, right, 1, 5));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_chained_addition() {
        // 1 2 + 3 + 4 + -> ((1 + 2) + 3) + 4
        let n1 = Expr::Number(Number::new("1".to_string(), 1, 1));
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 3));
        let add1 = Expr::BinaryOp(BinaryOp::new("+".to_string(), n1, n2, 1, 5));

        let n3 = Expr::Number(Number::new("3".to_string(), 1, 7));
        let add2 = Expr::BinaryOp(BinaryOp::new("+".to_string(), add1, n3, 1, 9));

        let n4 = Expr::Number(Number::new("4".to_string(), 1, 11));
        let ast = Expr::BinaryOp(BinaryOp::new("+".to_string(), add2, n4, 1, 13));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_multiplication() {
        let left = Expr::Number(Number::new("4".to_string(), 1, 1));
        let right = Expr::Number(Number::new("7".to_string(), 1, 3));
        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), left, right, 1, 5));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$4 \\times 7$");
    }

    #[test]
    fn test_multiplication_with_addition() {
        // 2 3 4 * + -> 2 + (3 * 4)
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 1));
        let n3 = Expr::Number(Number::new("3".to_string(), 1, 3));
        let n4 = Expr::Number(Number::new("4".to_string(), 1, 5));
        let mult = Expr::BinaryOp(BinaryOp::new("*".to_string(), n3, n4, 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("+".to_string(), n2, mult, 1, 9));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$2 + 3 \\times 4$");
    }

    #[test]
    fn test_division() {
        let left = Expr::Number(Number::new("10".to_string(), 1, 1));
        let right = Expr::Number(Number::new("2".to_string(), 1, 3));
        let ast = Expr::BinaryOp(BinaryOp::new("/".to_string(), left, right, 1, 5));
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$10 \\div 2$");
    }

    #[test]
    fn test_chained_division() {
        // 100 10 / 5 / 2 / -> ((100 / 10) / 5) / 2
        let n100 = Expr::Number(Number::new("100".to_string(), 1, 1));
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 3));
        let div1 = Expr::BinaryOp(BinaryOp::new("/".to_string(), n100, n10, 1, 5));

        let n5 = Expr::Number(Number::new("5".to_string(), 1, 7));
        let div2 = Expr::BinaryOp(BinaryOp::new("/".to_string(), div1, n5, 1, 9));

        let n2 = Expr::Number(Number::new("2".to_string(), 1, 11));
        let ast = Expr::BinaryOp(BinaryOp::new("/".to_string(), div2, n2, 1, 13));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$100 \\div 10 \\div 5 \\div 2$");
    }

    // Precedence tests
    #[test]
    fn test_precedence_addition_in_multiplication_left() {
        // 5 3 + 2 * -> (5 + 3) * 2
        let n5 = Expr::Number(Number::new("5".to_string(), 1, 1));
        let n3 = Expr::Number(Number::new("3".to_string(), 1, 3));
        let add = Expr::BinaryOp(BinaryOp::new("+".to_string(), n5, n3, 1, 5));

        let n2 = Expr::Number(Number::new("2".to_string(), 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), add, n2, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_precedence_addition_in_multiplication_right() {
        // 2 3 4 + * -> 2 * (3 + 4)
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 1));
        let n3 = Expr::Number(Number::new("3".to_string(), 1, 3));
        let n4 = Expr::Number(Number::new("4".to_string(), 1, 5));
        let add = Expr::BinaryOp(BinaryOp::new("+".to_string(), n3, n4, 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), n2, add, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$2 \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_both_additions_in_multiplication() {
        // 1 2 + 3 4 + * -> (1 + 2) * (3 + 4)
        let n1 = Expr::Number(Number::new("1".to_string(), 1, 1));
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 3));
        let add1 = Expr::BinaryOp(BinaryOp::new("+".to_string(), n1, n2, 1, 5));

        let n3 = Expr::Number(Number::new("3".to_string(), 1, 7));
        let n4 = Expr::Number(Number::new("4".to_string(), 1, 9));
        let add2 = Expr::BinaryOp(BinaryOp::new("+".to_string(), n3, n4, 1, 11));

        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), add1, add2, 1, 13));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$( 1 + 2 ) \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex_with_division() {
        // 10 2 / 3 + 4 * -> (10 / 2 + 3) * 4
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 1));
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 3));
        let div = Expr::BinaryOp(BinaryOp::new("/".to_string(), n10, n2, 1, 5));

        let n3 = Expr::Number(Number::new("3".to_string(), 1, 7));
        let add = Expr::BinaryOp(BinaryOp::new("+".to_string(), div, n3, 1, 9));

        let n4 = Expr::Number(Number::new("4".to_string(), 1, 11));
        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), add, n4, 1, 13));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$( 10 \\div 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_precedence_alternative_addition_in_multiplication() {
        // 2 3 + 4 * -> (2 + 3) * 4
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 1));
        let n3 = Expr::Number(Number::new("3".to_string(), 1, 3));
        let add = Expr::BinaryOp(BinaryOp::new("+".to_string(), n2, n3, 1, 5));

        let n4 = Expr::Number(Number::new("4".to_string(), 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("*".to_string(), add, n4, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$( 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_left_associativity_subtraction() {
        // 10 5 - 2 - -> (10 - 5) - 2 = 10 - 5 - 2 (no parens on left)
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 1));
        let n5 = Expr::Number(Number::new("5".to_string(), 1, 3));
        let sub1 = Expr::BinaryOp(BinaryOp::new("-".to_string(), n10, n5, 1, 5));

        let n2 = Expr::Number(Number::new("2".to_string(), 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("-".to_string(), sub1, n2, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$10 - 5 - 2$");
    }

    #[test]
    fn test_right_associativity_subtraction_needs_parens() {
        // 10 5 2 - - -> 10 - (5 - 2) (parens needed on right)
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 1));
        let n5 = Expr::Number(Number::new("5".to_string(), 1, 3));
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 5));
        let sub2 = Expr::BinaryOp(BinaryOp::new("-".to_string(), n5, n2, 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("-".to_string(), n10, sub2, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$10 - ( 5 - 2 )$");
    }

    #[test]
    fn test_left_associativity_division() {
        // 100 10 / 2 / -> (100 / 10) / 2 = 100 / 10 / 2 (no parens on left)
        let n100 = Expr::Number(Number::new("100".to_string(), 1, 1));
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 3));
        let div1 = Expr::BinaryOp(BinaryOp::new("/".to_string(), n100, n10, 1, 5));

        let n2 = Expr::Number(Number::new("2".to_string(), 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("/".to_string(), div1, n2, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$100 \\div 10 \\div 2$");
    }

    #[test]
    fn test_right_associativity_division_needs_parens() {
        // 100 10 2 / / -> 100 / (10 / 2) (parens needed on right)
        let n100 = Expr::Number(Number::new("100".to_string(), 1, 1));
        let n10 = Expr::Number(Number::new("10".to_string(), 1, 3));
        let n2 = Expr::Number(Number::new("2".to_string(), 1, 5));
        let div2 = Expr::BinaryOp(BinaryOp::new("/".to_string(), n10, n2, 1, 7));
        let ast = Expr::BinaryOp(BinaryOp::new("/".to_string(), n100, div2, 1, 9));

        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$100 \\div ( 10 \\div 2 )$");
    }
}

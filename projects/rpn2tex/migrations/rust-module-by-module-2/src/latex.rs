//! LaTeX code generation from Abstract Syntax Trees.
//!
//! This module converts AST nodes to LaTeX source code with proper operator
//! precedence handling and parenthesization. The generator ensures that
//! expressions are rendered correctly based on mathematical precedence rules.

use crate::ast::Expr;

/// Converts AST expressions to LaTeX source code.
///
/// The generator handles operator precedence and adds parentheses only when
/// necessary to preserve the correct evaluation order. The output is formatted
/// in LaTeX math mode with proper spacing.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::Expr;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let generator = LaTeXGenerator::new();
///
/// // Simple addition: 5 + 3
/// let expr = Expr::BinaryOp {
///     line: 1,
///     column: 5,
///     operator: "+".to_string(),
///     left: Box::new(Expr::Number {
///         line: 1,
///         column: 1,
///         value: "5".to_string(),
///     }),
///     right: Box::new(Expr::Number {
///         line: 1,
///         column: 3,
///         value: "3".to_string(),
///     }),
/// };
///
/// let latex = generator.generate(&expr);
/// assert_eq!(latex, "$5 + 3$");
/// ```
#[derive(Debug, Clone, Copy, Default)]
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
    pub const fn new() -> Self {
        Self
    }

    /// Generates LaTeX source code from an AST expression.
    ///
    /// The output is wrapped in LaTeX inline math mode delimiters (`$...$`)
    /// and includes proper operator symbols and parenthesization.
    ///
    /// # Arguments
    ///
    /// * `ast` - The root expression node to convert
    ///
    /// # Returns
    ///
    /// A string containing the LaTeX representation, wrapped in `$...$`
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::Expr;
    /// use rpn2tex::latex::LaTeXGenerator;
    ///
    /// let generator = LaTeXGenerator::new();
    ///
    /// // Number literal
    /// let num = Expr::Number {
    ///     line: 1,
    ///     column: 1,
    ///     value: "42".to_string(),
    /// };
    /// assert_eq!(generator.generate(&num), "$42$");
    ///
    /// // Multiplication
    /// let mult = Expr::BinaryOp {
    ///     line: 1,
    ///     column: 5,
    ///     operator: "*".to_string(),
    ///     left: Box::new(Expr::Number {
    ///         line: 1,
    ///         column: 1,
    ///         value: "4".to_string(),
    ///     }),
    ///     right: Box::new(Expr::Number {
    ///         line: 1,
    ///         column: 3,
    ///         value: "7".to_string(),
    ///     }),
    /// };
    /// assert_eq!(generator.generate(&mult), r"$4 \times 7$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let inner = self.visit(ast);
        format!("${}$", inner)
    }

    /// Visits an expression node and generates its LaTeX representation.
    ///
    /// This is the main recursive traversal function that handles both
    /// number literals and binary operations.
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => value.clone(),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => self.visit_binary_op(operator, left, right),
        }
    }

    /// Visits a binary operation node and generates its LaTeX representation.
    ///
    /// Handles operator precedence and adds parentheses when necessary.
    fn visit_binary_op(&self, operator: &str, left: &Expr, right: &Expr) -> String {
        let parent_precedence = Self::precedence(operator);
        let latex_op = Self::operator_to_latex(operator);

        let mut left_text = self.visit(left);
        if self.needs_parens(left, parent_precedence, false) {
            left_text = format!("( {} )", left_text);
        }

        let mut right_text = self.visit(right);
        if self.needs_parens(right, parent_precedence, true) {
            right_text = format!("( {} )", right_text);
        }

        format!("{} {} {}", left_text, latex_op, right_text)
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. The child has lower precedence than the parent
    /// 2. The child has equal precedence, is on the right, and the operator
    ///    is left-associative (- or /)
    ///
    /// # Arguments
    ///
    /// * `child` - The child expression to check
    /// * `parent_precedence` - The precedence level of the parent operator
    /// * `is_right` - Whether the child is the right operand
    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        match child {
            Expr::Number { .. } => false,
            Expr::BinaryOp { operator, .. } => {
                let child_precedence = Self::precedence(operator);

                if child_precedence < parent_precedence {
                    // Lower precedence always needs parens
                    true
                } else if child_precedence == parent_precedence && is_right {
                    // Equal precedence on right side needs parens for left-associative ops
                    // Subtraction and division are left-associative
                    matches!(operator.as_str(), "-" | "/")
                } else {
                    false
                }
            }
        }
    }

    /// Returns the precedence level for an operator.
    ///
    /// Higher numbers indicate higher precedence (tighter binding).
    ///
    /// # Precedence levels
    ///
    /// * Level 1: Addition (+), Subtraction (-)
    /// * Level 2: Multiplication (*), Division (/)
    fn precedence(operator: &str) -> i32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Converts an operator symbol to its LaTeX representation.
    ///
    /// # Mappings
    ///
    /// * `+` → `+`
    /// * `-` → `-`
    /// * `*` → `\times`
    /// * `/` → `\div`
    fn operator_to_latex(operator: &str) -> String {
        match operator {
            "+" => "+".to_string(),
            "-" => "-".to_string(),
            "*" => r"\times".to_string(),
            "/" => r"\div".to_string(),
            _ => operator.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_number(value: &str) -> Expr {
        Expr::Number {
            line: 1,
            column: 1,
            value: value.to_string(),
        }
    }

    fn make_binop(operator: &str, left: Expr, right: Expr) -> Expr {
        Expr::BinaryOp {
            line: 1,
            column: 1,
            operator: operator.to_string(),
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    #[test]
    fn test_simple_number() {
        let generator = LaTeXGenerator::new();
        let expr = make_number("42");
        assert_eq!(generator.generate(&expr), "$42$");
    }

    #[test]
    fn test_decimal_number() {
        let generator = LaTeXGenerator::new();
        let expr = make_number("3.14");
        assert_eq!(generator.generate(&expr), "$3.14$");
    }

    #[test]
    fn test_negative_number() {
        let generator = LaTeXGenerator::new();
        let expr = make_number("-5");
        assert_eq!(generator.generate(&expr), "$-5$");
    }

    #[test]
    fn test_basic_addition() {
        let generator = LaTeXGenerator::new();
        let expr = make_binop("+", make_number("5"), make_number("3"));
        assert_eq!(generator.generate(&expr), "$5 + 3$");
    }

    #[test]
    fn test_basic_subtraction() {
        let generator = LaTeXGenerator::new();
        let expr = make_binop("-", make_number("5"), make_number("3"));
        assert_eq!(generator.generate(&expr), "$5 - 3$");
    }

    #[test]
    fn test_basic_multiplication() {
        let generator = LaTeXGenerator::new();
        let expr = make_binop("*", make_number("4"), make_number("7"));
        assert_eq!(generator.generate(&expr), r"$4 \times 7$");
    }

    #[test]
    fn test_basic_division() {
        let generator = LaTeXGenerator::new();
        let expr = make_binop("/", make_number("10"), make_number("2"));
        assert_eq!(generator.generate(&expr), r"$10 \div 2$");
    }

    #[test]
    fn test_addition_then_multiplication() {
        // (5 + 3) * 2 - addition has lower precedence, needs parens
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("5"), make_number("3"));
        let expr = make_binop("*", add, make_number("2"));
        assert_eq!(generator.generate(&expr), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_multiplication_then_addition() {
        // 5 * 3 + 2 - multiplication has higher precedence, no parens needed
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("5"), make_number("3"));
        let expr = make_binop("+", mult, make_number("2"));
        assert_eq!(generator.generate(&expr), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_division_then_multiplication() {
        // 10 / 2 * 5 - same precedence, left-to-right
        let generator = LaTeXGenerator::new();
        let div = make_binop("/", make_number("10"), make_number("2"));
        let expr = make_binop("*", div, make_number("5"));
        assert_eq!(generator.generate(&expr), r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_subtraction_chain() {
        // 5 - 3 - 2 - left-associative
        let generator = LaTeXGenerator::new();
        let sub1 = make_binop("-", make_number("5"), make_number("3"));
        let expr = make_binop("-", sub1, make_number("2"));
        assert_eq!(generator.generate(&expr), "$5 - 3 - 2$");
    }

    #[test]
    fn test_division_chain() {
        // 100 / 10 / 5 / 2
        let generator = LaTeXGenerator::new();
        let div1 = make_binop("/", make_number("100"), make_number("10"));
        let div2 = make_binop("/", div1, make_number("5"));
        let expr = make_binop("/", div2, make_number("2"));
        assert_eq!(generator.generate(&expr), r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_addition_chain() {
        // 1 + 2 + 3 + 4
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("1"), make_number("2"));
        let add2 = make_binop("+", add1, make_number("3"));
        let expr = make_binop("+", add2, make_number("4"));
        assert_eq!(generator.generate(&expr), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_multiplication_precedence_over_addition() {
        // 2 + 3 * 4 - multiplication binds tighter
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("3"), make_number("4"));
        let expr = make_binop("+", make_number("2"), mult);
        assert_eq!(generator.generate(&expr), r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_addition_as_left_of_multiplication() {
        // (2 + 3) * 4 - addition on left needs parens
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("2"), make_number("3"));
        let expr = make_binop("*", add, make_number("4"));
        assert_eq!(generator.generate(&expr), r"$( 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_addition_as_right_of_multiplication() {
        // 2 * (3 + 4) - addition on right needs parens
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("3"), make_number("4"));
        let expr = make_binop("*", make_number("2"), add);
        assert_eq!(generator.generate(&expr), r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_multiplication_as_left_of_addition() {
        // 2 * 3 + 4 - no parens needed
        let generator = LaTeXGenerator::new();
        let mult = make_binop("*", make_number("2"), make_number("3"));
        let expr = make_binop("+", mult, make_number("4"));
        assert_eq!(generator.generate(&expr), r"$2 \times 3 + 4$");
    }

    #[test]
    fn test_floating_point_multiplication() {
        // 3.14 * 2
        let generator = LaTeXGenerator::new();
        let expr = make_binop("*", make_number("3.14"), make_number("2"));
        assert_eq!(generator.generate(&expr), r"$3.14 \times 2$");
    }

    #[test]
    fn test_floating_point_addition() {
        // 1.5 + 0.5
        let generator = LaTeXGenerator::new();
        let expr = make_binop("+", make_number("1.5"), make_number("0.5"));
        assert_eq!(generator.generate(&expr), "$1.5 + 0.5$");
    }

    #[test]
    fn test_multiple_additions_with_multiplication() {
        // (1 + 2) * (3 + 4)
        let generator = LaTeXGenerator::new();
        let add1 = make_binop("+", make_number("1"), make_number("2"));
        let add2 = make_binop("+", make_number("3"), make_number("4"));
        let expr = make_binop("*", add1, add2);
        assert_eq!(generator.generate(&expr), r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_complex_expression() {
        // (10 / 2 + 3) * 4
        // First: 10 / 2 = a
        // Then: a + 3 = b
        // Finally: b * 4
        let generator = LaTeXGenerator::new();
        let div = make_binop("/", make_number("10"), make_number("2"));
        let add = make_binop("+", div, make_number("3"));
        let expr = make_binop("*", add, make_number("4"));
        assert_eq!(generator.generate(&expr), r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_function() {
        assert_eq!(LaTeXGenerator::precedence("+"), 1);
        assert_eq!(LaTeXGenerator::precedence("-"), 1);
        assert_eq!(LaTeXGenerator::precedence("*"), 2);
        assert_eq!(LaTeXGenerator::precedence("/"), 2);
    }

    #[test]
    fn test_operator_to_latex() {
        assert_eq!(LaTeXGenerator::operator_to_latex("+"), "+");
        assert_eq!(LaTeXGenerator::operator_to_latex("-"), "-");
        assert_eq!(LaTeXGenerator::operator_to_latex("*"), r"\times");
        assert_eq!(LaTeXGenerator::operator_to_latex("/"), r"\div");
    }

    #[test]
    fn test_needs_parens_number() {
        let generator = LaTeXGenerator::new();
        let num = make_number("5");
        assert!(!generator.needs_parens(&num, 2, false));
        assert!(!generator.needs_parens(&num, 2, true));
    }

    #[test]
    fn test_needs_parens_lower_precedence() {
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("2"), make_number("3"));
        // Addition (precedence 1) as child of multiplication (precedence 2)
        assert!(generator.needs_parens(&add, 2, false));
        assert!(generator.needs_parens(&add, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_left() {
        let generator = LaTeXGenerator::new();
        let sub = make_binop("-", make_number("5"), make_number("3"));
        // Subtraction as left child of another subtraction - no parens
        assert!(!generator.needs_parens(&sub, 1, false));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_subtraction() {
        let generator = LaTeXGenerator::new();
        let sub = make_binop("-", make_number("3"), make_number("2"));
        // Subtraction as right child of subtraction - needs parens
        assert!(generator.needs_parens(&sub, 1, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_division() {
        let generator = LaTeXGenerator::new();
        let div = make_binop("/", make_number("10"), make_number("2"));
        // Division as right child of division - needs parens
        assert!(generator.needs_parens(&div, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_addition() {
        let generator = LaTeXGenerator::new();
        let add = make_binop("+", make_number("2"), make_number("3"));
        // Addition as right child of addition - no parens (commutative)
        assert!(!generator.needs_parens(&add, 1, true));
    }

    #[test]
    fn test_default_constructor() {
        let generator = LaTeXGenerator;
        let expr = make_number("42");
        assert_eq!(generator.generate(&expr), "$42$");
    }
}

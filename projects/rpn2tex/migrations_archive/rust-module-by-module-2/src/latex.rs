//! LaTeX generation from AST.
//!
//! This module converts abstract syntax trees (AST) into LaTeX math mode format.
//! It handles operator precedence and parenthesization to ensure correct mathematical
//! representation of the expression tree.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::ast::Expr;
//! use rpn2tex::latex::LaTeXGenerator;
//!
//! let left = Expr::number("5", 1, 1);
//! let right = Expr::number("3", 1, 3);
//! let add = Expr::binary_op("+", left, right, 1, 2);
//!
//! let generator = LaTeXGenerator::new();
//! let latex = generator.generate(&add);
//! assert_eq!(latex, "$5 + 3$");
//! ```

use crate::ast::Expr;
use std::collections::HashMap;

/// Generator for converting AST to LaTeX format.
///
/// This struct maintains operator mappings and precedence levels used to
/// generate properly formatted LaTeX output with correct parenthesization.
#[derive(Debug, Clone)]
pub struct LaTeXGenerator {
    /// Maps operator symbols to their LaTeX equivalents
    binary_ops: HashMap<String, String>,
    /// Maps operator symbols to their precedence levels
    precedence: HashMap<String, i32>,
}

impl LaTeXGenerator {
    /// Creates a new LaTeX generator with default operator mappings.
    ///
    /// # Operator Mappings
    ///
    /// - `+` → `+`
    /// - `-` → `-`
    /// - `*` → `\times`
    /// - `/` → `\div`
    ///
    /// # Precedence Levels
    ///
    /// - Addition (`+`) and Subtraction (`-`): level 1
    /// - Multiplication (`*`) and Division (`/`): level 2
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
        let mut binary_ops = HashMap::new();
        binary_ops.insert("+".to_string(), "+".to_string());
        binary_ops.insert("-".to_string(), "-".to_string());
        binary_ops.insert("*".to_string(), r"\times".to_string());
        binary_ops.insert("/".to_string(), r"\div".to_string());

        let mut precedence = HashMap::new();
        precedence.insert("+".to_string(), 1);
        precedence.insert("-".to_string(), 1);
        precedence.insert("*".to_string(), 2);
        precedence.insert("/".to_string(), 2);

        Self {
            binary_ops,
            precedence,
        }
    }

    /// Generates LaTeX code from an AST.
    ///
    /// The output is wrapped in `$...$` delimiters for LaTeX math mode.
    /// Operators are separated by spaces, and parentheses are added as needed
    /// based on precedence rules.
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
    /// let left = Expr::number("5", 1, 1);
    /// let right = Expr::number("3", 1, 3);
    /// let add = Expr::binary_op("+", left, right, 1, 2);
    ///
    /// let generator = LaTeXGenerator::new();
    /// assert_eq!(generator.generate(&add), "$5 + 3$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${}$", content)
    }

    /// Visits an AST node and generates its LaTeX representation.
    ///
    /// This is the main dispatcher that routes to specialized visitor methods
    /// based on the node type.
    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number { value, .. } => self.visit_number(value),
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => self.visit_binary_op(operator, left, right),
        }
    }

    /// Visits a number node and returns its string value.
    ///
    /// Numbers are returned exactly as they appear in the input,
    /// preserving formatting like "3.14" vs "3.140".
    fn visit_number(&self, value: &str) -> String {
        value.to_string()
    }

    /// Visits a binary operation node and generates its LaTeX representation.
    ///
    /// This method:
    /// 1. Looks up the LaTeX operator symbol
    /// 2. Recursively visits left and right operands
    /// 3. Adds parentheses to operands when needed based on precedence
    /// 4. Formats the result with spaces around operators
    fn visit_binary_op(&self, operator: &str, left: &Expr, right: &Expr) -> String {
        let op_latex = self.binary_ops.get(operator).unwrap();
        let my_precedence = *self.precedence.get(operator).unwrap();

        // Generate left operand, adding parens if needed
        let mut left_str = self.visit(left);
        if self.needs_parens(left, my_precedence, false) {
            left_str = format!("( {} )", left_str);
        }

        // Generate right operand, adding parens if needed
        let mut right_str = self.visit(right);
        if self.needs_parens(right, my_precedence, true) {
            right_str = format!("( {} )", right_str);
        }

        format!("{} {} {}", left_str, op_latex, right_str)
    }

    /// Determines whether a child expression needs parentheses.
    ///
    /// This implements the critical precedence logic:
    ///
    /// # Rules
    ///
    /// 1. Numbers never need parentheses
    /// 2. Lower precedence child always needs parentheses
    /// 3. Equal precedence on right side needs parentheses for `-` and `/`
    ///    (handles left-associativity)
    /// 4. All other cases don't need parentheses
    ///
    /// # Arguments
    ///
    /// * `child` - The child expression to check
    /// * `parent_precedence` - The precedence level of the parent operator
    /// * `is_right` - Whether this child is the right operand
    ///
    /// # Examples
    ///
    /// ```
    /// // For expression: (5 + 3) * 2
    /// // When visiting *, the left child (5 + 3) needs parens because:
    /// // - child precedence (1) < parent precedence (2)
    ///
    /// // For expression: 5 * 3 + 2
    /// // When visiting +, the left child (5 * 3) doesn't need parens because:
    /// // - child precedence (2) > parent precedence (1)
    ///
    /// // For expression: 5 - 3 - 2
    /// // When visiting outer -, the right child (3 - 2) doesn't exist in this form
    /// // This expression is actually: (5 - 3) - 2
    /// // The left child (5 - 3) doesn't need parens (left side, equal precedence)
    /// ```
    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        match child {
            Expr::Number { .. } => false,
            Expr::BinaryOp { operator, .. } => {
                let child_precedence = *self.precedence.get(operator.as_str()).unwrap();

                // Lower precedence always needs parens
                if child_precedence < parent_precedence {
                    return true;
                }

                // Equal precedence on right side needs parens for non-commutative operators
                // (handles left-associativity of - and /)
                child_precedence == parent_precedence
                    && is_right
                    && (operator == "-" || operator == "/")
            }
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
    fn test_generate_simple_number() {
        let generator = LaTeXGenerator::new();
        let num = Expr::number("42", 1, 1);
        assert_eq!(generator.generate(&num), "$42$");
    }

    #[test]
    fn test_generate_simple_addition() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", left, right, 1, 2);
        assert_eq!(generator.generate(&add), "$5 + 3$");
    }

    #[test]
    fn test_generate_simple_subtraction() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let sub = Expr::binary_op("-", left, right, 1, 2);
        assert_eq!(generator.generate(&sub), "$5 - 3$");
    }

    #[test]
    fn test_generate_multiplication() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("4", 1, 1);
        let right = Expr::number("7", 1, 3);
        let mult = Expr::binary_op("*", left, right, 1, 2);
        assert_eq!(generator.generate(&mult), r"$4 \times 7$");
    }

    #[test]
    fn test_generate_division() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("10", 1, 1);
        let right = Expr::number("2", 1, 4);
        let div = Expr::binary_op("/", left, right, 1, 3);
        assert_eq!(generator.generate(&div), r"$10 \div 2$");
    }

    #[test]
    fn test_precedence_parens_needed_left_child() {
        // 5 3 + 2 * → ( 5 + 3 ) * 2
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", left, right, 1, 2);

        let num2 = Expr::number("2", 1, 5);
        let mult = Expr::binary_op("*", add, num2, 1, 4);

        assert_eq!(generator.generate(&mult), r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_no_parens_needed() {
        // 5 3 * 2 + → 5 * 3 + 2
        let generator = LaTeXGenerator::new();
        let left = Expr::number("5", 1, 1);
        let right = Expr::number("3", 1, 3);
        let mult = Expr::binary_op("*", left, right, 1, 2);

        let num2 = Expr::number("2", 1, 5);
        let add = Expr::binary_op("+", mult, num2, 1, 4);

        assert_eq!(generator.generate(&add), r"$5 \times 3 + 2$");
    }

    #[test]
    fn test_left_associativity_subtraction() {
        // 5 3 - 2 - → 5 - 3 - 2 (no parens on left side)
        let generator = LaTeXGenerator::new();
        let n5 = Expr::number("5", 1, 1);
        let n3 = Expr::number("3", 1, 3);
        let sub1 = Expr::binary_op("-", n5, n3, 1, 2);

        let n2 = Expr::number("2", 1, 5);
        let sub2 = Expr::binary_op("-", sub1, n2, 1, 4);

        assert_eq!(generator.generate(&sub2), "$5 - 3 - 2$");
    }

    #[test]
    fn test_right_operand_needs_parens() {
        // 2 3 4 + * → 2 * ( 3 + 4 )
        let generator = LaTeXGenerator::new();
        let num2 = Expr::number("2", 1, 1);
        let num3 = Expr::number("3", 1, 3);
        let num4 = Expr::number("4", 1, 5);
        let add = Expr::binary_op("+", num3, num4, 1, 4);
        let mult = Expr::binary_op("*", num2, add, 1, 2);

        assert_eq!(generator.generate(&mult), r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_float_numbers() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("3.14", 1, 1);
        let right = Expr::number("2", 1, 6);
        let mult = Expr::binary_op("*", left, right, 1, 5);
        assert_eq!(generator.generate(&mult), r"$3.14 \times 2$");
    }

    #[test]
    fn test_complex_expression_both_parens() {
        // 1 2 + 3 4 + * → ( 1 + 2 ) * ( 3 + 4 )
        let generator = LaTeXGenerator::new();
        let n1 = Expr::number("1", 1, 1);
        let n2 = Expr::number("2", 1, 3);
        let add1 = Expr::binary_op("+", n1, n2, 1, 2);

        let n3 = Expr::number("3", 1, 5);
        let n4 = Expr::number("4", 1, 7);
        let add2 = Expr::binary_op("+", n3, n4, 1, 6);

        let mult = Expr::binary_op("*", add1, add2, 1, 4);
        assert_eq!(generator.generate(&mult), r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_mixed_operations() {
        // 10 2 / 3 + 4 * → ( 10 / 2 + 3 ) * 4
        let generator = LaTeXGenerator::new();
        let n10 = Expr::number("10", 1, 1);
        let n2 = Expr::number("2", 1, 4);
        let div = Expr::binary_op("/", n10, n2, 1, 3);

        let n3 = Expr::number("3", 1, 6);
        let add = Expr::binary_op("+", div, n3, 1, 5);

        let n4 = Expr::number("4", 1, 8);
        let mult = Expr::binary_op("*", add, n4, 1, 7);

        assert_eq!(generator.generate(&mult), r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_left_to_right_division() {
        // 10 2 / 5 * → 10 / 2 * 5
        let generator = LaTeXGenerator::new();
        let n10 = Expr::number("10", 1, 1);
        let n2 = Expr::number("2", 1, 4);
        let div = Expr::binary_op("/", n10, n2, 1, 3);

        let n5 = Expr::number("5", 1, 6);
        let mult = Expr::binary_op("*", div, n5, 1, 5);

        assert_eq!(generator.generate(&mult), r"$10 \div 2 \times 5$");
    }

    #[test]
    fn test_multiple_divisions() {
        // 100 10 / 5 / 2 / → 100 / 10 / 5 / 2
        let generator = LaTeXGenerator::new();
        let n100 = Expr::number("100", 1, 1);
        let n10 = Expr::number("10", 1, 5);
        let div1 = Expr::binary_op("/", n100, n10, 1, 4);

        let n5 = Expr::number("5", 1, 8);
        let div2 = Expr::binary_op("/", div1, n5, 1, 7);

        let n2 = Expr::number("2", 1, 10);
        let div3 = Expr::binary_op("/", div2, n2, 1, 9);

        assert_eq!(generator.generate(&div3), r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_multiple_additions() {
        // 1 2 + 3 + 4 + → 1 + 2 + 3 + 4
        let generator = LaTeXGenerator::new();
        let n1 = Expr::number("1", 1, 1);
        let n2 = Expr::number("2", 1, 3);
        let add1 = Expr::binary_op("+", n1, n2, 1, 2);

        let n3 = Expr::number("3", 1, 5);
        let add2 = Expr::binary_op("+", add1, n3, 1, 4);

        let n4 = Expr::number("4", 1, 7);
        let add3 = Expr::binary_op("+", add2, n4, 1, 6);

        assert_eq!(generator.generate(&add3), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_precedence_mult_in_addition() {
        // 2 3 4 * + → 2 + 3 * 4
        let generator = LaTeXGenerator::new();
        let n2 = Expr::number("2", 1, 1);
        let n3 = Expr::number("3", 1, 3);
        let n4 = Expr::number("4", 1, 5);
        let mult = Expr::binary_op("*", n3, n4, 1, 4);
        let add = Expr::binary_op("+", n2, mult, 1, 2);

        assert_eq!(generator.generate(&add), r"$2 + 3 \times 4$");
    }

    #[test]
    fn test_precedence_addition_in_mult_left() {
        // 2 3 + 4 * → ( 2 + 3 ) * 4
        let generator = LaTeXGenerator::new();
        let n2 = Expr::number("2", 1, 1);
        let n3 = Expr::number("3", 1, 3);
        let add = Expr::binary_op("+", n2, n3, 1, 2);

        let n4 = Expr::number("4", 1, 5);
        let mult = Expr::binary_op("*", add, n4, 1, 4);

        assert_eq!(generator.generate(&mult), r"$( 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_no_parens_mult_left_of_addition() {
        // 2 3 * 4 + → 2 * 3 + 4
        let generator = LaTeXGenerator::new();
        let n2 = Expr::number("2", 1, 1);
        let n3 = Expr::number("3", 1, 3);
        let mult = Expr::binary_op("*", n2, n3, 1, 2);

        let n4 = Expr::number("4", 1, 5);
        let add = Expr::binary_op("+", mult, n4, 1, 4);

        assert_eq!(generator.generate(&add), r"$2 \times 3 + 4$");
    }

    #[test]
    fn test_float_addition() {
        let generator = LaTeXGenerator::new();
        let left = Expr::number("1.5", 1, 1);
        let right = Expr::number("0.5", 1, 5);
        let add = Expr::binary_op("+", left, right, 1, 4);
        assert_eq!(generator.generate(&add), "$1.5 + 0.5$");
    }

    #[test]
    fn test_default_trait() {
        let generator = LaTeXGenerator::default();
        let num = Expr::number("42", 1, 1);
        assert_eq!(generator.generate(&num), "$42$");
    }
}

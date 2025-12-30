//! LaTeX code generator for AST nodes.

use crate::ast::{BinaryOp, Expr, Number};

/// A generator that converts AST nodes to LaTeX code.
#[derive(Debug)]
pub struct LatexGenerator;

impl LatexGenerator {
    /// Creates a new LaTeX generator.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::latex::LatexGenerator;
    /// let generator = LatexGenerator::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Generates LaTeX code from an AST expression.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::latex::LatexGenerator;
    /// # use rpn2tex::ast::{Expr, Number};
    /// let generator = LatexGenerator::new();
    /// let expr = Expr::Number(Number::new("42".to_string(), 1, 1));
    /// let latex = generator.generate(&expr);
    /// assert_eq!(latex, "$42$");
    /// ```
    #[must_use]
    pub fn generate(&self, expr: &Expr) -> String {
        let content = self.visit_expr(expr);
        format!("${}$", content)
    }

    /// Visits an expression node and generates its LaTeX representation.
    #[must_use]
    fn visit_expr(&self, expr: &Expr) -> String {
        match expr {
            Expr::Number(num) => self.visit_number(num),
            Expr::BinaryOp(op) => self.visit_binary_op(op),
        }
    }

    /// Visits a Number node and generates its LaTeX representation.
    #[must_use]
    fn visit_number(&self, node: &Number) -> String {
        node.value.clone()
    }

    /// Visits a BinaryOp node and generates its LaTeX representation.
    #[must_use]
    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        let parent_precedence = self.get_precedence(&node.operator);
        let op_latex = self.operator_to_latex(&node.operator);

        let mut left = self.visit_expr(&node.left);
        if self.needs_parens(&node.left, parent_precedence, false) {
            left = format!("( {} )", left);
        }

        let mut right = self.visit_expr(&node.right);
        if self.needs_parens(&node.right, parent_precedence, true) {
            right = format!("( {} )", right);
        }

        format!("{} {} {}", left, op_latex, right)
    }

    /// Gets the precedence level of an operator.
    ///
    /// Precedence levels:
    /// - Addition (+) and Subtraction (-): precedence level 1 (lower, binds looser)
    /// - Multiplication (*) and Division (/): precedence level 2 (higher, binds tighter)
    #[must_use]
    fn get_precedence(&self, operator: &str) -> i32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child operator has lower precedence than parent
    /// 2. Equal precedence on right side for non-associative operators (- and /)
    #[must_use]
    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        match child {
            Expr::Number(_) => false,
            Expr::BinaryOp(binop) => {
                let child_precedence = self.get_precedence(&binop.operator);

                // Rule 1: Lower precedence needs parens
                if child_precedence < parent_precedence {
                    return true;
                }

                // Rule 2: Equal precedence on right for non-associative operators
                if child_precedence == parent_precedence
                    && is_right
                    && (binop.operator == "-" || binop.operator == "/")
                {
                    return true;
                }

                false
            }
        }
    }

    /// Converts an operator string to its LaTeX representation.
    #[must_use]
    fn operator_to_latex<'a>(&self, operator: &'a str) -> &'a str {
        match operator {
            "+" => "+",
            "-" => "-",
            "*" => r"\times",
            "/" => r"\div",
            _ => operator,
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
    use crate::ast::Number;

    #[test]
    fn test_generate_integer() {
        let generator = LatexGenerator::new();
        let expr = Expr::Number(Number::new("42".to_string(), 1, 1));
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$42$");
    }

    #[test]
    fn test_generate_decimal() {
        let generator = LatexGenerator::new();
        let expr = Expr::Number(Number::new("3.14".to_string(), 1, 1));
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_negative() {
        let generator = LatexGenerator::new();
        let expr = Expr::Number(Number::new("-5".to_string(), 1, 1));
        let latex = generator.generate(&expr);
        assert_eq!(latex, "$-5$");
    }
}

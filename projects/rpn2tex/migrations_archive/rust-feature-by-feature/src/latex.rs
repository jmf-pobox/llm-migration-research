//! LaTeX generator for rpn2tex - converts AST to LaTeX.
//!
//! This module converts the AST into LaTeX math mode output.
//!
//! # Key concepts
//!
//! * Visitor pattern for traversing AST nodes
//! * Operator precedence for parenthesization
//! * LaTeX math mode output
//!
//! The generator produces infix notation from the parsed RPN tree.

use crate::ast::{BinaryOp, Expr, Number};
use std::collections::HashMap;

/// Converts rpn2tex AST to LaTeX source code.
///
/// Manages operator precedence to insert parentheses only where needed.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
/// use rpn2tex::latex::LaTeXGenerator;
///
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// let ast = Parser::new(tokens).parse().unwrap();
/// let latex = LaTeXGenerator::new().generate(&ast);
/// assert_eq!(latex, "$5 + 3$");
/// ```
pub struct LaTeXGenerator {
    binary_ops: HashMap<String, String>,
    precedence: HashMap<String, i32>,
}

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

    /// Generates LaTeX from AST.
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
    /// let ast = Parser::new(tokens).parse().unwrap();
    /// let latex = LaTeXGenerator::new().generate(&ast);
    /// assert_eq!(latex, "$5$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &Expr) -> String {
        let content = self.visit(ast);
        format!("${content}$")
    }

    fn visit(&self, node: &Expr) -> String {
        match node {
            Expr::Number(num) => self.visit_number(num),
            Expr::BinaryOp(op) => self.visit_binary_op(op),
        }
    }

    fn visit_number(&self, node: &Number) -> String {
        node.value.clone()
    }

    fn visit_binary_op(&self, node: &BinaryOp) -> String {
        let op_latex = self.binary_ops.get(&node.operator).unwrap();
        let my_precedence = *self.precedence.get(&node.operator).unwrap();

        // Generate left operand, adding parens if needed
        let mut left = self.visit(&node.left);
        if self.needs_parens(&node.left, my_precedence, false) {
            left = format!("( {left} )");
        }

        // Generate right operand, adding parens if needed
        let mut right = self.visit(&node.right);
        if self.needs_parens(&node.right, my_precedence, true) {
            right = format!("( {right} )");
        }

        format!("{left} {op_latex} {right}")
    }

    fn needs_parens(&self, child: &Expr, parent_precedence: i32, is_right: bool) -> bool {
        if let Expr::BinaryOp(child_op) = child {
            let child_precedence = *self.precedence.get(&child_op.operator).unwrap();

            // Lower precedence always needs parens
            if child_precedence < parent_precedence {
                return true;
            }

            // Equal precedence on right side needs parens for non-commutative operators
            // (handles left-associativity of - and /)
            child_precedence == parent_precedence
                && is_right
                && (child_op.operator == "-" || child_op.operator == "/")
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
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn test_generate_simple_number() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$5$");
    }

    #[test]
    fn test_generate_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_generate_multiplication() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        let latex = LaTeXGenerator::new().generate(&ast);
        assert_eq!(latex, r"$4 \times 7$");
    }
}

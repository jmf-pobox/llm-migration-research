//! LaTeX code generator for AST nodes.

use crate::ast::ASTNode;

/// A LaTeX generator that converts an AST into LaTeX code.
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::{ASTNode, Number};
/// use rpn2tex::latex::Generator;
///
/// let node = ASTNode::Number(Number::new("42", 1, 1));
/// let generator = Generator::new();
/// let latex = generator.generate(&[node]).unwrap();
/// assert_eq!(latex, "$42$");
/// ```
#[derive(Debug, Default)]
pub struct Generator;

impl Generator {
    /// Creates a new LaTeX generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Returns the precedence level for an operator.
    ///
    /// Precedence levels:
    /// - 1: + and - (lowest)
    /// - 2: * and / (highest)
    #[must_use]
    fn precedence(&self, operator: &str) -> i32 {
        match operator {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. Child has lower precedence than parent
    /// 2. Child has equal precedence and is on the right side
    ///    (for left-associative operators like - and /)
    #[must_use]
    fn needs_parens(&self, child: &ASTNode, parent_precedence: i32, is_right: bool) -> bool {
        // Numbers never need parens
        let ASTNode::BinaryOp(child_binop) = child else {
            return false;
        };

        let child_precedence = self.precedence(child_binop.operator());

        // Rule 1: Lower precedence always needs parens
        if child_precedence < parent_precedence {
            return true;
        }

        // Rule 2: Equal precedence on right side needs parens for
        // non-commutative operators (- and /)
        child_precedence == parent_precedence
            && is_right
            && matches!(child_binop.operator(), "-" | "/")
    }

    /// Generates LaTeX code from an AST.
    ///
    /// Wraps the result in math mode ($...$).
    ///
    /// # Errors
    ///
    /// Returns an error if the AST is malformed or empty.
    pub fn generate(&self, ast: &[ASTNode]) -> Result<String, String> {
        if ast.is_empty() {
            return Ok(String::new());
        }

        // For a valid RPN expression, we should have exactly one node left
        if ast.len() == 1 {
            let latex = self.visit(&ast[0]);
            Ok(format!("${latex}$"))
        } else {
            Err("Multiple root nodes - expression may be incomplete".to_string())
        }
    }

    fn visit(&self, node: &ASTNode) -> String {
        match node {
            ASTNode::Number(num) => self.visit_number(num),
            ASTNode::BinaryOp(binop) => self.visit_binary_op(binop),
        }
    }

    fn visit_number(&self, node: &crate::ast::Number) -> String {
        node.value().to_string()
    }

    fn visit_binary_op(&self, node: &crate::ast::BinaryOp) -> String {
        let operator = node.operator();
        let my_precedence = self.precedence(operator);

        // Generate left operand, adding parens if needed
        let mut left = self.visit(node.left());
        if self.needs_parens(node.left(), my_precedence, false) {
            left = format!("( {left} )");
        }

        // Generate right operand, adding parens if needed
        let mut right = self.visit(node.right());
        if self.needs_parens(node.right(), my_precedence, true) {
            right = format!("( {right} )");
        }

        // Map operators to LaTeX equivalents
        let latex_op = match operator {
            "*" => r"\times",
            "/" => r"\div",
            _ => operator,
        };

        // Add spacing around the operator
        format!("{left} {latex_op} {right}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Number;

    #[test]
    fn test_generate_number() {
        let node = ASTNode::Number(Number::new("42", 1, 1));
        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, "$42$");
    }

    #[test]
    fn test_generate_decimal() {
        let node = ASTNode::Number(Number::new("3.14", 1, 1));
        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, "$3.14$");
    }

    #[test]
    fn test_generate_empty() {
        let generator = Generator::new();
        let latex = generator.generate(&[]).unwrap();
        assert_eq!(latex, "");
    }

    #[test]
    fn test_generate_addition() {
        use crate::ast::{BinaryOp, Number};

        let left = ASTNode::Number(Number::new("5", 1, 1));
        let right = ASTNode::Number(Number::new("3", 1, 3));
        let node = ASTNode::BinaryOp(BinaryOp::new("+", left, right, 1, 5));

        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, "$5 + 3$");
    }

    #[test]
    fn test_generate_chained_addition() {
        use crate::ast::{BinaryOp, Number};

        // Build: 1 2 + 3 + => ((1 + 2) + 3)
        let one = ASTNode::Number(Number::new("1", 1, 1));
        let two = ASTNode::Number(Number::new("2", 1, 3));
        let three = ASTNode::Number(Number::new("3", 1, 7));

        let inner = ASTNode::BinaryOp(BinaryOp::new("+", one, two, 1, 5));
        let outer = ASTNode::BinaryOp(BinaryOp::new("+", inner, three, 1, 9));

        let generator = Generator::new();
        let latex = generator.generate(&[outer]).unwrap();
        assert_eq!(latex, "$1 + 2 + 3$");
    }

    #[test]
    fn test_generate_subtraction() {
        use crate::ast::{BinaryOp, Number};

        let left = ASTNode::Number(Number::new("5", 1, 1));
        let right = ASTNode::Number(Number::new("3", 1, 3));
        let node = ASTNode::BinaryOp(BinaryOp::new("-", left, right, 1, 5));

        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, "$5 - 3$");
    }

    #[test]
    fn test_generate_chained_subtraction() {
        use crate::ast::{BinaryOp, Number};

        // Build: 5 3 - 2 - => ((5 - 3) - 2)
        let five = ASTNode::Number(Number::new("5", 1, 1));
        let three = ASTNode::Number(Number::new("3", 1, 3));
        let two = ASTNode::Number(Number::new("2", 1, 7));

        let inner = ASTNode::BinaryOp(BinaryOp::new("-", five, three, 1, 5));
        let outer = ASTNode::BinaryOp(BinaryOp::new("-", inner, two, 1, 9));

        let generator = Generator::new();
        let latex = generator.generate(&[outer]).unwrap();
        assert_eq!(latex, "$5 - 3 - 2$");
    }

    #[test]
    fn test_generate_negative_number() {
        use crate::ast::Number;

        let node = ASTNode::Number(Number::new("-5", 1, 1));
        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, "$-5$");
    }

    #[test]
    fn test_generate_division() {
        use crate::ast::{BinaryOp, Number};

        let left = ASTNode::Number(Number::new("10", 1, 1));
        let right = ASTNode::Number(Number::new("2", 1, 4));
        let node = ASTNode::BinaryOp(BinaryOp::new("/", left, right, 1, 6));

        let generator = Generator::new();
        let latex = generator.generate(&[node]).unwrap();
        assert_eq!(latex, r"$10 \div 2$");
    }

    #[test]
    fn test_generate_chained_division() {
        use crate::ast::{BinaryOp, Number};

        // Build: 100 10 / 5 / 2 / => (((100 / 10) / 5) / 2)
        let hundred = ASTNode::Number(Number::new("100", 1, 1));
        let ten = ASTNode::Number(Number::new("10", 1, 5));
        let five = ASTNode::Number(Number::new("5", 1, 10));
        let two = ASTNode::Number(Number::new("2", 1, 14));

        let inner = ASTNode::BinaryOp(BinaryOp::new("/", hundred, ten, 1, 8));
        let mid = ASTNode::BinaryOp(BinaryOp::new("/", inner, five, 1, 12));
        let outer = ASTNode::BinaryOp(BinaryOp::new("/", mid, two, 1, 16));

        let generator = Generator::new();
        let latex = generator.generate(&[outer]).unwrap();
        assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
    }

    #[test]
    fn test_precedence_addition_then_multiplication() {
        use crate::ast::{BinaryOp, Number};

        // Build: 5 3 + 2 * => (5 + 3) * 2
        let five = ASTNode::Number(Number::new("5", 1, 1));
        let three = ASTNode::Number(Number::new("3", 1, 3));
        let two = ASTNode::Number(Number::new("2", 1, 7));

        let add = ASTNode::BinaryOp(BinaryOp::new("+", five, three, 1, 5));
        let mul = ASTNode::BinaryOp(BinaryOp::new("*", add, two, 1, 9));

        let generator = Generator::new();
        let latex = generator.generate(&[mul]).unwrap();
        assert_eq!(latex, r"$( 5 + 3 ) \times 2$");
    }

    #[test]
    fn test_precedence_multiplication_then_addition() {
        use crate::ast::{BinaryOp, Number};

        // Build: 2 3 4 + * => 2 * (3 + 4)
        let two = ASTNode::Number(Number::new("2", 1, 1));
        let three = ASTNode::Number(Number::new("3", 1, 3));
        let four = ASTNode::Number(Number::new("4", 1, 5));

        let add = ASTNode::BinaryOp(BinaryOp::new("+", three, four, 1, 7));
        let mul = ASTNode::BinaryOp(BinaryOp::new("*", two, add, 1, 9));

        let generator = Generator::new();
        let latex = generator.generate(&[mul]).unwrap();
        assert_eq!(latex, r"$2 \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_complex_expression() {
        use crate::ast::{BinaryOp, Number};

        // Build: 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
        let one = ASTNode::Number(Number::new("1", 1, 1));
        let two = ASTNode::Number(Number::new("2", 1, 3));
        let three = ASTNode::Number(Number::new("3", 1, 7));
        let four = ASTNode::Number(Number::new("4", 1, 9));

        let add1 = ASTNode::BinaryOp(BinaryOp::new("+", one, two, 1, 5));
        let add2 = ASTNode::BinaryOp(BinaryOp::new("+", three, four, 1, 11));
        let mul = ASTNode::BinaryOp(BinaryOp::new("*", add1, add2, 1, 13));

        let generator = Generator::new();
        let latex = generator.generate(&[mul]).unwrap();
        assert_eq!(latex, r"$( 1 + 2 ) \times ( 3 + 4 )$");
    }

    #[test]
    fn test_precedence_division_addition() {
        use crate::ast::{BinaryOp, Number};

        // Build: 10 2 / 3 + 4 * => ((10 / 2) + 3) * 4
        let ten = ASTNode::Number(Number::new("10", 1, 1));
        let two = ASTNode::Number(Number::new("2", 1, 4));
        let three = ASTNode::Number(Number::new("3", 1, 8));
        let four = ASTNode::Number(Number::new("4", 1, 12));

        let div = ASTNode::BinaryOp(BinaryOp::new("/", ten, two, 1, 6));
        let add = ASTNode::BinaryOp(BinaryOp::new("+", div, three, 1, 10));
        let mul = ASTNode::BinaryOp(BinaryOp::new("*", add, four, 1, 14));

        let generator = Generator::new();
        let latex = generator.generate(&[mul]).unwrap();
        assert_eq!(latex, r"$( 10 \div 2 + 3 ) \times 4$");
    }

    #[test]
    fn test_precedence_no_parens_needed() {
        use crate::ast::{BinaryOp, Number};

        // Build: 2 3 * 4 + => (2 * 3) + 4 (no parens needed)
        let two = ASTNode::Number(Number::new("2", 1, 1));
        let three = ASTNode::Number(Number::new("3", 1, 3));
        let four = ASTNode::Number(Number::new("4", 1, 7));

        let mul = ASTNode::BinaryOp(BinaryOp::new("*", two, three, 1, 5));
        let add = ASTNode::BinaryOp(BinaryOp::new("+", mul, four, 1, 9));

        let generator = Generator::new();
        let latex = generator.generate(&[add]).unwrap();
        assert_eq!(latex, r"$2 \times 3 + 4$");
    }
}

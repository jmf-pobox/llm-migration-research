//! LaTeX generation from AST nodes.
//!
//! This module converts Abstract Syntax Tree nodes into LaTeX math mode strings
//! with proper operator precedence and parenthesization.

use crate::ast::ASTNode;

/// Generator that converts AST to LaTeX format.
///
/// The generator uses a visitor pattern to traverse the AST and generates
/// infix notation (the opposite of input RPN). It handles operator precedence
/// and adds parentheses when necessary to preserve the correct order of operations.
///
/// # Operator Precedence
///
/// - Addition/Subtraction: precedence level 1 (lower)
/// - Multiplication/Division: precedence level 2 (higher)
///
/// # Parenthesization Rules
///
/// Parentheses are added when:
/// 1. A child expression has lower precedence than its parent
/// 2. A child is on the right side of a non-commutative operator (-, /) with equal precedence
///
/// # LaTeX Output Format
///
/// - Operators: ` + `, ` - `, ` \times `, ` \div ` (with spaces)
/// - Parentheses: `( expr )` (with spaces inside)
/// - Math mode: `$...$` (inline math)
/// - Numbers: rendered as-is (e.g., "42", "3.14", "-5")
///
/// # Examples
///
/// ```
/// use rpn2tex::ast::ASTNode;
/// use rpn2tex::latex::LatexGenerator;
///
/// let generator = LatexGenerator::new();
///
/// // Simple addition: 5 + 3
/// let left = ASTNode::number("5", 1, 1);
/// let right = ASTNode::number("3", 1, 3);
/// let ast = ASTNode::binary_op("+", left, right, 1, 5);
/// assert_eq!(generator.generate(&ast), "$5 + 3$");
///
/// // With parentheses: (5 + 3) * 2
/// let five = ASTNode::number("5", 1, 1);
/// let three = ASTNode::number("3", 1, 3);
/// let sum = ASTNode::binary_op("+", five, three, 1, 5);
/// let two = ASTNode::number("2", 1, 7);
/// let product = ASTNode::binary_op("*", sum, two, 1, 9);
/// assert_eq!(generator.generate(&product), "$( 5 + 3 ) \\times 2$");
/// ```
#[derive(Debug, Clone)]
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
    pub const fn new() -> Self {
        Self
    }

    /// Generates LaTeX string from an AST node.
    ///
    /// The generated LaTeX is wrapped in `$...$` delimiters for inline math mode.
    ///
    /// # Arguments
    ///
    /// * `ast` - The root AST node to convert
    ///
    /// # Returns
    ///
    /// A LaTeX string like `"$5 + 3$"`
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::ast::ASTNode;
    /// use rpn2tex::latex::LatexGenerator;
    ///
    /// let generator = LatexGenerator::new();
    /// let num = ASTNode::number("42", 1, 1);
    /// assert_eq!(generator.generate(&num), "$42$");
    /// ```
    #[must_use]
    pub fn generate(&self, ast: &ASTNode) -> String {
        let content = self.generate_node(ast, None);
        format!("${}$", content)
    }

    /// Recursively generates LaTeX for an AST node.
    ///
    /// This is the core visitor method that handles both Number and BinaryOp nodes.
    ///
    /// # Arguments
    ///
    /// * `node` - The AST node to convert
    /// * `_parent_op` - The parent operator (for precedence checking), if any
    ///
    /// # Returns
    ///
    /// LaTeX string for the node (without math mode delimiters)
    fn generate_node(&self, node: &ASTNode, _parent_op: Option<&str>) -> String {
        match node {
            ASTNode::Number { value, .. } => value.clone(),
            ASTNode::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                let op_latex = self.operator_to_latex(operator);
                let my_precedence = self.precedence(operator);

                // Generate left operand
                let left_str = self.generate_node(left, Some(operator));
                let left_with_parens = if self.needs_parens(left, my_precedence, false) {
                    format!("( {} )", left_str)
                } else {
                    left_str
                };

                // Generate right operand
                let right_str = self.generate_node(right, Some(operator));
                let right_with_parens = if self.needs_parens(right, my_precedence, true) {
                    format!("( {} )", right_str)
                } else {
                    right_str
                };

                format!("{} {} {}", left_with_parens, op_latex, right_with_parens)
            }
        }
    }

    /// Converts an operator string to its LaTeX representation.
    ///
    /// # Arguments
    ///
    /// * `op` - The operator string ("+", "-", "*", or "/")
    ///
    /// # Returns
    ///
    /// The LaTeX representation of the operator
    fn operator_to_latex(&self, op: &str) -> &'static str {
        match op {
            "+" => "+",
            "-" => "-",
            "*" => "\\times",
            "/" => "\\div",
            _ => unreachable!("Invalid operator: {}", op), // Shouldn't happen with valid AST
        }
    }

    /// Returns the precedence level of an operator.
    ///
    /// Higher values mean tighter binding (evaluated first).
    ///
    /// # Arguments
    ///
    /// * `op` - The operator string
    ///
    /// # Returns
    ///
    /// Precedence level: 1 for +/-, 2 for */
    fn precedence(&self, op: &str) -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0, // Fallback
        }
    }

    /// Determines if a child expression needs parentheses.
    ///
    /// Parentheses are needed when:
    /// 1. The child has lower precedence than the parent
    /// 2. The child is on the right side of a non-commutative operator (-, /)
    ///    with equal precedence (to enforce left-associativity)
    ///
    /// # Arguments
    ///
    /// * `child` - The child AST node
    /// * `parent_precedence` - The precedence of the parent operator
    /// * `is_right` - Whether this child is the right operand
    ///
    /// # Returns
    ///
    /// `true` if parentheses are needed, `false` otherwise
    fn needs_parens(&self, child: &ASTNode, parent_precedence: i32, is_right: bool) -> bool {
        // Numbers never need parentheses
        let child_op = match child.as_operator() {
            Some(op) => op,
            None => return false,
        };

        let child_precedence = self.precedence(child_op);

        // Lower precedence always needs parens
        if child_precedence < parent_precedence {
            return true;
        }

        // Equal precedence on right side of non-commutative operator needs parens
        // This handles cases like: 5 - (3 - 2) and 10 / (5 / 2)
        if child_precedence == parent_precedence && is_right {
            matches!(child_op, "-" | "/")
        } else {
            false
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

    #[test]
    fn test_simple_number() {
        let gen = LatexGenerator::new();
        let num = ASTNode::number("42", 1, 1);
        assert_eq!(gen.generate(&num), "$42$");
    }

    #[test]
    fn test_floating_point_number() {
        let gen = LatexGenerator::new();
        let num = ASTNode::number("3.14", 1, 1);
        assert_eq!(gen.generate(&num), "$3.14$");
    }

    #[test]
    fn test_negative_number() {
        let gen = LatexGenerator::new();
        let num = ASTNode::number("-5", 1, 1);
        assert_eq!(gen.generate(&num), "$-5$");
    }

    #[test]
    fn test_simple_addition() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr = ASTNode::binary_op("+", left, right, 1, 5);
        assert_eq!(gen.generate(&expr), "$5 + 3$");
    }

    #[test]
    fn test_simple_subtraction() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let expr = ASTNode::binary_op("-", left, right, 1, 5);
        assert_eq!(gen.generate(&expr), "$5 - 3$");
    }

    #[test]
    fn test_simple_multiplication() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("4", 1, 1);
        let right = ASTNode::number("7", 1, 3);
        let expr = ASTNode::binary_op("*", left, right, 1, 5);
        assert_eq!(gen.generate(&expr), "$4 \\times 7$");
    }

    #[test]
    fn test_simple_division() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("10", 1, 1);
        let right = ASTNode::number("2", 1, 4);
        let expr = ASTNode::binary_op("/", left, right, 1, 6);
        assert_eq!(gen.generate(&expr), "$10 \\div 2$");
    }

    #[test]
    fn test_precedence_addition_times_multiplication() {
        // (5 + 3) * 2
        let gen = LatexGenerator::new();
        let five = ASTNode::number("5", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let sum = ASTNode::binary_op("+", five, three, 1, 5);
        let two = ASTNode::number("2", 1, 7);
        let product = ASTNode::binary_op("*", sum, two, 1, 9);
        assert_eq!(gen.generate(&product), "$( 5 + 3 ) \\times 2$");
    }

    #[test]
    fn test_precedence_multiplication_plus_addition() {
        // 5 * 3 + 2 (no parens needed)
        let gen = LatexGenerator::new();
        let five = ASTNode::number("5", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let product = ASTNode::binary_op("*", five, three, 1, 5);
        let two = ASTNode::number("2", 1, 7);
        let sum = ASTNode::binary_op("+", product, two, 1, 9);
        assert_eq!(gen.generate(&sum), "$5 \\times 3 + 2$");
    }

    #[test]
    fn test_left_associative_division() {
        // 10 / 2 * 5 => (10 / 2) * 5
        let gen = LatexGenerator::new();
        let ten = ASTNode::number("10", 1, 1);
        let two = ASTNode::number("2", 1, 4);
        let div = ASTNode::binary_op("/", ten, two, 1, 6);
        let five = ASTNode::number("5", 1, 8);
        let product = ASTNode::binary_op("*", div, five, 1, 10);
        assert_eq!(gen.generate(&product), "$10 \\div 2 \\times 5$");
    }

    #[test]
    fn test_left_associative_subtraction() {
        // 5 - 3 - 2 => (5 - 3) - 2
        let gen = LatexGenerator::new();
        let five = ASTNode::number("5", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let first_sub = ASTNode::binary_op("-", five, three, 1, 5);
        let two = ASTNode::number("2", 1, 7);
        let second_sub = ASTNode::binary_op("-", first_sub, two, 1, 9);
        assert_eq!(gen.generate(&second_sub), "$5 - 3 - 2$");
    }

    #[test]
    fn test_chained_division() {
        // 100 / 10 / 5 / 2 => ((100 / 10) / 5) / 2
        let gen = LatexGenerator::new();
        let hundred = ASTNode::number("100", 1, 1);
        let ten = ASTNode::number("10", 1, 5);
        let div1 = ASTNode::binary_op("/", hundred, ten, 1, 9);
        let five = ASTNode::number("5", 1, 13);
        let div2 = ASTNode::binary_op("/", div1, five, 1, 17);
        let two = ASTNode::number("2", 1, 21);
        let div3 = ASTNode::binary_op("/", div2, two, 1, 25);
        assert_eq!(gen.generate(&div3), "$100 \\div 10 \\div 5 \\div 2$");
    }

    #[test]
    fn test_chained_addition() {
        // 1 + 2 + 3 + 4
        let gen = LatexGenerator::new();
        let one = ASTNode::number("1", 1, 1);
        let two = ASTNode::number("2", 1, 3);
        let add1 = ASTNode::binary_op("+", one, two, 1, 5);
        let three = ASTNode::number("3", 1, 7);
        let add2 = ASTNode::binary_op("+", add1, three, 1, 9);
        let four = ASTNode::number("4", 1, 11);
        let add3 = ASTNode::binary_op("+", add2, four, 1, 13);
        assert_eq!(gen.generate(&add3), "$1 + 2 + 3 + 4$");
    }

    #[test]
    fn test_precedence_addition_after_multiplication() {
        // 2 + 3 * 4 (no parens needed)
        let gen = LatexGenerator::new();
        let three = ASTNode::number("3", 1, 3);
        let four = ASTNode::number("4", 1, 5);
        let product = ASTNode::binary_op("*", three, four, 1, 7);
        let two = ASTNode::number("2", 1, 1);
        let sum = ASTNode::binary_op("+", two, product, 1, 9);
        assert_eq!(gen.generate(&sum), "$2 + 3 \\times 4$");
    }

    #[test]
    fn test_precedence_multiplication_with_right_addition() {
        // 2 * (3 + 4)
        let gen = LatexGenerator::new();
        let three = ASTNode::number("3", 1, 3);
        let four = ASTNode::number("4", 1, 5);
        let sum = ASTNode::binary_op("+", three, four, 1, 7);
        let two = ASTNode::number("2", 1, 1);
        let product = ASTNode::binary_op("*", two, sum, 1, 9);
        assert_eq!(gen.generate(&product), "$2 \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_multiplication_then_addition() {
        // 2 * 3 + 4
        let gen = LatexGenerator::new();
        let two = ASTNode::number("2", 1, 1);
        let three = ASTNode::number("3", 1, 3);
        let product = ASTNode::binary_op("*", two, three, 1, 5);
        let four = ASTNode::number("4", 1, 7);
        let sum = ASTNode::binary_op("+", product, four, 1, 9);
        assert_eq!(gen.generate(&sum), "$2 \\times 3 + 4$");
    }

    #[test]
    fn test_floating_point_multiplication() {
        let gen = LatexGenerator::new();
        let pi = ASTNode::number("3.14", 1, 1);
        let two = ASTNode::number("2", 1, 6);
        let product = ASTNode::binary_op("*", pi, two, 1, 8);
        assert_eq!(gen.generate(&product), "$3.14 \\times 2$");
    }

    #[test]
    fn test_floating_point_addition() {
        let gen = LatexGenerator::new();
        let one_half = ASTNode::number("1.5", 1, 1);
        let point_five = ASTNode::number("0.5", 1, 5);
        let sum = ASTNode::binary_op("+", one_half, point_five, 1, 9);
        assert_eq!(gen.generate(&sum), "$1.5 + 0.5$");
    }

    #[test]
    fn test_multiple_subexpressions() {
        // (1 + 2) * (3 + 4)
        let gen = LatexGenerator::new();
        let one = ASTNode::number("1", 1, 1);
        let two = ASTNode::number("2", 1, 3);
        let left_sum = ASTNode::binary_op("+", one, two, 1, 5);
        let three = ASTNode::number("3", 1, 7);
        let four = ASTNode::number("4", 1, 9);
        let right_sum = ASTNode::binary_op("+", three, four, 1, 11);
        let product = ASTNode::binary_op("*", left_sum, right_sum, 1, 13);
        assert_eq!(gen.generate(&product), "$( 1 + 2 ) \\times ( 3 + 4 )$");
    }

    #[test]
    fn test_complex_expression() {
        // (10 / 2 + 3) * 4
        let gen = LatexGenerator::new();
        let ten = ASTNode::number("10", 1, 1);
        let two = ASTNode::number("2", 1, 4);
        let div = ASTNode::binary_op("/", ten, two, 1, 6);
        let three = ASTNode::number("3", 1, 8);
        let sum = ASTNode::binary_op("+", div, three, 1, 10);
        let four = ASTNode::number("4", 1, 12);
        let product = ASTNode::binary_op("*", sum, four, 1, 14);
        assert_eq!(gen.generate(&product), "$( 10 \\div 2 + 3 ) \\times 4$");
    }

    #[test]
    fn test_right_associative_subtraction_needs_parens() {
        // 5 - (3 - 2)
        let gen = LatexGenerator::new();
        let three = ASTNode::number("3", 1, 3);
        let two = ASTNode::number("2", 1, 5);
        let inner_sub = ASTNode::binary_op("-", three, two, 1, 7);
        let five = ASTNode::number("5", 1, 1);
        let outer_sub = ASTNode::binary_op("-", five, inner_sub, 1, 9);
        assert_eq!(gen.generate(&outer_sub), "$5 - ( 3 - 2 )$");
    }

    #[test]
    fn test_right_associative_division_needs_parens() {
        // 10 / (5 / 2)
        let gen = LatexGenerator::new();
        let five = ASTNode::number("5", 1, 4);
        let two = ASTNode::number("2", 1, 6);
        let inner_div = ASTNode::binary_op("/", five, two, 1, 8);
        let ten = ASTNode::number("10", 1, 1);
        let outer_div = ASTNode::binary_op("/", ten, inner_div, 1, 10);
        assert_eq!(gen.generate(&outer_div), "$10 \\div ( 5 \\div 2 )$");
    }

    #[test]
    fn test_default_trait() {
        let gen = LatexGenerator::default();
        let num = ASTNode::number("5", 1, 1);
        assert_eq!(gen.generate(&num), "$5$");
    }

    #[test]
    fn test_precedence_function() {
        let gen = LatexGenerator::new();
        assert_eq!(gen.precedence("+"), 1);
        assert_eq!(gen.precedence("-"), 1);
        assert_eq!(gen.precedence("*"), 2);
        assert_eq!(gen.precedence("/"), 2);
    }

    #[test]
    fn test_operator_to_latex() {
        let gen = LatexGenerator::new();
        assert_eq!(gen.operator_to_latex("+"), "+");
        assert_eq!(gen.operator_to_latex("-"), "-");
        assert_eq!(gen.operator_to_latex("*"), "\\times");
        assert_eq!(gen.operator_to_latex("/"), "\\div");
    }

    #[test]
    fn test_needs_parens_number_node() {
        let gen = LatexGenerator::new();
        let num = ASTNode::number("5", 1, 1);
        assert!(!gen.needs_parens(&num, 1, false));
        assert!(!gen.needs_parens(&num, 2, true));
    }

    #[test]
    fn test_needs_parens_lower_precedence() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let add = ASTNode::binary_op("+", left, right, 1, 5);

        // Addition (prec 1) inside multiplication (prec 2) needs parens
        assert!(gen.needs_parens(&add, 2, false));
        assert!(gen.needs_parens(&add, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_left_side() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let sub = ASTNode::binary_op("-", left, right, 1, 5);

        // Subtraction on left side of subtraction doesn't need parens (left-associative)
        assert!(!gen.needs_parens(&sub, 1, false));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_side_subtraction() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("3", 1, 1);
        let right = ASTNode::number("2", 1, 3);
        let sub = ASTNode::binary_op("-", left, right, 1, 5);

        // Subtraction on right side of subtraction needs parens
        assert!(gen.needs_parens(&sub, 1, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_side_division() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("2", 1, 3);
        let div = ASTNode::binary_op("/", left, right, 1, 5);

        // Division on right side of division needs parens
        assert!(gen.needs_parens(&div, 2, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_side_addition() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let add = ASTNode::binary_op("+", left, right, 1, 5);

        // Addition on right side of addition doesn't need parens (commutative)
        assert!(!gen.needs_parens(&add, 1, true));
    }

    #[test]
    fn test_needs_parens_equal_precedence_right_side_multiplication() {
        let gen = LatexGenerator::new();
        let left = ASTNode::number("5", 1, 1);
        let right = ASTNode::number("3", 1, 3);
        let mult = ASTNode::binary_op("*", left, right, 1, 5);

        // Multiplication on right side of multiplication doesn't need parens (commutative)
        assert!(!gen.needs_parens(&mult, 2, true));
    }
}

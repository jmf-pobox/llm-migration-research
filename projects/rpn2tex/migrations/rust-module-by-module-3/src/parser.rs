//! Parser for converting token streams to Abstract Syntax Trees.
//!
//! This module implements a stack-based RPN (Reverse Polish Notation) parser that
//! converts a sequence of tokens into an abstract syntax tree representation.
//!
//! # Algorithm
//!
//! The parser uses a classic stack-based algorithm for RPN:
//! 1. Numbers are pushed onto the stack as Number nodes
//! 2. Operators pop two operands (right first, then left) and create BinaryOp nodes
//! 3. At EOF, exactly one node should remain on the stack
//!
//! # Examples
//!
//! ```
//! use rpn2tex::parser::Parser;
//! use rpn2tex::tokens::{Token, TokenType};
//!
//! // Parse "5 3 +"
//! let tokens = vec![
//!     Token::new(TokenType::Number, "5".to_string(), 1, 1),
//!     Token::new(TokenType::Number, "3".to_string(), 1, 3),
//!     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
//!     Token::new_without_value(TokenType::Eof, 1, 6),
//! ];
//!
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().unwrap();
//! assert_eq!(ast.as_operator(), Some("+"));
//! ```

use crate::ast::ASTNode;
use crate::tokens::{Token, TokenType};

/// Parser for converting token streams to ASTs using RPN algorithm.
///
/// The parser maintains an internal stack and processes tokens sequentially:
/// - Numbers are converted to Number nodes and pushed
/// - Operators pop two operands and create BinaryOp nodes
/// - Final validation ensures exactly one node remains
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::Parser;
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let tokens = vec![
///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
///     Token::new_without_value(TokenType::Eof, 1, 6),
/// ];
///
/// let mut parser = Parser::new(tokens);
/// let result = parser.parse();
/// assert!(result.is_ok());
/// ```
#[derive(Debug)]
pub struct Parser {
    /// The list of tokens to parse
    tokens: Vec<Token>,
    /// Current position in the token list (0-based)
    position: usize,
}

impl Parser {
    /// Creates a new parser with the given token list.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of tokens from the lexer, must end with EOF
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42".to_string(), 1, 1),
    ///     Token::new_without_value(TokenType::Eof, 1, 3),
    /// ];
    ///
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse the token stream into an AST.
    ///
    /// Uses stack-based RPN algorithm to build the abstract syntax tree.
    ///
    /// # Returns
    ///
    /// * `Ok(ASTNode)` - The root node of the parsed AST
    /// * `Err(String)` - Error message if parsing fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Input is empty
    /// - Operators have insufficient operands
    /// - Too many values remain on stack (missing operators)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// // Valid input: "5 3 +"
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
    ///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
    ///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
    ///     Token::new_without_value(TokenType::Eof, 1, 6),
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// assert!(parser.parse().is_ok());
    ///
    /// // Invalid input: "5 3" (missing operator)
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
    ///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
    ///     Token::new_without_value(TokenType::Eof, 1, 5),
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// assert!(parser.parse().is_err());
    /// ```
    pub fn parse(&mut self) -> Result<ASTNode, String> {
        let mut stack: Vec<ASTNode> = Vec::new();

        // Process tokens until EOF
        while !self.at_end() {
            let token = self.current_token();

            match token.token_type() {
                TokenType::Number => {
                    // Create number node and push to stack
                    let value = token.value().unwrap_or("").to_string();
                    let node = ASTNode::number(value, token.line() as u32, token.column() as u32);
                    stack.push(node);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Multiply | TokenType::Divide => {
                    // Check we have enough operands
                    if stack.len() < 2 {
                        let op_str = match token.token_type() {
                            TokenType::Plus => "+",
                            TokenType::Minus => "-",
                            TokenType::Multiply => "*",
                            TokenType::Divide => "/",
                            _ => unreachable!(),
                        };
                        return Err(format!("Operator '{}' requires two operands", op_str));
                    }

                    // Pop operands (RIGHT FIRST, then LEFT - critical for non-commutative ops)
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Get operator string
                    let operator = match token.token_type() {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Multiply => "*",
                        TokenType::Divide => "/",
                        _ => unreachable!(),
                    };

                    // Create binary operation node
                    let node = ASTNode::binary_op(
                        operator,
                        left,
                        right,
                        token.line() as u32,
                        token.column() as u32,
                    );
                    stack.push(node);
                    self.advance();
                }
                TokenType::Eof => {
                    break;
                }
            }
        }

        // Validate final state
        if stack.is_empty() {
            return Err("Empty expression".to_string());
        }

        if stack.len() > 1 {
            return Err(format!(
                "Invalid RPN: {} values remain on stack (missing operators?)",
                stack.len()
            ));
        }

        // Return the single remaining node
        Ok(stack.pop().unwrap())
    }

    /// Returns the current token without consuming it.
    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    /// Checks if we've reached the end of the token stream.
    fn at_end(&self) -> bool {
        self.current_token().token_type() == TokenType::Eof
    }

    /// Advances to the next token.
    fn advance(&mut self) {
        if !self.at_end() {
            self.position += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_addition() {
        // "5 3 +"
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new_without_value(TokenType::Eof, 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("+"));
        assert_eq!(ast.left().unwrap().as_number(), Some("5"));
        assert_eq!(ast.right().unwrap().as_number(), Some("3"));
    }

    #[test]
    fn test_parse_simple_subtraction() {
        // "5 3 -"
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
            Token::new_without_value(TokenType::Eof, 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("-"));
        assert_eq!(ast.left().unwrap().as_number(), Some("5"));
        assert_eq!(ast.right().unwrap().as_number(), Some("3"));
    }

    #[test]
    fn test_parse_simple_multiplication() {
        // "4 7 *"
        let tokens = vec![
            Token::new(TokenType::Number, "4".to_string(), 1, 1),
            Token::new(TokenType::Number, "7".to_string(), 1, 3),
            Token::new(TokenType::Multiply, "*".to_string(), 1, 5),
            Token::new_without_value(TokenType::Eof, 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("*"));
        assert_eq!(ast.left().unwrap().as_number(), Some("4"));
        assert_eq!(ast.right().unwrap().as_number(), Some("7"));
    }

    #[test]
    fn test_parse_simple_division() {
        // "10 2 /"
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 4),
            Token::new(TokenType::Divide, "/".to_string(), 1, 6),
            Token::new_without_value(TokenType::Eof, 1, 7),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("/"));
        assert_eq!(ast.left().unwrap().as_number(), Some("10"));
        assert_eq!(ast.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_single_number() {
        // "5"
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new_without_value(TokenType::Eof, 1, 2),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_number(), Some("5"));
    }

    #[test]
    fn test_parse_nested_expression() {
        // "5 3 + 2 *" => (5+3)*2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Multiply, "*".to_string(), 1, 9),
            Token::new_without_value(TokenType::Eof, 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Root should be multiplication
        assert_eq!(ast.as_operator(), Some("*"));

        // Left child should be addition
        let left = ast.left().unwrap();
        assert_eq!(left.as_operator(), Some("+"));
        assert_eq!(left.left().unwrap().as_number(), Some("5"));
        assert_eq!(left.right().unwrap().as_number(), Some("3"));

        // Right child should be number 2
        assert_eq!(ast.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_left_associativity_subtraction() {
        // "5 3 - 2 -" => (5-3)-2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Minus, "-".to_string(), 1, 9),
            Token::new_without_value(TokenType::Eof, 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Root should be subtraction
        assert_eq!(ast.as_operator(), Some("-"));

        // Left child should be subtraction (5-3)
        let left = ast.left().unwrap();
        assert_eq!(left.as_operator(), Some("-"));
        assert_eq!(left.left().unwrap().as_number(), Some("5"));
        assert_eq!(left.right().unwrap().as_number(), Some("3"));

        // Right child should be number 2
        assert_eq!(ast.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_left_associativity_division() {
        // "100 10 / 5 / 2 /" => ((100/10)/5)/2
        let tokens = vec![
            Token::new(TokenType::Number, "100".to_string(), 1, 1),
            Token::new(TokenType::Number, "10".to_string(), 1, 5),
            Token::new(TokenType::Divide, "/".to_string(), 1, 8),
            Token::new(TokenType::Number, "5".to_string(), 1, 10),
            Token::new(TokenType::Divide, "/".to_string(), 1, 12),
            Token::new(TokenType::Number, "2".to_string(), 1, 14),
            Token::new(TokenType::Divide, "/".to_string(), 1, 16),
            Token::new_without_value(TokenType::Eof, 1, 17),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Verify structure: root is division
        assert_eq!(ast.as_operator(), Some("/"));

        // Left is also division
        let left1 = ast.left().unwrap();
        assert_eq!(left1.as_operator(), Some("/"));

        // Left-left is also division
        let left2 = left1.left().unwrap();
        assert_eq!(left2.as_operator(), Some("/"));
        assert_eq!(left2.left().unwrap().as_number(), Some("100"));
        assert_eq!(left2.right().unwrap().as_number(), Some("10"));
    }

    #[test]
    fn test_parse_chained_addition() {
        // "1 2 + 3 + 4 +" => ((1+2)+3)+4
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "3".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
            Token::new(TokenType::Number, "4".to_string(), 1, 11),
            Token::new(TokenType::Plus, "+".to_string(), 1, 13),
            Token::new_without_value(TokenType::Eof, 1, 14),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Root is addition
        assert_eq!(ast.as_operator(), Some("+"));
        assert_eq!(ast.right().unwrap().as_number(), Some("4"));

        // Left is addition
        let left1 = ast.left().unwrap();
        assert_eq!(left1.as_operator(), Some("+"));
        assert_eq!(left1.right().unwrap().as_number(), Some("3"));

        // Left-left is addition
        let left2 = left1.left().unwrap();
        assert_eq!(left2.as_operator(), Some("+"));
        assert_eq!(left2.left().unwrap().as_number(), Some("1"));
        assert_eq!(left2.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_complex_expression() {
        // "2 3 4 * +" => 2+(3*4)
        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Number, "4".to_string(), 1, 5),
            Token::new(TokenType::Multiply, "*".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
            Token::new_without_value(TokenType::Eof, 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Root is addition
        assert_eq!(ast.as_operator(), Some("+"));
        assert_eq!(ast.left().unwrap().as_number(), Some("2"));

        // Right is multiplication
        let right = ast.right().unwrap();
        assert_eq!(right.as_operator(), Some("*"));
        assert_eq!(right.left().unwrap().as_number(), Some("3"));
        assert_eq!(right.right().unwrap().as_number(), Some("4"));
    }

    #[test]
    fn test_parse_floating_point() {
        // "3.14 2 *"
        let tokens = vec![
            Token::new(TokenType::Number, "3.14".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 6),
            Token::new(TokenType::Multiply, "*".to_string(), 1, 8),
            Token::new_without_value(TokenType::Eof, 1, 9),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("*"));
        assert_eq!(ast.left().unwrap().as_number(), Some("3.14"));
        assert_eq!(ast.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_negative_numbers() {
        // "-5 3 +"
        let tokens = vec![
            Token::new(TokenType::Number, "-5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 4),
            Token::new(TokenType::Plus, "+".to_string(), 1, 6),
            Token::new_without_value(TokenType::Eof, 1, 7),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.as_operator(), Some("+"));
        assert_eq!(ast.left().unwrap().as_number(), Some("-5"));
        assert_eq!(ast.right().unwrap().as_number(), Some("3"));
    }

    #[test]
    fn test_parse_empty_expression_error() {
        // Just EOF
        let tokens = vec![Token::new_without_value(TokenType::Eof, 1, 1)];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Empty expression");
    }

    #[test]
    fn test_parse_missing_operator_error() {
        // "5 3" (missing operator)
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new_without_value(TokenType::Eof, 1, 5),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid RPN: 2 values remain on stack (missing operators?)"
        );
    }

    #[test]
    fn test_parse_insufficient_operands_error() {
        // "5 3 + +" (extra operator)
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Plus, "+".to_string(), 1, 7),
            Token::new_without_value(TokenType::Eof, 1, 8),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Operator '+' requires two operands");
    }

    #[test]
    fn test_parse_operator_order_for_subtraction() {
        // Test that operands are popped in correct order for subtraction
        // "10 3 -" should give (10-3), not (3-10)
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 4),
            Token::new(TokenType::Minus, "-".to_string(), 1, 6),
            Token::new_without_value(TokenType::Eof, 1, 7),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Verify left is 10 and right is 3
        assert_eq!(ast.left().unwrap().as_number(), Some("10"));
        assert_eq!(ast.right().unwrap().as_number(), Some("3"));
    }

    #[test]
    fn test_parse_operator_order_for_division() {
        // Test that operands are popped in correct order for division
        // "10 2 /" should give (10/2), not (2/10)
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 4),
            Token::new(TokenType::Divide, "/".to_string(), 1, 6),
            Token::new_without_value(TokenType::Eof, 1, 7),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Verify left is 10 and right is 2
        assert_eq!(ast.left().unwrap().as_number(), Some("10"));
        assert_eq!(ast.right().unwrap().as_number(), Some("2"));
    }

    #[test]
    fn test_parse_multiple_subexpressions() {
        // "1 2 + 3 4 + *" => (1+2)*(3+4)
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "3".to_string(), 1, 7),
            Token::new(TokenType::Number, "4".to_string(), 1, 9),
            Token::new(TokenType::Plus, "+".to_string(), 1, 11),
            Token::new(TokenType::Multiply, "*".to_string(), 1, 13),
            Token::new_without_value(TokenType::Eof, 1, 14),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Root is multiplication
        assert_eq!(ast.as_operator(), Some("*"));

        // Left is (1+2)
        let left = ast.left().unwrap();
        assert_eq!(left.as_operator(), Some("+"));
        assert_eq!(left.left().unwrap().as_number(), Some("1"));
        assert_eq!(left.right().unwrap().as_number(), Some("2"));

        // Right is (3+4)
        let right = ast.right().unwrap();
        assert_eq!(right.as_operator(), Some("+"));
        assert_eq!(right.left().unwrap().as_number(), Some("3"));
        assert_eq!(right.right().unwrap().as_number(), Some("4"));
    }

    #[test]
    fn test_parse_position_preservation() {
        // Verify that position information is preserved in AST nodes
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new_without_value(TokenType::Eof, 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Check operator position
        assert_eq!(ast.line(), 1);
        assert_eq!(ast.column(), 5);

        // Check left operand position
        let left = ast.left().unwrap();
        assert_eq!(left.line(), 1);
        assert_eq!(left.column(), 1);

        // Check right operand position
        let right = ast.right().unwrap();
        assert_eq!(right.line(), 1);
        assert_eq!(right.column(), 3);
    }

    #[test]
    fn test_parse_all_operators() {
        // Test each operator type
        let operators = [
            (TokenType::Plus, "+"),
            (TokenType::Minus, "-"),
            (TokenType::Multiply, "*"),
            (TokenType::Divide, "/"),
        ];

        for (token_type, op_str) in operators {
            let tokens = vec![
                Token::new(TokenType::Number, "5".to_string(), 1, 1),
                Token::new(TokenType::Number, "3".to_string(), 1, 3),
                Token::new(token_type, op_str.to_string(), 1, 5),
                Token::new_without_value(TokenType::Eof, 1, 6),
            ];

            let mut parser = Parser::new(tokens);
            let ast = parser.parse().unwrap();
            assert_eq!(ast.as_operator(), Some(op_str));
        }
    }

    #[test]
    fn test_parse_insufficient_operands_for_each_operator() {
        let operators = [
            (TokenType::Plus, "+"),
            (TokenType::Minus, "-"),
            (TokenType::Multiply, "*"),
            (TokenType::Divide, "/"),
        ];

        for (token_type, op_str) in operators {
            // Single number followed by operator
            let tokens = vec![
                Token::new(TokenType::Number, "5".to_string(), 1, 1),
                Token::new(token_type, op_str.to_string(), 1, 3),
                Token::new_without_value(TokenType::Eof, 1, 4),
            ];

            let mut parser = Parser::new(tokens);
            let result = parser.parse();
            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                format!("Operator '{}' requires two operands", op_str)
            );
        }
    }
}

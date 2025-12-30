//! RPN Parser for converting token streams to Abstract Syntax Trees.
//!
//! This module implements a stack-based parser for Reverse Polish Notation (RPN)
//! expressions. The parser consumes a stream of tokens and constructs an Abstract
//! Syntax Tree (AST) representing the mathematical expression.
//!
//! # RPN Parsing Algorithm
//!
//! The parser uses a stack-based algorithm:
//! 1. Process tokens left to right
//! 2. For numbers: push onto stack as Number nodes
//! 3. For operators: pop two operands, create BinaryOp, push result
//! 4. At the end: stack should contain exactly one node (the root)
//!
//! # Examples
//!
//! ```
//! use rpn2tex::{Token, TokenType, Parser};
//!
//! let tokens = vec![
//!     Token::new(TokenType::Number, "5".to_string(), 1, 1),
//!     Token::new(TokenType::Number, "3".to_string(), 1, 3),
//!     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
//!     Token::new(TokenType::Eof, "".to_string(), 1, 6),
//! ];
//!
//! let parser = Parser::new(tokens);
//! let ast = parser.parse().expect("Failed to parse");
//! ```

use crate::ast::{AstNode, Operator};
use crate::error::Rpn2TexError;
use crate::tokens::{Token, TokenType};

/// Parser for converting token streams into Abstract Syntax Trees.
///
/// The parser implements a stack-based RPN evaluation algorithm that constructs
/// an AST by processing tokens from left to right.
#[derive(Debug)]
pub struct Parser {
    /// The token stream to parse
    tokens: Vec<Token>,
    /// Current position in the token stream
    position: usize,
}

impl Parser {
    /// Creates a new parser with the given token stream.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of tokens to parse (must end with EOF token)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Token, TokenType, Parser};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42".to_string(), 1, 1),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 3),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parses the token stream into an Abstract Syntax Tree.
    ///
    /// This is the main entry point for parsing. It processes all tokens using
    /// a stack-based RPN algorithm and returns the root AST node.
    ///
    /// # Errors
    ///
    /// Returns `Rpn2TexError::ParserError` if:
    /// - The expression is empty
    /// - An operator has insufficient operands
    /// - Too many operands remain after parsing (missing operators)
    /// - An unexpected token is encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Token, TokenType, Parser};
    ///
    /// // Parse "5 3 +"
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
    ///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
    ///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 6),
    /// ];
    ///
    /// let parser = Parser::new(tokens);
    /// let ast = parser.parse().expect("Parse should succeed");
    /// ```
    pub fn parse(mut self) -> Result<AstNode, Rpn2TexError> {
        let mut stack: Vec<AstNode> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.token_type {
                TokenType::Number => {
                    // Parse the number from the lexeme
                    let value = token.lexeme.parse::<f64>().map_err(|_| {
                        Rpn2TexError::parser_error(
                            format!("Invalid number format: '{}'", token.lexeme),
                            token.line,
                            token.column,
                        )
                    })?;

                    stack.push(AstNode::number(value));
                    self.advance();
                }

                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    // Operators require two operands
                    if stack.len() < 2 {
                        return Err(Rpn2TexError::parser_error(
                            format!(
                                "Operator '{}' requires two operands, but only {} available",
                                token.lexeme,
                                stack.len()
                            ),
                            token.line,
                            token.column,
                        ));
                    }

                    // Pop right operand first, then left (RPN stack order)
                    let right = stack.pop().unwrap(); // Safe: checked length above
                    let left = stack.pop().unwrap(); // Safe: checked length above

                    // Map token type to operator
                    let operator = self.token_to_operator(&token.token_type);

                    // Create binary operation node
                    let binary_op = AstNode::binary_op(left, operator, right);
                    stack.push(binary_op);
                    self.advance();
                }

                TokenType::Eof => {
                    break;
                }

                _ => {
                    return Err(Rpn2TexError::parser_error(
                        format!("Unexpected token: '{}'", token.lexeme),
                        token.line,
                        token.column,
                    ));
                }
            }
        }

        // Validate final stack state
        if stack.is_empty() {
            return Err(Rpn2TexError::parser_error("Empty expression", 1, 1));
        }

        if stack.len() > 1 {
            // Get position of first remaining token for error reporting
            // Create a default token to avoid borrow checker issues
            let default_token = Token::new(TokenType::Eof, String::new(), 1, 1);
            let first_token = self.tokens.first().unwrap_or(&default_token);
            return Err(Rpn2TexError::parser_error(
                format!(
                    "Invalid RPN expression: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                first_token.line,
                first_token.column,
            ));
        }

        // Return the root node
        Ok(stack.pop().unwrap()) // Safe: checked that stack has exactly one element
    }

    /// Maps a token type to its corresponding operator.
    fn token_to_operator(&self, token_type: &TokenType) -> Operator {
        match token_type {
            TokenType::Plus => Operator::Add,
            TokenType::Minus => Operator::Subtract,
            TokenType::Star => Operator::Multiply,
            TokenType::Slash => Operator::Divide,
            _ => unreachable!("token_to_operator called with non-operator token"),
        }
    }

    /// Returns the current token without advancing.
    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    /// Advances to the next token.
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    /// Checks if we've reached the end of the token stream.
    fn at_end(&self) -> bool {
        self.position >= self.tokens.len() || self.current().token_type == TokenType::Eof
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![
            Token::new(TokenType::Number, "42".to_string(), 1, 1),
            Token::new(TokenType::Eof, "".to_string(), 1, 3),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        assert!(matches!(ast, AstNode::Number(x) if (x - 42.0).abs() < 1e-10));
    }

    #[test]
    fn test_parse_floating_point() {
        let tokens = vec![
            Token::new(TokenType::Number, "3.14".to_string(), 1, 1),
            Token::new(TokenType::Eof, "".to_string(), 1, 5),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        assert!(matches!(ast, AstNode::Number(x) if (x - 3.14).abs() < 1e-10));
    }

    #[test]
    fn test_parse_negative_number() {
        let tokens = vec![
            Token::new(TokenType::Number, "-5".to_string(), 1, 1),
            Token::new(TokenType::Eof, "".to_string(), 1, 3),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        assert!(matches!(ast, AstNode::Number(x) if (x + 5.0).abs() < 1e-10));
    }

    #[test]
    fn test_parse_simple_addition() {
        // 5 3 +
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Eof, "".to_string(), 1, 6),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        match ast {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator, Operator::Add);
                assert!(matches!(*left, AstNode::Number(x) if (x - 5.0).abs() < 1e-10));
                assert!(matches!(*right, AstNode::Number(x) if (x - 3.0).abs() < 1e-10));
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_simple_subtraction() {
        // 5 3 -
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
            Token::new(TokenType::Eof, "".to_string(), 1, 6),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        match ast {
            AstNode::BinaryOp { operator, .. } => {
                assert_eq!(operator, Operator::Subtract);
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_simple_multiplication() {
        // 4 7 *
        let tokens = vec![
            Token::new(TokenType::Number, "4".to_string(), 1, 1),
            Token::new(TokenType::Number, "7".to_string(), 1, 3),
            Token::new(TokenType::Star, "*".to_string(), 1, 5),
            Token::new(TokenType::Eof, "".to_string(), 1, 6),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        match ast {
            AstNode::BinaryOp { operator, .. } => {
                assert_eq!(operator, Operator::Multiply);
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_simple_division() {
        // 10 2 /
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 4),
            Token::new(TokenType::Slash, "/".to_string(), 1, 6),
            Token::new(TokenType::Eof, "".to_string(), 1, 7),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        match ast {
            AstNode::BinaryOp { operator, .. } => {
                assert_eq!(operator, Operator::Divide);
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // 5 3 + 2 * => (5 + 3) * 2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Star, "*".to_string(), 1, 9),
            Token::new(TokenType::Eof, "".to_string(), 1, 10),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        // Root should be multiplication
        match ast {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator, Operator::Multiply);

                // Left should be addition (5 + 3)
                match *left {
                    AstNode::BinaryOp {
                        operator: left_op, ..
                    } => {
                        assert_eq!(left_op, Operator::Add);
                    }
                    _ => panic!("Expected left to be BinaryOp"),
                }

                // Right should be number 2
                assert!(matches!(*right, AstNode::Number(x) if (x - 2.0).abs() < 1e-10));
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_multiple_operations() {
        // 2 3 4 * + => 2 + (3 * 4)
        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Number, "4".to_string(), 1, 5),
            Token::new(TokenType::Star, "*".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
            Token::new(TokenType::Eof, "".to_string(), 1, 10),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        // Root should be addition
        match ast {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator, Operator::Add);

                // Left should be number 2
                assert!(matches!(*left, AstNode::Number(x) if (x - 2.0).abs() < 1e-10));

                // Right should be multiplication (3 * 4)
                match *right {
                    AstNode::BinaryOp {
                        operator: right_op, ..
                    } => {
                        assert_eq!(right_op, Operator::Multiply);
                    }
                    _ => panic!("Expected right to be BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_error_empty_expression() {
        let tokens = vec![Token::new(TokenType::Eof, "".to_string(), 1, 1)];

        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { message, .. }) => {
                assert!(message.contains("Empty expression"));
            }
            _ => panic!("Expected ParserError with empty expression message"),
        }
    }

    #[test]
    fn test_parse_error_insufficient_operands() {
        // 3 + (only one operand for binary operator)
        let tokens = vec![
            Token::new(TokenType::Number, "3".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 3),
            Token::new(TokenType::Eof, "".to_string(), 1, 4),
        ];

        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError {
                message,
                line,
                column,
            }) => {
                assert!(message.contains("requires two operands"));
                assert_eq!(line, 1);
                assert_eq!(column, 3);
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_parse_error_too_many_operands() {
        // 5 3 2 (three operands, no operator)
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Number, "2".to_string(), 1, 5),
            Token::new(TokenType::Eof, "".to_string(), 1, 6),
        ];

        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { message, .. }) => {
                assert!(message.contains("3 values remain on stack"));
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_parse_error_invalid_number_format() {
        let tokens = vec![
            Token::new(TokenType::Number, "not_a_number".to_string(), 1, 1),
            Token::new(TokenType::Eof, "".to_string(), 1, 13),
        ];

        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        match result {
            Err(Rpn2TexError::ParserError { message, .. }) => {
                assert!(message.contains("Invalid number format"));
            }
            _ => panic!("Expected ParserError"),
        }
    }

    #[test]
    fn test_parse_chained_operations() {
        // 5 3 - 2 - => (5 - 3) - 2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Minus, "-".to_string(), 1, 9),
            Token::new(TokenType::Eof, "".to_string(), 1, 10),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        // Verify the tree structure: outer subtraction
        match ast {
            AstNode::BinaryOp { left, operator, .. } => {
                assert_eq!(operator, Operator::Subtract);
                // Left child should also be subtraction
                assert!(matches!(
                    *left,
                    AstNode::BinaryOp {
                        operator: Operator::Subtract,
                        ..
                    }
                ));
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_both_operands_expressions() {
        // 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "3".to_string(), 1, 7),
            Token::new(TokenType::Number, "4".to_string(), 1, 9),
            Token::new(TokenType::Plus, "+".to_string(), 1, 11),
            Token::new(TokenType::Star, "*".to_string(), 1, 13),
            Token::new(TokenType::Eof, "".to_string(), 1, 14),
        ];

        let parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse successfully");

        // Root should be multiplication
        match ast {
            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                assert_eq!(operator, Operator::Multiply);

                // Both children should be additions
                assert!(matches!(
                    *left,
                    AstNode::BinaryOp {
                        operator: Operator::Add,
                        ..
                    }
                ));
                assert!(matches!(
                    *right,
                    AstNode::BinaryOp {
                        operator: Operator::Add,
                        ..
                    }
                ));
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
}

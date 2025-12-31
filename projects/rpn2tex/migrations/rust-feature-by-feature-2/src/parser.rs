//! Parser for RPN expressions.
//!
//! This module provides a parser that converts tokens into an Abstract Syntax Tree (AST).

use crate::ast::{BinaryOp, Expr, Number};
use crate::error::ParserError;
use crate::tokens::{Token, TokenType};

/// A parser that converts tokens into an AST.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Creates a new parser for the given tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![Token::new(TokenType::Number, "5".to_string(), 1, 1)];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parses the tokens into an AST.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![Token::new(TokenType::Number, "5".to_string(), 1, 1)];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse().unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if the input is empty or invalid.
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.token_type {
                TokenType::Number => {
                    let num_node = Number::new(token.value.clone(), token.line, token.column);
                    stack.push(Expr::Number(num_node));
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::InsufficientOperands {
                            operator: token.value.clone(),
                            line: token.line,
                            column: token.column,
                        });
                    }

                    // Pop RIGHT operand first, then LEFT operand
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Star => "*",
                        TokenType::Slash => "/",
                        _ => unreachable!(),
                    };

                    let op_node = BinaryOp::new(
                        operator.to_string(),
                        Box::new(left),
                        Box::new(right),
                        token.line,
                        token.column,
                    );
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
            }
        }

        stack.pop().ok_or(ParserError::EmptyInput)
    }

    fn at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    fn current(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![Token::new(TokenType::Number, "5".to_string(), 1, 1)];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::Number(num) => {
                assert_eq!(num.value, "5");
                assert_eq!(num.line, 1);
                assert_eq!(num.column, 1);
            }
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_parse_float() {
        let tokens = vec![Token::new(TokenType::Number, "3.14".to_string(), 1, 1)];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::Number(num) => assert_eq!(num.value, "3.14"),
            _ => panic!("Expected Number variant"),
        }
    }

    #[test]
    fn test_parse_empty_input() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result {
            Err(ParserError::EmptyInput) => {}
            _ => panic!("Expected EmptyInput error"),
        }
    }

    #[test]
    fn test_parse_addition() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::Number(right)) => {
                        assert_eq!(left.value, "5");
                        assert_eq!(right.value, "3");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_addition() {
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "3".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be: (1 + 2) + 3
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match *op.left {
                    Expr::BinaryOp(left_op) => {
                        assert_eq!(left_op.operator, "+");
                    }
                    _ => panic!("Expected BinaryOp for left operand"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result {
            Err(ParserError::InsufficientOperands { operator, .. }) => {
                assert_eq!(operator, "+");
            }
            _ => panic!("Expected InsufficientOperands error"),
        }
    }

    #[test]
    fn test_parse_subtraction() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "-");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::Number(right)) => {
                        assert_eq!(left.value, "5");
                        assert_eq!(right.value, "3");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_subtraction() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Minus, "-".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Minus, "-".to_string(), 1, 9),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be: (5 - 3) - 2
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "-");
                match *op.left {
                    Expr::BinaryOp(left_op) => {
                        assert_eq!(left_op.operator, "-");
                    }
                    _ => panic!("Expected BinaryOp for left operand"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_subtraction_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Minus, "-".to_string(), 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result {
            Err(ParserError::InsufficientOperands { operator, .. }) => {
                assert_eq!(operator, "-");
            }
            _ => panic!("Expected InsufficientOperands error"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        let tokens = vec![
            Token::new(TokenType::Number, "4".to_string(), 1, 1),
            Token::new(TokenType::Number, "7".to_string(), 1, 3),
            Token::new(TokenType::Star, "*".to_string(), 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "*");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::Number(right)) => {
                        assert_eq!(left.value, "4");
                        assert_eq!(right.value, "7");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_float_multiplication() {
        let tokens = vec![
            Token::new(TokenType::Number, "3.14".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 6),
            Token::new(TokenType::Star, "*".to_string(), 1, 8),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "*");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::Number(right)) => {
                        assert_eq!(left.value, "3.14");
                        assert_eq!(right.value, "2");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Star, "*".to_string(), 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result {
            Err(ParserError::InsufficientOperands { operator, .. }) => {
                assert_eq!(operator, "*");
            }
            _ => panic!("Expected InsufficientOperands error"),
        }
    }

    #[test]
    fn test_parse_mixed_operations() {
        // Test: 2 3 4 * +
        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Number, "4".to_string(), 1, 5),
            Token::new(TokenType::Star, "*".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be: 2 + (3 * 4)
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::BinaryOp(right_op)) => {
                        assert_eq!(left.value, "2");
                        assert_eq!(right_op.operator, "*");
                    }
                    _ => panic!("Expected Number and BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division() {
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 4),
            Token::new(TokenType::Slash, "/".to_string(), 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "/");
                match (*op.left, *op.right) {
                    (Expr::Number(left), Expr::Number(right)) => {
                        assert_eq!(left.value, "10");
                        assert_eq!(right.value, "2");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_division() {
        let tokens = vec![
            Token::new(TokenType::Number, "100".to_string(), 1, 1),
            Token::new(TokenType::Number, "10".to_string(), 1, 5),
            Token::new(TokenType::Slash, "/".to_string(), 1, 8),
            Token::new(TokenType::Number, "5".to_string(), 1, 10),
            Token::new(TokenType::Slash, "/".to_string(), 1, 12),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be: (100 / 10) / 5
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "/");
                match *op.left {
                    Expr::BinaryOp(left_op) => {
                        assert_eq!(left_op.operator, "/");
                    }
                    _ => panic!("Expected BinaryOp for left operand"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Slash, "/".to_string(), 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result {
            Err(ParserError::InsufficientOperands { operator, .. }) => {
                assert_eq!(operator, "/");
            }
            _ => panic!("Expected InsufficientOperands error"),
        }
    }

    #[test]
    fn test_parse_mixed_division_multiplication() {
        // Test: 10 2 / 5 *
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 4),
            Token::new(TokenType::Slash, "/".to_string(), 1, 6),
            Token::new(TokenType::Number, "5".to_string(), 1, 8),
            Token::new(TokenType::Star, "*".to_string(), 1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be: (10 / 2) * 5
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "*");
                match *op.left {
                    Expr::BinaryOp(left_op) => {
                        assert_eq!(left_op.operator, "/");
                    }
                    _ => panic!("Expected BinaryOp for left operand"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}

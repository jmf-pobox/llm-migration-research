//! RPN parser for converting token streams to AST.
//!
//! This module implements a stack-based parser for Reverse Polish Notation (RPN).
//! It takes a sequence of tokens and builds an Abstract Syntax Tree (AST) representing
//! the mathematical expression.
//!
//! # Algorithm
//!
//! The parser uses a stack-based algorithm:
//! 1. Numbers are pushed onto the stack
//! 2. Operators pop two operands from the stack and push the result
//! 3. At the end, exactly one expression should remain on the stack
//!
//! # Examples
//!
//! ```
//! use rpn2tex::tokens::{Token, TokenType};
//! use rpn2tex::parser::Parser;
//!
//! let tokens = vec![
//!     Token::new(TokenType::Number, "5".to_string(), 1, 1),
//!     Token::new(TokenType::Number, "3".to_string(), 1, 3),
//!     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
//!     Token::new(TokenType::Eof, "".to_string(), 1, 6),
//! ];
//!
//! let parser = Parser::new(tokens);
//! let expr = parser.parse().unwrap();
//! ```

use crate::ast::Expr;
use crate::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Error type for parser failures.
///
/// Contains the error message and the token where the error occurred.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// The error message
    pub message: String,
    /// The token where the error occurred
    pub token: Token,
}

impl ParserError {
    /// Creates a new parser error.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `token` - The token where the error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    /// use rpn2tex::parser::ParserError;
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
    /// let error = ParserError::new("Operator requires two operands".to_string(), token);
    /// ```
    #[must_use]
    pub fn new(message: String, token: Token) -> Self {
        Self { message, token }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.token.line, self.token.column
        )
    }
}

impl Error for ParserError {}

/// Stack-based parser for RPN expressions.
///
/// Converts a stream of tokens into an Abstract Syntax Tree (AST) using
/// a stack-based algorithm suitable for postfix notation.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
/// use rpn2tex::parser::Parser;
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
/// let expr = parser.parse().unwrap();
/// assert_eq!(expr.line(), 1);
/// assert_eq!(expr.column(), 5);
/// ```
#[derive(Debug, Clone)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser from a vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The token stream to parse
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    /// use rpn2tex::parser::Parser;
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42".to_string(), 1, 1),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 3),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream into an AST expression.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if:
    /// - The expression is empty
    /// - An operator doesn't have enough operands
    /// - Too many values remain on the stack (missing operators)
    /// - An unexpected token is encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    /// use rpn2tex::parser::Parser;
    ///
    /// // Valid expression: "5 3 +"
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
    ///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
    ///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 6),
    /// ];
    /// let parser = Parser::new(tokens);
    /// let result = parser.parse();
    /// assert!(result.is_ok());
    ///
    /// // Invalid expression: "+" (missing operands)
    /// let tokens = vec![
    ///     Token::new(TokenType::Plus, "+".to_string(), 1, 1),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 2),
    /// ];
    /// let parser = Parser::new(tokens);
    /// let result = parser.parse();
    /// assert!(result.is_err());
    /// ```
    pub fn parse(mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.type_ {
                TokenType::Number => {
                    let node = Expr::number(token.value.clone(), token.line, token.column);
                    stack.push(node);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    if stack.len() < 2 {
                        let operator = token.value.clone();
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", operator),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let operator = match token.type_ {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    let node = Expr::binary_op(operator, left, right, token.line, token.column);
                    stack.push(node);
                    self.advance();
                }
                TokenType::Eof => {
                    break;
                }
            }
        }

        // Final validation
        if stack.is_empty() {
            let eof_token = if self.pos < self.tokens.len() {
                self.tokens[self.pos].clone()
            } else {
                Token::new(TokenType::Eof, String::new(), 1, 1)
            };
            return Err(ParserError::new("Empty expression".to_string(), eof_token));
        }

        if stack.len() > 1 {
            let eof_token = if self.pos < self.tokens.len() {
                self.tokens[self.pos].clone()
            } else {
                Token::new(TokenType::Eof, String::new(), 1, 1)
            };
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token,
            ));
        }

        Ok(stack.pop().unwrap())
    }

    /// Returns a reference to the current token.
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Checks if the parser has reached the end of the token stream.
    #[must_use]
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.current().type_ == TokenType::Eof
    }

    /// Advances to the next token and returns the previous one.
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![
            Token::new(TokenType::Number, "42".to_string(), 1, 1),
            Token::new(TokenType::Eof, String::new(), 1, 3),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        let expr = result.unwrap();
        assert!(matches!(expr, Expr::Number { .. }));
    }

    #[test]
    fn test_parse_simple_addition() {
        // "5 3 +"
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Eof, String::new(), 1, 6),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        let expr = result.unwrap();
        assert!(matches!(expr, Expr::BinaryOp { .. }));
    }

    #[test]
    fn test_parse_nested_expression() {
        // "5 3 + 2 *" -> (5 + 3) * 2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Mult, "*".to_string(), 1, 9),
            Token::new(TokenType::Eof, String::new(), 1, 10),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                assert!(matches!(*left, Expr::BinaryOp { .. }));
                assert!(matches!(*right, Expr::Number { .. }));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_operator_without_operands() {
        let tokens = vec![
            Token::new(TokenType::Plus, "+".to_string(), 1, 1),
            Token::new(TokenType::Eof, String::new(), 1, 2),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_parse_operator_with_one_operand() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 3),
            Token::new(TokenType::Eof, String::new(), 1, 4),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_parse_too_many_values() {
        // "5 3" - missing operator
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Eof, String::new(), 1, 5),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("2 values remain on stack"));
    }

    #[test]
    fn test_parse_empty_expression() {
        let tokens = vec![Token::new(TokenType::Eof, String::new(), 1, 1)];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Empty expression"));
    }

    #[test]
    fn test_parse_all_operators() {
        let operators = vec![
            (TokenType::Plus, "+"),
            (TokenType::Minus, "-"),
            (TokenType::Mult, "*"),
            (TokenType::Div, "/"),
        ];

        for (token_type, op_str) in operators {
            let tokens = vec![
                Token::new(TokenType::Number, "5".to_string(), 1, 1),
                Token::new(TokenType::Number, "3".to_string(), 1, 3),
                Token::new(token_type, op_str.to_string(), 1, 5),
                Token::new(TokenType::Eof, String::new(), 1, 6),
            ];
            let parser = Parser::new(tokens);
            let result = parser.parse();
            assert!(result.is_ok());
            let expr = result.unwrap();
            match expr {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, op_str),
                _ => panic!("Expected BinaryOp"),
            }
        }
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        let error = ParserError::new("Operator requires two operands".to_string(), token);
        let display = format!("{}", error);
        assert!(display.contains("Operator requires two operands"));
        assert!(display.contains("line 2"));
        assert!(display.contains("column 5"));
    }

    #[test]
    fn test_parser_error_clone() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 1);
        let error1 = ParserError::new("Test error".to_string(), token);
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_complex_nested_expression() {
        // "1 2 + 3 4 + *" -> (1 + 2) * (3 + 4)
        let tokens = vec![
            Token::new(TokenType::Number, "1".to_string(), 1, 1),
            Token::new(TokenType::Number, "2".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "3".to_string(), 1, 7),
            Token::new(TokenType::Number, "4".to_string(), 1, 9),
            Token::new(TokenType::Plus, "+".to_string(), 1, 11),
            Token::new(TokenType::Mult, "*".to_string(), 1, 13),
            Token::new(TokenType::Eof, String::new(), 1, 14),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                assert!(matches!(*left, Expr::BinaryOp { .. }));
                assert!(matches!(*right, Expr::BinaryOp { .. }));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_position_information_preserved() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 2, 3),
            Token::new(TokenType::Number, "3".to_string(), 2, 5),
            Token::new(TokenType::Plus, "+".to_string(), 2, 7),
            Token::new(TokenType::Eof, String::new(), 2, 8),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
        let expr = result.unwrap();
        assert_eq!(expr.line(), 2);
        assert_eq!(expr.column(), 7);
    }
}

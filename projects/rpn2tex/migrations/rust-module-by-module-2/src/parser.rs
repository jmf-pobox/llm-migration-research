//! RPN parser that converts token streams into Abstract Syntax Trees.
//!
//! This module implements a stack-based parser for Reverse Polish Notation (RPN)
//! expressions. The parser consumes a token stream produced by the lexer and
//! constructs an Abstract Syntax Tree (AST) representing the mathematical expression.
//!
//! # Algorithm
//!
//! The parser uses the classic RPN evaluation algorithm:
//! 1. Initialize an empty stack
//! 2. For each token:
//!    - If NUMBER: push a Number node onto the stack
//!    - If OPERATOR: pop two operands, create a BinaryOp node, push it back
//!    - If EOF: stop processing
//! 3. Verify the stack has exactly one element (the root expression)
//!
//! # Examples
//!
//! ```
//! use rpn2tex::parser::Parser;
//! use rpn2tex::tokens::{Token, TokenType};
//! use rpn2tex::ast::Expr;
//!
//! let tokens = vec![
//!     Token::new(TokenType::Number, "5".to_string(), 1, 1),
//!     Token::new(TokenType::Number, "3".to_string(), 1, 3),
//!     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
//!     Token::new(TokenType::Eof, String::new(), 1, 6),
//! ];
//!
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().expect("parse failed");
//!
//! // The AST should be a BinaryOp with 5 and 3 as operands
//! match ast {
//!     Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
//!     _ => panic!("Expected BinaryOp"),
//! }
//! ```

use crate::ast::Expr;
use crate::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Error type for parser failures.
///
/// Represents errors that occur during RPN parsing, such as:
/// - Too few operands for an operator
/// - Empty expressions
/// - Missing operators (too many values remaining)
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::ParserError;
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
/// let error = ParserError::new("Too few operands for operator", token);
///
/// assert_eq!(error.token().line, 1);
/// assert_eq!(error.token().column, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Error message describing what went wrong
    message: String,
    /// The token where the error occurred
    token: Token,
}

impl ParserError {
    /// Creates a new parser error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `token` - The token where the error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Eof, String::new(), 1, 10);
    /// let error = ParserError::new("Empty expression", token);
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, token: Token) -> Self {
        Self {
            message: message.into(),
            token,
        }
    }

    /// Returns a reference to the error message.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Eof, String::new(), 1, 1);
    /// let error = ParserError::new("Test error", token);
    /// assert_eq!(error.message(), "Test error");
    /// ```
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns a reference to the token where the error occurred.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
    /// let error = ParserError::new("Error", token);
    /// assert_eq!(error.token().line, 2);
    /// assert_eq!(error.token().column, 5);
    /// ```
    #[must_use]
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line {}, column {}: {}",
            self.token.line, self.token.column, self.message
        )
    }
}

impl Error for ParserError {}

/// Parser for RPN expressions.
///
/// The parser takes a token stream from the lexer and constructs an Abstract
/// Syntax Tree (AST) using a stack-based algorithm. It validates the RPN
/// expression structure and reports errors for malformed expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::Parser;
/// use rpn2tex::tokens::{Token, TokenType};
///
/// // Parse "5 3 +"
/// let tokens = vec![
///     Token::new(TokenType::Number, "5".to_string(), 1, 1),
///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
///     Token::new(TokenType::Eof, String::new(), 1, 6),
/// ];
///
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse().expect("parse failed");
/// ```
#[derive(Debug)]
pub struct Parser {
    /// Token stream from the lexer
    tokens: Vec<Token>,
    /// Current position in the token list (0-based)
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given token stream.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The token stream produced by the lexer
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![Token::new(TokenType::Eof, String::new(), 1, 1)];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream into an Abstract Syntax Tree.
    ///
    /// This is the main entry point for the parser. It processes all tokens
    /// and returns the root expression node of the AST.
    ///
    /// # Errors
    ///
    /// Returns a [`ParserError`] if:
    /// - The expression is empty (no tokens before EOF)
    /// - An operator doesn't have enough operands (stack underflow)
    /// - Too many values remain on the stack (missing operators)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42".to_string(), 1, 1),
    ///     Token::new(TokenType::Eof, String::new(), 1, 4),
    /// ];
    ///
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse().expect("parse failed");
    /// ```
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.token_type {
                TokenType::Number => {
                    // Push number onto stack
                    let num = Expr::Number {
                        line: token.line,
                        column: token.column,
                        value: token.value.clone(),
                    };
                    stack.push(num);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Check we have enough operands
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            "Too few operands for operator",
                            token.clone(),
                        ));
                    }

                    // Pop right operand, then left operand
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Get operator symbol
                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    // Create binary operation node
                    let binary_op = Expr::BinaryOp {
                        line: token.line,
                        column: token.column,
                        operator: operator.to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    stack.push(binary_op);
                    self.advance();
                }
                TokenType::Eof => {
                    break;
                }
            }
        }

        // Validate final stack state
        if stack.is_empty() {
            // No expression at all
            let eof_token = self.current();
            return Err(ParserError::new("Empty expression", eof_token.clone()));
        }

        if stack.len() > 1 {
            // Too many values on stack - missing operators
            // Report error at the EOF token
            let eof_token = self.current();
            return Err(ParserError::new(
                "Too many values on stack (missing operators)",
                eof_token.clone(),
            ));
        }

        // Return the single expression on the stack
        Ok(stack.pop().unwrap())
    }

    /// Returns the current token without advancing.
    ///
    /// # Panics
    ///
    /// Panics if called when position is beyond the token list.
    /// Use `at_end()` to check before calling.
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Checks if the parser is at the end of the token stream.
    ///
    /// Returns true if the current token is EOF or position is at/beyond
    /// the end of the token list.
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.current().token_type == TokenType::Eof
    }

    /// Consumes the current token and advances to the next one.
    ///
    /// Returns a reference to the token that was current before advancing.
    fn advance(&mut self) -> &Token {
        let current_pos = self.pos;
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        &self.tokens[current_pos]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a simple token
    fn num(value: &str, line: u32, col: u32) -> Token {
        Token::new(TokenType::Number, value.to_string(), line, col)
    }

    fn op(op_type: TokenType, value: &str, line: u32, col: u32) -> Token {
        Token::new(op_type, value.to_string(), line, col)
    }

    fn eof(line: u32, col: u32) -> Token {
        Token::new(TokenType::Eof, String::new(), line, col)
    }

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![num("42", 1, 1), eof(1, 4)];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::Number { value, .. } => assert_eq!(value, "42"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_simple_addition() {
        // "5 3 +"
        let tokens = vec![
            num("5", 1, 1),
            num("3", 1, 3),
            op(TokenType::Plus, "+", 1, 5),
            eof(1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "+");
                match *left {
                    Expr::Number { ref value, .. } => assert_eq!(value, "5"),
                    _ => panic!("Expected left to be Number"),
                }
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "3"),
                    _ => panic!("Expected right to be Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_simple_subtraction() {
        // "10 3 -"
        let tokens = vec![
            num("10", 1, 1),
            num("3", 1, 4),
            op(TokenType::Minus, "-", 1, 6),
            eof(1, 7),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "-"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        // "4 7 *"
        let tokens = vec![
            num("4", 1, 1),
            num("7", 1, 3),
            op(TokenType::Mult, "*", 1, 5),
            eof(1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "*"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division() {
        // "10 2 /"
        let tokens = vec![
            num("10", 1, 1),
            num("2", 1, 4),
            op(TokenType::Div, "/", 1, 6),
            eof(1, 7),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "/"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_nested_expression() {
        // "5 3 + 2 *" -> (5 + 3) * 2
        let tokens = vec![
            num("5", 1, 1),
            num("3", 1, 3),
            op(TokenType::Plus, "+", 1, 5),
            num("2", 1, 7),
            op(TokenType::Mult, "*", 1, 9),
            eof(1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                // Left should be (5 + 3)
                match *left {
                    Expr::BinaryOp {
                        operator: ref op, ..
                    } => assert_eq!(op, "+"),
                    _ => panic!("Expected left to be BinaryOp"),
                }
                // Right should be 2
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected right to be Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        // "1 2 + 3 4 + *" -> (1 + 2) * (3 + 4)
        let tokens = vec![
            num("1", 1, 1),
            num("2", 1, 3),
            op(TokenType::Plus, "+", 1, 5),
            num("3", 1, 7),
            num("4", 1, 9),
            op(TokenType::Plus, "+", 1, 11),
            op(TokenType::Mult, "*", 1, 13),
            eof(1, 14),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                // Both left and right should be BinaryOps
                match *left {
                    Expr::BinaryOp {
                        operator: ref op, ..
                    } => assert_eq!(op, "+"),
                    _ => panic!("Expected left to be BinaryOp"),
                }
                match *right {
                    Expr::BinaryOp {
                        operator: ref op, ..
                    } => assert_eq!(op, "+"),
                    _ => panic!("Expected right to be BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_decimal_numbers() {
        // "3.14 2.0 +"
        let tokens = vec![
            num("3.14", 1, 1),
            num("2.0", 1, 6),
            op(TokenType::Plus, "+", 1, 10),
            eof(1, 11),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp { left, right, .. } => {
                match *left {
                    Expr::Number { ref value, .. } => assert_eq!(value, "3.14"),
                    _ => panic!("Expected left to be Number"),
                }
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "2.0"),
                    _ => panic!("Expected right to be Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_negative_number() {
        // "-5 3 +"
        let tokens = vec![
            num("-5", 1, 1),
            num("3", 1, 4),
            op(TokenType::Plus, "+", 1, 6),
            eof(1, 7),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        match ast {
            Expr::BinaryOp { left, .. } => match *left {
                Expr::Number { ref value, .. } => assert_eq!(value, "-5"),
                _ => panic!("Expected left to be Number"),
            },
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_error_empty_expression() {
        let tokens = vec![eof(1, 1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("Empty expression"));
    }

    #[test]
    fn test_parse_error_too_few_operands() {
        // "5 +" - missing one operand
        let tokens = vec![num("5", 1, 1), op(TokenType::Plus, "+", 1, 3), eof(1, 4)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("Too few operands"));
        assert_eq!(err.token().token_type, TokenType::Plus);
    }

    #[test]
    fn test_parse_error_missing_operator() {
        // "5 3" - missing operator
        let tokens = vec![num("5", 1, 1), num("3", 1, 3), eof(1, 5)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("Too many values"));
    }

    #[test]
    fn test_parse_error_only_operator() {
        // "+" - no operands
        let tokens = vec![op(TokenType::Plus, "+", 1, 1), eof(1, 2)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message().contains("Too few operands"));
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        let error = ParserError::new("Test error", token);
        let display = format!("{}", error);

        assert!(display.contains("Line 2, column 5"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_parser_error_accessors() {
        let token = Token::new(TokenType::Minus, "-".to_string(), 3, 7);
        let error = ParserError::new("Access test", token.clone());

        assert_eq!(error.message(), "Access test");
        assert_eq!(error.token().token_type, TokenType::Minus);
        assert_eq!(error.token().line, 3);
        assert_eq!(error.token().column, 7);
    }

    #[test]
    fn test_position_tracking() {
        // Verify that position information is preserved in the AST
        let tokens = vec![
            num("5", 1, 1),
            num("3", 1, 5),
            op(TokenType::Plus, "+", 1, 9),
            eof(1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        assert_eq!(ast.line(), 1);
        assert_eq!(ast.column(), 9); // Column of the operator

        match ast {
            Expr::BinaryOp { left, right, .. } => {
                assert_eq!(left.line(), 1);
                assert_eq!(left.column(), 1);
                assert_eq!(right.line(), 1);
                assert_eq!(right.column(), 5);
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_multiple_operations_left_to_right() {
        // "5 3 - 2 -" -> (5 - 3) - 2
        let tokens = vec![
            num("5", 1, 1),
            num("3", 1, 3),
            op(TokenType::Minus, "-", 1, 5),
            num("2", 1, 7),
            op(TokenType::Minus, "-", 1, 9),
            eof(1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        // Should be (5 - 3) - 2, which is left-associative
        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "-");
                // Left should be (5 - 3)
                match *left {
                    Expr::BinaryOp {
                        operator: ref op, ..
                    } => assert_eq!(op, "-"),
                    _ => panic!("Expected left to be BinaryOp"),
                }
                // Right should be 2
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected right to be Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_all_operators() {
        let operators = vec![
            (TokenType::Plus, "+"),
            (TokenType::Minus, "-"),
            (TokenType::Mult, "*"),
            (TokenType::Div, "/"),
        ];

        for (token_type, op_str) in operators {
            let tokens = vec![
                num("5", 1, 1),
                num("3", 1, 3),
                op(token_type, op_str, 1, 5),
                eof(1, 6),
            ];
            let mut parser = Parser::new(tokens);
            let ast = parser.parse().expect("parse failed");

            match ast {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, op_str),
                _ => panic!("Expected BinaryOp for operator {}", op_str),
            }
        }
    }

    #[test]
    fn test_parser_new() {
        let tokens = vec![eof(1, 1)];
        let parser = Parser::new(tokens);
        assert_eq!(parser.pos, 0);
    }

    #[test]
    fn test_deeply_nested_expression() {
        // "1 2 + 3 + 4 +" -> ((1 + 2) + 3) + 4
        let tokens = vec![
            num("1", 1, 1),
            num("2", 1, 3),
            op(TokenType::Plus, "+", 1, 5),
            num("3", 1, 7),
            op(TokenType::Plus, "+", 1, 9),
            num("4", 1, 11),
            op(TokenType::Plus, "+", 1, 13),
            eof(1, 14),
        ];
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("parse failed");

        // Root should be the last + operation
        match ast {
            Expr::BinaryOp {
                operator, right, ..
            } => {
                assert_eq!(operator, "+");
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "4"),
                    _ => panic!("Expected right to be Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}

//! RPN parser module.
//!
//! This module provides a parser for converting a sequence of tokens into an
//! Abstract Syntax Tree (AST) using stack-based RPN parsing algorithm.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::parser::Parser;
//!
//! let mut lexer = Lexer::new("5 3 +");
//! let tokens = lexer.tokenize().unwrap();
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().unwrap();
//! ```

use crate::ast::Expr;
use crate::tokens::{Token, TokenType};
use std::fmt;

/// An error that occurred during parsing.
///
/// Contains a message describing the error and the token where the error occurred.
#[derive(Debug, Clone, PartialEq)]
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
    /// * `message` - A description of the error
    /// * `token` - The token where the error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+", 1, 5);
    /// let error = ParserError::new("Insufficient operands", token);
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, token: Token) -> Self {
        Self {
            message: message.into(),
            token,
        }
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

impl std::error::Error for ParserError {}

/// A parser that converts a sequence of tokens into an AST.
///
/// The parser uses a stack-based algorithm to process Reverse Polish Notation (RPN):
/// - When a number token is encountered, push a Number node onto the stack
/// - When an operator token is encountered, pop two operands from the stack,
///   create a BinaryOp node, and push it back onto the stack
/// - At EOF, the stack should contain exactly one element (the root of the AST)
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
///
/// let mut lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// let mut parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A vector of tokens to parse
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    /// use rpn2tex::parser::Parser;
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5", 1, 1),
    ///     Token::new(TokenType::Number, "3", 1, 3),
    ///     Token::new(TokenType::Plus, "+", 1, 5),
    ///     Token::new(TokenType::Eof, "", 1, 6),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the tokens into an AST.
    ///
    /// Uses a stack-based algorithm to process RPN expressions:
    /// 1. For each token in sequence:
    ///    - If NUMBER: push Number node onto stack
    ///    - If operator: pop 2 operands, create BinaryOp, push result
    /// 2. At EOF: validate stack has exactly 1 element
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if:
    /// - An operator doesn't have enough operands on the stack
    /// - The expression is empty (no values on stack at EOF)
    /// - Multiple values remain on the stack at EOF (missing operators)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::ast::Expr;
    ///
    /// let mut lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    ///
    /// match ast {
    ///     Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
    ///     _ => panic!("Expected BinaryOp"),
    /// }
    /// ```
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.token_type {
                TokenType::Number => {
                    // Push number onto stack
                    let num_node = Expr::number(token.value.clone(), token.line, token.column);
                    stack.push(num_node);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Map token type to operator string
                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    let op_node = Expr::binary_op(operator, left, right, token.line, token.column);
                    stack.push(op_node);
                    self.advance();
                }
                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            let eof_token = self.tokens.last().unwrap().clone();
            return Err(ParserError::new("Empty expression", eof_token));
        }

        if stack.len() > 1 {
            let eof_token = self.tokens.last().unwrap().clone();
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token,
            ));
        }

        Ok(stack.into_iter().next().unwrap())
    }

    /// Returns a reference to the current token.
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Checks if the parser is at the end of the token stream.
    fn at_end(&self) -> bool {
        self.tokens[self.pos].token_type == TokenType::Eof
    }

    /// Advances to the next token if not at end.
    fn advance(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_simple_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_simple_subtraction() {
        let mut lexer = Lexer::new("10 5 -");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "-"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_simple_multiplication() {
        let mut lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "*"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_simple_division() {
        let mut lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "/"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_precedence() {
        let mut lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                match *left {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
                    _ => panic!("Expected nested BinaryOp"),
                }
                match *right {
                    Expr::Number { value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiple_operations() {
        let mut lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_floating_point() {
        let mut lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                match *left {
                    Expr::Number { value, .. } => assert_eq!(value, "3.14"),
                    _ => panic!("Expected Number"),
                }
                match *right {
                    Expr::Number { value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut lexer = Lexer::new("2 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                match *left {
                    Expr::Number { value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected Number"),
                }
                match *right {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_insufficient_operands() {
        let mut lexer = Lexer::new("5 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_parse_excess_operands() {
        let mut lexer = Lexer::new("5 3 2 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("Invalid RPN"));
        assert!(err.message.contains("2 values remain on stack"));
    }

    #[test]
    fn test_parse_empty_expression() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());

        let err = result.unwrap_err();
        assert!(err.message.contains("Empty expression"));
    }

    #[test]
    fn test_parse_single_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number { value, .. } => assert_eq!(value, "42"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::Plus, "+", 1, 5);
        let error = ParserError::new("Test error", token);
        let display = format!("{error}");
        assert!(display.contains("Test error"));
        assert!(display.contains("line 1"));
        assert!(display.contains("column 5"));
    }

    #[test]
    fn test_parser_error_clone() {
        let token = Token::new(TokenType::Plus, "+", 1, 5);
        let error = ParserError::new("Test error", token);
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_parse_negative_number() {
        let mut lexer = Lexer::new("-5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp { operator, left, .. } => {
                assert_eq!(operator, "+");
                match *left {
                    Expr::Number { value, .. } => assert_eq!(value, "-5"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_left_associativity() {
        let mut lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should parse as (5-3)-2
        match ast {
            Expr::BinaryOp { operator, left, .. } => {
                assert_eq!(operator, "-");
                match *left {
                    Expr::BinaryOp { operator, .. } => assert_eq!(operator, "-"),
                    _ => panic!("Expected nested BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiple_operators_on_stack() {
        let mut lexer = Lexer::new("1 2 + 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should parse as (1+2)*(3+4)
        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                match (*left, *right) {
                    (
                        Expr::BinaryOp {
                            operator: left_op, ..
                        },
                        Expr::BinaryOp {
                            operator: right_op, ..
                        },
                    ) => {
                        assert_eq!(left_op, "+");
                        assert_eq!(right_op, "+");
                    }
                    _ => panic!("Expected two BinaryOp nodes"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}

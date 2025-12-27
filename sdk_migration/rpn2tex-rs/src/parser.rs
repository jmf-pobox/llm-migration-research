//! Parsing token streams into Abstract Syntax Trees using stack-based RPN algorithm.
//!
//! This module provides the `Parser` struct for converting token streams
//! into expression trees. It implements a stack-based RPN parser that
//! validates operand counts and produces well-formed ASTs.

use crate::ast::Expr;
use crate::tokens::{Token, TokenType};
use std::fmt;

/// Error that occurs during parsing.
///
/// Contains the error message and the token where the error occurred.
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::ParserError;
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token {
///     type_: TokenType::Plus,
///     value: "+".to_string(),
///     line: 1,
///     column: 5,
/// };
/// let error = ParserError {
///     message: "Not enough operands".to_string(),
///     token,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// The error message
    pub message: String,
    /// The token where the error occurred
    pub token: Token,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ParserError at line {}, column {}: {}",
            self.token.line, self.token.column, self.message
        )
    }
}

impl std::error::Error for ParserError {}

/// Parser for RPN expressions.
///
/// Implements a stack-based RPN parsing algorithm:
/// - Numbers are pushed onto the stack
/// - Operators pop two operands, create a binary operation node, and push the result
/// - At the end, the stack must contain exactly one expression
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
/// use rpn2tex::ast::Expr;
///
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
///
/// // Should produce: BinaryOp("+", Number("5"), Number("3"))
/// match ast {
///     Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
///     _ => panic!("Expected BinaryOp"),
/// }
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser for the given token stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream into an Abstract Syntax Tree.
    ///
    /// Uses a stack-based algorithm to process RPN expressions:
    /// 1. Numbers are pushed onto the stack as expression nodes
    /// 2. Operators pop two operands, create a `BinaryOp` node, and push it
    /// 3. After processing all tokens, exactly one expression should remain
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if:
    /// - The expression is empty
    /// - An operator doesn't have enough operands (stack has < 2 elements)
    /// - Too many operands remain (stack has > 1 element at the end)
    /// - An unexpected token type is encountered
    ///
    /// # Panics
    ///
    /// This function does not panic. All `unwrap()` calls are protected by
    /// length checks that ensure the stack has the required elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    ///
    /// // Valid expression
    /// let lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// let result = parser.parse();
    /// assert!(result.is_ok());
    ///
    /// // Invalid: not enough operands
    /// let lexer = Lexer::new("5 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// let result = parser.parse();
    /// assert!(result.is_err());
    /// ```
    pub fn parse(mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.type_ {
                TokenType::Number => {
                    // Create a Number node and push it onto the stack
                    let node = Expr::Number {
                        line: token.line,
                        column: token.column,
                        value: token.value.clone(),
                    };
                    stack.push(node);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Check that we have at least 2 operands on the stack
                    if stack.len() < 2 {
                        return Err(ParserError {
                            message: format!("Not enough operands for operator '{}'", token.value),
                            token: token.clone(),
                        });
                    }

                    // Pop right operand first (stack is LIFO)
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Create a BinaryOp node
                    let node = Expr::BinaryOp {
                        line: token.line,
                        column: token.column,
                        operator: token.value.clone(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    stack.push(node);
                    self.advance();
                }
                TokenType::Eof => {
                    break;
                }
            }
        }

        // Validate final stack state
        match stack.len() {
            0 => {
                // Get the EOF token for error reporting
                let default_token = Token {
                    type_: TokenType::Eof,
                    value: String::new(),
                    line: 1,
                    column: 1,
                };
                let eof_token = self.tokens.last().unwrap_or(&default_token);
                Err(ParserError {
                    message: "Empty expression".to_string(),
                    token: eof_token.clone(),
                })
            }
            1 => Ok(stack.pop().unwrap()),
            n => {
                // Get the EOF token for error reporting
                let default_token = Token {
                    type_: TokenType::Eof,
                    value: String::new(),
                    line: 1,
                    column: 1,
                };
                let eof_token = self.tokens.last().unwrap_or(&default_token);
                Err(ParserError {
                    message: format!("{n} values remain on stack"),
                    token: eof_token.clone(),
                })
            }
        }
    }

    /// Checks if we've reached the end of the token stream.
    #[must_use]
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.tokens[self.pos].type_, TokenType::Eof)
    }

    /// Returns the current token without advancing.
    #[must_use]
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Advances to the next token.
    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_single_number() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        match result {
            Expr::Number { value, .. } => assert_eq!(value, "5"),
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_simple_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        match result {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "+");
                match *left {
                    Expr::Number { ref value, .. } => assert_eq!(value, "5"),
                    _ => panic!("Expected Number for left operand"),
                }
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "3"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_all_operators() {
        let test_cases = vec![
            ("5 3 +", "+"),
            ("5 3 -", "-"),
            ("4 7 *", "*"),
            ("10 2 /", "/"),
        ];

        for (input, expected_op) in test_cases {
            let lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();
            let parser = Parser::new(tokens);
            let result = parser.parse().unwrap();

            match result {
                Expr::BinaryOp { operator, .. } => assert_eq!(operator, expected_op),
                _ => panic!("Expected BinaryOp for input: {input}"),
            }
        }
    }

    #[test]
    fn test_complex_expression() {
        // "5 3 + 2 *" should parse to: BinaryOp("*", BinaryOp("+", 5, 3), 2)
        let lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        match result {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "*");
                // Left should be a BinaryOp
                match *left {
                    Expr::BinaryOp {
                        operator: ref left_op,
                        ..
                    } => assert_eq!(left_op, "+"),
                    _ => panic!("Expected BinaryOp for left operand"),
                }
                // Right should be a Number
                match *right {
                    Expr::Number { ref value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_nested_expression() {
        // "2 3 4 * +" should parse to: BinaryOp("+", 2, BinaryOp("*", 3, 4))
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        match result {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "+");
                // Left should be a Number
                match *left {
                    Expr::Number { ref value, .. } => assert_eq!(value, "2"),
                    _ => panic!("Expected Number for left operand"),
                }
                // Right should be a BinaryOp
                match *right {
                    Expr::BinaryOp {
                        operator: ref right_op,
                        ..
                    } => assert_eq!(right_op, "*"),
                    _ => panic!("Expected BinaryOp for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_error_empty_expression() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Empty expression"));
    }

    #[test]
    fn test_error_not_enough_operands() {
        let lexer = Lexer::new("5 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Not enough operands"));
    }

    #[test]
    fn test_error_too_many_operands() {
        let lexer = Lexer::new("5 3");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("2 values remain on stack"));
    }

    #[test]
    fn test_error_display() {
        let token = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 5,
        };
        let error = ParserError {
            message: "Test error".to_string(),
            token,
        };
        let display = format!("{error}");
        assert!(display.contains("line 1"));
        assert!(display.contains("column 5"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        // The operator should have position information from the token
        match result {
            Expr::BinaryOp {
                line, column, left, ..
            } => {
                assert_eq!(line, 1);
                assert_eq!(column, 5);
                // Check left operand position
                assert_eq!(left.line(), 1);
                assert_eq!(left.column(), 1);
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_decimal_numbers() {
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse().unwrap();

        match result {
            Expr::BinaryOp { left, .. } => match *left {
                Expr::Number { ref value, .. } => assert_eq!(value, "3.14"),
                _ => panic!("Expected Number for left operand"),
            },
            _ => panic!("Expected BinaryOp node"),
        }
    }

    // I/O Contract validation tests

    #[test]
    fn test_io_contract_case_1() {
        // "5 3 +" should parse to BinaryOp("+", Number("5"), Number("3"))
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());

        let ast = result.unwrap();
        match ast {
            Expr::BinaryOp {
                operator,
                left,
                right,
                ..
            } => {
                assert_eq!(operator, "+");
                assert!(matches!(*left, Expr::Number { .. }));
                assert!(matches!(*right, Expr::Number { .. }));
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_io_contract_case_6() {
        // "5 3 + 2 *" should parse correctly
        let lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_io_contract_case_7() {
        // "5 3 * 2 +" should parse correctly
        let lexer = Lexer::new("5 3 * 2 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_io_contract_long_chain() {
        // "1 2 + 3 + 4 +" should parse correctly
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_io_contract_complex_case_20() {
        // "1 2 + 3 4 + *" should parse correctly
        let lexer = Lexer::new("1 2 + 3 4 + *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_error_equality() {
        let token1 = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 5,
        };
        let token2 = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 5,
        };

        let error1 = ParserError {
            message: "Test".to_string(),
            token: token1,
        };
        let error2 = ParserError {
            message: "Test".to_string(),
            token: token2,
        };

        assert_eq!(error1, error2);
    }

    #[test]
    fn test_clone() {
        let token = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 5,
        };
        let error = ParserError {
            message: "Test".to_string(),
            token,
        };

        let cloned = error.clone();
        assert_eq!(error, cloned);
    }
}

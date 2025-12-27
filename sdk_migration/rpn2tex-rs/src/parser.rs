//! RPN expression parser.
//!
//! This module provides a parser for Reverse Polish Notation (RPN) expressions,
//! converting a token stream into an Abstract Syntax Tree (AST).

use crate::ast::{BinaryOp, Expr, Number};
use crate::tokens::{Token, TokenType};
use std::fmt;

/// Error that occurs during parsing.
///
/// Contains the error message and the token where the error occurred,
/// allowing for detailed error reporting with position information.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Parser, Token, TokenType};
///
/// let tokens = vec![
///     Token::new(TokenType::PLUS, "+", 1, 1),
///     Token::new(TokenType::EOF, "", 1, 2),
/// ];
/// let mut parser = Parser::new(tokens);
/// let result = parser.parse();
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Error message describing what went wrong
    pub message: String,
    /// Token where the error occurred
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
    /// use rpn2tex::{ParserError, Token, TokenType};
    ///
    /// let token = Token::new(TokenType::PLUS, "+", 1, 1);
    /// let error = ParserError::new("Insufficient operands", token);
    /// assert_eq!(error.message, "Insufficient operands");
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

/// Parser for RPN expressions.
///
/// Converts a stream of tokens into an Abstract Syntax Tree (AST) using
/// a stack-based algorithm. Numbers are pushed onto the stack, and operators
/// pop two operands to create binary operation nodes.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Parser, Token, TokenType, Expr, Number};
///
/// // Parse "3 4 +"
/// let tokens = vec![
///     Token::new(TokenType::NUMBER, "3", 1, 1),
///     Token::new(TokenType::NUMBER, "4", 1, 3),
///     Token::new(TokenType::PLUS, "+", 1, 5),
///     Token::new(TokenType::EOF, "", 1, 6),
/// ];
/// let mut parser = Parser::new(tokens);
/// let result = parser.parse();
/// assert!(result.is_ok());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given token stream.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Vector of tokens to parse
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Parser, Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::NUMBER, "42", 1, 1),
    ///     Token::new(TokenType::EOF, "", 1, 3),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the token stream into an AST.
    ///
    /// Uses a stack-based algorithm to parse RPN expressions:
    /// - Numbers are pushed onto the stack
    /// - Operators pop two operands and create a binary operation node
    /// - The final stack must contain exactly one expression
    ///
    /// # Returns
    ///
    /// * `Ok(Expr)` - The root AST node if parsing succeeds
    /// * `Err(ParserError)` - Error with position information if parsing fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - An operator has insufficient operands
    /// - An unexpected token type is encountered
    /// - The expression is empty
    /// - Multiple values remain on the stack (missing operators)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Parser, Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::NUMBER, "3", 1, 1),
    ///     Token::new(TokenType::NUMBER, "4", 1, 3),
    ///     Token::new(TokenType::PLUS, "+", 1, 5),
    ///     Token::new(TokenType::EOF, "", 1, 6),
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// ```
    ///
    /// # Panics
    ///
    /// Does not panic under normal usage.
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.token_type {
                TokenType::NUMBER => {
                    let node = Expr::Number(Number::new(&token.value, token.line, token.column));
                    stack.push(node);
                    self.advance();
                }
                TokenType::PLUS | TokenType::MINUS | TokenType::MULT | TokenType::DIV => {
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    // Pop right first, then left (stack order)
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Map token type to operator string
                    let operator = match token.token_type {
                        TokenType::PLUS => "+",
                        TokenType::MINUS => "-",
                        TokenType::MULT => "*",
                        TokenType::DIV => "/",
                        _ => unreachable!(),
                    };

                    let node = Expr::BinaryOp(BinaryOp::new(
                        operator,
                        left,
                        right,
                        token.line,
                        token.column,
                    ));
                    stack.push(node);
                    self.advance();
                }
                TokenType::EOF => {
                    break;
                }
            }
        }

        // Validate final state
        match stack.len() {
            0 => {
                let last_token = if self.pos > 0 && self.pos <= self.tokens.len() {
                    self.tokens[self.pos - 1].clone()
                } else {
                    Token::new(TokenType::EOF, "", 1, 1)
                };
                Err(ParserError::new("Empty expression", last_token))
            }
            1 => Ok(stack.pop().unwrap()),
            n => {
                let last_token = if self.pos > 0 && self.pos <= self.tokens.len() {
                    self.tokens[self.pos - 1].clone()
                } else {
                    Token::new(TokenType::EOF, "", 1, 1)
                };
                Err(ParserError::new(
                    format!("Invalid RPN: {n} values remain on stack (missing operators?)"),
                    last_token,
                ))
            }
        }
    }

    /// Returns a reference to the current token.
    ///
    /// # Panics
    ///
    /// Panics if position is beyond the token stream (should not happen in normal usage).
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Checks if the current token is EOF.
    #[must_use]
    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.current().token_type == TokenType::EOF
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

    #[test]
    fn test_parser_error_new() {
        let token = Token::new(TokenType::PLUS, "+", 1, 5);
        let error = ParserError::new("Test error", token.clone());
        assert_eq!(error.message, "Test error");
        assert_eq!(error.token, token);
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::PLUS, "+", 2, 10);
        let error = ParserError::new("Insufficient operands", token);
        assert_eq!(
            error.to_string(),
            "Insufficient operands at line 2, column 10"
        );
    }

    #[test]
    fn test_parser_new() {
        let tokens = vec![
            Token::new(TokenType::NUMBER, "42", 1, 1),
            Token::new(TokenType::EOF, "", 1, 3),
        ];
        let parser = Parser::new(tokens.clone());
        assert_eq!(parser.tokens, tokens);
        assert_eq!(parser.pos, 0);
    }

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![
            Token::new(TokenType::NUMBER, "42", 1, 1),
            Token::new(TokenType::EOF, "", 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::Number(n) => {
                assert_eq!(n.value, "42");
                assert_eq!(n.line, 1);
                assert_eq!(n.column, 1);
            }
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_simple_addition() {
        // "3 4 +"
        let tokens = vec![
            Token::new(TokenType::NUMBER, "3", 1, 1),
            Token::new(TokenType::NUMBER, "4", 1, 3),
            Token::new(TokenType::PLUS, "+", 1, 5),
            Token::new(TokenType::EOF, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator, "+");
                assert_eq!(binop.line, 1);
                assert_eq!(binop.column, 5);

                // Check left operand
                match binop.left.as_ref() {
                    Expr::Number(n) => assert_eq!(n.value, "3"),
                    _ => panic!("Expected Number in left"),
                }

                // Check right operand
                match binop.right.as_ref() {
                    Expr::Number(n) => assert_eq!(n.value, "4"),
                    _ => panic!("Expected Number in right"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_subtraction() {
        // "10 5 -"
        let tokens = vec![
            Token::new(TokenType::NUMBER, "10", 1, 1),
            Token::new(TokenType::NUMBER, "5", 1, 4),
            Token::new(TokenType::MINUS, "-", 1, 6),
            Token::new(TokenType::EOF, "", 1, 7),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator, "-");
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        // "6 7 *"
        let tokens = vec![
            Token::new(TokenType::NUMBER, "6", 1, 1),
            Token::new(TokenType::NUMBER, "7", 1, 3),
            Token::new(TokenType::MULT, "*", 1, 5),
            Token::new(TokenType::EOF, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator, "*");
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division() {
        // "8 2 /"
        let tokens = vec![
            Token::new(TokenType::NUMBER, "8", 1, 1),
            Token::new(TokenType::NUMBER, "2", 1, 3),
            Token::new(TokenType::DIV, "/", 1, 5),
            Token::new(TokenType::EOF, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator, "/");
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_nested_expression() {
        // "3 4 + 5 *" = (3 + 4) * 5
        let tokens = vec![
            Token::new(TokenType::NUMBER, "3", 1, 1),
            Token::new(TokenType::NUMBER, "4", 1, 3),
            Token::new(TokenType::PLUS, "+", 1, 5),
            Token::new(TokenType::NUMBER, "5", 1, 7),
            Token::new(TokenType::MULT, "*", 1, 9),
            Token::new(TokenType::EOF, "", 1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(mult_op) => {
                assert_eq!(mult_op.operator, "*");

                // Left should be (3 + 4)
                match mult_op.left.as_ref() {
                    Expr::BinaryOp(add_op) => {
                        assert_eq!(add_op.operator, "+");
                        match add_op.left.as_ref() {
                            Expr::Number(n) => assert_eq!(n.value, "3"),
                            _ => panic!("Expected Number"),
                        }
                        match add_op.right.as_ref() {
                            Expr::Number(n) => assert_eq!(n.value, "4"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp in left"),
                }

                // Right should be 5
                match mult_op.right.as_ref() {
                    Expr::Number(n) => assert_eq!(n.value, "5"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_complex_nested() {
        // "1 2 + 3 4 + *" = (1 + 2) * (3 + 4)
        let tokens = vec![
            Token::new(TokenType::NUMBER, "1", 1, 1),
            Token::new(TokenType::NUMBER, "2", 1, 3),
            Token::new(TokenType::PLUS, "+", 1, 5),
            Token::new(TokenType::NUMBER, "3", 1, 7),
            Token::new(TokenType::NUMBER, "4", 1, 9),
            Token::new(TokenType::PLUS, "+", 1, 11),
            Token::new(TokenType::MULT, "*", 1, 13),
            Token::new(TokenType::EOF, "", 1, 14),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(mult_op) => {
                assert_eq!(mult_op.operator, "*");

                // Both left and right should be BinaryOps
                match mult_op.left.as_ref() {
                    Expr::BinaryOp(add_op) => assert_eq!(add_op.operator, "+"),
                    _ => panic!("Expected BinaryOp"),
                }
                match mult_op.right.as_ref() {
                    Expr::BinaryOp(add_op) => assert_eq!(add_op.operator, "+"),
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_empty_expression() {
        let tokens = vec![Token::new(TokenType::EOF, "", 1, 1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("Empty expression"));
    }

    #[test]
    fn test_parse_missing_operands() {
        // Just "+" without operands
        let tokens = vec![
            Token::new(TokenType::PLUS, "+", 1, 1),
            Token::new(TokenType::EOF, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("requires two operands"));
        assert_eq!(error.token.token_type, TokenType::PLUS);
    }

    #[test]
    fn test_parse_insufficient_operands() {
        // "3 +" - only one operand
        let tokens = vec![
            Token::new(TokenType::NUMBER, "3", 1, 1),
            Token::new(TokenType::PLUS, "+", 1, 3),
            Token::new(TokenType::EOF, "", 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("requires two operands"));
    }

    #[test]
    fn test_parse_extra_operands() {
        // "3 4 5 +" - leaves 3 on stack
        let tokens = vec![
            Token::new(TokenType::NUMBER, "3", 1, 1),
            Token::new(TokenType::NUMBER, "4", 1, 3),
            Token::new(TokenType::NUMBER, "5", 1, 5),
            Token::new(TokenType::PLUS, "+", 1, 7),
            Token::new(TokenType::EOF, "", 1, 8),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message.contains("2 values remain on stack"));
        assert!(error.message.contains("missing operators"));
    }

    #[test]
    fn test_parse_all_operators() {
        let operators = vec![
            (TokenType::PLUS, "+"),
            (TokenType::MINUS, "-"),
            (TokenType::MULT, "*"),
            (TokenType::DIV, "/"),
        ];

        for (token_type, op_str) in operators {
            let tokens = vec![
                Token::new(TokenType::NUMBER, "1", 1, 1),
                Token::new(TokenType::NUMBER, "2", 1, 3),
                Token::new(token_type, op_str, 1, 5),
                Token::new(TokenType::EOF, "", 1, 6),
            ];
            let mut parser = Parser::new(tokens);
            let result = parser.parse();

            assert!(result.is_ok());
            match result.unwrap() {
                Expr::BinaryOp(binop) => assert_eq!(binop.operator, op_str),
                _ => panic!("Expected BinaryOp"),
            }
        }
    }

    #[test]
    fn test_parse_position_tracking() {
        // Verify that position information is preserved
        let tokens = vec![
            Token::new(TokenType::NUMBER, "3", 2, 5),
            Token::new(TokenType::NUMBER, "4", 2, 7),
            Token::new(TokenType::PLUS, "+", 2, 9),
            Token::new(TokenType::EOF, "", 2, 10),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.line, 2);
                assert_eq!(binop.column, 9);

                match binop.left.as_ref() {
                    Expr::Number(n) => {
                        assert_eq!(n.line, 2);
                        assert_eq!(n.column, 5);
                    }
                    _ => panic!("Expected Number"),
                }

                match binop.right.as_ref() {
                    Expr::Number(n) => {
                        assert_eq!(n.line, 2);
                        assert_eq!(n.column, 7);
                    }
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_rpn_evaluation_order() {
        // "5 3 -" should be 5 - 3 (not 3 - 5)
        // In the AST: left=5, right=3
        let tokens = vec![
            Token::new(TokenType::NUMBER, "5", 1, 1),
            Token::new(TokenType::NUMBER, "3", 1, 3),
            Token::new(TokenType::MINUS, "-", 1, 5),
            Token::new(TokenType::EOF, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        match result.unwrap() {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator, "-");
                match binop.left.as_ref() {
                    Expr::Number(n) => assert_eq!(n.value, "5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right.as_ref() {
                    Expr::Number(n) => assert_eq!(n.value, "3"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parser_error_token_information() {
        let tokens = vec![
            Token::new(TokenType::NUMBER, "42", 3, 10),
            Token::new(TokenType::PLUS, "+", 3, 13),
            Token::new(TokenType::EOF, "", 3, 14),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.token.line, 3);
        assert_eq!(error.token.column, 13);
        assert_eq!(error.token.token_type, TokenType::PLUS);
    }

    #[test]
    fn test_parser_clone() {
        let tokens = vec![
            Token::new(TokenType::NUMBER, "42", 1, 1),
            Token::new(TokenType::EOF, "", 1, 3),
        ];
        let parser = Parser::new(tokens);
        let cloned = parser.clone();
        assert_eq!(parser, cloned);
    }

    #[test]
    fn test_parser_error_clone() {
        let token = Token::new(TokenType::PLUS, "+", 1, 5);
        let error = ParserError::new("Test", token);
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }
}

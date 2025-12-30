//! Stack-based RPN parser that converts tokens into an Abstract Syntax Tree.
//!
//! This module implements a single-pass parser for Reverse Polish Notation (RPN)
//! expressions. Unlike recursive descent parsers for infix notation, RPN parsing
//! uses a simple stack-based algorithm:
//!
//! 1. When you see a number, push it onto the stack
//! 2. When you see an operator, pop two operands, create a binary operation node, push result
//! 3. At EOF, the stack should contain exactly one element: the AST root
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
//!     Token::new(TokenType::Eof, "".to_string(), 1, 6),
//! ];
//!
//! let mut parser = Parser::new(tokens);
//! let ast = parser.parse().expect("valid RPN");
//! ```
//!
//! # Stack Evolution Example
//!
//! For the input "5 3 + 2 *":
//! ```text
//! Token   Action              Stack
//! -----   ------              -----
//! 5       push Number(5)      [5]
//! 3       push Number(3)      [5, 3]
//! +       pop, create BinOp   [5+3]
//! 2       push Number(2)      [5+3, 2]
//! *       pop, create BinOp   [(5+3)*2]
//! EOF     validate & return   Result: BinaryOp(*, BinaryOp(+, 5, 3), 2)
//! ```

use std::error::Error;
use std::fmt;

use crate::ast::Expr;
use crate::tokens::{Token, TokenType};

/// Error type for parser failures.
///
/// Contains the error message and the token where the error occurred,
/// providing context for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::ParserError;
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
/// let err = ParserError::new("Operator '+' requires two operands".to_string(), token);
/// println!("{}", err); // Displays error with position information
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// The error message describing what went wrong.
    pub message: String,
    /// The token where the error occurred.
    pub token: Token,
}

impl ParserError {
    /// Creates a new `ParserError`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
    /// let err = ParserError::new("Not enough operands".to_string(), token);
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
            self.message,
            self.token.line(),
            self.token.column()
        )
    }
}

impl Error for ParserError {}

/// Stack-based RPN parser.
///
/// Converts a token stream into an Abstract Syntax Tree using a stack
/// to accumulate operands and build expression trees when operators are encountered.
///
/// # Examples
///
/// ```
/// use rpn2tex::parser::Parser;
/// use rpn2tex::tokens::{Token, TokenType};
/// use rpn2tex::ast::Expr;
///
/// let tokens = vec![
///     Token::new(TokenType::Number, "2".to_string(), 1, 1),
///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
///     Token::new(TokenType::Eof, "".to_string(), 1, 6),
/// ];
///
/// let mut parser = Parser::new(tokens);
/// match parser.parse() {
///     Ok(ast) => println!("Parsed successfully"),
///     Err(e) => eprintln!("Parse error: {}", e),
/// }
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new `Parser` with the given token list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42".to_string(), 1, 1),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 3),
    /// ];
    ///
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses the token stream into an Abstract Syntax Tree.
    ///
    /// Uses a stack-based algorithm to process RPN expressions:
    /// - Numbers are pushed onto the stack
    /// - Operators pop two operands, create a binary operation, and push the result
    /// - At EOF, validates that exactly one expression remains on the stack
    ///
    /// # Returns
    ///
    /// The root expression node of the AST.
    ///
    /// # Errors
    ///
    /// Returns `ParserError` if:
    /// - An operator doesn't have enough operands on the stack
    /// - The expression is empty
    /// - Multiple values remain on the stack (missing operators)
    /// - An unexpected token is encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// // Parse "2 3 + 4 *" → (2 + 3) * 4
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "2".to_string(), 1, 1),
    ///     Token::new(TokenType::Number, "3".to_string(), 1, 3),
    ///     Token::new(TokenType::Plus, "+".to_string(), 1, 5),
    ///     Token::new(TokenType::Number, "4".to_string(), 1, 7),
    ///     Token::new(TokenType::Mult, "*".to_string(), 1, 9),
    ///     Token::new(TokenType::Eof, "".to_string(), 1, 10),
    /// ];
    ///
    /// let mut parser = Parser::new(tokens);
    /// let ast = parser.parse().expect("valid RPN expression");
    /// ```
    #[allow(clippy::missing_panics_doc)] // Internal invariants prevent panics
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current_token();

            match token.token_type() {
                TokenType::Number => {
                    // Push number onto stack
                    let num_node = Expr::Number {
                        line: token.line(),
                        column: token.column(),
                        value: token.value().to_string(),
                    };
                    stack.push(num_node);
                    self.advance();
                }

                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value()),
                            token.clone(),
                        ));
                    }

                    // Safe to unwrap: we just checked stack.len() >= 2
                    let right = stack.pop().expect("stack has at least 2 elements");
                    let left = stack.pop().expect("stack has at least 2 elements");

                    // Map token type to operator string
                    let operator = match token.token_type() {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    let op_node = Expr::BinaryOp {
                        line: token.line(),
                        column: token.column(),
                        operator: operator.to_string(),
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                    stack.push(op_node);
                    self.advance();
                }

                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            // Safe to unwrap: parser constructor requires non-empty token list with EOF
            let eof_token = self.tokens.last().expect("token list must have EOF token");
            return Err(ParserError::new(
                "Empty expression".to_string(),
                eof_token.clone(),
            ));
        }

        if stack.len() > 1 {
            // Safe to unwrap: parser constructor requires non-empty token list with EOF
            let eof_token = self.tokens.last().expect("token list must have EOF token");
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token.clone(),
            ));
        }

        // Safe to unwrap: we just checked stack has exactly 1 element
        Ok(stack
            .into_iter()
            .next()
            .expect("stack has exactly 1 element"))
    }

    /// Returns the current token without advancing.
    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Checks if we've reached the end of input (EOF token).
    fn at_end(&self) -> bool {
        matches!(self.tokens[self.current].token_type(), TokenType::Eof)
    }

    /// Advances to the next token.
    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_addition() {
        // "5 3 +"
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Eof, String::new(), 1, 6),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();

        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "+");
            assert!(matches!(*left, Expr::Number { value, .. } if value == "5"));
            assert!(matches!(*right, Expr::Number { value, .. } if value == "3"));
        } else {
            panic!("Expected BinaryOp");
        }
    }

    #[test]
    fn test_nested_expression() {
        // "5 3 + 2 *" → (5 + 3) * 2
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Number, "2".to_string(), 1, 7),
            Token::new(TokenType::Mult, "*".to_string(), 1, 9),
            Token::new(TokenType::Eof, String::new(), 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();

        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "*");
            assert!(matches!(*left, Expr::BinaryOp { operator, .. } if operator == "+"));
            assert!(matches!(*right, Expr::Number { value, .. } if value == "2"));
        } else {
            panic!("Expected BinaryOp");
        }
    }

    #[test]
    fn test_right_nested_expression() {
        // "2 3 4 * +" → 2 + (3 * 4)
        let tokens = vec![
            Token::new(TokenType::Number, "2".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Number, "4".to_string(), 1, 5),
            Token::new(TokenType::Mult, "*".to_string(), 1, 7),
            Token::new(TokenType::Plus, "+".to_string(), 1, 9),
            Token::new(TokenType::Eof, String::new(), 1, 10),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();

        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "+");
            assert!(matches!(*left, Expr::Number { value, .. } if value == "2"));
            assert!(matches!(*right, Expr::BinaryOp { operator, .. } if operator == "*"));
        } else {
            panic!("Expected BinaryOp");
        }
    }

    #[test]
    fn test_complex_nested() {
        // "1 2 + 3 4 + *" → (1 + 2) * (3 + 4)
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

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_ok());
        let expr = result.unwrap();

        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "*");
            assert!(matches!(*left, Expr::BinaryOp { operator, .. } if operator == "+"));
            assert!(matches!(*right, Expr::BinaryOp { operator, .. } if operator == "+"));
        } else {
            panic!("Expected BinaryOp");
        }
    }

    #[test]
    fn test_error_not_enough_operands() {
        // "5 +" - missing second operand
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 3),
            Token::new(TokenType::Eof, String::new(), 1, 4),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_error_empty_expression() {
        // Empty input
        let tokens = vec![Token::new(TokenType::Eof, String::new(), 1, 1)];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Empty expression");
    }

    #[test]
    fn test_error_too_many_values() {
        // "5 3" - missing operator
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Eof, String::new(), 1, 4),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("2 values remain on stack"));
        assert!(err.message.contains("missing operators"));
    }

    #[test]
    fn test_all_operators() {
        // Test +, -, *, /
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

            let mut parser = Parser::new(tokens);
            let result = parser.parse();

            assert!(result.is_ok());
            if let Ok(Expr::BinaryOp { operator, .. }) = result {
                assert_eq!(operator, op_str);
            } else {
                panic!("Expected BinaryOp for {}", op_str);
            }
        }
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        let err = ParserError::new("Test error".to_string(), token);
        let display = format!("{err}");
        assert!(display.contains("Test error"));
        assert!(display.contains("line 2"));
        assert!(display.contains("column 5"));
    }
}

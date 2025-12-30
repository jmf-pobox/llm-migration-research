//! Error types for rpn2tex.
//!
//! This module defines error types for lexer and parser errors.

use std::fmt;

use crate::tokens::Token;

/// Lexer error with position information.
///
/// Raised when lexer encounters invalid input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// Description of the error
    pub message: String,
    /// Line number where error occurred (1-based)
    pub line: usize,
    /// Column number where error occurred (1-based)
    pub column: usize,
}

impl LexerError {
    /// Create a new lexer error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::LexerError;
    ///
    /// let err = LexerError::new("Unexpected character '@'".to_string(), 1, 5);
    /// ```
    #[must_use]
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexerError {}

/// Parser error with token context.
///
/// Raised when parser encounters invalid input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Description of the error
    pub message: String,
    /// The token where error occurred
    pub token: Token,
}

impl ParserError {
    /// Create a new parser error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ParserError;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Eof, "".to_string(), 1, 5);
    /// let err = ParserError::new("Empty expression".to_string(), token);
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

impl std::error::Error for ParserError {}

/// Error type for rpn2tex operations.
#[derive(Debug)]
pub enum Error {
    /// Lexer error
    Lexer(LexerError),
    /// Parser error
    Parser(ParserError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lexer(e) => write!(f, "{e}"),
            Self::Parser(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<LexerError> for Error {
    fn from(err: LexerError) -> Self {
        Self::Lexer(err)
    }
}

impl From<ParserError> for Error {
    fn from(err: ParserError) -> Self {
        Self::Parser(err)
    }
}

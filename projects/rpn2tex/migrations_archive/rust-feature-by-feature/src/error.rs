//! Error types and formatting for rpn2tex.
//!
//! This module defines error types for lexing and parsing operations.

use std::fmt;

/// Error that occurs during lexical analysis.
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
    /// Creates a new lexer error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::LexerError;
    ///
    /// let err = LexerError::new("Unexpected character '@'".to_string(), 1, 5);
    /// ```
    #[must_use]
    pub const fn new(message: String, line: usize, column: usize) -> Self {
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

/// Error that occurs during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Description of the error
    pub message: String,
    /// Line number where error occurred (1-based)
    pub line: usize,
    /// Column number where error occurred (1-based)
    pub column: usize,
}

impl ParserError {
    /// Creates a new parser error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ParserError;
    ///
    /// let err = ParserError::new("Not enough operands".to_string(), 1, 5);
    /// ```
    #[must_use]
    pub const fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at line {}, column {}",
            self.message, self.line, self.column
        )
    }
}

impl std::error::Error for ParserError {}

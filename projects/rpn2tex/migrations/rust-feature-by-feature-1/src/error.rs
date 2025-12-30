//! Error types for RPN to LaTeX conversion.
//!
//! This module defines error types used throughout the conversion pipeline.

use std::error::Error;
use std::fmt;

use crate::Token;

/// An error that occurred during lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct LexerError {
    message: String,
    line: usize,
    column: usize,
}

impl LexerError {
    /// Creates a new lexer error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::LexerError;
    ///
    /// let error = LexerError::new("Unexpected character", 1, 5);
    /// assert_eq!(error.line(), 1);
    /// assert_eq!(error.column(), 5);
    /// ```
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the line number where the error occurred.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number where the error occurred.
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer error at {}:{}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for LexerError {}

/// An error that occurred during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct ParserError {
    message: String,
    token: Token,
}

impl ParserError {
    /// Creates a new parser error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{ParserError, Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Eof, "", 1, 5);
    /// let error = ParserError::new("Unexpected end of input", token);
    /// ```
    pub fn new(message: impl Into<String>, token: Token) -> Self {
        Self {
            message: message.into(),
            token,
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the token where the error occurred.
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser error at {}:{}: {}",
            self.token.line(),
            self.token.column(),
            self.message
        )
    }
}

impl Error for ParserError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenType;

    #[test]
    fn test_lexer_error() {
        let error = LexerError::new("Invalid character", 2, 10);
        assert_eq!(error.message(), "Invalid character");
        assert_eq!(error.line(), 2);
        assert_eq!(error.column(), 10);
    }

    #[test]
    fn test_lexer_error_display() {
        let error = LexerError::new("Test error", 1, 5);
        let display = format!("{}", error);
        assert!(display.contains("Lexer error"));
        assert!(display.contains("1:5"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_parser_error() {
        let token = Token::new(TokenType::Number, "42", 1, 1);
        let error = ParserError::new("Stack underflow", token.clone());
        assert_eq!(error.message(), "Stack underflow");
        assert_eq!(error.token(), &token);
    }

    #[test]
    fn test_parser_error_display() {
        let token = Token::new(TokenType::Eof, "", 3, 15);
        let error = ParserError::new("Unexpected EOF", token);
        let display = format!("{}", error);
        assert!(display.contains("Parser error"));
        assert!(display.contains("3:15"));
        assert!(display.contains("Unexpected EOF"));
    }
}

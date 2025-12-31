//! Error types for the rpn2tex compiler.

use std::error::Error;
use std::fmt;

/// Errors that can occur during lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
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
    /// let err = LexerError::new("Unexpected character '@'", 1, 5);
    /// assert_eq!(err.message, "Unexpected character '@'");
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for LexerError {}

/// Errors that can occur during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    pub message: String,
    pub line: usize,
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
    /// let err = ParserError::new("Empty expression", 1, 1);
    /// assert_eq!(err.message, "Empty expression");
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for ParserError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_error_creation() {
        let err = LexerError::new("Unexpected character '@'", 1, 5);
        assert_eq!(err.message, "Unexpected character '@'");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 5);
    }

    #[test]
    fn test_parser_error_creation() {
        let err = ParserError::new("Empty expression", 1, 1);
        assert_eq!(err.message, "Empty expression");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 1);
    }

    #[test]
    fn test_lexer_error_display() {
        let err = LexerError::new("Unexpected character '@'", 1, 5);
        assert_eq!(format!("{err}"), "Error: Unexpected character '@'");
    }

    #[test]
    fn test_parser_error_display() {
        let err = ParserError::new("Empty expression", 1, 1);
        assert_eq!(format!("{err}"), "Error: Empty expression");
    }
}

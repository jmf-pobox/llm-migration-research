//! Error types for rpn2tex
//!
//! This module provides error types for lexical analysis and parsing errors
//! that occur during RPN expression processing.

use std::fmt;

/// Error types that can occur during RPN processing
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub enum Rpn2TexError {
    /// Error during lexical analysis (tokenization)
    LexerError {
        /// Description of the error
        message: String,
        /// 1-based line number where error occurred
        line: usize,
        /// 1-based column number where error occurred
        column: usize,
    },
    /// Error during parsing
    ParserError {
        /// Description of the error
        message: String,
        /// 1-based line number where error occurred
        line: usize,
        /// 1-based column number where error occurred
        column: usize,
    },
}

impl Rpn2TexError {
    /// Create a new lexer error
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `line` - 1-based line number where error occurred
    /// * `column` - 1-based column number where error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Rpn2TexError;
    ///
    /// let error = Rpn2TexError::lexer_error("Unexpected character '^'", 1, 5);
    /// assert_eq!(error.to_string(), "[LexerError] at line 1, column 5: Unexpected character '^'");
    /// ```
    pub fn lexer_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::LexerError {
            message: message.into(),
            line,
            column,
        }
    }

    /// Create a new parser error
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `line` - 1-based line number where error occurred
    /// * `column` - 1-based column number where error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Rpn2TexError;
    ///
    /// let error = Rpn2TexError::parser_error("Not enough operands", 1, 3);
    /// assert_eq!(error.to_string(), "[ParserError] at line 1, column 3: Not enough operands");
    /// ```
    pub fn parser_error(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self::ParserError {
            message: message.into(),
            line,
            column,
        }
    }

    /// Get the line number where the error occurred
    #[must_use]
    pub const fn line(&self) -> usize {
        match self {
            Self::LexerError { line, .. } | Self::ParserError { line, .. } => *line,
        }
    }

    /// Get the column number where the error occurred
    #[must_use]
    pub const fn column(&self) -> usize {
        match self {
            Self::LexerError { column, .. } | Self::ParserError { column, .. } => *column,
        }
    }

    /// Get the error message
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::LexerError { message, .. } | Self::ParserError { message, .. } => message,
        }
    }
}

impl fmt::Display for Rpn2TexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexerError {
                message,
                line,
                column,
            } => write!(
                f,
                "[LexerError] at line {}, column {}: {}",
                line, column, message
            ),
            Self::ParserError {
                message,
                line,
                column,
            } => write!(
                f,
                "[ParserError] at line {}, column {}: {}",
                line, column, message
            ),
        }
    }
}

impl std::error::Error for Rpn2TexError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_error_creation() {
        let error = Rpn2TexError::lexer_error("Unexpected character '^'", 1, 5);
        match error {
            Rpn2TexError::LexerError {
                message,
                line,
                column,
            } => {
                assert_eq!(message, "Unexpected character '^'");
                assert_eq!(line, 1);
                assert_eq!(column, 5);
            }
            _ => panic!("Expected LexerError variant"),
        }
    }

    #[test]
    fn test_parser_error_creation() {
        let error = Rpn2TexError::parser_error("Not enough operands", 1, 3);
        match error {
            Rpn2TexError::ParserError {
                message,
                line,
                column,
            } => {
                assert_eq!(message, "Not enough operands");
                assert_eq!(line, 1);
                assert_eq!(column, 3);
            }
            _ => panic!("Expected ParserError variant"),
        }
    }

    #[test]
    fn test_lexer_error_display() {
        let error = Rpn2TexError::lexer_error("Unexpected character '^'", 1, 5);
        assert_eq!(
            error.to_string(),
            "[LexerError] at line 1, column 5: Unexpected character '^'"
        );
    }

    #[test]
    fn test_parser_error_display() {
        let error = Rpn2TexError::parser_error("Not enough operands", 2, 10);
        assert_eq!(
            error.to_string(),
            "[ParserError] at line 2, column 10: Not enough operands"
        );
    }

    #[test]
    fn test_error_accessors() {
        let error = Rpn2TexError::lexer_error("Test error", 3, 7);
        assert_eq!(error.line(), 3);
        assert_eq!(error.column(), 7);
        assert_eq!(error.message(), "Test error");
    }

    #[test]
    fn test_error_clone() {
        let error1 = Rpn2TexError::parser_error("Original", 1, 1);
        let error2 = error1.clone();
        assert_eq!(error1, error2);
    }

    #[test]
    fn test_error_debug() {
        let error = Rpn2TexError::lexer_error("Debug test", 1, 1);
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("LexerError"));
        assert!(debug_str.contains("Debug test"));
    }

    #[test]
    fn test_error_is_error_trait() {
        let error: Box<dyn std::error::Error> = Box::new(Rpn2TexError::lexer_error("Test", 1, 1));
        assert!(error.to_string().contains("LexerError"));
    }

    #[test]
    fn test_string_conversion() {
        let error = Rpn2TexError::lexer_error(String::from("Owned string"), 1, 1);
        assert_eq!(error.message(), "Owned string");
    }
}

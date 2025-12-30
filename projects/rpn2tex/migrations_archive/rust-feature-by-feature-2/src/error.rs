//! Error types for the RPN to LaTeX converter.

use std::error::Error;
use std::fmt;

/// Errors that can occur during lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    /// An unexpected character was encountered.
    UnexpectedCharacter {
        /// The unexpected character.
        character: char,
        /// Line number where the error occurred.
        line: usize,
        /// Column number where the error occurred.
        column: usize,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter {
                character,
                line,
                column,
            } => write!(
                f,
                "Unexpected character '{}' at line {}, column {}",
                character, line, column
            ),
        }
    }
}

impl Error for LexerError {}

/// Errors that can occur during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// Not enough operands for an operation.
    #[allow(dead_code)]
    InsufficientOperands {
        /// The operation that failed.
        operation: String,
        /// Line number where the error occurred.
        line: usize,
        /// Column number where the error occurred.
        column: usize,
    },
    /// Unexpected token encountered.
    UnexpectedToken {
        /// Description of what was unexpected.
        message: String,
        /// Line number where the error occurred.
        line: usize,
        /// Column number where the error occurred.
        column: usize,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientOperands {
                operation,
                line,
                column,
            } => write!(
                f,
                "Insufficient operands for operation '{}' at line {}, column {}",
                operation, line, column
            ),
            Self::UnexpectedToken {
                message,
                line,
                column,
            } => write!(f, "{} at line {}, column {}", message, line, column),
        }
    }
}

impl Error for ParserError {}

/// Utility for formatting errors with source context.
pub struct ErrorFormatter;

impl ErrorFormatter {
    /// Formats an error with source context.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::error::{ErrorFormatter, LexerError};
    /// let error = LexerError::UnexpectedCharacter {
    ///     character: '$',
    ///     line: 1,
    ///     column: 5,
    /// };
    /// let formatted = ErrorFormatter::format(&error, "5 3 $ +");
    /// assert!(formatted.contains("Unexpected character"));
    /// ```
    #[must_use]
    pub fn format<E: fmt::Display>(error: &E, source: &str) -> String {
        format!("Error: {}\nSource: {}", error, source)
    }
}

//! Error types for the RPN to LaTeX converter.

use std::fmt;

/// Errors that can occur during lexing, parsing, or generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Lexer encountered an unexpected character.
    LexerError {
        /// The error message.
        message: String,
        /// Line number (1-based).
        line: usize,
        /// Column number (1-based).
        column: usize,
    },
    /// Parser encountered an error.
    ParserError {
        /// The error message.
        message: String,
        /// Line number (1-based).
        line: usize,
        /// Column number (1-based).
        column: usize,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LexerError {
                message,
                line,
                column,
            } => write!(f, "Lexer error at line {line}, column {column}: {message}"),
            Self::ParserError {
                message,
                line,
                column,
            } => write!(f, "Parser error at line {line}, column {column}: {message}"),
        }
    }
}

impl std::error::Error for Error {}

/// Result type for operations that can fail with an Error.
pub type Result<T> = std::result::Result<T, Error>;

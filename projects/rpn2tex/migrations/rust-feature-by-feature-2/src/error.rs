//! Error types for the rpn2tex compiler.
//!
//! This module defines error types for lexing, parsing, and generation phases.

use std::fmt;

/// Errors that can occur during lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerError {
    /// An unexpected character was encountered.
    UnexpectedCharacter {
        ch: char,
        line: usize,
        column: usize,
    },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedCharacter { ch, line, column } => {
                write!(
                    f,
                    "Unexpected character '{}' at line {}, column {}",
                    ch, line, column
                )
            }
        }
    }
}

impl std::error::Error for LexerError {}

/// Errors that can occur during parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    /// An empty input was provided.
    EmptyInput,
    /// Insufficient operands for an operator.
    InsufficientOperands {
        operator: String,
        line: usize,
        column: usize,
    },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "Empty input"),
            Self::InsufficientOperands {
                operator,
                line,
                column,
            } => {
                write!(
                    f,
                    "Operator '{}' requires two operands at line {}, column {}",
                    operator, line, column
                )
            }
        }
    }
}

impl std::error::Error for ParserError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_error_display() {
        let error = LexerError::UnexpectedCharacter {
            ch: '!',
            line: 1,
            column: 5,
        };
        assert_eq!(
            error.to_string(),
            "Unexpected character '!' at line 1, column 5"
        );
    }

    #[test]
    fn test_parser_error_display() {
        let error = ParserError::EmptyInput;
        assert_eq!(error.to_string(), "Empty input");
    }
}

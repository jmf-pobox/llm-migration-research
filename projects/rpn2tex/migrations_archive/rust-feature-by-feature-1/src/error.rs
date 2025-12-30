//! Error types for rpn2tex.
//!
//! This module defines error types for lexer and parser errors with
//! position information for helpful error messages.

use std::error::Error;
use std::fmt;

use crate::tokens::Token;

/// Error that occurs during lexical analysis.
///
/// # Examples
///
/// ```
/// use rpn2tex::error::LexerError;
///
/// let err = LexerError::new("Unexpected character '@'".to_string(), 1, 5);
/// assert_eq!(err.line, 1);
/// assert_eq!(err.column, 5);
/// ```
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

impl Error for LexerError {}

/// Error that occurs during parsing.
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    /// Description of the error
    pub message: String,
    /// The token where error occurred
    pub token: Token,
}

impl ParserError {
    /// Create a new parser error.
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

impl Error for ParserError {}

/// Formats errors with source context.
///
/// Provides gcc/rustc-style error output with line numbers and caret positioning.
///
/// # Examples
///
/// ```
/// use rpn2tex::error::ErrorFormatter;
///
/// let formatter = ErrorFormatter::new("5 3 +");
/// let formatted = formatter.format_error("Test error", 1, 5);
/// assert!(formatted.contains("^"));
/// ```
#[derive(Debug, Clone)]
pub struct ErrorFormatter {
    lines: Vec<String>,
}

impl ErrorFormatter {
    /// Create a new error formatter with source text.
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let lines = source.lines().map(String::from).collect();
        Self { lines }
    }

    /// Format an error with source context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::error::ErrorFormatter;
    ///
    /// let formatter = ErrorFormatter::new("5 3 @");
    /// let output = formatter.format_error("Unexpected character '@'", 1, 5);
    /// assert!(output.contains("Error:"));
    /// assert!(output.contains("^"));
    /// ```
    #[must_use]
    pub fn format_error(&self, message: &str, line: usize, column: usize) -> String {
        let mut parts = vec![format!("Error: {message}"), String::new()];

        let context = self.get_context(line, column, 1);
        parts.push(context);

        parts.join("\n")
    }

    fn get_context(&self, line: usize, column: usize, context_lines: usize) -> String {
        let error_idx = line.saturating_sub(1);
        let start_idx = error_idx.saturating_sub(context_lines);
        let end_idx = (error_idx + context_lines + 1).min(self.lines.len());

        let max_line_num = end_idx;
        let num_width = max_line_num.to_string().len();

        let mut result_lines = Vec::new();

        for idx in start_idx..end_idx {
            let line_num = idx + 1;
            let line_content = self.lines.get(idx).map_or("", String::as_str);

            let prefix = format!("{line_num:>num_width$} | ");
            result_lines.push(format!("{prefix}{line_content}"));

            if idx == error_idx {
                let caret_prefix = format!("{:num_width$} | ", "");
                let caret_pos = column.saturating_sub(1);
                let caret_line = format!("{caret_prefix}{:caret_pos$}^", "");
                result_lines.push(caret_line);
            }
        }

        result_lines.join("\n")
    }
}

//! Token types for lexical analysis.
//!
//! This module defines tokens that represent the basic elements of RPN expressions.

/// Represents the type of a token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub enum TokenType {
    /// A numeric literal (e.g., "5", "3.14", "-2")
    Number,
    /// Addition operator (+)
    Plus,
    /// Subtraction operator (-)
    Minus,
    /// Multiplication operator (*)
    Star,
    /// Division operator (/)
    Slash,
    /// End of file marker
    Eof,
}

/// A lexical token with type, value, and position information.
///
/// Tokens are immutable and include position tracking for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42", 1, 1);
/// assert_eq!(token.type_(), TokenType::Number);
/// assert_eq!(token.value(), "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use]
pub struct Token {
    type_: TokenType,
    value: String,
    line: usize,
    column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Arguments
    ///
    /// * `type_` - The type of the token
    /// * `value` - The string value of the token
    /// * `line` - The line number (1-based)
    /// * `column` - The column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "3.14", 1, 5);
    /// assert_eq!(token.line(), 1);
    /// assert_eq!(token.column(), 5);
    /// ```
    pub fn new(type_: TokenType, value: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            type_,
            value: value.into(),
            line,
            column,
        }
    }

    /// Returns the token type.
    pub const fn type_(&self) -> TokenType {
        self.type_
    }

    /// Returns the token value as a string slice.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the line number (1-based).
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number (1-based).
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42", 1, 1);
        assert_eq!(token.type_(), TokenType::Number);
        assert_eq!(token.value(), "42");
        assert_eq!(token.line(), 1);
        assert_eq!(token.column(), 1);
    }

    #[test]
    fn test_token_with_string() {
        let s = String::from("3.14");
        let token = Token::new(TokenType::Number, s, 2, 5);
        assert_eq!(token.value(), "3.14");
        assert_eq!(token.line(), 2);
        assert_eq!(token.column(), 5);
    }

    #[test]
    fn test_token_eof() {
        let token = Token::new(TokenType::Eof, "", 1, 10);
        assert_eq!(token.type_(), TokenType::Eof);
        assert_eq!(token.value(), "");
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TokenType::Number, "100", 3, 2);
        let token2 = token1.clone();
        assert_eq!(token1, token2);
    }
}

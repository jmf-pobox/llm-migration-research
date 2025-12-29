//! Token types and data structures for the lexer.
//!
//! This module defines the fundamental building blocks for lexical analysis:
//! - `TokenType`: Enum representing different types of tokens
//! - `Token`: Structure containing token information including type, value, and position

use std::fmt;

/// Represents the type of a token in the RPN expression.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal
    Number,
    /// Addition operator (+)
    Plus,
    /// Subtraction operator (-)
    Minus,
    /// Multiplication operator (*)
    Mult,
    /// Division operator (/)
    Div,
    /// End of file/input marker
    Eof,
}

/// A token with its type, value, and position information.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
/// assert_eq!(token.token_type(), TokenType::Number);
/// assert_eq!(token.value(), "42");
/// assert_eq!(token.line(), 1);
/// assert_eq!(token.column(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    type_: TokenType,
    value: String,
    line: u32,
    column: u32,
}

impl Token {
    /// Creates a new `Token`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, value: String, line: u32, column: u32) -> Self {
        Self {
            type_: token_type,
            value,
            line,
            column,
        }
    }

    /// Returns the token type.
    #[must_use]
    pub fn token_type(&self) -> TokenType {
        self.type_
    }

    /// Returns the token value as a string slice.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the line number (1-based).
    #[must_use]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column number (1-based).
    #[must_use]
    pub fn column(&self) -> u32 {
        self.column
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({:?}, '{}', line={}, col={})",
            self.type_, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "123".to_string(), 1, 1);
        assert_eq!(token.token_type(), TokenType::Number);
        assert_eq!(token.value(), "123");
        assert_eq!(token.line(), 1);
        assert_eq!(token.column(), 1);
    }

    #[test]
    fn test_token_types() {
        let types = vec![
            TokenType::Number,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Mult,
            TokenType::Div,
            TokenType::Eof,
        ];

        for token_type in types {
            let token = Token::new(token_type, String::new(), 1, 1);
            assert_eq!(token.token_type(), token_type);
        }
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        let display = format!("{token}");
        assert!(display.contains("Plus"));
        assert!(display.contains("+"));
        assert!(display.contains("line=2"));
        assert!(display.contains("col=5"));
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let token2 = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let token3 = Token::new(TokenType::Number, "43".to_string(), 1, 1);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TokenType::Mult, "*".to_string(), 3, 7);
        let token2 = token1.clone();
        assert_eq!(token1, token2);
    }
}

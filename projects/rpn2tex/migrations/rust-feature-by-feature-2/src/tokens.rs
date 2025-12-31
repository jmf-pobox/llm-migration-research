//! Token definitions for the RPN lexer.
//!
//! This module defines the token types and token structures used by the lexer
//! to represent the input stream.

/// Token types recognized by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal (integer or floating-point).
    Number,
    /// Addition operator (+).
    Plus,
    /// Subtraction operator (-).
    Minus,
    /// Multiplication operator (*).
    Star,
    /// Division operator (/).
    Slash,
}

/// A token with its type, value, and position information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of this token.
    pub token_type: TokenType,
    /// The string representation of the token value.
    pub value: String,
    /// Line number (1-based).
    pub line: usize,
    /// Column number (1-based).
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// assert_eq!(token.value, "42");
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value,
            line,
            column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "5".to_string(), 1, 1);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "5");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TokenType::Number, "3.14".to_string(), 1, 1);
        let token2 = Token::new(TokenType::Number, "3.14".to_string(), 1, 1);
        assert_eq!(token1, token2);
    }
}

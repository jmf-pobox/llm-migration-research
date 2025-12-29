//! Token types and structures for the RPN lexer.
//!
//! This module defines the token types recognized by the lexer and the
//! token structure that holds token data along with position information.

use std::fmt;

/// Represents the different types of tokens in the RPN language.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal (integer or floating-point)
    Number,

    /// Addition operator '+'
    Plus,

    /// Subtraction operator '-'
    Minus,

    /// Multiplication operator '*'
    Mult,

    /// Division operator '/'
    Div,

    /// End-of-file marker
    Eof,
}

/// A token with its type, value, and position information.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42", 1, 5);
/// assert_eq!(token.token_type, TokenType::Number);
/// assert_eq!(token.value, "42");
/// assert_eq!(token.line, 1);
/// assert_eq!(token.column, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of this token
    pub token_type: TokenType,

    /// The string value of this token
    pub value: String,

    /// Line number where this token appears (1-based)
    pub line: usize,

    /// Column number where this token starts (1-based)
    pub column: usize,
}

impl Token {
    /// Creates a new token with the given type, value, and position.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token
    /// * `value` - The string value of the token
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+", 1, 3);
    /// assert_eq!(token.token_type, TokenType::Plus);
    /// ```
    #[must_use]
    pub fn new(
        token_type: TokenType,
        value: impl Into<String>,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            value: value.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({:?}, '{}', {}:{})",
            self.token_type, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42", 1, 5);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_token_creation_with_string() {
        let token = Token::new(TokenType::Number, String::from("123"), 2, 10);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "123");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 10);
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+", 1, 3);
        let display = format!("{token}");
        assert!(display.contains("Plus"));
        assert!(display.contains("+"));
        assert!(display.contains("1:3"));
    }

    #[test]
    fn test_token_types() {
        assert_eq!(TokenType::Number, TokenType::Number);
        assert_ne!(TokenType::Plus, TokenType::Minus);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TokenType::Mult, "*", 1, 1);
        let token2 = Token::new(TokenType::Mult, "*", 1, 1);
        let token3 = Token::new(TokenType::Div, "/", 1, 1);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
    }

    #[test]
    fn test_token_clone() {
        let token = Token::new(TokenType::Number, "3.14", 1, 1);
        let cloned = token.clone();
        assert_eq!(token, cloned);
    }

    #[test]
    fn test_all_token_types() {
        let number = Token::new(TokenType::Number, "42", 1, 1);
        let plus = Token::new(TokenType::Plus, "+", 1, 1);
        let minus = Token::new(TokenType::Minus, "-", 1, 1);
        let mult = Token::new(TokenType::Mult, "*", 1, 1);
        let div = Token::new(TokenType::Div, "/", 1, 1);
        let eof = Token::new(TokenType::Eof, "", 1, 1);

        assert_eq!(number.token_type, TokenType::Number);
        assert_eq!(plus.token_type, TokenType::Plus);
        assert_eq!(minus.token_type, TokenType::Minus);
        assert_eq!(mult.token_type, TokenType::Mult);
        assert_eq!(div.token_type, TokenType::Div);
        assert_eq!(eof.token_type, TokenType::Eof);
    }

    #[test]
    fn test_token_debug_format() {
        let token = Token::new(TokenType::Number, "42", 1, 5);
        let debug = format!("{token:?}");
        assert!(debug.contains("Number"));
        assert!(debug.contains("42"));
    }
}

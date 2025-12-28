//! Token types and token representation for the lexer.
//!
//! This module defines the token types that can appear in RPN expressions
//! and the `Token` struct that represents a token with its value and position.

use std::fmt;

/// Types of tokens that can appear in RPN expressions.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::TokenType;
///
/// let token_type = TokenType::Plus;
/// assert_eq!(token_type, TokenType::Plus);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// A numeric literal (integer or decimal)
    Number,
    /// The addition operator `+`
    Plus,
    /// The subtraction operator `-`
    Minus,
    /// The multiplication operator `*`
    Mult,
    /// The division operator `/`
    Div,
    /// End of file marker
    Eof,
}

/// A token with its type, value, and position information.
///
/// Positions are 1-based (first line/column is 1, not 0).
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token {
///     type_: TokenType::Number,
///     value: "42".to_string(),
///     line: 1,
///     column: 1,
/// };
/// assert_eq!(token.value, "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of this token
    pub type_: TokenType,
    /// The string value of this token
    pub value: String,
    /// The line number where this token appears (1-based)
    pub line: u32,
    /// The column number where this token appears (1-based)
    pub column: u32,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token({:?}, '{}', {}:{})",
            self.type_, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token {
            type_: TokenType::Number,
            value: "42".to_string(),
            line: 1,
            column: 5,
        };

        assert_eq!(token.type_, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Plus, TokenType::Plus);
        assert_ne!(TokenType::Plus, TokenType::Minus);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 3,
        };

        let token2 = Token {
            type_: TokenType::Plus,
            value: "+".to_string(),
            line: 1,
            column: 3,
        };

        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_display() {
        let token = Token {
            type_: TokenType::Number,
            value: "3.14".to_string(),
            line: 2,
            column: 7,
        };

        let display = format!("{}", token);
        assert!(display.contains("Number"));
        assert!(display.contains("3.14"));
        assert!(display.contains("2:7"));
    }

    #[test]
    fn test_token_type_copy() {
        let t1 = TokenType::Mult;
        let t2 = t1; // Copy, not move
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_all_token_types() {
        let types = vec![
            TokenType::Number,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Mult,
            TokenType::Div,
            TokenType::Eof,
        ];

        for token_type in types {
            let token = Token {
                type_: token_type,
                value: String::new(),
                line: 1,
                column: 1,
            };
            assert_eq!(token.type_, token_type);
        }
    }
}

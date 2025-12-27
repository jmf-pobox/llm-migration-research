//! Token types and definitions for the RPN lexer.
//!
//! This module defines the core token types used in lexical analysis,
//! including numeric literals and arithmetic operators.

use std::fmt;

/// Token types recognized by the lexer.
///
/// # Examples
///
/// ```
/// use rpn2tex::TokenType;
///
/// let token_type = TokenType::NUMBER;
/// assert_eq!(token_type, TokenType::NUMBER);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal token
    NUMBER,
    /// Addition operator (+)
    PLUS,
    /// Subtraction operator (-)
    MINUS,
    /// Multiplication operator (*)
    MULT,
    /// Division operator (/)
    DIV,
    /// End of file marker
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::NUMBER => "NUMBER",
            Self::PLUS => "PLUS",
            Self::MINUS => "MINUS",
            Self::MULT => "MULT",
            Self::DIV => "DIV",
            Self::EOF => "EOF",
        };
        write!(f, "{name}")
    }
}

/// A token with type, value, and position information.
///
/// Represents a single lexical token with its type, string value,
/// and source location (line and column numbers, both 1-based).
///
/// # Examples
///
/// ```
/// use rpn2tex::{Token, TokenType};
///
/// let token = Token::new(TokenType::NUMBER, "42", 1, 1);
/// assert_eq!(token.token_type, TokenType::NUMBER);
/// assert_eq!(token.value, "42");
/// assert_eq!(token.line, 1);
/// assert_eq!(token.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of this token
    pub token_type: TokenType,
    /// The string value of this token
    pub value: String,
    /// Line number (1-based)
    pub line: u32,
    /// Column number (1-based)
    pub column: u32,
}

impl Token {
    /// Creates a new token.
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
    /// use rpn2tex::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::PLUS, "+", 1, 5);
    /// assert_eq!(token.token_type, TokenType::PLUS);
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, value: impl Into<String>, line: u32, column: u32) -> Self {
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
            "Token({}, '{}', {}:{})",
            self.token_type, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::NUMBER, TokenType::NUMBER);
        assert_eq!(TokenType::PLUS, TokenType::PLUS);
        assert_ne!(TokenType::NUMBER, TokenType::PLUS);
    }

    #[test]
    fn test_token_type_display() {
        assert_eq!(TokenType::NUMBER.to_string(), "NUMBER");
        assert_eq!(TokenType::PLUS.to_string(), "PLUS");
        assert_eq!(TokenType::MINUS.to_string(), "MINUS");
        assert_eq!(TokenType::MULT.to_string(), "MULT");
        assert_eq!(TokenType::DIV.to_string(), "DIV");
        assert_eq!(TokenType::EOF.to_string(), "EOF");
    }

    #[test]
    fn test_token_construction() {
        let token = Token::new(TokenType::NUMBER, "42", 1, 1);
        assert_eq!(token.token_type, TokenType::NUMBER);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_construction_with_string() {
        let value = String::from("3.14");
        let token = Token::new(TokenType::NUMBER, value, 2, 5);
        assert_eq!(token.token_type, TokenType::NUMBER);
        assert_eq!(token.value, "3.14");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::NUMBER, "42", 1, 1);
        assert_eq!(token.to_string(), "Token(NUMBER, '42', 1:1)");

        let token = Token::new(TokenType::PLUS, "+", 1, 5);
        assert_eq!(token.to_string(), "Token(PLUS, '+', 1:5)");

        let token = Token::new(TokenType::EOF, "", 3, 10);
        assert_eq!(token.to_string(), "Token(EOF, '', 3:10)");
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TokenType::NUMBER, "42", 1, 1);
        let token2 = Token::new(TokenType::NUMBER, "42", 1, 1);
        let token3 = Token::new(TokenType::NUMBER, "43", 1, 1);
        let token4 = Token::new(TokenType::PLUS, "42", 1, 1);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
        assert_ne!(token1, token4);
    }

    #[test]
    fn test_token_clone() {
        let token = Token::new(TokenType::MULT, "*", 2, 3);
        let cloned = token.clone();
        assert_eq!(token, cloned);
        assert_eq!(token.token_type, cloned.token_type);
        assert_eq!(token.value, cloned.value);
        assert_eq!(token.line, cloned.line);
        assert_eq!(token.column, cloned.column);
    }

    #[test]
    fn test_all_token_types() {
        let tokens = vec![
            Token::new(TokenType::NUMBER, "42", 1, 1),
            Token::new(TokenType::PLUS, "+", 1, 2),
            Token::new(TokenType::MINUS, "-", 1, 3),
            Token::new(TokenType::MULT, "*", 1, 4),
            Token::new(TokenType::DIV, "/", 1, 5),
            Token::new(TokenType::EOF, "", 1, 6),
        ];

        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[1].token_type, TokenType::PLUS);
        assert_eq!(tokens[2].token_type, TokenType::MINUS);
        assert_eq!(tokens[3].token_type, TokenType::MULT);
        assert_eq!(tokens[4].token_type, TokenType::DIV);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }
}

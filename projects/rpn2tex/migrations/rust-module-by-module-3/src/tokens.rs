//! Token types and data structures for the RPN lexer.
//!
//! This module defines the fundamental token types used in lexical analysis
//! and the Token structure that holds token information including position data.

use std::fmt;

/// Represents the type of token recognized by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// End of file marker
    Eof,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number => write!(f, "Number"),
            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Mult => write!(f, "Mult"),
            Self::Div => write!(f, "Div"),
            Self::Eof => write!(f, "Eof"),
        }
    }
}

/// Represents a token with its type, value, and position information.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
/// assert_eq!(token.type_, TokenType::Number);
/// assert_eq!(token.value, "42");
/// assert_eq!(token.line, 1);
/// assert_eq!(token.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of token
    pub type_: TokenType,
    /// The string value of the token
    pub value: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
}

impl Token {
    /// Creates a new token with the specified attributes.
    ///
    /// # Arguments
    ///
    /// * `type_` - The type of the token
    /// * `value` - The string value of the token
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
    /// ```
    #[must_use]
    pub fn new(type_: TokenType, value: String, line: usize, column: usize) -> Self {
        Self {
            type_,
            value,
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
            self.type_, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        assert_eq!(token.type_, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        assert_eq!(format!("{}", token), "Token(Plus, '+', 2:5)");
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TokenType::Minus, "-".to_string(), 1, 3);
        let token2 = Token::new(TokenType::Minus, "-".to_string(), 1, 3);
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TokenType::Mult, "*".to_string(), 3, 7);
        let token2 = token1.clone();
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_all_token_types() {
        let tokens = vec![
            Token::new(TokenType::Number, "123".to_string(), 1, 1),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
            Token::new(TokenType::Minus, "-".to_string(), 1, 7),
            Token::new(TokenType::Mult, "*".to_string(), 1, 9),
            Token::new(TokenType::Div, "/".to_string(), 1, 11),
            Token::new(TokenType::Eof, "".to_string(), 1, 13),
        ];

        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[1].type_, TokenType::Plus);
        assert_eq!(tokens[2].type_, TokenType::Minus);
        assert_eq!(tokens[3].type_, TokenType::Mult);
        assert_eq!(tokens[4].type_, TokenType::Div);
        assert_eq!(tokens[5].type_, TokenType::Eof);
    }

    #[test]
    fn test_token_type_display() {
        assert_eq!(format!("{}", TokenType::Number), "Number");
        assert_eq!(format!("{}", TokenType::Plus), "Plus");
        assert_eq!(format!("{}", TokenType::Minus), "Minus");
        assert_eq!(format!("{}", TokenType::Mult), "Mult");
        assert_eq!(format!("{}", TokenType::Div), "Div");
        assert_eq!(format!("{}", TokenType::Eof), "Eof");
    }

    #[test]
    fn test_token_debug() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let debug_str = format!("{:?}", token);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
    }
}

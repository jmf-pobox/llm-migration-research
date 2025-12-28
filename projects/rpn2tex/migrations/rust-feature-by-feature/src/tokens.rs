//! Token types and definitions for rpn2tex lexer.
//!
//! This module defines the token types recognized by the RPN lexer.
//!
//! # Token Types
//!
//! * `NUMBER` - Numeric literals (integers and decimals)
//! * `PLUS` - Addition operator (+)
//! * `MINUS` - Subtraction operator (-)
//! * `MULT` - Multiplication operator (*)
//! * `DIV` - Division operator (/)
//! * `EOF` - End of file marker

/// Token types for rpn2tex lexer.
///
/// Each token type represents a distinct lexical element in RPN expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric values: 5, 3.14, -2
    Number,
    /// + (addition)
    Plus,
    /// - (subtraction)
    Minus,
    /// * (multiplication)
    Mult,
    /// / (division)
    Div,
    /// End of input
    Eof,
}

/// A lexical token with type, value, and position.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 5);
/// assert_eq!(token.token_type, TokenType::Number);
/// assert_eq!(token.value, "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The token type (from TokenType enum)
    pub token_type: TokenType,
    /// The string value of the token
    pub value: String,
    /// Line number (1-based) where token appears
    pub line: usize,
    /// Column number (1-based) where token starts
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
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 3);
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

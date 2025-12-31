//! Token types and token data structures for lexical analysis.
//!
//! This module defines the token types and token data structure used by the lexer
//! to represent individual lexical units in RPN expressions.

use std::fmt;

/// Represents the different types of tokens in RPN expressions.
///
/// # Variants
///
/// * `Number` - Numeric literals (integers and decimals)
/// * `Plus` - Addition operator (+)
/// * `Minus` - Subtraction operator (-)
/// * `Mult` - Multiplication operator (*)
/// * `Div` - Division operator (/)
/// * `Eof` - End of file marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Numeric literals (integers and decimals)
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

/// Represents a single lexical token with position information.
///
/// Tokens are immutable and contain the token type, string value,
/// and position information (line and column) for error reporting.
///
/// # Fields
///
/// * `token_type` - The type of the token
/// * `value` - The string value of the token
/// * `line` - 1-based line number where the token appears
/// * `column` - 1-based column number where the token starts
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
/// assert_eq!(token.token_type, TokenType::Number);
/// assert_eq!(token.value, "42");
/// assert_eq!(token.line, 1);
/// assert_eq!(token.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of the token
    pub token_type: TokenType,
    /// The string value of the token
    pub value: String,
    /// 1-based line number where the token appears
    pub line: u32,
    /// 1-based column number where the token starts
    pub column: u32,
}

impl Token {
    /// Creates a new token with the specified type, value, and position.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token
    /// * `value` - The string value of the token
    /// * `line` - 1-based line number (must be >= 1)
    /// * `column` - 1-based column number (must be >= 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
    /// assert_eq!(token.token_type, TokenType::Plus);
    /// assert_eq!(token.value, "+");
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, value: String, line: u32, column: u32) -> Self {
        Self {
            token_type,
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
            "Token({:?}, '{}', line={}, column={})",
            self.token_type, self.value, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Number, TokenType::Number);
        assert_ne!(TokenType::Number, TokenType::Plus);
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_with_operator() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.value, "+");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_token_with_decimal_number() {
        let token = Token::new(TokenType::Number, "3.14".to_string(), 2, 10);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "3.14");
        assert_eq!(token.line, 2);
        assert_eq!(token.column, 10);
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
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
        let display = format!("{}", token);
        assert!(display.contains("Plus"));
        assert!(display.contains("+"));
        assert!(display.contains("line=1"));
        assert!(display.contains("column=5"));
    }

    #[test]
    fn test_all_token_types() {
        let types = [
            TokenType::Number,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Mult,
            TokenType::Div,
            TokenType::Eof,
        ];

        // Just verify they can all be created and cloned
        for token_type in types {
            let cloned = token_type;
            assert_eq!(token_type, cloned);
        }
    }

    #[test]
    fn test_eof_token() {
        let token = Token::new(TokenType::Eof, String::new(), 1, 10);
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.value, "");
    }

    #[test]
    fn test_negative_number_token() {
        let token = Token::new(TokenType::Number, "-42".to_string(), 1, 1);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "-42");
    }

    #[test]
    fn test_token_position_tracking() {
        // Test that position information is preserved correctly
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
        ];

        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].line, 1);
        assert_eq!(tokens[2].column, 5);
    }

    #[test]
    fn test_multiline_token_positions() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 2, 1),
            Token::new(TokenType::Plus, "+".to_string(), 3, 1),
        ];

        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[2].line, 3);
    }
}

//! Token types for lexical analysis.

/// Types of tokens recognized by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal (e.g., "5", "3.14", "-2")
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

/// A token produced by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line: usize,
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
    /// let token = Token::new(TokenType::Number, "42", 1, 1);
    /// assert_eq!(token.value, "42");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42", 1, 1);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_with_decimal() {
        let token = Token::new(TokenType::Number, "3.14", 1, 5);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.value, "3.14");
    }

    #[test]
    fn test_eof_token() {
        let token = Token::new(TokenType::Eof, "", 1, 10);
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.value, "");
    }

    #[test]
    fn test_plus_token() {
        let token = Token::new(TokenType::Plus, "+", 1, 3);
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.value, "+");
    }

    #[test]
    fn test_minus_token() {
        let token = Token::new(TokenType::Minus, "-", 1, 3);
        assert_eq!(token.token_type, TokenType::Minus);
        assert_eq!(token.value, "-");
    }

    #[test]
    fn test_star_token() {
        let token = Token::new(TokenType::Star, "*", 1, 3);
        assert_eq!(token.token_type, TokenType::Star);
        assert_eq!(token.value, "*");
    }

    #[test]
    fn test_slash_token() {
        let token = Token::new(TokenType::Slash, "/", 1, 3);
        assert_eq!(token.token_type, TokenType::Slash);
        assert_eq!(token.value, "/");
    }
}

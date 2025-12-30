//! Token definitions for the rpn2tex lexer.
//!
//! This module provides the core token types used throughout the RPN to LaTeX conversion
//! pipeline. It defines [`TokenType`] for classifying tokens and [`Token`] for representing
//! individual lexical tokens with position information.

/// Represents the type of a lexical token.
///
/// The tokenizer recognizes numeric literals, arithmetic operators, and end-of-file markers.
///
/// # Examples
///
/// ```
/// use rpn2tex::TokenType;
///
/// let token_type = TokenType::Number;
/// assert_eq!(format!("{:?}", token_type), "Number");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    /// Numeric literal (e.g., `5`, `3.14`, `-2`)
    Number,

    /// Addition operator (`+`)
    Plus,

    /// Subtraction operator (`-`)
    Minus,

    /// Multiplication operator (`*`)
    Star,

    /// Division operator (`/`)
    Slash,

    /// Exponentiation operator (`^`) - not currently supported
    Caret,

    /// Left parenthesis (`(`) - reserved for future use
    LeftParen,

    /// Right parenthesis (`)`) - reserved for future use
    RightParen,

    /// End of input marker
    Eof,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "NUMBER"),
            Self::Plus => write!(f, "PLUS"),
            Self::Minus => write!(f, "MINUS"),
            Self::Star => write!(f, "STAR"),
            Self::Slash => write!(f, "SLASH"),
            Self::Caret => write!(f, "CARET"),
            Self::LeftParen => write!(f, "LEFT_PAREN"),
            Self::RightParen => write!(f, "RIGHT_PAREN"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}

/// Represents a single lexical token with position information.
///
/// A token combines a [`TokenType`] classification with the original lexeme text
/// and its position in the source code for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
/// assert_eq!(token.token_type, TokenType::Number);
/// assert_eq!(token.lexeme, "42");
/// assert_eq!(token.line, 1);
/// assert_eq!(token.column, 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type/classification of this token
    pub token_type: TokenType,

    /// The original text that this token represents
    pub lexeme: String,

    /// The 1-based line number where this token appears
    pub line: usize,

    /// The 1-based column number where this token starts
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type/classification of the token
    /// * `lexeme` - The original text representing the token
    /// * `line` - The 1-based line number (must be >= 1)
    /// * `column` - The 1-based column number (must be >= 1)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Plus, "+".to_string(), 1, 3);
    /// assert_eq!(token.token_type, TokenType::Plus);
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }

    /// Returns a debug representation of the token.
    ///
    /// This is useful for debugging and testing purposes, providing a human-readable
    /// view of the token's contents.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// let debug_str = token.debug_repr();
    /// assert!(debug_str.contains("NUMBER"));
    /// assert!(debug_str.contains("42"));
    /// ```
    #[must_use]
    pub fn debug_repr(&self) -> String {
        format!(
            "Token({}, '{}', line={}, col={})",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token({}, '{}', {}:{})",
            self.token_type, self.lexeme, self.line, self.column
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_debug() {
        assert_eq!(format!("{:?}", TokenType::Number), "Number");
        assert_eq!(format!("{:?}", TokenType::Plus), "Plus");
        assert_eq!(format!("{:?}", TokenType::Eof), "Eof");
    }

    #[test]
    fn test_token_type_display() {
        assert_eq!(format!("{}", TokenType::Number), "NUMBER");
        assert_eq!(format!("{}", TokenType::Plus), "PLUS");
        assert_eq!(format!("{}", TokenType::Minus), "MINUS");
        assert_eq!(format!("{}", TokenType::Star), "STAR");
        assert_eq!(format!("{}", TokenType::Slash), "SLASH");
        assert_eq!(format!("{}", TokenType::Eof), "EOF");
    }

    #[test]
    fn test_token_type_equality() {
        assert_eq!(TokenType::Number, TokenType::Number);
        assert_ne!(TokenType::Number, TokenType::Plus);
    }

    #[test]
    fn test_token_type_copy() {
        let t1 = TokenType::Number;
        let t2 = t1; // Copy, not move
        assert_eq!(t1, t2);
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        assert_eq!(token.token_type, TokenType::Number);
        assert_eq!(token.lexeme, "42");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 1);
    }

    #[test]
    fn test_token_with_operator() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 3);
        assert_eq!(token.token_type, TokenType::Plus);
        assert_eq!(token.lexeme, "+");
    }

    #[test]
    fn test_token_with_multiline_position() {
        let token = Token::new(TokenType::Number, "123".to_string(), 5, 10);
        assert_eq!(token.line, 5);
        assert_eq!(token.column, 10);
    }

    #[test]
    fn test_token_debug_repr() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let debug = token.debug_repr();
        assert!(debug.contains("NUMBER"));
        assert!(debug.contains("42"));
        assert!(debug.contains("line=1"));
        assert!(debug.contains("col=1"));
    }

    #[test]
    fn test_token_display() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 2, 5);
        let display = format!("{}", token);
        assert!(display.contains("PLUS"));
        assert!(display.contains("+"));
        assert!(display.contains("2:5"));
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let token2 = token1.clone();
        assert_eq!(token1, token2);
        assert_eq!(token1.lexeme, token2.lexeme);
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
    fn test_token_with_floating_point() {
        let token = Token::new(TokenType::Number, "3.14".to_string(), 1, 1);
        assert_eq!(token.lexeme, "3.14");
    }

    #[test]
    fn test_token_with_negative_number() {
        let token = Token::new(TokenType::Number, "-5".to_string(), 1, 1);
        assert_eq!(token.lexeme, "-5");
    }

    #[test]
    fn test_eof_token() {
        let token = Token::new(TokenType::Eof, "".to_string(), 1, 10);
        assert_eq!(token.token_type, TokenType::Eof);
        assert_eq!(token.lexeme, "");
    }

    #[test]
    fn test_all_operator_types() {
        let plus = Token::new(TokenType::Plus, "+".to_string(), 1, 1);
        let minus = Token::new(TokenType::Minus, "-".to_string(), 1, 1);
        let star = Token::new(TokenType::Star, "*".to_string(), 1, 1);
        let slash = Token::new(TokenType::Slash, "/".to_string(), 1, 1);

        assert_eq!(plus.lexeme, "+");
        assert_eq!(minus.lexeme, "-");
        assert_eq!(star.lexeme, "*");
        assert_eq!(slash.lexeme, "/");
    }
}

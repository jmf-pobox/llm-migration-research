//! Token types and data structures for lexical analysis.
//!
//! This module defines the token representation used throughout the lexer/parser pipeline.
//! Tokens represent atomic units of RPN expressions, including numbers, operators, and
//! end-of-file markers.

/// Enumeration of all possible token types in RPN expressions.
///
/// Each variant represents a different category of lexical token that can appear
/// in the input text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// Numeric literal (integer or decimal)
    Number,
    /// Addition operator: `+`
    Plus,
    /// Subtraction operator: `-`
    Minus,
    /// Multiplication operator: `*`
    Multiply,
    /// Division operator: `/`
    Divide,
    /// End of file marker
    Eof,
}

/// A lexical token with position information.
///
/// Represents a single token in the input stream, carrying its type, value,
/// and position information for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
/// assert_eq!(token.token_type(), TokenType::Number);
/// assert_eq!(token.value(), Some("42"));
/// assert_eq!(token.line(), 1);
/// assert_eq!(token.column(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    value: Option<String>,
    line: usize,
    column: usize,
}

impl Token {
    /// Creates a new token with the given type, value, and position.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token
    /// * `value` - The string value of the token (e.g., "42", "+")
    /// * `line` - The line number where the token appears (1-based)
    /// * `column` - The column number where the token starts (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let num_token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// let op_token = Token::new(TokenType::Plus, "+".to_string(), 1, 3);
    /// ```
    #[must_use]
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        let value = if value.is_empty() { None } else { Some(value) };
        Self {
            token_type,
            value,
            line,
            column,
        }
    }

    /// Creates a new token with no value (for operators and EOF).
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token
    /// * `line` - The line number where the token appears (1-based)
    /// * `column` - The column number where the token starts (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let eof = Token::new_without_value(TokenType::Eof, 1, 10);
    /// assert_eq!(eof.value(), None);
    /// ```
    #[must_use]
    pub fn new_without_value(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            value: None,
            line,
            column,
        }
    }

    /// Returns the token type.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// assert_eq!(token.token_type(), TokenType::Number);
    /// ```
    #[must_use]
    pub const fn token_type(&self) -> TokenType {
        self.token_type
    }

    /// Returns the token's string value, if present.
    ///
    /// Returns `None` for operators and EOF tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// assert_eq!(token.value(), Some("42"));
    ///
    /// let eof = Token::new_without_value(TokenType::Eof, 1, 10);
    /// assert_eq!(eof.value(), None);
    /// ```
    #[must_use]
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }

    /// Returns the line number where the token appears (1-based).
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 2, 5);
    /// assert_eq!(token.line(), 2);
    /// ```
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number where the token starts (1-based).
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let token = Token::new(TokenType::Number, "42".to_string(), 2, 5);
    /// assert_eq!(token.column(), 5);
    /// ```
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = match self.token_type {
            TokenType::Number => "NUMBER",
            TokenType::Plus => "PLUS",
            TokenType::Minus => "MINUS",
            TokenType::Multiply => "MULTIPLY",
            TokenType::Divide => "DIVIDE",
            TokenType::Eof => "EOF",
        };

        match &self.value {
            Some(v) => write!(
                f,
                "Token({}, '{}', {}:{})",
                type_name, v, self.line, self.column
            ),
            None => write!(f, "Token({}, {}:{})", type_name, self.line, self.column),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_type_variants() {
        // Test all token type variants exist
        let types = [
            TokenType::Number,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Multiply,
            TokenType::Divide,
            TokenType::Eof,
        ];

        // Test that they can be compared for equality
        assert_eq!(TokenType::Number, TokenType::Number);
        assert_ne!(TokenType::Number, TokenType::Plus);

        // Test that all types are different
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i == j {
                    assert_eq!(t1, t2);
                } else {
                    assert_ne!(t1, t2);
                }
            }
        }
    }

    #[test]
    fn test_token_creation_with_value() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);

        assert_eq!(token.token_type(), TokenType::Number);
        assert_eq!(token.value(), Some("42"));
        assert_eq!(token.line(), 1);
        assert_eq!(token.column(), 1);
    }

    #[test]
    fn test_token_creation_without_value() {
        let token = Token::new_without_value(TokenType::Eof, 1, 10);

        assert_eq!(token.token_type(), TokenType::Eof);
        assert_eq!(token.value(), None);
        assert_eq!(token.line(), 1);
        assert_eq!(token.column(), 10);
    }

    #[test]
    fn test_token_empty_string_becomes_none() {
        let token = Token::new(TokenType::Eof, String::new(), 1, 1);
        assert_eq!(token.value(), None);
    }

    #[test]
    fn test_token_position_tracking() {
        let token = Token::new(TokenType::Number, "3.14".to_string(), 5, 12);

        assert_eq!(token.line(), 5);
        assert_eq!(token.column(), 12);
    }

    #[test]
    fn test_token_all_operator_types() {
        let plus = Token::new(TokenType::Plus, "+".to_string(), 1, 1);
        let minus = Token::new(TokenType::Minus, "-".to_string(), 1, 3);
        let multiply = Token::new(TokenType::Multiply, "*".to_string(), 1, 5);
        let divide = Token::new(TokenType::Divide, "/".to_string(), 1, 7);

        assert_eq!(plus.token_type(), TokenType::Plus);
        assert_eq!(minus.token_type(), TokenType::Minus);
        assert_eq!(multiply.token_type(), TokenType::Multiply);
        assert_eq!(divide.token_type(), TokenType::Divide);
    }

    #[test]
    fn test_token_display_with_value() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 5);
        let display = format!("{}", token);

        assert_eq!(display, "Token(NUMBER, '42', 1:5)");
    }

    #[test]
    fn test_token_display_without_value() {
        let token = Token::new_without_value(TokenType::Eof, 2, 10);
        let display = format!("{}", token);

        assert_eq!(display, "Token(EOF, 2:10)");
    }

    #[test]
    fn test_token_display_all_types() {
        let tokens = vec![
            (
                Token::new(TokenType::Number, "42".to_string(), 1, 1),
                "Token(NUMBER, '42', 1:1)",
            ),
            (
                Token::new(TokenType::Plus, "+".to_string(), 1, 3),
                "Token(PLUS, '+', 1:3)",
            ),
            (
                Token::new(TokenType::Minus, "-".to_string(), 1, 5),
                "Token(MINUS, '-', 1:5)",
            ),
            (
                Token::new(TokenType::Multiply, "*".to_string(), 1, 7),
                "Token(MULTIPLY, '*', 1:7)",
            ),
            (
                Token::new(TokenType::Divide, "/".to_string(), 1, 9),
                "Token(DIVIDE, '/', 1:9)",
            ),
            (
                Token::new_without_value(TokenType::Eof, 1, 11),
                "Token(EOF, 1:11)",
            ),
        ];

        for (token, expected) in tokens {
            assert_eq!(format!("{}", token), expected);
        }
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TokenType::Number, "42".to_string(), 1, 1);
        let token2 = token1.clone();

        assert_eq!(token1, token2);
        assert_eq!(token1.token_type(), token2.token_type());
        assert_eq!(token1.value(), token2.value());
        assert_eq!(token1.line(), token2.line());
        assert_eq!(token1.column(), token2.column());
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
    fn test_token_negative_number() {
        let token = Token::new(TokenType::Number, "-5".to_string(), 1, 1);

        assert_eq!(token.value(), Some("-5"));
        assert_eq!(token.token_type(), TokenType::Number);
    }

    #[test]
    fn test_token_floating_point() {
        let token = Token::new(TokenType::Number, "3.14".to_string(), 1, 1);

        assert_eq!(token.value(), Some("3.14"));
        assert_eq!(token.token_type(), TokenType::Number);
    }

    #[test]
    fn test_token_debug_format() {
        let token = Token::new(TokenType::Number, "42".to_string(), 1, 5);
        let debug = format!("{:?}", token);

        // Just verify debug format is non-empty and contains relevant info
        assert!(debug.contains("Token"));
        assert!(debug.contains("Number"));
        assert!(debug.contains("42"));
    }

    #[test]
    fn test_token_with_operator() {
        let token = Token::new(TokenType::Plus, "+".to_string(), 1, 5);
        assert_eq!(token.token_type(), TokenType::Plus);
        assert_eq!(token.value(), Some("+"));
        assert_eq!(token.line(), 1);
        assert_eq!(token.column(), 5);
    }

    #[test]
    fn test_token_with_decimal_number() {
        let token = Token::new(TokenType::Number, "3.14".to_string(), 2, 10);
        assert_eq!(token.token_type(), TokenType::Number);
        assert_eq!(token.value(), Some("3.14"));
        assert_eq!(token.line(), 2);
        assert_eq!(token.column(), 10);
    }

    #[test]
    fn test_token_position_sequence() {
        // Test that position information is preserved correctly
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 1, 3),
            Token::new(TokenType::Plus, "+".to_string(), 1, 5),
        ];

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].column(), 3);
        assert_eq!(tokens[2].line(), 1);
        assert_eq!(tokens[2].column(), 5);
    }

    #[test]
    fn test_multiline_token_positions() {
        let tokens = vec![
            Token::new(TokenType::Number, "5".to_string(), 1, 1),
            Token::new(TokenType::Number, "3".to_string(), 2, 1),
            Token::new(TokenType::Plus, "+".to_string(), 3, 1),
        ];

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[1].line(), 2);
        assert_eq!(tokens[2].line(), 3);
    }
}

//! Token types for the RPN to LaTeX lexer.

/// Types of tokens recognized by the lexer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TokenType {
    /// A numeric literal (integer or decimal).
    Number,
    /// Addition operator (+).
    Plus,
    /// Subtraction operator (-).
    Minus,
    /// Multiplication operator (*).
    Mult,
    /// Division operator (/).
    Div,
}

/// A token produced by the lexer.
///
/// # Examples
///
/// ```
/// use rpn2tex::tokens::{Token, TokenType};
///
/// let token = Token::new(TokenType::Number, "42", 1, 1);
/// assert_eq!(token.token_type(), TokenType::Number);
/// assert_eq!(token.value(), "42");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    token_type: TokenType,
    value: String,
    line: usize,
    column: usize,
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

    /// Returns the type of this token.
    #[must_use]
    pub const fn token_type(&self) -> TokenType {
        self.token_type
    }

    /// Returns the string value of this token.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Returns the line number (1-based).
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Returns the column number (1-based).
    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

//! Token types for the RPN to LaTeX converter.

/// Token types recognized by the lexer.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// End of file marker.
    Eof,
}

/// A token with type, value, and position information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    /// The type of token.
    pub token_type: TokenType,
    /// The lexeme (string content) of the token.
    pub lexeme: String,
    /// Line number where the token appears (1-indexed).
    pub line: usize,
    /// Column number where the token starts (1-indexed).
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::tokens::{Token, TokenType};
    /// let token = Token::new(TokenType::Number, "42".to_string(), 1, 1);
    /// assert_eq!(token.lexeme, "42");
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
}

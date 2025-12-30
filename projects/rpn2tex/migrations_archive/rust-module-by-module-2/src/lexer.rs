//! Lexical analysis for RPN expressions.
//!
//! This module provides a lexer that tokenizes RPN input text into a stream
//! of tokens, tracking position information for error reporting.

use crate::tokens::{Token, TokenType};
use std::fmt;

/// Error that occurs during lexical analysis.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::LexerError;
///
/// let error = LexerError::new("Unexpected character '^'", 1, 5);
/// assert_eq!(error.message, "Unexpected character '^'");
/// assert_eq!(error.line, 1);
/// assert_eq!(error.column, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// The error message
    pub message: String,

    /// Line number where the error occurred (1-based)
    pub line: usize,

    /// Column number where the error occurred (1-based)
    pub column: usize,
}

impl LexerError {
    /// Creates a new lexer error.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::LexerError;
    ///
    /// let error = LexerError::new("Invalid token", 2, 10);
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexerError {}

/// Lexer for tokenizing RPN expressions.
///
/// The lexer scans input text character by character, producing a stream
/// of tokens. It tracks line and column positions for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let mut lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
/// assert_eq!(tokens[0].token_type, TokenType::Number);
/// assert_eq!(tokens[0].value, "5");
/// ```
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input text.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let chars = text.chars().collect();
        Self {
            chars,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the input text into a vector of tokens.
    ///
    /// Returns an error if invalid characters are encountered.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if:
    /// - An unexpected character is encountered (e.g., `^`)
    /// - Any other lexical error occurs
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let mut lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens[0].token_type, TokenType::Number);
    /// assert_eq!(tokens[1].token_type, TokenType::Number);
    /// assert_eq!(tokens[2].token_type, TokenType::Plus);
    /// assert_eq!(tokens[3].token_type, TokenType::Eof);
    /// ```
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_whitespace();
            if self.at_end() {
                break;
            }
            tokens.push(self.scan_token()?);
        }

        // Add EOF token
        tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));
        Ok(tokens)
    }

    /// Checks if the lexer has reached the end of input.
    fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Peeks at the current character without consuming it.
    ///
    /// Returns `None` if at end of input.
    fn peek(&self) -> Option<char> {
        if self.at_end() {
            None
        } else {
            Some(self.chars[self.pos])
        }
    }

    /// Advances to the next character and returns the current one.
    ///
    /// Updates line and column tracking based on the character.
    fn advance(&mut self) -> char {
        let ch = self.chars[self.pos];
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    /// Skips whitespace characters, updating position tracking.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scans the next token from the input.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an unexpected character is encountered.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self
            .peek()
            .ok_or_else(|| LexerError::new("Unexpected end of input", start_line, start_column))?;

        // Single-character operators
        match ch {
            '+' => {
                self.advance();
                Ok(Token::new(TokenType::Plus, "+", start_line, start_column))
            }
            '-' => {
                self.advance();
                // Check for negative number
                if let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        return self.scan_number("-", start_line, start_column);
                    }
                }
                Ok(Token::new(TokenType::Minus, "-", start_line, start_column))
            }
            '*' => {
                self.advance();
                Ok(Token::new(TokenType::Mult, "*", start_line, start_column))
            }
            '/' => {
                self.advance();
                Ok(Token::new(TokenType::Div, "/", start_line, start_column))
            }
            _ if ch.is_ascii_digit() => self.scan_number("", start_line, start_column),
            _ => Err(LexerError::new(
                format!("Unexpected character '{ch}'"),
                start_line,
                start_column,
            )),
        }
    }

    /// Scans a number token (integer or floating-point).
    ///
    /// # Arguments
    ///
    /// * `prefix` - Optional prefix (e.g., "-" for negative numbers)
    /// * `start_line` - Line where the number starts
    /// * `start_column` - Column where the number starts
    ///
    /// # Errors
    ///
    /// This method does not return errors, but is marked for consistency with scan_token.
    fn scan_number(
        &mut self,
        prefix: &str,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexerError> {
        let mut value = prefix.to_string();

        // Integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.advance());
            } else {
                break;
            }
        }

        // Decimal part (optional)
        if let Some('.') = self.peek() {
            value.push(self.advance()); // consume '.'
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    value.push(self.advance());
                } else {
                    break;
                }
            }
        }

        Ok(Token::new(
            TokenType::Number,
            value,
            start_line,
            start_column,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_tokenize_basic_subtraction() {
        let mut lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, MINUS, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
    }

    #[test]
    fn test_tokenize_multiplication() {
        let mut lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[2].token_type, TokenType::Mult);
        assert_eq!(tokens[2].value, "*");
    }

    #[test]
    fn test_tokenize_division() {
        let mut lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[2].token_type, TokenType::Div);
        assert_eq!(tokens[2].value, "/");
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER(-5), EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
    }

    #[test]
    fn test_tokenize_negative_in_expression() {
        let mut lexer = Lexer::new("5 -3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER(5), NUMBER(-3), PLUS, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "-3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tokenize_minus_as_operator() {
        let mut lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, MINUS, NUMBER, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[1].value, "-");
        assert_eq!(tokens[2].value, "3");
    }

    #[test]
    fn test_tokenize_float() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_float_addition() {
        let mut lexer = Lexer::new("1.5 0.5 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "1.5");
        assert_eq!(tokens[1].value, "0.5");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let mut lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 5, 3, +, 2, *, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].value, "2");
        assert_eq!(tokens[4].token_type, TokenType::Mult);
    }

    #[test]
    fn test_tokenize_invalid_char() {
        let mut lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Unexpected character '^'"));
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 5);
    }

    #[test]
    fn test_tokenize_invalid_char_early() {
        let mut lexer = Lexer::new("2 3 ^ 4 *");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Unexpected character '^'"));
        // Error should stop at first invalid character
        assert_eq!(err.column, 5);
    }

    #[test]
    fn test_tokenize_multiple_invalid_chars() {
        let mut lexer = Lexer::new("2 3 4 ^ ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Unexpected character '^'"));
        // Should stop at first invalid character
        assert_eq!(err.column, 7);
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].line, 1);
        assert_eq!(tokens[2].column, 5);
    }

    #[test]
    fn test_multiline_tracking() {
        let mut lexer = Lexer::new("5 3\n+ 2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[2].column, 1);
        assert_eq!(tokens[3].line, 2);
        assert_eq!(tokens[3].column, 3);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \n  \t  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_lexer_error_display() {
        let error = LexerError::new("Test error", 2, 5);
        let display = format!("{error}");
        assert!(display.contains("Line 2"));
        assert!(display.contains("column 5"));
        assert!(display.contains("Test error"));
    }

    #[test]
    fn test_lexer_error_equality() {
        let err1 = LexerError::new("Error", 1, 1);
        let err2 = LexerError::new("Error", 1, 1);
        let err3 = LexerError::new("Different", 1, 1);
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_all_operators() {
        let mut lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // +, -, *, /, EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Mult);
        assert_eq!(tokens[3].token_type, TokenType::Div);
    }

    #[test]
    fn test_consecutive_numbers() {
        let mut lexer = Lexer::new("123 456 789");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 3 numbers + EOF
        assert_eq!(tokens[0].value, "123");
        assert_eq!(tokens[1].value, "456");
        assert_eq!(tokens[2].value, "789");
    }
}

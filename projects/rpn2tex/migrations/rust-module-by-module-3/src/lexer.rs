//! Lexer for tokenizing RPN input text.
//!
//! This module provides character-by-character tokenization of RPN expressions,
//! with position tracking for error reporting.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::lexer::Lexer;
//! use rpn2tex::tokens::TokenType;
//!
//! let mut lexer = Lexer::new("5 3 +");
//! let tokens = lexer.tokenize().unwrap();
//! assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
//! assert_eq!(tokens[0].token_type(), TokenType::Number);
//! assert_eq!(tokens[2].token_type(), TokenType::Plus);
//! ```

use crate::error::ErrorFormatter;
use crate::tokens::{Token, TokenType};

/// A lexer that tokenizes RPN input text character by character.
///
/// The lexer maintains position information (line and column) as it scans through
/// the input, which is used for error reporting.
///
/// # Position Tracking
///
/// - Line and column numbers are 1-based (start at 1)
/// - Column increments on each character
/// - Line increments on newline, and column resets to 1
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let mut lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
///
/// assert_eq!(tokens[0].value(), Some("5"));
/// assert_eq!(tokens[0].line(), 1);
/// assert_eq!(tokens[0].column(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct Lexer {
    /// The input text to tokenize
    input: String,
    /// Current position in input (byte offset)
    position: usize,
    /// Current line number (1-based)
    line: usize,
    /// Current column number (1-based)
    column: usize,
}

impl Lexer {
    /// Creates a new lexer with the given input text.
    ///
    /// Initializes position tracking at line 1, column 1.
    ///
    /// # Arguments
    ///
    /// * `input` - The input text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the entire input text.
    ///
    /// Returns a vector of tokens ending with an EOF token.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<Token>)` on success, or `Err(String)` with a formatted error message.
    ///
    /// # Errors
    ///
    /// Returns an error if an invalid character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let mut lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens.len(), 4);
    /// assert_eq!(tokens[3].token_type(), TokenType::Eof);
    /// ```
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            match self.scan_token() {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            }
        }

        // Add EOF token
        tokens.push(Token::new_without_value(
            TokenType::Eof,
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    /// Checks if we're at the end of input.
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Returns the current character without consuming it.
    fn current_char(&self) -> Option<char> {
        if self.is_at_end() {
            None
        } else {
            self.input[self.position..].chars().next()
        }
    }

    /// Consumes and returns the current character, updating position tracking.
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char() {
            self.position += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    /// Skips over whitespace characters (space, tab, newline, carriage return).
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scans and returns the next token.
    fn scan_token(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.current_char().ok_or_else(|| {
            self.format_error("Unexpected end of input", start_line, start_column)
        })?;

        match ch {
            '+' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Plus,
                    "+".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '-' => {
                self.advance();
                // Check if next character is a digit (negative number)
                if let Some(next_ch) = self.current_char() {
                    if next_ch.is_ascii_digit() {
                        // This is a negative number, scan it
                        return self.scan_number("-".to_string(), start_line, start_column);
                    }
                }
                // It's a minus operator
                Ok(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '*' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Multiply,
                    "*".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '/' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Divide,
                    "/".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '0'..='9' => self.scan_number(String::new(), start_line, start_column),
            _ => Err(self.format_error(
                &format!("Unexpected character '{ch}'"),
                start_line,
                start_column,
            )),
        }
    }

    /// Scans a numeric literal (integer or floating-point).
    fn scan_number(
        &mut self,
        prefix: String,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, String> {
        let mut value = prefix;

        // Scan integer part
        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if let Some('.') = self.current_char() {
            value.push('.');
            self.advance();

            // Scan fractional part
            while let Some(ch) = self.current_char() {
                if ch.is_ascii_digit() {
                    value.push(ch);
                    self.advance();
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

    /// Formats an error message with source context.
    fn format_error(&self, message: &str, line: usize, column: usize) -> String {
        let formatter = ErrorFormatter::new(&self.input);
        formatter.format_error(message, line, column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_single_number() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_tokenize_simple_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), Some("3"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[3].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_tokenize_all_operators() {
        let mut lexer = Lexer::new("5 3 + 2 - 4 * 10 /");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[4].token_type(), TokenType::Minus);
        assert_eq!(tokens[6].token_type(), TokenType::Multiply);
        assert_eq!(tokens[8].token_type(), TokenType::Divide);
    }

    #[test]
    fn test_tokenize_floating_point() {
        let mut lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), Some("3.14"));
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), Some("2"));
        assert_eq!(tokens[2].token_type(), TokenType::Multiply);
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut lexer = Lexer::new("-5 3 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), Some("-5"));
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), Some("3"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }

    #[test]
    fn test_tokenize_negative_float() {
        let mut lexer = Lexer::new("-3.14");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), Some("-3.14"));
    }

    #[test]
    fn test_tokenize_minus_as_operator() {
        let mut lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[2].token_type(), TokenType::Minus);
    }

    #[test]
    fn test_tokenize_position_tracking() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].column(), 3);
        assert_eq!(tokens[2].line(), 1);
        assert_eq!(tokens[2].column(), 5);
    }

    #[test]
    fn test_tokenize_multiline() {
        let mut lexer = Lexer::new("5\n3\n+");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].line(), 2);
        assert_eq!(tokens[1].column(), 1);
        assert_eq!(tokens[2].line(), 3);
        assert_eq!(tokens[2].column(), 1);
    }

    #[test]
    fn test_tokenize_tabs_and_spaces() {
        let mut lexer = Lexer::new("5\t3  +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[1].value(), Some("3"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }

    #[test]
    fn test_tokenize_invalid_character() {
        let mut lexer = Lexer::new("5 3 @");
        let result = lexer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected character '@'"));
    }

    #[test]
    fn test_tokenize_invalid_character_position() {
        let mut lexer = Lexer::new("5 3 @");
        let result = lexer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("1 | 5 3 @"));
        assert!(err.contains("^"));
    }

    #[test]
    fn test_tokenize_multiple_digits() {
        let mut lexer = Lexer::new("42 123 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].value(), Some("42"));
        assert_eq!(tokens[1].value(), Some("123"));
    }

    #[test]
    fn test_tokenize_decimal_only() {
        let mut lexer = Lexer::new("0.5");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].value(), Some("0.5"));
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let mut lexer = Lexer::new("5 3 + 2 *");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[1].value(), Some("3"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[3].value(), Some("2"));
        assert_eq!(tokens[4].token_type(), TokenType::Multiply);
        assert_eq!(tokens[5].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_tokenize_no_spaces() {
        let mut lexer = Lexer::new("5 3+");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[1].value(), Some("3"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }

    #[test]
    fn test_tokenize_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let mut lexer = Lexer::new("   \t\n  ");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_tokenize_carriage_return() {
        let mut lexer = Lexer::new("5\r\n3");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value(), Some("5"));
        assert_eq!(tokens[1].value(), Some("3"));
    }

    #[test]
    fn test_tokenize_large_numbers() {
        let mut lexer = Lexer::new("1234567890 98765.4321 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].value(), Some("1234567890"));
        assert_eq!(tokens[1].value(), Some("98765.4321"));
    }

    #[test]
    fn test_tokenize_leading_zero() {
        let mut lexer = Lexer::new("0 01 001");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].value(), Some("0"));
        assert_eq!(tokens[1].value(), Some("01"));
        assert_eq!(tokens[2].value(), Some("001"));
    }

    #[test]
    fn test_tokenize_eof_position() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();

        let eof = &tokens[tokens.len() - 1];
        assert_eq!(eof.token_type(), TokenType::Eof);
        assert_eq!(eof.line(), 1);
        assert_eq!(eof.column(), 6); // After the last character
    }

    #[test]
    fn test_minus_followed_by_space_is_operator() {
        let mut lexer = Lexer::new("- 5");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].token_type(), TokenType::Minus);
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), Some("5"));
    }

    #[test]
    fn test_consecutive_operators() {
        let mut lexer = Lexer::new("+-*/");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type(), TokenType::Plus);
        assert_eq!(tokens[1].token_type(), TokenType::Minus);
        assert_eq!(tokens[2].token_type(), TokenType::Multiply);
        assert_eq!(tokens[3].token_type(), TokenType::Divide);
        assert_eq!(tokens[4].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_lexer_clone() {
        let lexer1 = Lexer::new("5 3 +");
        let mut lexer2 = lexer1.clone();

        let tokens = lexer2.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn test_column_tracking_after_newline() {
        let mut lexer = Lexer::new("5\n3");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].line(), 2);
        assert_eq!(tokens[1].column(), 1);
    }

    #[test]
    fn test_multiple_newlines() {
        let mut lexer = Lexer::new("5\n\n\n3");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[1].line(), 4);
    }

    #[test]
    fn test_error_on_unsupported_exponentiation() {
        let mut lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected character '^'"));
    }

    #[test]
    fn test_error_on_letter() {
        let mut lexer = Lexer::new("invalid");
        let result = lexer.tokenize();

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Unexpected character 'i'"));
    }

    #[test]
    fn test_decimal_in_middle_of_expression() {
        let mut lexer = Lexer::new("1.5 0.5 +");
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].value(), Some("1.5"));
        assert_eq!(tokens[1].value(), Some("0.5"));
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }
}

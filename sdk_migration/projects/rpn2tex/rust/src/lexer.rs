//! Tokenization of RPN expressions into token streams.
//!
//! This module provides the `Lexer` struct for scanning character streams
//! and converting them into tokens with position tracking.

use crate::tokens::{Token, TokenType};
use std::fmt;

/// Error that occurs during lexical analysis.
///
/// Contains the error message and position information for reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::LexerError;
///
/// let error = LexerError {
///     message: "Unexpected character '^'".to_string(),
///     line: 1,
///     column: 5,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// The error message
    pub message: String,
    /// The line number where the error occurred (1-based)
    pub line: u32,
    /// The column number where the error occurred (1-based)
    pub column: u32,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LexerError at line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for LexerError {}

/// Tokenizer for RPN expressions.
///
/// Scans character-by-character, aggregating numbers, recognizing operators,
/// and tracking position for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens[0].type_, TokenType::Number);
/// assert_eq!(tokens[1].type_, TokenType::Number);
/// assert_eq!(tokens[2].type_, TokenType::Plus);
/// assert_eq!(tokens[3].type_, TokenType::Eof);
/// ```
pub struct Lexer {
    text: String,
    pos: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    /// Creates a new lexer for the given input text.
    ///
    /// Position tracking starts at line 1, column 1.
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
        Self {
            text: text.into(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the input text into a vector of tokens.
    ///
    /// Returns a vector of tokens ending with an EOF token, or a `LexerError`
    /// if an unexpected character is encountered.
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an unexpected character (like `^`) is found.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// let result = lexer.tokenize();
    /// assert!(result.is_ok());
    ///
    /// let lexer = Lexer::new("5 ^");
    /// let result = lexer.tokenize();
    /// assert!(result.is_err());
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at_end() {
                break;
            }

            let token = self.scan_token()?;
            tokens.push(token);
        }

        // Add EOF token
        tokens.push(Token {
            type_: TokenType::Eof,
            value: String::new(),
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    /// Checks if we've reached the end of the input.
    #[must_use]
    fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    /// Returns the current character without advancing.
    #[must_use]
    fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    /// Returns the next character without advancing.
    #[must_use]
    fn peek_next(&self) -> Option<char> {
        self.text.chars().nth(self.pos + 1)
    }

    /// Advances position and returns the current character.
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
            self.pos += 1;
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

    /// Skips whitespace characters (space, tab, newline, CR).
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scans a single token from the current position.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().ok_or_else(|| LexerError {
            message: "Unexpected end of input".to_string(),
            line: self.line,
            column: self.column,
        })?;

        match ch {
            '+' => {
                self.advance();
                Ok(Token {
                    type_: TokenType::Plus,
                    value: "+".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '-' => {
                // Lookahead for negative numbers
                if let Some(next_ch) = self.peek_next() {
                    if next_ch.is_ascii_digit() {
                        return Ok(self.scan_number("-".to_string(), start_line, start_column));
                    }
                }
                self.advance();
                Ok(Token {
                    type_: TokenType::Minus,
                    value: "-".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '*' => {
                self.advance();
                Ok(Token {
                    type_: TokenType::Mult,
                    value: "*".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            '/' => {
                self.advance();
                Ok(Token {
                    type_: TokenType::Div,
                    value: "/".to_string(),
                    line: start_line,
                    column: start_column,
                })
            }
            _ if ch.is_ascii_digit() => {
                Ok(self.scan_number(String::new(), start_line, start_column))
            }
            _ => Err(LexerError {
                message: format!("Unexpected character '{ch}'"),
                line: self.line,
                column: self.column,
            }),
        }
    }

    /// Scans a number (integer or decimal) from the current position.
    fn scan_number(&mut self, prefix: String, start_line: u32, start_column: u32) -> Token {
        let mut value = prefix;

        // If prefix is "-", advance past it first
        if value == "-" {
            self.advance();
        }

        // Scan integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if let Some('.') = self.peek() {
            value.push('.');
            self.advance();

            // Scan fractional part
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    value.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        Token {
            type_: TokenType::Number,
            value,
            line: start_line,
            column: start_column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number() {
        let lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER + EOF
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[1].type_, TokenType::Eof);
    }

    #[test]
    fn test_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_negative_number() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
    }

    #[test]
    fn test_single_operators() {
        let operators = vec![
            ("+", TokenType::Plus),
            ("-", TokenType::Minus),
            ("*", TokenType::Mult),
            ("/", TokenType::Div),
        ];

        for (input, expected_type) in operators {
            let lexer = Lexer::new(input);
            let tokens = lexer.tokenize().unwrap();
            assert_eq!(tokens.len(), 2); // OPERATOR + EOF
            assert_eq!(tokens[0].type_, expected_type);
            assert_eq!(tokens[0].value, input);
        }
    }

    #[test]
    fn test_simple_expression() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].type_, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].type_, TokenType::Plus);
        assert_eq!(tokens[3].type_, TokenType::Eof);
    }

    #[test]
    fn test_complex_expression() {
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].type_, TokenType::Mult);
    }

    #[test]
    fn test_whitespace_handling() {
        let lexer = Lexer::new("  5  \t 3  \n +  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].type_, TokenType::Plus);
    }

    #[test]
    fn test_unexpected_character_error() {
        let lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 5);
        assert!(error.message.contains('^'));
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5\n3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[1].column, 1);
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[2].column, 3);
    }

    #[test]
    fn test_minus_as_operator_vs_negative_number() {
        // Minus followed by space should be an operator
        let lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].type_, TokenType::Minus);
        assert_eq!(tokens[2].type_, TokenType::Number);
        assert_eq!(tokens[2].value, "3");

        // Minus followed immediately by digit should be negative number
        let lexer = Lexer::new("-3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER, EOF
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "-3");
    }

    #[test]
    fn test_eof_token() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let eof = &tokens[tokens.len() - 1];
        assert_eq!(eof.type_, TokenType::Eof);
        assert_eq!(eof.value, "");
    }

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1); // Only EOF
        assert_eq!(tokens[0].type_, TokenType::Eof);
    }

    #[test]
    fn test_error_display() {
        let error = LexerError {
            message: "Test error".to_string(),
            line: 2,
            column: 5,
        };
        let display = format!("{error}");
        assert!(display.contains("line 2"));
        assert!(display.contains("column 5"));
        assert!(display.contains("Test error"));
    }

    // I/O Contract validation tests

    #[test]
    fn test_io_contract_case_1() {
        // "5 3 +" should tokenize to [NUMBER("5"), NUMBER("3"), PLUS, EOF]
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].type_, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].type_, TokenType::Plus);
        assert_eq!(tokens[3].type_, TokenType::Eof);
    }

    #[test]
    fn test_io_contract_case_5() {
        // "2 3 ^" should return LexerError at line 1, column 5
        let lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 5);
        assert!(error.message.contains('^'));
    }

    #[test]
    fn test_io_contract_negative_number() {
        // "-5" should tokenize to [NUMBER("-5"), EOF]
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
        assert_eq!(tokens[1].type_, TokenType::Eof);
    }

    #[test]
    fn test_io_contract_case_18() {
        // "3.14 2 *" should tokenize to [NUMBER("3.14"), NUMBER("2"), MULT, EOF]
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].type_, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].type_, TokenType::Mult);
        assert_eq!(tokens[3].type_, TokenType::Eof);
    }
}

//! Tokenizes RPN input text into a stream of tokens.
//!
//! This module provides lexical analysis for RPN expressions, converting
//! raw text input into a sequence of tokens that can be parsed into an AST.

use crate::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Error type for lexical analysis failures.
///
/// Raised when the lexer encounters invalid input such as unexpected
/// characters or malformed tokens.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::LexerError;
///
/// let error = LexerError::new("Unexpected character '@'", 1, 5);
/// assert_eq!(error.line(), 1);
/// assert_eq!(error.column(), 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// Error description
    message: String,
    /// 1-based line number where the error occurred
    line: u32,
    /// 1-based column number where the error occurred
    column: u32,
}

impl LexerError {
    /// Creates a new lexer error.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `line` - 1-based line number where the error occurred
    /// * `column` - 1-based column number where the error occurred
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::LexerError;
    ///
    /// let error = LexerError::new("Invalid token", 2, 10);
    /// assert_eq!(error.message(), "Invalid token");
    /// ```
    #[must_use]
    pub fn new(message: impl Into<String>, line: u32, column: u32) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }

    /// Returns the error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns the line number where the error occurred (1-based).
    #[must_use]
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns the column number where the error occurred (1-based).
    #[must_use]
    pub fn column(&self) -> u32 {
        self.column
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

impl Error for LexerError {}

/// Tokenizes RPN input text.
///
/// The `Lexer` performs character-by-character scanning of RPN input,
/// producing a token stream. It tracks position information (line and column)
/// for error reporting.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().expect("tokenize failed");
/// assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
/// assert_eq!(tokens[0].token_type, TokenType::Number);
/// assert_eq!(tokens[0].value, "5");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    /// Input text to tokenize
    text: String,
    /// Current position in text (0-based)
    pos: usize,
    /// Current line number (1-based)
    line: u32,
    /// Current column number (1-based)
    column: u32,
}

impl Lexer {
    /// Creates a new lexer for the given input text.
    ///
    /// Position tracking starts at line 1, column 1 (1-based indexing).
    ///
    /// # Arguments
    ///
    /// * `text` - The RPN input text to tokenize
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

    /// Tokenizes the entire input text.
    ///
    /// Returns a vector of tokens, including a final EOF token.
    /// If the input contains invalid characters or malformed tokens,
    /// returns a `LexerError`.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if the input contains:
    /// - Unexpected characters that are not valid operators or number components
    /// - Malformed tokens
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let lexer = Lexer::new("10 2 /");
    /// let tokens = lexer.tokenize().expect("tokenize failed");
    /// assert_eq!(tokens[0].token_type, TokenType::Number);
    /// assert_eq!(tokens[0].value, "10");
    /// assert_eq!(tokens[1].token_type, TokenType::Number);
    /// assert_eq!(tokens[1].value, "2");
    /// assert_eq!(tokens[2].token_type, TokenType::Div);
    /// assert_eq!(tokens[3].token_type, TokenType::Eof);
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_whitespace();
            if self.at_end() {
                break;
            }
            tokens.push(self.scan_token()?);
        }

        // Always append EOF token
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    /// Checks if we've reached the end of input.
    fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    /// Returns the current character without advancing position.
    fn peek(&self) -> char {
        self.text[self.pos..].chars().next().unwrap_or('\0')
    }

    /// Consumes and returns the current character, updating position tracking.
    fn advance(&mut self) -> char {
        let ch = self.peek();
        if ch == '\0' {
            return ch;
        }

        // Update position
        self.pos += ch.len_utf8();

        // Update line and column tracking
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        ch
    }

    /// Skips whitespace characters (space, tab, newline, carriage return).
    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            let ch = self.peek();
            if matches!(ch, ' ' | '\t' | '\n' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scans the next token from the input.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.advance();

        match ch {
            '+' => Ok(Token::new(
                TokenType::Plus,
                "+".to_string(),
                start_line,
                start_column,
            )),
            '*' => Ok(Token::new(
                TokenType::Mult,
                "*".to_string(),
                start_line,
                start_column,
            )),
            '/' => Ok(Token::new(
                TokenType::Div,
                "/".to_string(),
                start_line,
                start_column,
            )),
            '-' => {
                // Minus can be subtraction operator OR negative number prefix
                // If followed immediately by a digit (no whitespace), it's a negative number
                if !self.at_end() && self.peek().is_ascii_digit() {
                    self.scan_number("-".to_string(), start_line, start_column)
                } else {
                    Ok(Token::new(
                        TokenType::Minus,
                        "-".to_string(),
                        start_line,
                        start_column,
                    ))
                }
            }
            _ if ch.is_ascii_digit() => self.scan_number(ch.to_string(), start_line, start_column),
            _ => Err(LexerError::new(
                format!("Unexpected character '{}'", ch),
                start_line,
                start_column,
            )),
        }
    }

    /// Scans a number token (integer or decimal).
    ///
    /// Numbers consist of digits with an optional decimal point.
    /// The prefix parameter contains any characters already consumed (e.g., "-" or first digit).
    fn scan_number(
        &mut self,
        mut prefix: String,
        start_line: u32,
        start_column: u32,
    ) -> Result<Token, LexerError> {
        // Scan integer part
        while !self.at_end() && self.peek().is_ascii_digit() {
            prefix.push(self.advance());
        }

        // Scan optional decimal part
        if !self.at_end() && self.peek() == '.' {
            // Look ahead to ensure there's a digit after the decimal point
            let pos_after_dot = self.pos + 1;
            if pos_after_dot < self.text.len()
                && self.text[pos_after_dot..]
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_ascii_digit())
            {
                prefix.push(self.advance()); // consume '.'
                while !self.at_end() && self.peek().is_ascii_digit() {
                    prefix.push(self.advance());
                }
            }
        }

        Ok(Token::new(
            TokenType::Number,
            prefix,
            start_line,
            start_column,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_error_creation() {
        let error = LexerError::new("Test error", 1, 5);
        assert_eq!(error.message(), "Test error");
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), 5);
    }

    #[test]
    fn test_lexer_error_display() {
        let error = LexerError::new("Unexpected character", 2, 10);
        let display = format!("{}", error);
        assert!(display.contains("Line 2, column 10"));
        assert!(display.contains("Unexpected character"));
    }

    #[test]
    fn test_tokenize_empty_string() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_tokenize_single_number() {
        let lexer = Lexer::new("42");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 2); // number + EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_tokenize_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_negative_number() {
        let lexer = Lexer::new("-42");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-42");
    }

    #[test]
    fn test_tokenize_operators() {
        let lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 5); // 4 operators + EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Mult);
        assert_eq!(tokens[3].token_type, TokenType::Div);
        assert_eq!(tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn test_tokenize_simple_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[2].value, "+");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let lexer = Lexer::new("10 2 / 5 *");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 6); // 10, 2, /, 5, *, EOF
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Div);
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].token_type, TokenType::Mult);
    }

    #[test]
    fn test_tokenize_with_decimal_numbers() {
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Mult);
    }

    #[test]
    fn test_tokenize_multiple_decimals() {
        let lexer = Lexer::new("1.5 0.5 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "1.5");
        assert_eq!(tokens[1].value, "0.5");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tokenize_minus_as_operator() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[2].value, "-");
    }

    #[test]
    fn test_tokenize_minus_as_negative_number() {
        let lexer = Lexer::new("-5 3 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tokenize_mixed_minus_usage() {
        // "0 5 -" means 0 - 5 (subtraction)
        let lexer = Lexer::new("0 5 -");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "0");
        assert_eq!(tokens[1].value, "5");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
    }

    #[test]
    fn test_tokenize_error_unexpected_character() {
        let lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.line(), 1);
        assert_eq!(error.column(), 5);
        assert!(error.message().contains("^"));
    }

    #[test]
    fn test_tokenize_error_in_middle() {
        let lexer = Lexer::new("2 3 ^ 4 *");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.column(), 5); // Position of '^'
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        // 5 at column 1, 3 at column 3, + at column 5
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].column, 5);
    }

    #[test]
    fn test_multiline_position_tracking() {
        let lexer = Lexer::new("5 3\n+ 2");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1); // 5
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3); // 3
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[2].column, 1); // +
        assert_eq!(tokens[3].line, 2);
        assert_eq!(tokens[3].column, 3); // 2
    }

    #[test]
    fn test_whitespace_skipping() {
        let lexer = Lexer::new("  5   3   +  ");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_tabs_and_newlines() {
        let lexer = Lexer::new("5\t3\n+");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_no_whitespace() {
        // This should work but operators won't be separate from numbers
        // Actually with current implementation, "53+" would tokenize as "53" then error on '+'
        // But "5+3" would be "5" + "3"? No, it would be "5" then error.
        // Let's test valid case
        let lexer = Lexer::new("5 3+");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
    }

    #[test]
    fn test_large_numbers() {
        let lexer = Lexer::new("999999 1000000 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].value, "999999");
        assert_eq!(tokens[1].value, "1000000");
    }

    #[test]
    fn test_many_decimal_places() {
        let lexer = Lexer::new("3.14159265359");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].value, "3.14159265359");
    }

    #[test]
    fn test_zero() {
        let lexer = Lexer::new("0");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].value, "0");
    }

    #[test]
    fn test_zero_decimal() {
        let lexer = Lexer::new("0.0");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].value, "0.0");
    }

    #[test]
    fn test_negative_decimal() {
        let lexer = Lexer::new("-3.14");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens[0].value, "-3.14");
    }

    #[test]
    fn test_eof_position() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().expect("tokenize failed");
        let eof_token = tokens.last().unwrap();
        assert_eq!(eof_token.token_type, TokenType::Eof);
        // EOF should be at the position after the last character
        assert_eq!(eof_token.line, 1);
    }

    #[test]
    fn test_clone_lexer() {
        let lexer1 = Lexer::new("5 3 +");
        let lexer2 = lexer1.clone();

        let tokens1 = lexer1.tokenize().expect("tokenize failed");
        let tokens2 = lexer2.tokenize().expect("tokenize failed");

        assert_eq!(tokens1, tokens2);
    }

    #[test]
    fn test_error_equality() {
        let err1 = LexerError::new("Test", 1, 5);
        let err2 = LexerError::new("Test", 1, 5);
        let err3 = LexerError::new("Other", 1, 5);

        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_carriage_return_handling() {
        let lexer = Lexer::new("5\r\n3\r\n+");
        let tokens = lexer.tokenize().expect("tokenize failed");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }
}

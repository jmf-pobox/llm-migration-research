//! Lexical analysis for RPN expressions.
//!
//! This module provides the [`Lexer`] struct for tokenizing RPN mathematical expressions.
//! It converts raw text input into a stream of tokens, handling numbers (integers and
//! floating-point), operators, and whitespace while tracking position information for
//! error reporting.
//!
//! # Examples
//!
//! ```
//! use rpn2tex::Lexer;
//!
//! let lexer = Lexer::new("5 3 +");
//! let tokens = lexer.tokenize().unwrap();
//! assert_eq!(tokens.len(), 4); // Number, Number, Plus, Eof
//! ```

use crate::error::Rpn2TexError;
use crate::tokens::{Token, TokenType};

/// A lexical analyzer for RPN expressions.
///
/// The lexer scans input text character by character, producing a sequence of tokens.
/// It tracks line and column positions for error reporting and handles:
/// - Numbers (integers and floating-point)
/// - Operators (+, -, *, /)
/// - Whitespace (spaces, tabs, newlines)
///
/// # Examples
///
/// ```
/// use rpn2tex::Lexer;
///
/// let lexer = Lexer::new("3.14 2 *");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 4); // Number, Number, Star, Eof
/// ```
#[derive(Debug, Clone)]
pub struct Lexer {
    /// The source text being tokenized
    source: String,
    /// Current position in the source (0-based byte index)
    position: usize,
    /// Current line number (1-based)
    line: usize,
    /// Current column number (1-based)
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given source text.
    ///
    /// # Arguments
    ///
    /// * `source` - The source text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the entire source text.
    ///
    /// Scans the source and produces a vector of tokens, ending with an EOF token.
    /// Returns an error if an invalid character is encountered.
    ///
    /// # Errors
    ///
    /// Returns a [`Rpn2TexError::LexerError`] if:
    /// - An unexpected character is encountered
    /// - A malformed number is detected
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens.len(), 4);
    /// ```
    ///
    /// ```
    /// use rpn2tex::Lexer;
    ///
    /// let lexer = Lexer::new("2 3 ^");
    /// let result = lexer.tokenize();
    /// assert!(result.is_err());
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, Rpn2TexError> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }
            tokens.push(self.scan_token()?);
        }

        // Add EOF token
        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    /// Checks if we've reached the end of the source.
    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    /// Peeks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    /// Peeks at the next character without consuming it.
    fn peek_next(&self) -> Option<char> {
        let mut chars = self.source[self.position..].chars();
        chars.next();
        chars.next()
    }

    /// Advances to the next character and returns it.
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.peek() {
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

    /// Skips whitespace characters (spaces, tabs, newlines, carriage returns).
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
    fn scan_token(&mut self) -> Result<Token, Rpn2TexError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().ok_or_else(|| {
            Rpn2TexError::lexer_error("Unexpected end of input", self.line, self.column)
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
                // Check if this is a negative number or a minus operator
                if let Some(next_ch) = self.peek_next() {
                    if next_ch.is_ascii_digit() {
                        // This is a negative number
                        return self.scan_number(start_line, start_column);
                    }
                }
                // This is a minus operator
                self.advance();
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
                    TokenType::Star,
                    "*".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '/' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Slash,
                    "/".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '0'..='9' => self.scan_number(start_line, start_column),
            _ => Err(Rpn2TexError::lexer_error(
                format!("Unexpected character '{}'", ch),
                start_line,
                start_column,
            )),
        }
    }

    /// Scans a number (integer or floating-point) from the current position.
    fn scan_number(
        &mut self,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, Rpn2TexError> {
        let mut lexeme = String::new();

        // Handle negative sign
        if self.peek() == Some('-') {
            lexeme.push('-');
            self.advance();
        }

        // Scan integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                lexeme.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.peek() == Some('.') {
            // Look ahead to ensure there's a digit after the decimal point
            if let Some(next_ch) = self.peek_next() {
                if next_ch.is_ascii_digit() {
                    lexeme.push('.');
                    self.advance(); // consume '.'

                    // Scan decimal part
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            lexeme.push(ch);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        Ok(Token::new(
            TokenType::Number,
            lexeme,
            start_line,
            start_column,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_single_number() {
        let lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "42");
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_floating_point_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "3.14");
    }

    #[test]
    fn test_negative_number() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "-5");
    }

    #[test]
    fn test_simple_expression() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].lexeme, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[2].lexeme, "+");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_all_operators() {
        let lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].token_type, TokenType::Slash);
        assert_eq!(tokens[4].token_type, TokenType::Eof);
    }

    #[test]
    fn test_whitespace_handling() {
        let lexer = Lexer::new("  5  \t3\n+  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].line, 1);
        assert_eq!(tokens[2].column, 5);
    }

    #[test]
    fn test_multiline_position_tracking() {
        let lexer = Lexer::new("5\n3\n+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 2);
        assert_eq!(tokens[1].column, 1);
        assert_eq!(tokens[2].line, 3);
        assert_eq!(tokens[2].column, 1);
    }

    #[test]
    fn test_unexpected_character() {
        let lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        if let Err(Rpn2TexError::LexerError {
            message,
            line,
            column,
        }) = result
        {
            assert!(message.contains("Unexpected character"));
            assert!(message.contains("^"));
            assert_eq!(line, 1);
            assert_eq!(column, 5);
        } else {
            panic!("Expected LexerError");
        }
    }

    #[test]
    fn test_complex_expression() {
        let lexer = Lexer::new("3.14 2 * 5 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].lexeme, "3.14");
        assert_eq!(tokens[1].lexeme, "2");
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[3].lexeme, "5");
        assert_eq!(tokens[4].token_type, TokenType::Plus);
    }

    #[test]
    fn test_minus_operator_vs_negative_number() {
        // "5 - 3" should tokenize as number, minus, number
        let lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].lexeme, "5");
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].lexeme, "3");

        // "5 -3" should tokenize as number, number (negative)
        let lexer = Lexer::new("5 -3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].lexeme, "5");
        assert_eq!(tokens[1].lexeme, "-3");
    }

    #[test]
    fn test_consecutive_numbers() {
        let lexer = Lexer::new("10 20 30");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].lexeme, "10");
        assert_eq!(tokens[1].lexeme, "20");
        assert_eq!(tokens[2].lexeme, "30");
    }

    #[test]
    fn test_io_contract_case_1() {
        // "5 3 +" should produce tokens
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].lexeme, "5");
        assert_eq!(tokens[1].lexeme, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_io_contract_case_5_error() {
        // "2 3 ^" should produce error at line 1, column 5
        let lexer = Lexer::new("2 3 ^");
        let result = lexer.tokenize();
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.line(), 1);
            assert_eq!(error.column(), 5);
            assert!(error.message().contains("^"));
        }
    }

    #[test]
    fn test_io_contract_case_18_float() {
        // "3.14 2 *" should handle floating point
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].lexeme, "3.14");
        assert_eq!(tokens[1].lexeme, "2");
        assert_eq!(tokens[2].token_type, TokenType::Star);
    }
}

//! Lexer for rpn2tex - converts text into tokens.
//!
//! This module tokenizes RPN (Reverse Polish Notation) expressions.

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// Tokenizes RPN input text.
///
/// The lexer scans input character by character, producing tokens for
/// numbers and EOF markers. Whitespace is used as a delimiter and is
/// otherwise ignored.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let mut lexer = Lexer::new("5 3.14");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 3); // 5, 3.14, EOF
/// assert_eq!(tokens[0].token_type, TokenType::Number);
/// ```
#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer with input text.
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

    /// Tokenize the entire input text.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an invalid character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let mut lexer = Lexer::new("5 3.14");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens.len(), 3);
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

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if matches!(ch, ' ' | '\t' | '\n' | '\r') {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().ok_or_else(|| {
            LexerError::new(
                "Unexpected end of input".to_string(),
                self.line,
                self.column,
            )
        })?;

        // Single-character operators
        if ch == '+' {
            self.advance();
            return Ok(Token::new(
                TokenType::Plus,
                "+".to_string(),
                start_line,
                start_column,
            ));
        }

        if ch == '-' {
            self.advance();
            // Check if this starts a negative number
            if let Some(next_ch) = self.peek() {
                if next_ch.is_ascii_digit() {
                    // It's a negative number like "-5"
                    return self.scan_number("-".to_string(), start_line, start_column);
                }
            }
            // Otherwise it's the subtraction operator
            return Ok(Token::new(
                TokenType::Minus,
                "-".to_string(),
                start_line,
                start_column,
            ));
        }

        if ch == '*' {
            self.advance();
            return Ok(Token::new(
                TokenType::Mult,
                "*".to_string(),
                start_line,
                start_column,
            ));
        }

        if ch == '/' {
            self.advance();
            return Ok(Token::new(
                TokenType::Div,
                "/".to_string(),
                start_line,
                start_column,
            ));
        }

        if ch.is_ascii_digit() {
            self.scan_number(String::new(), start_line, start_column)
        } else {
            Err(LexerError::new(
                format!("Unexpected character '{ch}'"),
                start_line,
                start_column,
            ))
        }
    }

    fn scan_number(
        &mut self,
        prefix: String,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexerError> {
        let mut value = prefix;

        // Integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part (optional)
        if let Some('.') = self.peek() {
            value.push('.');
            self.advance();
            while let Some(ch) = self.peek() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_decimal() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_multiple_numbers() {
        let mut lexer = Lexer::new("5 3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3.14");
    }
}

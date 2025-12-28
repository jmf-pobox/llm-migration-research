//! Lexer for rpn2tex - converts text into tokens.
//!
//! This module tokenizes RPN (Reverse Polish Notation) expressions.
//!
//! The lexer converts input text like "5 3 +" into a stream of tokens.
//!
//! # Key concepts
//!
//! * Character-by-character scanning
//! * Position tracking (line, column)
//! * Token generation
//! * Error handling with position information

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// Tokenizes RPN input text.
///
/// The lexer scans input character by character, producing tokens for:
/// * Numbers (integers and decimals)
/// * Operators (+, -, *, /)
/// * EOF marker
///
/// Whitespace is used as a delimiter and is otherwise ignored.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
///
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
/// ```
pub struct Lexer {
    text: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer with input text.
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
    /// # Errors
    ///
    /// Returns `LexerError` if an invalid character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("2 3 +");
    /// let tokens = lexer.tokenize().unwrap();
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

        // Add EOF token
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
            if ch.is_whitespace() {
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
                start_line,
                start_column,
            )
        })?;

        // Single-character operators
        match ch {
            '+' => {
                self.advance();
                return Ok(Token::new(
                    TokenType::Plus,
                    "+".to_string(),
                    start_line,
                    start_column,
                ));
            }
            '-' => {
                self.advance();
                // Check if this is a negative number (digit follows immediately)
                if let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        return self.scan_number("-".to_string(), start_line, start_column);
                    }
                }
                return Ok(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    start_line,
                    start_column,
                ));
            }
            '*' => {
                self.advance();
                return Ok(Token::new(
                    TokenType::Mult,
                    "*".to_string(),
                    start_line,
                    start_column,
                ));
            }
            '/' => {
                self.advance();
                return Ok(Token::new(
                    TokenType::Div,
                    "/".to_string(),
                    start_line,
                    start_column,
                ));
            }
            _ => {}
        }

        // Numbers
        if ch.is_ascii_digit() {
            return self.scan_number(String::new(), start_line, start_column);
        }

        // Unknown character
        Err(LexerError::new(
            format!("Unexpected character '{ch}'"),
            start_line,
            start_column,
        ))
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
    fn test_tokenize_simple_number() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
    }

    #[test]
    fn test_tokenize_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // NUMBER, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_tokenize_operators() {
        let lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // PLUS, MINUS, MULT, DIV, EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].token_type, TokenType::Mult);
        assert_eq!(tokens[3].token_type, TokenType::Div);
    }

    #[test]
    fn test_tokenize_rpn_expression() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
    }
}

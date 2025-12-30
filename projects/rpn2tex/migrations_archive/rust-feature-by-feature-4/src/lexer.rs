//! Lexer for rpn2tex - converts text into tokens.
//!
//! This module tokenizes RPN (Reverse Polish Notation) expressions.

use std::iter::Peekable;
use std::str::Chars;

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// Tokenizes RPN input text.
///
/// The lexer scans input character by character, producing tokens for:
/// - Numbers (integers and decimals)
/// - EOF marker
///
/// Whitespace is used as a delimiter and is otherwise ignored.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 2); // NUMBER and EOF
/// ```
pub struct Lexer<'a> {
    /// Iterator over characters in the input
    chars: Peekable<Chars<'a>>,
    /// Current line number (1-based)
    line: usize,
    /// Current column number (1-based)
    column: usize,
}

impl<'a> Lexer<'a> {
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
    pub fn new(text: &'a str) -> Self {
        Self {
            chars: text.chars().peekable(),
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input text.
    ///
    /// Returns a list of tokens, ending with an EOF token.
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
    /// let lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.peek().is_some() {
            self.skip_whitespace();
            if self.peek().is_none() {
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

    /// Look at current character without consuming it.
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Consume and return the current character.
    fn advance(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    /// Skip over whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scan and return the next token.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an invalid character is encountered.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let Some(ch) = self.peek() else {
            return Err(LexerError::new(
                "Unexpected end of input".to_string(),
                start_line,
                start_column,
            ));
        };

        // Numbers
        if ch.is_ascii_digit() {
            return self.scan_number(String::new(), start_line, start_column);
        }

        // Handle negative numbers and subtraction operator
        if ch == '-' {
            self.advance();
            // Check if this is a negative number (digit follows immediately)
            if let Some(next_ch) = self.peek() {
                if next_ch.is_ascii_digit() {
                    return self.scan_number("-".to_string(), start_line, start_column);
                }
            }
            // Otherwise, it's a subtraction operator
            return Ok(Token::new(
                TokenType::Minus,
                "-".to_string(),
                start_line,
                start_column,
            ));
        }

        // Addition operator
        if ch == '+' {
            self.advance();
            return Ok(Token::new(
                TokenType::Plus,
                "+".to_string(),
                start_line,
                start_column,
            ));
        }

        // Multiplication operator
        if ch == '*' {
            self.advance();
            return Ok(Token::new(
                TokenType::Mult,
                "*".to_string(),
                start_line,
                start_column,
            ));
        }

        // Division operator
        if ch == '/' {
            self.advance();
            return Ok(Token::new(
                TokenType::Div,
                "/".to_string(),
                start_line,
                start_column,
            ));
        }

        // Unknown character
        Err(LexerError::new(
            format!("Unexpected character '{ch}'"),
            start_line,
            start_column,
        ))
    }

    /// Scan a numeric literal.
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
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
    }

    #[test]
    fn test_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_division_operator() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, DIV, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Div);
        assert_eq!(tokens[2].value, "/");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }
}

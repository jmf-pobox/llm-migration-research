//! Lexical analysis for RPN expressions.
//!
//! This module provides the `Lexer` for tokenizing RPN input strings into
//! a sequence of tokens (numbers and operators). It handles integers, decimals,
//! negative numbers, and the four basic arithmetic operators.

use crate::tokens::{Token, TokenType};

/// Error type for lexical analysis failures.
///
/// Represents errors that occur during tokenization, such as encountering
/// unexpected characters in the input stream.
///
/// # Examples
///
/// ```
/// use rpn2tex::LexerError;
///
/// let error = LexerError {
///     message: "Unexpected character '!'".to_string(),
///     line: 1,
///     column: 7,
/// };
/// assert_eq!(error.to_string(), "Line 1, column 7: Unexpected character '!'");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Line {line}, column {column}: {message}")]
pub struct LexerError {
    /// Error description
    pub message: String,
    /// Line number where the error occurred (1-based)
    pub line: u32,
    /// Column number where the error occurred (1-based)
    pub column: u32,
}

/// Tokenizes RPN expression strings into a sequence of tokens.
///
/// The lexer maintains position tracking (line and column numbers) and
/// recognizes numeric literals (integers and decimals, including negative),
/// operators (+, -, *, /), and handles whitespace appropriately.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Lexer, TokenType};
///
/// let mut lexer = Lexer::new("3 4 +");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 4); // 3, 4, +, EOF
/// assert_eq!(tokens[0].token_type, TokenType::NUMBER);
/// assert_eq!(tokens[1].token_type, TokenType::NUMBER);
/// assert_eq!(tokens[2].token_type, TokenType::PLUS);
/// assert_eq!(tokens[3].token_type, TokenType::EOF);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lexer {
    text: String,
    chars: Vec<char>,
    pos: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    /// Creates a new lexer with the given input text.
    ///
    /// The lexer starts at line 1, column 1 (1-based indexing).
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::Lexer;
    ///
    /// let lexer = Lexer::new("3 4 +");
    /// ```
    #[must_use]
    pub fn new(text: impl Into<String>) -> Self {
        let text = text.into();
        let chars: Vec<char> = text.chars().collect();
        Self {
            text,
            chars,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the input text into a sequence of tokens.
    ///
    /// This is the main entry point for lexical analysis. It processes the
    /// entire input and returns all tokens, including a final EOF token.
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an unexpected character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Lexer, TokenType};
    ///
    /// let mut lexer = Lexer::new("3.14 -2 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens[0].value, "3.14");
    /// assert_eq!(tokens[1].value, "-2");
    /// assert_eq!(tokens[2].token_type, TokenType::PLUS);
    /// ```
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_whitespace();
            if self.at_end() {
                break;
            }
            let token = self.scan_token()?;
            tokens.push(token);
        }

        // Add EOF token
        tokens.push(Token::new(TokenType::EOF, "", self.line, self.column));

        Ok(tokens)
    }

    /// Checks if we've reached the end of the input.
    fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Peeks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        if self.at_end() {
            None
        } else {
            Some(self.chars[self.pos])
        }
    }

    /// Advances to the next character and returns the current one.
    ///
    /// Also updates line and column tracking.
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

    /// Skips over whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Scans and returns the next token from the input.
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an unexpected character is encountered.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().expect("scan_token called at end of input");

        let token = match ch {
            '+' => {
                self.advance();
                Token::new(TokenType::PLUS, "+", start_line, start_column)
            }
            '-' => {
                // Check if this is a negative number or minus operator
                self.advance();
                if let Some(next_ch) = self.peek() {
                    if next_ch.is_ascii_digit() {
                        // Negative number
                        self.scan_number(String::from("-"), start_line, start_column)
                    } else {
                        // Minus operator
                        Token::new(TokenType::MINUS, "-", start_line, start_column)
                    }
                } else {
                    // Minus operator at end of input
                    Token::new(TokenType::MINUS, "-", start_line, start_column)
                }
            }
            '*' => {
                self.advance();
                Token::new(TokenType::MULT, "*", start_line, start_column)
            }
            '/' => {
                self.advance();
                Token::new(TokenType::DIV, "/", start_line, start_column)
            }
            '0'..='9' => self.scan_number(String::new(), start_line, start_column),
            _ => {
                return Err(LexerError {
                    message: format!("Unexpected character '{ch}'"),
                    line: start_line,
                    column: start_column,
                });
            }
        };

        Ok(token)
    }

    /// Scans a number token (integer or decimal).
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix for the number (e.g., "-" for negative numbers)
    /// * `start_line` - The line where the number starts
    /// * `start_column` - The column where the number starts
    fn scan_number(&mut self, prefix: String, start_line: u32, start_column: u32) -> Token {
        let mut value = prefix;

        // Consume integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.advance());
            } else {
                break;
            }
        }

        // Check for decimal point
        if let Some('.') = self.peek() {
            value.push(self.advance()); // Consume '.'

            // Consume decimal part
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    value.push(self.advance());
                } else {
                    break;
                }
            }
        }

        Token::new(TokenType::NUMBER, value, start_line, start_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_whitespace_only() {
        let mut lexer = Lexer::new("   \n  \t  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_single_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "42");
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].token_type, TokenType::EOF);
    }

    #[test]
    fn test_decimal_number() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_negative_number() {
        let mut lexer = Lexer::new("-42");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "-42");
    }

    #[test]
    fn test_negative_decimal() {
        let mut lexer = Lexer::new("-3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "-3.14");
    }

    #[test]
    fn test_all_operators() {
        let mut lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].token_type, TokenType::PLUS);
        assert_eq!(tokens[1].token_type, TokenType::MINUS);
        assert_eq!(tokens[2].token_type, TokenType::MULT);
        assert_eq!(tokens[3].token_type, TokenType::DIV);
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn test_simple_expression() {
        let mut lexer = Lexer::new("3 4 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "3");
        assert_eq!(tokens[1].token_type, TokenType::NUMBER);
        assert_eq!(tokens[1].value, "4");
        assert_eq!(tokens[2].token_type, TokenType::PLUS);
        assert_eq!(tokens[3].token_type, TokenType::EOF);
    }

    #[test]
    fn test_complex_expression() {
        let mut lexer = Lexer::new("3 4 + 2 * 7 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].value, "3");
        assert_eq!(tokens[1].value, "4");
        assert_eq!(tokens[2].token_type, TokenType::PLUS);
        assert_eq!(tokens[3].value, "2");
        assert_eq!(tokens[4].token_type, TokenType::MULT);
        assert_eq!(tokens[5].value, "7");
        assert_eq!(tokens[6].token_type, TokenType::DIV);
        assert_eq!(tokens[7].token_type, TokenType::EOF);
    }

    #[test]
    fn test_negative_number_vs_minus_operator() {
        let mut lexer = Lexer::new("5 -3 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type, TokenType::NUMBER);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::NUMBER);
        assert_eq!(tokens[1].value, "-3");
        assert_eq!(tokens[2].token_type, TokenType::MINUS);
        assert_eq!(tokens[3].token_type, TokenType::EOF);
    }

    #[test]
    fn test_numbers_with_decimals() {
        let mut lexer = Lexer::new("3.14 2.718 1.0");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].value, "2.718");
        assert_eq!(tokens[2].value, "1.0");
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("3 4 +");
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
        let mut lexer = Lexer::new("3 4\n5 6");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
        assert_eq!(tokens[2].line, 2);
        assert_eq!(tokens[2].column, 1);
        assert_eq!(tokens[3].line, 2);
        assert_eq!(tokens[3].column, 3);
    }

    #[test]
    fn test_unexpected_character_error() {
        let mut lexer = Lexer::new("3 4 ! +");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.message, "Unexpected character '!'");
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 5);
    }

    #[test]
    fn test_error_display() {
        let error = LexerError {
            message: "Unexpected character '!'".to_string(),
            line: 1,
            column: 7,
        };
        assert_eq!(
            error.to_string(),
            "Line 1, column 7: Unexpected character '!'"
        );
    }

    #[test]
    fn test_number_at_end_of_input() {
        let mut lexer = Lexer::new("3 4 + 5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].token_type, TokenType::EOF);
    }

    #[test]
    fn test_decimal_at_end_of_input() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_minus_at_end_of_input() {
        let mut lexer = Lexer::new("3 4 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[2].token_type, TokenType::MINUS);
    }

    #[test]
    fn test_whitespace_variations() {
        let mut lexer = Lexer::new("3\t4\n5\r\n6");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].value, "3");
        assert_eq!(tokens[1].value, "4");
        assert_eq!(tokens[2].value, "5");
        assert_eq!(tokens[3].value, "6");
    }

    #[test]
    fn test_no_whitespace() {
        let mut lexer = Lexer::new("3+4*5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].value, "3");
        assert_eq!(tokens[1].token_type, TokenType::PLUS);
        assert_eq!(tokens[2].value, "4");
        assert_eq!(tokens[3].token_type, TokenType::MULT);
        assert_eq!(tokens[4].value, "5");
    }

    #[test]
    fn test_lexer_clone() {
        let lexer = Lexer::new("3 4 +");
        let cloned = lexer.clone();
        assert_eq!(lexer, cloned);
    }

    #[test]
    fn test_error_clone() {
        let error = LexerError {
            message: "Test error".to_string(),
            line: 1,
            column: 1,
        };
        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_multiple_errors() {
        let mut lexer = Lexer::new("3 ! 4");
        let result = lexer.tokenize();
        assert!(result.is_err());
        // Only first error is returned
        let error = result.unwrap_err();
        assert_eq!(error.column, 3);
    }

    #[test]
    fn test_zero() {
        let mut lexer = Lexer::new("0");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "0");
    }

    #[test]
    fn test_leading_zeros() {
        let mut lexer = Lexer::new("007");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "007");
    }

    #[test]
    fn test_decimal_without_trailing_digits() {
        let mut lexer = Lexer::new("3.");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "3.");
    }

    #[test]
    fn test_negative_decimal_without_trailing_digits() {
        let mut lexer = Lexer::new("-3.");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value, "-3.");
    }
}

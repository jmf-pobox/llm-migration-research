//! Lexer for rpn2tex - converts text into tokens.
//!
//! This module tokenizes RPN (Reverse Polish Notation) expressions.
//! The lexer converts input text like "5 3 +" into a stream of tokens.
//!
//! Key features:
//! - Character-by-character scanning
//! - Position tracking (line, column)
//! - Number parsing (integers and decimals, including negatives)
//! - Operator recognition (+, -, *, /)
//! - Error handling with position information

use std::error::Error;
use std::fmt;

use crate::tokens::{Token, TokenType};

/// Error type for lexer failures.
///
/// Contains position information to help identify where the error occurred.
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
/// assert_eq!(error.line, 1);
/// assert_eq!(error.column, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// Description of the error
    pub message: String,
    /// Line number where error occurred (1-based)
    pub line: u32,
    /// Column number where error occurred (1-based)
    pub column: u32,
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
/// The lexer scans input character by character, producing tokens for:
/// - Numbers (integers and decimals)
/// - Operators (+, -, *, /)
/// - EOF marker
///
/// Whitespace is used as a delimiter and is otherwise ignored.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let mut lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
/// assert_eq!(tokens[0].token_type(), TokenType::Number);
/// assert_eq!(tokens[0].value(), "5");
/// assert_eq!(tokens[2].token_type(), TokenType::Plus);
/// ```
#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: usize,
    line: u32,
    column: u32,
}

impl Lexer {
    /// Creates a new lexer with the given input text.
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
    /// Returns a vector of tokens, always ending with an EOF token.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an invalid character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let mut lexer = Lexer::new("2 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens[0].token_type(), TokenType::Number);
    /// assert_eq!(tokens[0].value(), "2");
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

    /// Looks at the current character without consuming it.
    fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    /// Consumes and returns the current character.
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.text.chars().nth(self.pos) {
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

    /// Scans and returns the next token.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an invalid character is encountered.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;

        let ch = self.peek().ok_or_else(|| LexerError {
            message: "Unexpected end of input".to_string(),
            line: self.line,
            column: self.column,
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
                        // It's a negative number
                        return Ok(self.scan_number("-".to_string(), start_line, start_column));
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
            return Ok(self.scan_number(String::new(), start_line, start_column));
        }

        // Unknown character
        Err(LexerError {
            message: format!("Unexpected character '{ch}'"),
            line: start_line,
            column: start_column,
        })
    }

    /// Scans a numeric literal.
    ///
    /// # Arguments
    ///
    /// * `prefix` - Any prefix already consumed (e.g., "-" for negatives)
    /// * `start_line` - Line where number started
    /// * `start_column` - Column where number started
    fn scan_number(&mut self, prefix: String, start_line: u32, start_column: u32) -> Token {
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

        Token::new(TokenType::Number, value, start_line, start_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operators() {
        let mut lexer = Lexer::new("+ - * /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 5); // 4 operators + EOF
        assert_eq!(tokens[0].token_type(), TokenType::Plus);
        assert_eq!(tokens[1].token_type(), TokenType::Minus);
        assert_eq!(tokens[2].token_type(), TokenType::Mult);
        assert_eq!(tokens[3].token_type(), TokenType::Div);
        assert_eq!(tokens[4].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("5 3.14 1.5 0.5 100");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 5 numbers + EOF
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].value(), "3.14");
        assert_eq!(tokens[2].value(), "1.5");
        assert_eq!(tokens[3].value(), "0.5");
        assert_eq!(tokens[4].value(), "100");
    }

    #[test]
    fn test_negative_numbers() {
        let mut lexer = Lexer::new("-3 -2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3); // 2 numbers + EOF
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-3");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "-2");
    }

    #[test]
    fn test_rpn_expression() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[3].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("  5  \t 3  \n  +  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("5 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(tokens[1].line(), 1);
        assert_eq!(tokens[1].column(), 3);
    }

    #[test]
    fn test_multiline() {
        let mut lexer = Lexer::new("5\n3\n+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[1].line(), 2);
        assert_eq!(tokens[2].line(), 3);
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("5 ^ 3");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("Unexpected character '^'"));
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 3);
    }

    #[test]
    fn test_minus_as_operator() {
        let mut lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Minus);
        assert_eq!(tokens[2].value(), "3");
    }

    #[test]
    fn test_decimal_numbers() {
        let mut lexer = Lexer::new("3.14 0.5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value(), "3.14");
        assert_eq!(tokens[1].value(), "0.5");
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_lexer_error_display() {
        let error = LexerError {
            message: "Test error".to_string(),
            line: 2,
            column: 5,
        };
        let display = format!("{error}");
        assert!(display.contains("Line 2"));
        assert!(display.contains("column 5"));
        assert!(display.contains("Test error"));
    }
}

#[cfg(test)]
mod io_contract_tests {
    use super::*;

    #[test]
    fn test_io_contract_basic_operators() {
        // Input: "+"
        let mut lexer = Lexer::new("+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2, "Should have Plus and EOF");
        assert_eq!(tokens[0].token_type(), TokenType::Plus);
        assert_eq!(tokens[0].value(), "+");
        assert_eq!(tokens[1].token_type(), TokenType::Eof);

        // Input: "-"
        let mut lexer = Lexer::new("-");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2, "Should have Minus and EOF");
        assert_eq!(tokens[0].token_type(), TokenType::Minus);
        assert_eq!(tokens[0].value(), "-");

        // Input: "*"
        let mut lexer = Lexer::new("*");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2, "Should have Mult and EOF");
        assert_eq!(tokens[0].token_type(), TokenType::Mult);
        assert_eq!(tokens[0].value(), "*");

        // Input: "/"
        let mut lexer = Lexer::new("/");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2, "Should have Div and EOF");
        assert_eq!(tokens[0].token_type(), TokenType::Div);
        assert_eq!(tokens[0].value(), "/");
    }

    #[test]
    fn test_io_contract_numbers() {
        // Integer
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");

        // Decimal
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "3.14");

        // Negative integer
        let mut lexer = Lexer::new("-2");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-2");

        // Negative decimal
        let mut lexer = Lexer::new("-5.5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-5.5");
    }

    #[test]
    fn test_io_contract_invalid_character() {
        // Input: "^"
        let mut lexer = Lexer::new("^");
        let result = lexer.tokenize();
        assert!(result.is_err(), "Should error on unexpected character");
        let err = result.unwrap_err();
        assert_eq!(err.message, "Unexpected character '^'");
        // Position should be 1-based
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 1);

        // Input: "5 ^ 3" - error at position 3
        let mut lexer = Lexer::new("5 ^ 3");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Unexpected character '^'");
        assert_eq!(err.line, 1);
        assert_eq!(err.column, 3);
    }

    #[test]
    fn test_io_contract_position_tracking_1based() {
        // First character should be at column 1
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1, "First column should be 1-based");

        // After space, column should be 3
        let mut lexer = Lexer::new("5 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].column(), 1);
        assert_eq!(
            tokens[1].column(),
            3,
            "Second token after space should be at column 3"
        );
    }

    #[test]
    fn test_io_contract_negative_number_detection() {
        // "-" followed by digit is negative number
        let mut lexer = Lexer::new("-5 -3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[0].token_type(),
            TokenType::Number,
            "Should parse as number"
        );
        assert_eq!(tokens[0].value(), "-5");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "-3");

        // "-" NOT followed by digit is operator
        let mut lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens[1].token_type(),
            TokenType::Minus,
            "Should parse as operator"
        );
        assert_eq!(tokens[1].value(), "-");
    }

    #[test]
    fn test_io_contract_whitespace_handling() {
        // Spaces should be skipped
        let mut lexer = Lexer::new("  5  3  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3); // 5, 3, EOF
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Eof);

        // Tabs should be skipped
        let mut lexer = Lexer::new("5\t3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);

        // Newlines should be skipped and tracked
        let mut lexer = Lexer::new("5\n3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[1].line(), 2);
    }

    #[test]
    fn test_io_contract_eof_always_appended() {
        // Even empty input has EOF
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Eof);

        // EOF always at end
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
        assert_eq!(tokens[tokens.len() - 1].token_type(), TokenType::Eof);
    }

    #[test]
    fn test_io_contract_complex_expression() {
        // "5 3 +" - typical RPN
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[3].token_type(), TokenType::Eof);

        // Negative numbers in expression
        let mut lexer = Lexer::new("-5 -3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value(), "-5");
        assert_eq!(tokens[1].value(), "-3");
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
    }
}

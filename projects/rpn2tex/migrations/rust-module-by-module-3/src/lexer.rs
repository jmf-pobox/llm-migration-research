//! Lexical analysis for RPN expressions.
//!
//! This module provides the `Lexer` type for tokenizing RPN expression strings
//! into token streams. It handles number literals, operators, and whitespace,
//! with proper error reporting including line and column information.

use crate::tokens::{Token, TokenType};
use std::error::Error;
use std::fmt;

/// Error type for lexical analysis failures.
///
/// Contains detailed position information (line and column) to help
/// identify where in the source text the error occurred.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::LexerError;
///
/// let error = LexerError::new("Unexpected character".to_string(), 1, 5);
/// assert_eq!(error.line, 1);
/// assert_eq!(error.column, 5);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError {
    /// The error message describing what went wrong
    pub message: String,
    /// Line number where the error occurred (1-based)
    pub line: usize,
    /// Column number where the error occurred (1-based)
    pub column: usize,
}

impl LexerError {
    /// Creates a new lexer error with the specified message and position.
    ///
    /// # Arguments
    ///
    /// * `message` - Description of the error
    /// * `line` - Line number (1-based)
    /// * `column` - Column number (1-based)
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::LexerError;
    ///
    /// let error = LexerError::new("Invalid token".to_string(), 2, 10);
    /// assert_eq!(format!("{}", error), "Line 2, column 10: Invalid token");
    /// ```
    #[must_use]
    pub fn new(message: String, line: usize, column: usize) -> Self {
        Self {
            message,
            line,
            column,
        }
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Line {}, column {}: {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for LexerError {}

/// Lexical analyzer for RPN expressions.
///
/// The lexer converts source text into a stream of tokens, handling:
/// - Number literals (integers and decimals)
/// - Operators (+, -, *, /)
/// - Whitespace (automatically skipped)
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::tokens::TokenType;
///
/// let lexer = Lexer::new("5 3 +".to_string());
/// let tokens = lexer.tokenize().unwrap();
///
/// assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
/// assert_eq!(tokens[0].type_, TokenType::Number);
/// assert_eq!(tokens[0].value, "5");
/// assert_eq!(tokens[1].type_, TokenType::Number);
/// assert_eq!(tokens[1].value, "3");
/// assert_eq!(tokens[2].type_, TokenType::Plus);
/// assert_eq!(tokens[3].type_, TokenType::Eof);
/// ```
#[derive(Debug)]
pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given source text.
    ///
    /// # Arguments
    ///
    /// * `text` - The source text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +".to_string());
    /// ```
    #[must_use]
    pub fn new(text: String) -> Self {
        let chars: Vec<char> = text.chars().collect();
        Self {
            chars,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the source text into a vector of tokens.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `Ok(Vec<Token>)` - The complete token stream (including EOF)
    /// - `Err(LexerError)` - An error with position information
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let lexer = Lexer::new("3.14 2 *".to_string());
    /// let tokens = lexer.tokenize().unwrap();
    ///
    /// assert_eq!(tokens[0].value, "3.14");
    /// assert_eq!(tokens[1].value, "2");
    /// assert_eq!(tokens[2].type_, TokenType::Mult);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an unexpected character is encountered.
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

    /// Checks if the lexer has reached the end of the input.
    fn at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    /// Peeks at the current character without advancing.
    ///
    /// Returns '\0' if at the end of input.
    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.chars[self.pos]
        }
    }

    /// Advances to the next character and returns the current one.
    ///
    /// Updates line and column tracking appropriately.
    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.pos += 1;

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
        while matches!(self.peek(), ' ' | '\t' | '\n' | '\r') {
            self.advance();
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
        let ch = self.peek();

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
                // Check if this is a negative number
                if self.peek().is_ascii_digit() {
                    Ok(self.scan_number("-".to_string(), start_line, start_column))
                } else {
                    Ok(Token::new(
                        TokenType::Minus,
                        "-".to_string(),
                        start_line,
                        start_column,
                    ))
                }
            }
            '*' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Mult,
                    "*".to_string(),
                    start_line,
                    start_column,
                ))
            }
            '/' => {
                self.advance();
                Ok(Token::new(
                    TokenType::Div,
                    "/".to_string(),
                    start_line,
                    start_column,
                ))
            }
            _ if ch.is_ascii_digit() => {
                Ok(self.scan_number(String::new(), start_line, start_column))
            }
            _ => Err(LexerError::new(
                format!("Unexpected character '{}'", ch),
                start_line,
                start_column,
            )),
        }
    }

    /// Scans a number literal (integer or decimal).
    ///
    /// # Arguments
    ///
    /// * `prefix` - Optional prefix (e.g., "-" for negative numbers)
    /// * `start_line` - Line where the number starts
    /// * `start_column` - Column where the number starts
    fn scan_number(&mut self, prefix: String, start_line: usize, start_column: usize) -> Token {
        let mut value = prefix;

        // Scan integer part
        while self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Scan decimal part if present
        if self.peek() == '.' {
            value.push(self.advance());
            while self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        Token::new(TokenType::Number, value, start_line, start_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let lexer = Lexer::new("5 3 +".to_string());
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
    fn test_decimal_number() {
        let lexer = Lexer::new("3.14 2 *".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].type_, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].type_, TokenType::Mult);
    }

    #[test]
    fn test_unexpected_character() {
        let lexer = Lexer::new("2 3 ^".to_string());
        let result = lexer.tokenize();

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.line, 1);
        assert_eq!(error.column, 5);
        assert!(error.message.contains("Unexpected character"));
    }

    #[test]
    fn test_negative_number() {
        let lexer = Lexer::new("-5".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
        assert_eq!(tokens[1].type_, TokenType::Eof);
    }

    #[test]
    fn test_all_operators() {
        let lexer = Lexer::new("+ - * /".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].type_, TokenType::Plus);
        assert_eq!(tokens[1].type_, TokenType::Minus);
        assert_eq!(tokens[2].type_, TokenType::Mult);
        assert_eq!(tokens[3].type_, TokenType::Div);
        assert_eq!(tokens[4].type_, TokenType::Eof);
    }

    #[test]
    fn test_error_display() {
        let error = LexerError::new("Test error".to_string(), 2, 10);
        assert_eq!(format!("{}", error), "Line 2, column 10: Test error");
    }

    #[test]
    fn test_whitespace_handling() {
        let lexer = Lexer::new("  5  \t  3  \n  +  ".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].type_, TokenType::Plus);
    }

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].type_, TokenType::Eof);
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5 3".to_string());
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
        assert_eq!(tokens[1].line, 1);
        assert_eq!(tokens[1].column, 3);
    }
}

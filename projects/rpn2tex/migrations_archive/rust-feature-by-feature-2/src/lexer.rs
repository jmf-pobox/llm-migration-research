//! Lexical analyzer for RPN expressions.

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// A lexical analyzer that converts source text into tokens.
#[derive(Debug)]
pub struct Lexer {
    /// The source text being analyzed.
    source: Vec<char>,
    /// Current position in the source.
    current: usize,
    /// Current line number (1-indexed).
    line: usize,
    /// Current column number (1-indexed).
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given source text.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::lexer::Lexer;
    /// let lexer = Lexer::new("42");
    /// ```
    #[must_use]
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Scans all tokens from the source.
    ///
    /// # Errors
    ///
    /// Returns `LexerError` if an unexpected character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::lexer::Lexer;
    /// let mut lexer = Lexer::new("42");
    /// let tokens = lexer.scan_tokens().unwrap();
    /// assert_eq!(tokens.len(), 2); // Number + EOF
    /// ```
    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while !self.at_end() {
            self.skip_whitespace();
            if self.at_end() {
                break;
            }

            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            self.line,
            self.column,
        ));

        Ok(tokens)
    }

    /// Scans a single token.
    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;
        let c = self.advance();

        if c.is_ascii_digit() {
            Ok(self.scan_number(c.to_string(), start_line, start_column))
        } else if c == '+' {
            Ok(Token::new(
                TokenType::Plus,
                "+".to_string(),
                start_line,
                start_column,
            ))
        } else if c == '-' {
            // Check if this is a negative number (digit follows immediately)
            if !self.at_end() && self.peek().is_ascii_digit() {
                // It's a negative number
                let mut prefix = String::from('-');
                prefix.push(self.advance());
                Ok(self.scan_number(prefix, start_line, start_column))
            } else {
                // It's a subtraction operator
                Ok(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    start_line,
                    start_column,
                ))
            }
        } else if c == '*' {
            Ok(Token::new(
                TokenType::Mult,
                "*".to_string(),
                start_line,
                start_column,
            ))
        } else if c == '/' {
            Ok(Token::new(
                TokenType::Div,
                "/".to_string(),
                start_line,
                start_column,
            ))
        } else {
            Err(LexerError::UnexpectedCharacter {
                character: c,
                line: start_line,
                column: start_column,
            })
        }
    }

    /// Scans a numeric literal (integer or decimal).
    fn scan_number(&mut self, prefix: String, start_line: usize, start_column: usize) -> Token {
        let mut value = prefix;

        // Integer part
        while !self.at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Decimal part (optional)
        if !self.at_end() && self.peek() == '.' {
            value.push(self.advance());
            while !self.at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        Token::new(TokenType::Number, value, start_line, start_column)
    }

    /// Skips whitespace characters.
    fn skip_whitespace(&mut self) {
        while !self.at_end() {
            let c = self.peek();
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Checks if we've reached the end of the source.
    #[must_use]
    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Returns the current character without consuming it.
    #[must_use]
    fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    /// Consumes and returns the current character.
    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_integer() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "42");
    }

    #[test]
    fn test_scan_decimal() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "3.14");
    }

    #[test]
    fn test_scan_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].lexeme, "-5");
    }

    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("  42  ");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].lexeme, "42");
    }
}

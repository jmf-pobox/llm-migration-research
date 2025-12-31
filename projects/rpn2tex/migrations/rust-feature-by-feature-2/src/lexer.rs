//! Lexical analyzer for RPN expressions.
//!
//! This module provides a lexer that scans input text and produces tokens.

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// A lexical analyzer that converts input text into tokens.
#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// ```
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Scans all tokens from the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let mut lexer = Lexer::new("5");
    /// let tokens = lexer.scan_tokens().unwrap();
    /// assert_eq!(tokens.len(), 1);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an unexpected character is encountered.
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

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LexerError> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.peek();

        if ch.is_ascii_digit() {
            Ok(self.scan_number(String::new(), start_line, start_column))
        } else if ch == '-' {
            self.advance();
            // Check if this is a negative number (digit follows immediately)
            if !self.at_end() && self.peek().is_ascii_digit() {
                Ok(self.scan_number("-".to_string(), start_line, start_column))
            } else {
                // It's a subtraction operator
                Ok(Token::new(
                    TokenType::Minus,
                    "-".to_string(),
                    start_line,
                    start_column,
                ))
            }
        } else if ch == '+' {
            self.advance();
            Ok(Token::new(
                TokenType::Plus,
                "+".to_string(),
                start_line,
                start_column,
            ))
        } else if ch == '*' {
            self.advance();
            Ok(Token::new(
                TokenType::Star,
                "*".to_string(),
                start_line,
                start_column,
            ))
        } else if ch == '/' {
            self.advance();
            Ok(Token::new(
                TokenType::Slash,
                "/".to_string(),
                start_line,
                start_column,
            ))
        } else {
            Err(LexerError::UnexpectedCharacter {
                ch,
                line: start_line,
                column: start_column,
            })
        }
    }

    fn scan_number(&mut self, prefix: String, start_line: usize, start_column: usize) -> Token {
        let mut value = prefix;

        // Scan integer part
        while !self.at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Scan decimal part (optional)
        if !self.at_end() && self.peek() == '.' {
            value.push(self.advance()); // consume '.'
            while !self.at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        Token::new(TokenType::Number, value, start_line, start_column)
    }

    fn skip_whitespace(&mut self) {
        while !self.at_end() && self.peek().is_whitespace() {
            self.advance();
        }
    }

    fn at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> char {
        self.input[self.position]
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        ch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_integer() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
    }

    #[test]
    fn test_scan_float() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_scan_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
    }

    #[test]
    fn test_scan_multiple_numbers() {
        let mut lexer = Lexer::new("5 3");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
    }

    #[test]
    fn test_scan_with_whitespace() {
        let mut lexer = Lexer::new("  5   3.14  ");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3.14");
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
    }

    #[test]
    fn test_unexpected_character() {
        let mut lexer = Lexer::new("!");
        let result = lexer.scan_tokens();
        assert!(result.is_err());
        match result {
            Err(LexerError::UnexpectedCharacter { ch, .. }) => assert_eq!(ch, '!'),
            _ => panic!("Expected UnexpectedCharacter error"),
        }
    }

    #[test]
    fn test_scan_plus_operator() {
        let mut lexer = Lexer::new("+");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[0].value, "+");
    }

    #[test]
    fn test_scan_addition_expression() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[2].value, "+");
    }

    #[test]
    fn test_scan_minus_operator() {
        let mut lexer = Lexer::new("-");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Minus);
        assert_eq!(tokens[0].value, "-");
    }

    #[test]
    fn test_scan_subtraction_expression() {
        let mut lexer = Lexer::new("5 3 -");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[2].value, "-");
    }

    #[test]
    fn test_distinguish_negative_from_minus() {
        // "-5" should be a negative number
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");

        // "5 - 3" should have minus operator
        let mut lexer2 = Lexer::new("5 - 3");
        let tokens2 = lexer2.scan_tokens().unwrap();
        assert_eq!(tokens2.len(), 3);
        assert_eq!(tokens2[0].token_type, TokenType::Number);
        assert_eq!(tokens2[1].token_type, TokenType::Minus);
        assert_eq!(tokens2[2].token_type, TokenType::Number);
    }

    #[test]
    fn test_scan_star_operator() {
        let mut lexer = Lexer::new("*");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Star);
        assert_eq!(tokens[0].value, "*");
    }

    #[test]
    fn test_scan_multiplication_expression() {
        let mut lexer = Lexer::new("4 7 *");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "4");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "7");
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[2].value, "*");
    }

    #[test]
    fn test_scan_float_multiplication() {
        let mut lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Star);
    }

    #[test]
    fn test_scan_slash_operator() {
        let mut lexer = Lexer::new("/");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[0].value, "/");
    }

    #[test]
    fn test_scan_division_expression() {
        let mut lexer = Lexer::new("10 2 /");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Slash);
        assert_eq!(tokens[2].value, "/");
    }

    #[test]
    fn test_scan_chained_division() {
        let mut lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].value, "100");
        assert_eq!(tokens[1].value, "10");
        assert_eq!(tokens[2].token_type, TokenType::Slash);
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].token_type, TokenType::Slash);
        assert_eq!(tokens[5].value, "2");
        assert_eq!(tokens[6].token_type, TokenType::Slash);
    }
}

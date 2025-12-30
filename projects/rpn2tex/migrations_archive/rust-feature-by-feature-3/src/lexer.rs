//! Lexical analyzer for RPN expressions.

use crate::error::{Error, Result};
use crate::tokens::{Token, TokenType};

/// A lexer that tokenizes RPN input.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
///
/// let lexer = Lexer::new("42");
/// let tokens = lexer.scan_tokens().unwrap();
/// assert_eq!(tokens.len(), 1);
/// ```
#[derive(Debug)]
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given source string.
    #[must_use]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Scans the source and returns all tokens.
    ///
    /// # Errors
    ///
    /// Returns an error if an invalid character is encountered.
    pub fn scan_tokens(mut self) -> Result<Vec<Token>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance();
        let start_line = self.line;
        let start_column = self.column - 1;

        match c {
            // Whitespace
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                self.column = 1;
                Ok(())
            }
            // Numbers
            '0'..='9' => self.scan_number(c),
            // Operators
            '+' => {
                self.tokens
                    .push(Token::new(TokenType::Plus, "+", start_line, start_column));
                Ok(())
            }
            '-' => {
                // Check if this is a negative number (digit follows immediately)
                if !self.is_at_end() && self.peek().is_ascii_digit() {
                    // It's a negative number
                    self.scan_negative_number(start_line, start_column)
                } else {
                    // It's a subtraction operator
                    self.tokens
                        .push(Token::new(TokenType::Minus, "-", start_line, start_column));
                    Ok(())
                }
            }
            '*' => {
                self.tokens
                    .push(Token::new(TokenType::Mult, "*", start_line, start_column));
                Ok(())
            }
            '/' => {
                self.tokens
                    .push(Token::new(TokenType::Div, "/", start_line, start_column));
                Ok(())
            }
            _ => Err(Error::LexerError {
                message: format!("Unexpected character: '{c}'"),
                line: self.line,
                column: self.column - 1,
            }),
        }
    }

    fn scan_number(&mut self, first_digit: char) -> Result<()> {
        let start_line = self.line;
        let start_column = self.column - 1;
        let mut value = String::from(first_digit);

        // Scan integer part
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Scan decimal part
        if !self.is_at_end() && self.peek() == '.' {
            value.push(self.advance());
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        self.tokens.push(Token::new(
            TokenType::Number,
            value,
            start_line,
            start_column,
        ));
        Ok(())
    }

    fn scan_negative_number(&mut self, start_line: usize, start_column: usize) -> Result<()> {
        let mut value = String::from("-");

        // Scan integer part
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            value.push(self.advance());
        }

        // Scan decimal part
        if !self.is_at_end() && self.peek() == '.' {
            value.push(self.advance());
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                value.push(self.advance());
            }
        }

        self.tokens.push(Token::new(
            TokenType::Number,
            value,
            start_line,
            start_column,
        ));
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += c.len_utf8();
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_integer() {
        let lexer = Lexer::new("42");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "42");
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
    }

    #[test]
    fn test_scan_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "3.14");
    }

    #[test]
    fn test_scan_with_whitespace() {
        let lexer = Lexer::new("  42  ");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].value(), "42");
    }

    #[test]
    fn test_scan_plus() {
        let lexer = Lexer::new("+");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Plus);
        assert_eq!(tokens[0].value(), "+");
    }

    #[test]
    fn test_scan_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Plus);
        assert_eq!(tokens[2].value(), "+");
    }

    #[test]
    fn test_scan_minus() {
        let lexer = Lexer::new("-");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Minus);
        assert_eq!(tokens[0].value(), "-");
    }

    #[test]
    fn test_scan_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].token_type(), TokenType::Minus);
        assert_eq!(tokens[2].value(), "-");
    }

    #[test]
    fn test_scan_negative_number() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-5");
    }

    #[test]
    fn test_scan_negative_decimal() {
        let lexer = Lexer::new("-3.14");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-3.14");
    }

    #[test]
    fn test_disambiguation_minus_with_space() {
        // "5 - 3" should be: Number(5), Minus, Number(3)
        let lexer = Lexer::new("5 - 3");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Minus);
        assert_eq!(tokens[1].value(), "-");
        assert_eq!(tokens[2].token_type(), TokenType::Number);
        assert_eq!(tokens[2].value(), "3");
    }

    #[test]
    fn test_disambiguation_minus_no_space() {
        // "5-3" should be: Number(5), Number(-3)
        let lexer = Lexer::new("5-3");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "-3");
    }

    #[test]
    fn test_scan_div() {
        let lexer = Lexer::new("/");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type(), TokenType::Div);
        assert_eq!(tokens[0].value(), "/");
    }

    #[test]
    fn test_scan_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.scan_tokens().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type(), TokenType::Number);
        assert_eq!(tokens[0].value(), "10");
        assert_eq!(tokens[1].token_type(), TokenType::Number);
        assert_eq!(tokens[1].value(), "2");
        assert_eq!(tokens[2].token_type(), TokenType::Div);
        assert_eq!(tokens[2].value(), "/");
    }
}

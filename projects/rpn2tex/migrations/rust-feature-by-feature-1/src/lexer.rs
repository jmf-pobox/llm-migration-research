//! Lexical analyzer for RPN expressions.
//!
//! This module provides tokenization of input text into a stream of tokens.

use crate::{LexerError, Token, TokenType};

/// A lexical analyzer that converts input text into tokens.
///
/// # Examples
///
/// ```
/// use rpn2tex::Lexer;
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// assert_eq!(tokens.len(), 2); // NUMBER, EOF
/// ```
#[derive(Debug)]
#[must_use]
pub struct Lexer {
    input: String,
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
    /// use rpn2tex::Lexer;
    ///
    /// let lexer = Lexer::new("3.14");
    /// ```
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenizes the input into a vector of tokens.
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an invalid character is encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Lexer, TokenType};
    ///
    /// let lexer = Lexer::new("42");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens[0].type_(), TokenType::Number);
    /// assert_eq!(tokens[0].value(), "42");
    /// ```
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.at_end() {
                tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));
                break;
            }

            let token = self.scan_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) -> char {
        let ch = self.peek().expect("Cannot advance past end");
        self.position += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
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
        let ch = self
            .peek()
            .ok_or_else(|| LexerError::new("Unexpected end of input", start_line, start_column))?;

        // Numbers
        if ch.is_ascii_digit() {
            return self.scan_number(String::new(), start_line, start_column);
        }

        // Negative numbers vs. subtraction operator (lookahead required)
        if ch == '-' {
            self.advance();
            if !self.at_end() && self.peek().is_some_and(|c| c.is_ascii_digit()) {
                return self.scan_number("-".to_string(), start_line, start_column);
            }
            // Otherwise, it's a subtraction operator
            return Ok(Token::new(TokenType::Minus, "-", start_line, start_column));
        }

        // Addition operator
        if ch == '+' {
            self.advance();
            return Ok(Token::new(TokenType::Plus, "+", start_line, start_column));
        }

        // Multiplication operator
        if ch == '*' {
            self.advance();
            return Ok(Token::new(TokenType::Star, "*", start_line, start_column));
        }

        // Division operator
        if ch == '/' {
            self.advance();
            return Ok(Token::new(TokenType::Slash, "/", start_line, start_column));
        }

        // Any other character is unexpected
        Err(LexerError::new(
            format!("Unexpected character '{}'", ch),
            start_line,
            start_column,
        ))
    }

    fn scan_number(
        &mut self,
        mut prefix: String,
        start_line: usize,
        start_column: usize,
    ) -> Result<Token, LexerError> {
        // Integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                prefix.push(self.advance());
            } else {
                break;
            }
        }

        // Decimal part (optional)
        if let Some('.') = self.peek() {
            prefix.push(self.advance());
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    prefix.push(self.advance());
                } else {
                    break;
                }
            }
        }

        Ok(Token::new(
            TokenType::Number,
            prefix,
            start_line,
            start_column,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_digit() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].type_(), TokenType::Eof);
    }

    #[test]
    fn test_multi_digit() {
        let lexer = Lexer::new("12345");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value(), "12345");
    }

    #[test]
    fn test_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "3.14");
    }

    #[test]
    fn test_negative_number() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value(), "-5");
    }

    #[test]
    fn test_negative_decimal() {
        let lexer = Lexer::new("-3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value(), "-3.14");
    }

    #[test]
    fn test_leading_zero() {
        let lexer = Lexer::new("01");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value(), "01");
    }

    #[test]
    fn test_trailing_decimal() {
        let lexer = Lexer::new("5.");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value(), "5.");
    }

    #[test]
    fn test_whitespace_handling() {
        let lexer = Lexer::new("  42  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].value(), "42");
    }

    #[test]
    fn test_position_tracking() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line(), 1);
        assert_eq!(tokens[0].column(), 1);
    }

    #[test]
    fn test_invalid_character() {
        let lexer = Lexer::new("@");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Unexpected character"));
    }

    #[test]
    fn test_empty_input() {
        let lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].type_(), TokenType::Eof);
    }

    #[test]
    fn test_very_long_decimal() {
        let lexer = Lexer::new("3.14159265358979");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].value(), "3.14159265358979");
    }

    #[test]
    fn test_plus_token() {
        let lexer = Lexer::new("+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Plus);
        assert_eq!(tokens[0].value(), "+");
        assert_eq!(tokens[1].type_(), TokenType::Eof);
    }

    #[test]
    fn test_addition_expression() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, PLUS, EOF
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].type_(), TokenType::Number);
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].type_(), TokenType::Plus);
        assert_eq!(tokens[2].value(), "+");
        assert_eq!(tokens[3].type_(), TokenType::Eof);
    }

    #[test]
    fn test_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 8); // 4 numbers, 3 plus, 1 EOF
        assert_eq!(tokens[0].value(), "1");
        assert_eq!(tokens[1].value(), "2");
        assert_eq!(tokens[2].type_(), TokenType::Plus);
        assert_eq!(tokens[3].value(), "3");
        assert_eq!(tokens[4].type_(), TokenType::Plus);
        assert_eq!(tokens[5].value(), "4");
        assert_eq!(tokens[6].type_(), TokenType::Plus);
    }

    #[test]
    fn test_minus_token() {
        let lexer = Lexer::new("-");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Minus);
        assert_eq!(tokens[0].value(), "-");
        assert_eq!(tokens[1].type_(), TokenType::Eof);
    }

    #[test]
    fn test_subtraction_expression() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, MINUS, EOF
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].type_(), TokenType::Number);
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].type_(), TokenType::Minus);
        assert_eq!(tokens[2].value(), "-");
        assert_eq!(tokens[3].type_(), TokenType::Eof);
    }

    #[test]
    fn test_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 3 numbers, 2 minus, 1 EOF
        assert_eq!(tokens[0].value(), "5");
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].type_(), TokenType::Minus);
        assert_eq!(tokens[3].value(), "2");
        assert_eq!(tokens[4].type_(), TokenType::Minus);
    }

    #[test]
    fn test_negative_vs_minus_operator() {
        // "-5" should be negative NUMBER
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "-5");

        // "5 -" should be NUMBER then MINUS
        let lexer = Lexer::new("5 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[1].type_(), TokenType::Minus);
    }

    #[test]
    fn test_minus_at_end() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        let minus_token = &tokens[2];
        assert_eq!(minus_token.type_(), TokenType::Minus);
        assert_eq!(minus_token.value(), "-");
    }

    #[test]
    fn test_star_token() {
        let lexer = Lexer::new("*");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Star);
        assert_eq!(tokens[0].value(), "*");
        assert_eq!(tokens[1].type_(), TokenType::Eof);
    }

    #[test]
    fn test_multiplication_expression() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, STAR, EOF
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "4");
        assert_eq!(tokens[1].type_(), TokenType::Number);
        assert_eq!(tokens[1].value(), "7");
        assert_eq!(tokens[2].type_(), TokenType::Star);
        assert_eq!(tokens[2].value(), "*");
        assert_eq!(tokens[3].type_(), TokenType::Eof);
    }

    #[test]
    fn test_mixed_operators() {
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 3 numbers, star, plus, EOF
        assert_eq!(tokens[0].value(), "2");
        assert_eq!(tokens[1].value(), "3");
        assert_eq!(tokens[2].value(), "4");
        assert_eq!(tokens[3].type_(), TokenType::Star);
        assert_eq!(tokens[4].type_(), TokenType::Plus);
    }

    #[test]
    fn test_slash_token() {
        let lexer = Lexer::new("/");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].type_(), TokenType::Slash);
        assert_eq!(tokens[0].value(), "/");
        assert_eq!(tokens[1].type_(), TokenType::Eof);
    }

    #[test]
    fn test_division_expression() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // NUMBER, NUMBER, SLASH, EOF
        assert_eq!(tokens[0].type_(), TokenType::Number);
        assert_eq!(tokens[0].value(), "10");
        assert_eq!(tokens[1].type_(), TokenType::Number);
        assert_eq!(tokens[1].value(), "2");
        assert_eq!(tokens[2].type_(), TokenType::Slash);
        assert_eq!(tokens[2].value(), "/");
        assert_eq!(tokens[3].type_(), TokenType::Eof);
    }

    #[test]
    fn test_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 8); // 4 numbers, 3 slash, 1 EOF
        assert_eq!(tokens[0].value(), "100");
        assert_eq!(tokens[1].value(), "10");
        assert_eq!(tokens[2].type_(), TokenType::Slash);
        assert_eq!(tokens[3].value(), "5");
        assert_eq!(tokens[4].type_(), TokenType::Slash);
        assert_eq!(tokens[5].value(), "2");
        assert_eq!(tokens[6].type_(), TokenType::Slash);
    }

    #[test]
    fn test_division_with_multiplication() {
        let lexer = Lexer::new("10 2 / 5 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 3 numbers, slash, star, EOF
        assert_eq!(tokens[0].value(), "10");
        assert_eq!(tokens[1].value(), "2");
        assert_eq!(tokens[2].type_(), TokenType::Slash);
        assert_eq!(tokens[3].value(), "5");
        assert_eq!(tokens[4].type_(), TokenType::Star);
    }
}

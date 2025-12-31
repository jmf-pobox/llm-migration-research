//! Lexical analysis for RPN expressions.

use crate::error::LexerError;
use crate::tokens::{Token, TokenType};

/// A lexer that converts source text into tokens.
#[derive(Debug)]
pub struct Lexer {
    text: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer for the given input text.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    ///
    /// let lexer = Lexer::new("5 3.14");
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

    /// Tokenizes the input text, returning a vector of tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::tokens::TokenType;
    ///
    /// let mut lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// assert_eq!(tokens.len(), 2); // NUMBER + EOF
    /// assert_eq!(tokens[0].token_type, TokenType::Number);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `LexerError` if an invalid character is encountered.
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
        tokens.push(Token::new(TokenType::Eof, "", self.line, self.column));

        Ok(tokens)
    }

    fn at_end(&self) -> bool {
        self.pos >= self.text.len()
    }

    fn peek(&self) -> Option<char> {
        self.text.chars().nth(self.pos)
    }

    fn advance(&mut self) -> char {
        let ch = self.text.chars().nth(self.pos).expect("Unexpected EOF");
        self.pos += ch.len_utf8();
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
        let ch = self.peek().unwrap();

        if ch.is_ascii_digit() {
            self.scan_number(String::new(), start_line, start_column)
        } else if ch == '+' {
            self.advance();
            Ok(Token::new(TokenType::Plus, "+", start_line, start_column))
        } else if ch == '-' {
            // Could be negative number or subtraction operator
            // In RPN, standalone "-" is always subtraction
            self.advance();
            // Check if this is a negative number (digit follows immediately)
            if let Some(next_ch) = self.peek() {
                if next_ch.is_ascii_digit() {
                    // It's a negative number
                    return self.scan_number("-".to_string(), start_line, start_column);
                }
            }
            Ok(Token::new(TokenType::Minus, "-", start_line, start_column))
        } else if ch == '*' {
            self.advance();
            Ok(Token::new(TokenType::Star, "*", start_line, start_column))
        } else if ch == '/' {
            self.advance();
            Ok(Token::new(TokenType::Slash, "/", start_line, start_column))
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

        // Scan integer part
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.advance());
            } else {
                break;
            }
        }

        // Scan decimal part (optional)
        if let Some('.') = self.peek() {
            value.push(self.advance()); // consume '.'

            // Must have at least one digit after decimal point
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    value.push(self.advance());
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
    fn test_single_number() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Eof);
    }

    #[test]
    fn test_decimal_number() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "3.14");
    }

    #[test]
    fn test_multiple_numbers() {
        let mut lexer = Lexer::new("5 3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 3); // Two numbers + EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3.14");
    }

    #[test]
    fn test_invalid_character() {
        let mut lexer = Lexer::new("5 @ 3");
        let result = lexer.tokenize();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("@"));
    }

    #[test]
    fn test_position_tracking() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].line, 1);
        assert_eq!(tokens[0].column, 1);
    }

    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 1); // Just EOF
        assert_eq!(tokens[0].token_type, TokenType::Eof);
    }

    #[test]
    fn test_whitespace_handling() {
        let mut lexer = Lexer::new("  5  ");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // Number + EOF
        assert_eq!(tokens[0].value, "5");
    }

    #[test]
    fn test_plus_operator() {
        let mut lexer = Lexer::new("+");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // Plus + EOF
        assert_eq!(tokens[0].token_type, TokenType::Plus);
        assert_eq!(tokens[0].value, "+");
    }

    #[test]
    fn test_simple_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[2].value, "+");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_chained_addition() {
        let mut lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 8); // 1, 2, +, 3, +, 4, +, EOF
        assert_eq!(tokens[0].value, "1");
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
        assert_eq!(tokens[3].value, "3");
        assert_eq!(tokens[4].token_type, TokenType::Plus);
        assert_eq!(tokens[5].value, "4");
        assert_eq!(tokens[6].token_type, TokenType::Plus);
    }

    #[test]
    fn test_minus_operator() {
        let mut lexer = Lexer::new("-");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // Minus + EOF
        assert_eq!(tokens[0].token_type, TokenType::Minus);
        assert_eq!(tokens[0].value, "-");
    }

    #[test]
    fn test_simple_subtraction() {
        let mut lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 5, 3, -, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[2].value, "-");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_chained_subtraction() {
        let mut lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 5, 3, -, 2, -, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].token_type, TokenType::Minus);
        assert_eq!(tokens[3].value, "2");
        assert_eq!(tokens[4].token_type, TokenType::Minus);
    }

    #[test]
    fn test_negative_number() {
        let mut lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // -5, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-5");
    }

    #[test]
    fn test_negative_decimal() {
        let mut lexer = Lexer::new("-3.14");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // -3.14, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "-3.14");
    }

    #[test]
    fn test_minus_with_space_before_digit() {
        // "5 - 3" should be: 5, -, 3 (not 5, -3)
        // because there's a space after "-"
        let mut lexer = Lexer::new("5 - 3");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 5, -, 3, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Minus);
        assert_eq!(tokens[2].value, "3");
    }

    #[test]
    fn test_negative_number_in_expression() {
        let mut lexer = Lexer::new("5 -3 +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 5, -3, +, EOF
        assert_eq!(tokens[0].value, "5");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "-3");
        assert_eq!(tokens[2].token_type, TokenType::Plus);
    }

    #[test]
    fn test_star_operator() {
        let mut lexer = Lexer::new("*");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // Star + EOF
        assert_eq!(tokens[0].token_type, TokenType::Star);
        assert_eq!(tokens[0].value, "*");
    }

    #[test]
    fn test_simple_multiplication() {
        let mut lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 4, 7, *, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "4");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "7");
        assert_eq!(tokens[2].token_type, TokenType::Star);
        assert_eq!(tokens[2].value, "*");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_multiplication_with_addition() {
        let mut lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 6); // 2, 3, 4, *, +, EOF
        assert_eq!(tokens[0].value, "2");
        assert_eq!(tokens[1].value, "3");
        assert_eq!(tokens[2].value, "4");
        assert_eq!(tokens[3].token_type, TokenType::Star);
        assert_eq!(tokens[4].token_type, TokenType::Plus);
    }

    #[test]
    fn test_slash_operator() {
        let mut lexer = Lexer::new("/");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2); // Slash + EOF
        assert_eq!(tokens[0].token_type, TokenType::Slash);
        assert_eq!(tokens[0].value, "/");
    }

    #[test]
    fn test_simple_division() {
        let mut lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 4); // 10, 2, /, EOF
        assert_eq!(tokens[0].token_type, TokenType::Number);
        assert_eq!(tokens[0].value, "10");
        assert_eq!(tokens[1].token_type, TokenType::Number);
        assert_eq!(tokens[1].value, "2");
        assert_eq!(tokens[2].token_type, TokenType::Slash);
        assert_eq!(tokens[2].value, "/");
        assert_eq!(tokens[3].token_type, TokenType::Eof);
    }

    #[test]
    fn test_chained_division() {
        let mut lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens.len(), 8); // 100, 10, /, 5, /, 2, /, EOF
        assert_eq!(tokens[0].value, "100");
        assert_eq!(tokens[1].value, "10");
        assert_eq!(tokens[2].token_type, TokenType::Slash);
        assert_eq!(tokens[3].value, "5");
        assert_eq!(tokens[4].token_type, TokenType::Slash);
        assert_eq!(tokens[5].value, "2");
        assert_eq!(tokens[6].token_type, TokenType::Slash);
    }
}

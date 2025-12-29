//! Parser for RPN expressions that builds an AST.

use crate::ast::{BinaryOp, Expr, Number};
use crate::error::ParserError;
use crate::tokens::{Token, TokenType};

/// A parser that converts tokens into an AST.
#[derive(Debug)]
pub struct Parser {
    /// The tokens to parse.
    tokens: Vec<Token>,
    /// Current position in the token list.
    current: usize,
}

impl Parser {
    /// Creates a new parser for the given tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::parser::Parser;
    /// # use rpn2tex::tokens::{Token, TokenType};
    /// let tokens = vec![Token::new(TokenType::Number, "42".to_string(), 1, 1)];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses the tokens into an AST expression.
    ///
    /// # Errors
    ///
    /// Returns `ParserError` if the tokens cannot be parsed into a valid expression.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rpn2tex::parser::Parser;
    /// # use rpn2tex::lexer::Lexer;
    /// let mut lexer = Lexer::new("42");
    /// let tokens = lexer.scan_tokens().unwrap();
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse().unwrap();
    /// ```
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.advance();

            match token.token_type {
                TokenType::Number => {
                    let number = Number::new(token.lexeme.clone(), token.line, token.column);
                    stack.push(Expr::Number(number));
                }
                TokenType::Plus => {
                    if stack.len() < 2 {
                        return Err(ParserError::UnexpectedToken {
                            message: "Operator '+' requires two operands".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    // Pop in reverse order: right first, then left
                    let right = Box::new(stack.pop().unwrap());
                    let left = Box::new(stack.pop().unwrap());
                    let binary_op =
                        BinaryOp::new("+".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(binary_op));
                }
                TokenType::Minus => {
                    if stack.len() < 2 {
                        return Err(ParserError::UnexpectedToken {
                            message: "Operator '-' requires two operands".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    // Pop in reverse order: right first, then left
                    let right = Box::new(stack.pop().unwrap());
                    let left = Box::new(stack.pop().unwrap());
                    let binary_op =
                        BinaryOp::new("-".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(binary_op));
                }
                TokenType::Mult => {
                    if stack.len() < 2 {
                        return Err(ParserError::UnexpectedToken {
                            message: "Operator '*' requires two operands".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    // Pop in reverse order: right first, then left
                    let right = Box::new(stack.pop().unwrap());
                    let left = Box::new(stack.pop().unwrap());
                    let binary_op =
                        BinaryOp::new("*".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(binary_op));
                }
                TokenType::Div => {
                    if stack.len() < 2 {
                        return Err(ParserError::UnexpectedToken {
                            message: "Operator '/' requires two operands".to_string(),
                            line: token.line,
                            column: token.column,
                        });
                    }
                    // Pop in reverse order: right first, then left
                    let right = Box::new(stack.pop().unwrap());
                    let left = Box::new(stack.pop().unwrap());
                    let binary_op =
                        BinaryOp::new("/".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(binary_op));
                }
                TokenType::Eof => break,
            }
        }

        // For now, we expect exactly one expression on the stack
        if stack.len() == 1 {
            Ok(stack.pop().unwrap())
        } else if stack.is_empty() {
            Err(ParserError::UnexpectedToken {
                message: "No expression found".to_string(),
                line: 1,
                column: 1,
            })
        } else {
            // Multiple values left on stack (shouldn't happen with just numbers)
            Err(ParserError::UnexpectedToken {
                message: format!("Expected single expression, found {} values", stack.len()),
                line: 1,
                column: 1,
            })
        }
    }

    /// Checks if we've reached the end of the token list.
    #[must_use]
    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek().token_type, TokenType::Eof)
    }

    /// Returns the current token without consuming it.
    #[must_use]
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Consumes and returns the current token.
    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if self.current < self.tokens.len() {
            self.current += 1;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_number() {
        let mut lexer = Lexer::new("42");
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::Number(num) => assert_eq!(num.value, "42"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_decimal() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::Number(num) => assert_eq!(num.value, "3.14"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match &*op.left {
                    Expr::Number(num) => assert_eq!(num.value, "5"),
                    _ => panic!("Expected Number for left"),
                }
                match &*op.right {
                    Expr::Number(num) => assert_eq!(num.value, "3"),
                    _ => panic!("Expected Number for right"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_addition() {
        let mut lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.scan_tokens().unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Should be (((1 + 2) + 3) + 4)
        match expr {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
            }
            _ => panic!("Expected BinaryOp"),
        }
    }
}

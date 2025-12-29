//! Parser for rpn2tex - converts tokens into AST.
//!
//! This module implements a stack-based RPN (Reverse Polish Notation) parser.

use crate::ast::{BinaryOp, Expr, Number};
use crate::error::ParserError;
use crate::tokens::{Token, TokenType};

/// Stack-based RPN parser.
///
/// Converts a token stream into an Abstract Syntax Tree.
/// Uses a stack to accumulate operands and build expression trees.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
///
/// let mut lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let ast = Parser::new(tokens).parse().unwrap();
/// ```
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Create a new parser with token list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    ///
    /// let mut lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse tokens into an AST.
    ///
    /// # Errors
    ///
    /// Returns `ParserError` if the input is invalid RPN.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    ///
    /// let mut lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// let ast = Parser::new(tokens).parse().unwrap();
    /// ```
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.token_type {
                TokenType::Number => {
                    let num_node = Number::new(token.value.clone(), token.line, token.column);
                    stack.push(Expr::Number(num_node));
                    self.advance();
                }
                TokenType::Plus => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let op_node =
                        BinaryOp::new("+".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Minus => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let op_node =
                        BinaryOp::new("-".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Mult => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let op_node =
                        BinaryOp::new("*".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let op_node =
                        BinaryOp::new("/".to_string(), left, right, token.line, token.column);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            let eof_token = self
                .tokens
                .last()
                .cloned()
                .unwrap_or_else(|| Token::new(TokenType::Eof, String::new(), 1, 1));
            return Err(ParserError::new("Empty expression".to_string(), eof_token));
        }

        if stack.len() > 1 {
            let eof_token = self
                .tokens
                .last()
                .cloned()
                .unwrap_or_else(|| Token::new(TokenType::Eof, String::new(), 1, 1));
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token,
            ));
        }

        Ok(stack.into_iter().next().unwrap())
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn at_end(&self) -> bool {
        matches!(self.tokens[self.pos].token_type, TokenType::Eof)
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_single_number() {
        let mut lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        match ast {
            Expr::Number(n) => assert_eq!(n.value, "5"),
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_decimal_number() {
        let mut lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        match ast {
            Expr::Number(n) => assert_eq!(n.value, "3.14"),
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_addition() {
        let mut lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match (*op.left, *op.right) {
                    (Expr::Number(l), Expr::Number(r)) => {
                        assert_eq!(l.value, "5");
                        assert_eq!(r.value, "3");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_empty_expression() {
        let mut lexer = Lexer::new("");
        let tokens = lexer.tokenize().unwrap();
        let result = Parser::new(tokens).parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_division() {
        let mut lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();
        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "/");
                match (*op.left, *op.right) {
                    (Expr::Number(l), Expr::Number(r)) => {
                        assert_eq!(l.value, "10");
                        assert_eq!(r.value, "2");
                    }
                    _ => panic!("Expected Number operands"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
}

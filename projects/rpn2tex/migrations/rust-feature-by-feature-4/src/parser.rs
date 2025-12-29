//! Parser for rpn2tex - converts tokens into AST.
//!
//! This module implements a stack-based RPN (Reverse Polish Notation) parser.

use crate::ast::{BinaryOp, Expr, Number};
use crate::error::ParserError;
use crate::tokens::{Token, TokenType};

/// Stack-based RPN parser.
///
/// Converts a token stream into an Abstract Syntax Tree.
/// Uses a stack to accumulate operands and build expression trees
/// when operators are encountered.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// ```
pub struct Parser {
    /// List of tokens to parse
    tokens: Vec<Token>,
    /// Current position in token list
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
    /// let lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parse tokens into an AST.
    ///
    /// Returns the root expression node of the AST.
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
    /// let lexer = Lexer::new("5");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// ```
    pub fn parse(mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.token_type {
                TokenType::Number => {
                    // Push number onto stack
                    let num_node = Number::new(token.line, token.column, token.value.clone());
                    stack.push(Expr::Number(num_node));
                    self.advance();
                }
                TokenType::Plus => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            "Not enough operands for '+' operator".to_string(),
                            token.clone(),
                        ));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let op_node =
                        BinaryOp::new(token.line, token.column, "+".to_string(), left, right);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Minus => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            "Not enough operands for '-' operator".to_string(),
                            token.clone(),
                        ));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let op_node =
                        BinaryOp::new(token.line, token.column, "-".to_string(), left, right);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Mult => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            "Not enough operands for '*' operator".to_string(),
                            token.clone(),
                        ));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let op_node =
                        BinaryOp::new(token.line, token.column, "*".to_string(), left, right);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            "Not enough operands for '/' operator".to_string(),
                            token.clone(),
                        ));
                    }
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    let op_node =
                        BinaryOp::new(token.line, token.column, "/".to_string(), left, right);
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }
                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            let eof_token = self.tokens.last().unwrap().clone();
            return Err(ParserError::new("Empty expression".to_string(), eof_token));
        }

        if stack.len() > 1 {
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                self.tokens.last().unwrap().clone(),
            ));
        }

        Ok(stack.into_iter().next().unwrap())
    }

    /// Get the current token.
    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Check if we've reached EOF.
    fn at_end(&self) -> bool {
        self.tokens[self.pos].token_type == TokenType::Eof
    }

    /// Consume current token and advance to next.
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
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => {
                assert_eq!(num.value, "5");
            }
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => {
                assert_eq!(num.value, "3.14");
            }
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "+");
                match *op.left {
                    Expr::Number(ref num) => assert_eq!(num.value, "5"),
                    _ => panic!("Expected Number for left operand"),
                }
                match *op.right {
                    Expr::Number(ref num) => assert_eq!(num.value, "3"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be ((1 + 2) + 3) + 4
        match ast {
            Expr::BinaryOp(_) => {} // Just verify it parses
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "-");
                match *op.left {
                    Expr::Number(ref num) => assert_eq!(num.value, "5"),
                    _ => panic!("Expected Number for left operand"),
                }
                match *op.right {
                    Expr::Number(ref num) => assert_eq!(num.value, "3"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be (5 - 3) - 2
        match ast {
            Expr::BinaryOp(_) => {} // Just verify it parses
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_multiplication() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "*");
                match *op.left {
                    Expr::Number(ref num) => assert_eq!(num.value, "4"),
                    _ => panic!("Expected Number for left operand"),
                }
                match *op.right {
                    Expr::Number(ref num) => assert_eq!(num.value, "7"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_mixed_mult_add() {
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be 2 + (3 * 4)
        match ast {
            Expr::BinaryOp(_) => {} // Just verify it parses
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(op) => {
                assert_eq!(op.operator, "/");
                match *op.left {
                    Expr::Number(ref num) => assert_eq!(num.value, "10"),
                    _ => panic!("Expected Number for left operand"),
                }
                match *op.right {
                    Expr::Number(ref num) => assert_eq!(num.value, "2"),
                    _ => panic!("Expected Number for right operand"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be ((100 / 10) / 5) / 2
        match ast {
            Expr::BinaryOp(_) => {} // Just verify it parses
            _ => panic!("Expected BinaryOp node"),
        }
    }
}

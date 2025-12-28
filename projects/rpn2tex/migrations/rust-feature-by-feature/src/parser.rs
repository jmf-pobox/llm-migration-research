//! Parser for rpn2tex - converts tokens into AST.
//!
//! This module implements a stack-based RPN (Reverse Polish Notation) parser.
//!
//! # RPN Parsing Algorithm
//!
//! 1. When you see a number, push it onto the stack
//! 2. When you see an operator, pop operands, create a node, push result
//! 3. At EOF, the stack should contain exactly one element: the AST root
//!
//! # Example
//!
//! Input: "5 3 + 2 *"
//!
//! Stack evolution:
//!
//! ```text
//! 5       -> [5]
//! 3       -> [5, 3]
//! +       -> [5+3]
//! 2       -> [5+3, 2]
//! *       -> [(5+3)*2]
//! ```
//!
//! Result: `BinaryOp("*", BinaryOp("+", 5, 3), 2)`

use crate::ast::{BinaryOp, Expr, Number, Position};
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
/// let lexer = Lexer::new("5 3 +");
/// let tokens = lexer.tokenize().unwrap();
/// let ast = Parser::new(tokens).parse().unwrap();
/// ```
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser with a token list.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::lexer::Lexer;
    /// use rpn2tex::parser::Parser;
    ///
    /// let lexer = Lexer::new("5 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses tokens into an AST.
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
    /// let lexer = Lexer::new("2 3 +");
    /// let tokens = lexer.tokenize().unwrap();
    /// let ast = Parser::new(tokens).parse().unwrap();
    /// ```
    pub fn parse(mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current();

            match token.token_type {
                TokenType::Number => {
                    // Push number onto stack
                    let num_node =
                        Number::new(Position::new(token.line, token.column), token.value.clone());
                    stack.push(Expr::Number(num_node));
                    self.advance();
                }

                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    // Pop two operands and create binary operation
                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!("Operator '{}' requires two operands", token.value),
                            token.line,
                            token.column,
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    // Map token type to operator string
                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Mult => "*",
                        TokenType::Div => "/",
                        _ => unreachable!(),
                    };

                    let op_node = BinaryOp::new(
                        Position::new(token.line, token.column),
                        operator.to_string(),
                        left,
                        right,
                    );
                    stack.push(Expr::BinaryOp(op_node));
                    self.advance();
                }

                TokenType::Eof => break,
            }
        }

        // Validate final state
        if stack.is_empty() {
            let eof_token = self.tokens.last().unwrap();
            return Err(ParserError::new(
                "Empty expression".to_string(),
                eof_token.line,
                eof_token.column,
            ));
        }

        if stack.len() > 1 {
            let eof_token = self.tokens.last().unwrap();
            return Err(ParserError::new(
                format!(
                    "Invalid RPN: {} values remain on stack (missing operators?)",
                    stack.len()
                ),
                eof_token.line,
                eof_token.column,
            ));
        }

        Ok(stack.into_iter().next().unwrap())
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn at_end(&self) -> bool {
        self.tokens[self.pos].token_type == TokenType::Eof
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
    fn test_parse_simple_number() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        if let Expr::Number(num) = ast {
            assert_eq!(num.value, "5");
        } else {
            panic!("Expected Number node");
        }
    }

    #[test]
    fn test_parse_decimal_number() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        if let Expr::Number(num) = ast {
            assert_eq!(num.value, "3.14");
        } else {
            panic!("Expected Number node");
        }
    }

    #[test]
    fn test_parse_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let ast = Parser::new(tokens).parse().unwrap();

        if let Expr::BinaryOp(op) = ast {
            assert_eq!(op.operator, "+");
        } else {
            panic!("Expected BinaryOp node");
        }
    }
}

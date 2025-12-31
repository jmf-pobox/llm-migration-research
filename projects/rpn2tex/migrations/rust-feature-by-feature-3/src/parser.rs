//! Parser for RPN expressions.

use crate::ast::Expr;
use crate::error::ParserError;
use crate::tokens::{Token, TokenType};

/// A parser that converts tokens into an abstract syntax tree.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser for the given tokens.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5", 1, 1),
    ///     Token::new(TokenType::Eof, "", 1, 2),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    /// Parses the tokens into an expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::parser::Parser;
    /// use rpn2tex::tokens::{Token, TokenType};
    /// use rpn2tex::ast::Expr;
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "5", 1, 1),
    ///     Token::new(TokenType::Eof, "", 1, 2),
    /// ];
    /// let mut parser = Parser::new(tokens);
    /// let expr = parser.parse().unwrap();
    /// assert!(matches!(expr, Expr::Number { .. }));
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if the expression is empty or invalid.
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current().clone();

            match token.token_type {
                TokenType::Number => {
                    let num_expr = Expr::number(&token.value, token.line, token.column);
                    stack.push(num_expr);
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
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

                    let operator = match token.token_type {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Star => "*",
                        TokenType::Slash => "/",
                        _ => unreachable!(),
                    };

                    let op_expr = Expr::binary_op(operator, left, right, token.line, token.column);
                    stack.push(op_expr);
                    self.advance();
                }
                TokenType::Eof => break,
            }
        }

        // Check that we have exactly one expression on the stack
        if stack.is_empty() {
            let eof_token = self.tokens.last().unwrap();
            return Err(ParserError::new(
                "Empty expression",
                eof_token.line,
                eof_token.column,
            ));
        }

        if stack.len() > 1 {
            let last_expr = stack.last().unwrap();
            return Err(ParserError::new(
                "Too many operands",
                last_expr.line(),
                last_expr.column(),
            ));
        }

        Ok(stack.pop().unwrap())
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len() || self.current().token_type == TokenType::Eof
    }

    fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_number() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Eof, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::Number { value, .. } = expr {
            assert_eq!(value, "5");
        } else {
            panic!("Expected Number expression");
        }
    }

    #[test]
    fn test_parse_decimal() {
        let tokens = vec![
            Token::new(TokenType::Number, "3.14", 1, 1),
            Token::new(TokenType::Eof, "", 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::Number { value, .. } = expr {
            assert_eq!(value, "3.14");
        } else {
            panic!("Expected Number expression");
        }
    }

    #[test]
    fn test_empty_expression() {
        let tokens = vec![Token::new(TokenType::Eof, "", 1, 1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Empty expression");
    }

    #[test]
    fn test_too_many_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Number, "3", 1, 3),
            Token::new(TokenType::Eof, "", 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.message, "Too many operands");
    }

    #[test]
    fn test_simple_addition() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Number, "3", 1, 3),
            Token::new(TokenType::Plus, "+", 1, 5),
            Token::new(TokenType::Eof, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "+");
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "5");
            } else {
                panic!("Expected Number for left operand");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "3");
            } else {
                panic!("Expected Number for right operand");
            }
        } else {
            panic!("Expected BinaryOp expression");
        }
    }

    #[test]
    fn test_chained_addition() {
        // "1 2 + 3 + 4 +" should parse as (((1+2)+3)+4)
        let tokens = vec![
            Token::new(TokenType::Number, "1", 1, 1),
            Token::new(TokenType::Number, "2", 1, 3),
            Token::new(TokenType::Plus, "+", 1, 5),
            Token::new(TokenType::Number, "3", 1, 7),
            Token::new(TokenType::Plus, "+", 1, 9),
            Token::new(TokenType::Number, "4", 1, 11),
            Token::new(TokenType::Plus, "+", 1, 13),
            Token::new(TokenType::Eof, "", 1, 14),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        // Verify it's a BinaryOp at the top level
        assert!(matches!(expr, Expr::BinaryOp { .. }));
    }

    #[test]
    fn test_addition_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Plus, "+", 1, 3),
            Token::new(TokenType::Eof, "", 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_addition_no_operands() {
        let tokens = vec![
            Token::new(TokenType::Plus, "+", 1, 1),
            Token::new(TokenType::Eof, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_simple_subtraction() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Number, "3", 1, 3),
            Token::new(TokenType::Minus, "-", 1, 5),
            Token::new(TokenType::Eof, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "-");
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "5");
            } else {
                panic!("Expected Number for left operand");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "3");
            } else {
                panic!("Expected Number for right operand");
            }
        } else {
            panic!("Expected BinaryOp expression");
        }
    }

    #[test]
    fn test_chained_subtraction() {
        // "5 3 - 2 -" should parse as ((5-3)-2)
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Number, "3", 1, 3),
            Token::new(TokenType::Minus, "-", 1, 5),
            Token::new(TokenType::Number, "2", 1, 7),
            Token::new(TokenType::Minus, "-", 1, 9),
            Token::new(TokenType::Eof, "", 1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        // Verify it's a BinaryOp at the top level
        assert!(matches!(expr, Expr::BinaryOp { .. }));
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "-");
            // Left should be (5-3)
            assert!(matches!(*left, Expr::BinaryOp { .. }));
            // Right should be 2
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "2");
            }
        }
    }

    #[test]
    fn test_subtraction_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Minus, "-", 1, 3),
            Token::new(TokenType::Eof, "", 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_subtraction_no_operands() {
        let tokens = vec![
            Token::new(TokenType::Minus, "-", 1, 1),
            Token::new(TokenType::Eof, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_negative_number() {
        let tokens = vec![
            Token::new(TokenType::Number, "-5", 1, 1),
            Token::new(TokenType::Eof, "", 1, 3),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::Number { value, .. } = expr {
            assert_eq!(value, "-5");
        } else {
            panic!("Expected Number expression");
        }
    }

    #[test]
    fn test_simple_multiplication() {
        let tokens = vec![
            Token::new(TokenType::Number, "4", 1, 1),
            Token::new(TokenType::Number, "7", 1, 3),
            Token::new(TokenType::Star, "*", 1, 5),
            Token::new(TokenType::Eof, "", 1, 6),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "*");
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "4");
            } else {
                panic!("Expected Number for left operand");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "7");
            } else {
                panic!("Expected Number for right operand");
            }
        } else {
            panic!("Expected BinaryOp expression");
        }
    }

    #[test]
    fn test_multiplication_with_addition() {
        // "2 3 4 * +" should parse as (2 + (3*4))
        let tokens = vec![
            Token::new(TokenType::Number, "2", 1, 1),
            Token::new(TokenType::Number, "3", 1, 3),
            Token::new(TokenType::Number, "4", 1, 5),
            Token::new(TokenType::Star, "*", 1, 7),
            Token::new(TokenType::Plus, "+", 1, 9),
            Token::new(TokenType::Eof, "", 1, 10),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        // Verify it's a BinaryOp at the top level (addition)
        if let Expr::BinaryOp { operator, .. } = &expr {
            assert_eq!(operator, "+");
        } else {
            panic!("Expected BinaryOp at top level");
        }
    }

    #[test]
    fn test_multiplication_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "5", 1, 1),
            Token::new(TokenType::Star, "*", 1, 3),
            Token::new(TokenType::Eof, "", 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_multiplication_no_operands() {
        let tokens = vec![
            Token::new(TokenType::Star, "*", 1, 1),
            Token::new(TokenType::Eof, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_simple_division() {
        let tokens = vec![
            Token::new(TokenType::Number, "10", 1, 1),
            Token::new(TokenType::Number, "2", 1, 4),
            Token::new(TokenType::Slash, "/", 1, 6),
            Token::new(TokenType::Eof, "", 1, 7),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "/");
            if let Expr::Number { value, .. } = *left {
                assert_eq!(value, "10");
            } else {
                panic!("Expected Number for left operand");
            }
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "2");
            } else {
                panic!("Expected Number for right operand");
            }
        } else {
            panic!("Expected BinaryOp expression");
        }
    }

    #[test]
    fn test_chained_division() {
        // "100 10 / 5 / 2 /" should parse as (((100/10)/5)/2)
        let tokens = vec![
            Token::new(TokenType::Number, "100", 1, 1),
            Token::new(TokenType::Number, "10", 1, 5),
            Token::new(TokenType::Slash, "/", 1, 8),
            Token::new(TokenType::Number, "5", 1, 10),
            Token::new(TokenType::Slash, "/", 1, 12),
            Token::new(TokenType::Number, "2", 1, 14),
            Token::new(TokenType::Slash, "/", 1, 16),
            Token::new(TokenType::Eof, "", 1, 17),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();
        // Verify it's a BinaryOp at the top level
        assert!(matches!(expr, Expr::BinaryOp { .. }));
        if let Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } = expr
        {
            assert_eq!(operator, "/");
            // Left should be ((100/10)/5)
            assert!(matches!(*left, Expr::BinaryOp { .. }));
            // Right should be 2
            if let Expr::Number { value, .. } = *right {
                assert_eq!(value, "2");
            }
        }
    }

    #[test]
    fn test_division_insufficient_operands() {
        let tokens = vec![
            Token::new(TokenType::Number, "10", 1, 1),
            Token::new(TokenType::Slash, "/", 1, 4),
            Token::new(TokenType::Eof, "", 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }

    #[test]
    fn test_division_no_operands() {
        let tokens = vec![
            Token::new(TokenType::Slash, "/", 1, 1),
            Token::new(TokenType::Eof, "", 1, 2),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("requires two operands"));
    }
}

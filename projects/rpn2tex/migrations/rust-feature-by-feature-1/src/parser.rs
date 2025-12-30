//! Parser for RPN expressions.
//!
//! This module implements a stack-based parser for Reverse Polish Notation expressions.

use crate::{BinaryOp, Expr, Number, ParserError, Token, TokenType};

/// A parser that converts a token stream into an Abstract Syntax Tree.
///
/// # Examples
///
/// ```
/// use rpn2tex::{Lexer, Parser};
///
/// let lexer = Lexer::new("5");
/// let tokens = lexer.tokenize().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// ```
#[derive(Debug)]
#[must_use]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser for the given token stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Parser, Token, TokenType};
    ///
    /// let tokens = vec![
    ///     Token::new(TokenType::Number, "42", 1, 1),
    ///     Token::new(TokenType::Eof, "", 1, 3),
    /// ];
    /// let parser = Parser::new(tokens);
    /// ```
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses the tokens into an AST.
    ///
    /// # Errors
    ///
    /// Returns a `ParserError` if:
    /// - The input is empty
    /// - There are multiple values on the stack at the end (missing operators)
    /// - An unexpected token is encountered
    ///
    /// # Examples
    ///
    /// ```
    /// use rpn2tex::{Lexer, Parser};
    ///
    /// let lexer = Lexer::new("3.14");
    /// let tokens = lexer.tokenize().unwrap();
    /// let parser = Parser::new(tokens);
    /// let ast = parser.parse().unwrap();
    /// ```
    pub fn parse(mut self) -> Result<Expr, ParserError> {
        let mut stack: Vec<Expr> = Vec::new();

        while !self.at_end() {
            let token = self.current_token();

            match token.type_() {
                TokenType::Number => {
                    let num = Number::new(token.value().to_string(), token.line(), token.column());
                    stack.push(Expr::Number(num));
                    self.advance();
                }
                TokenType::Plus | TokenType::Minus | TokenType::Star | TokenType::Slash => {
                    // Pop two operands
                    let operator = match token.type_() {
                        TokenType::Plus => "+",
                        TokenType::Minus => "-",
                        TokenType::Star => "*",
                        TokenType::Slash => "/",
                        _ => unreachable!(),
                    };

                    if stack.len() < 2 {
                        return Err(ParserError::new(
                            format!(
                                "Not enough operands for '{}' operator (need 2, have {})",
                                operator,
                                stack.len()
                            ),
                            token.clone(),
                        ));
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let binop = BinaryOp::new(operator, left, right, token.line(), token.column());
                    stack.push(Expr::BinaryOp(binop));
                    self.advance();
                }
                TokenType::Eof => {
                    break;
                }
            }
        }

        // Validate final stack state
        if stack.is_empty() {
            return Err(ParserError::new(
                "Empty expression",
                self.tokens[self.current].clone(),
            ));
        }

        if stack.len() > 1 {
            return Err(ParserError::new(
                format!(
                    "Expected single result, found {} values on stack",
                    stack.len()
                ),
                self.tokens[self.current].clone(),
            ));
        }

        Ok(stack.pop().unwrap())
    }

    fn at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.tokens[self.current].type_() == TokenType::Eof
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) {
        if !self.at_end() {
            self.current += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    #[test]
    fn test_parse_single_number() {
        let lexer = Lexer::new("5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => {
                assert_eq!(num.value(), "5");
                assert_eq!(num.line(), 1);
                assert_eq!(num.column(), 1);
            }
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => assert_eq!(num.value(), "3.14"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_negative() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => assert_eq!(num.value(), "-5"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_empty() {
        let tokens = vec![Token::new(TokenType::Eof, "", 1, 1)];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Empty expression"));
    }

    #[test]
    fn test_parse_multiple_numbers() {
        // Multiple numbers without operators should fail
        let lexer = Lexer::new("5 3");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Expected single result"));
    }

    #[test]
    fn test_parse_preserves_format() {
        let lexer = Lexer::new("01");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => assert_eq!(num.value(), "01"),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_trailing_decimal() {
        let lexer = Lexer::new("5.");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::Number(num) => assert_eq!(num.value(), "5."),
            _ => panic!("Expected Number"),
        }
    }

    #[test]
    fn test_parse_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "+");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "3"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 + 4 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be: BinaryOp("+", BinaryOp("+", BinaryOp("+", 1, 2), 3), 4)
        match ast {
            Expr::BinaryOp(outer) => {
                assert_eq!(outer.operator(), "+");
                match outer.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "4"),
                    _ => panic!("Expected Number"),
                }
                // Check nested structure exists
                match outer.left() {
                    Expr::BinaryOp(inner) => assert_eq!(inner.operator(), "+"),
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_addition_with_floats() {
        let lexer = Lexer::new("1.5 0.5 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "+");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "1.5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "0.5"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_addition_missing_operand() {
        let lexer = Lexer::new("5 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Not enough operands"));
    }

    #[test]
    fn test_parse_addition_extra_operand() {
        let lexer = Lexer::new("5 3 2 +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Expected single result"));
    }

    #[test]
    fn test_parse_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "-");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "3"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be: BinaryOp("-", BinaryOp("-", 5, 3), 2)
        // This represents left-associativity: (5 - 3) - 2
        match ast {
            Expr::BinaryOp(outer) => {
                assert_eq!(outer.operator(), "-");
                match outer.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "2"),
                    _ => panic!("Expected Number"),
                }
                match outer.left() {
                    Expr::BinaryOp(inner) => {
                        assert_eq!(inner.operator(), "-");
                        match inner.left() {
                            Expr::Number(n) => assert_eq!(n.value(), "5"),
                            _ => panic!("Expected Number"),
                        }
                        match inner.right() {
                            Expr::Number(n) => assert_eq!(n.value(), "3"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_subtraction_with_floats() {
        let lexer = Lexer::new("5.5 2.3 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "-");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "5.5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "2.3"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_subtraction_missing_operand() {
        let lexer = Lexer::new("5 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Not enough operands"));
    }

    #[test]
    fn test_parse_subtraction_extra_operand() {
        let lexer = Lexer::new("5 3 2 -");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Expected single result"));
    }

    #[test]
    fn test_parse_multiplication() {
        let lexer = Lexer::new("4 7 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "*");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "4"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "7"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication_with_floats() {
        let lexer = Lexer::new("3.14 2 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "*");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "3.14"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "2"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication_with_addition() {
        // "2 3 4 * +" should parse as BinaryOp("+", 2, BinaryOp("*", 3, 4))
        let lexer = Lexer::new("2 3 4 * +");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(outer) => {
                assert_eq!(outer.operator(), "+");
                match outer.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "2"),
                    _ => panic!("Expected Number"),
                }
                match outer.right() {
                    Expr::BinaryOp(inner) => {
                        assert_eq!(inner.operator(), "*");
                        match inner.left() {
                            Expr::Number(n) => assert_eq!(n.value(), "3"),
                            _ => panic!("Expected Number"),
                        }
                        match inner.right() {
                            Expr::Number(n) => assert_eq!(n.value(), "4"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_multiplication_missing_operand() {
        let lexer = Lexer::new("5 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Not enough operands"));
    }

    #[test]
    fn test_parse_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "/");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "10"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "2"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        // Should be: BinaryOp("/", BinaryOp("/", BinaryOp("/", 100, 10), 5), 2)
        // This represents left-associativity: ((100 / 10) / 5) / 2
        match ast {
            Expr::BinaryOp(outer) => {
                assert_eq!(outer.operator(), "/");
                match outer.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "2"),
                    _ => panic!("Expected Number"),
                }
                match outer.left() {
                    Expr::BinaryOp(middle) => {
                        assert_eq!(middle.operator(), "/");
                        match middle.right() {
                            Expr::Number(n) => assert_eq!(n.value(), "5"),
                            _ => panic!("Expected Number"),
                        }
                        match middle.left() {
                            Expr::BinaryOp(inner) => {
                                assert_eq!(inner.operator(), "/");
                                match inner.left() {
                                    Expr::Number(n) => assert_eq!(n.value(), "100"),
                                    _ => panic!("Expected Number"),
                                }
                                match inner.right() {
                                    Expr::Number(n) => assert_eq!(n.value(), "10"),
                                    _ => panic!("Expected Number"),
                                }
                            }
                            _ => panic!("Expected BinaryOp"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division_with_floats() {
        let lexer = Lexer::new("1.5 0.5 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "/");
                match binop.left() {
                    Expr::Number(n) => assert_eq!(n.value(), "1.5"),
                    _ => panic!("Expected Number"),
                }
                match binop.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "0.5"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division_with_multiplication() {
        // "10 2 / 5 *" should parse as BinaryOp("*", BinaryOp("/", 10, 2), 5)
        let lexer = Lexer::new("10 2 / 5 *");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            Expr::BinaryOp(outer) => {
                assert_eq!(outer.operator(), "*");
                match outer.left() {
                    Expr::BinaryOp(inner) => {
                        assert_eq!(inner.operator(), "/");
                        match inner.left() {
                            Expr::Number(n) => assert_eq!(n.value(), "10"),
                            _ => panic!("Expected Number"),
                        }
                        match inner.right() {
                            Expr::Number(n) => assert_eq!(n.value(), "2"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp"),
                }
                match outer.right() {
                    Expr::Number(n) => assert_eq!(n.value(), "5"),
                    _ => panic!("Expected Number"),
                }
            }
            _ => panic!("Expected BinaryOp"),
        }
    }

    #[test]
    fn test_parse_division_missing_operand() {
        let lexer = Lexer::new("10 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Not enough operands"));
    }

    #[test]
    fn test_parse_division_extra_operand() {
        let lexer = Lexer::new("10 2 5 /");
        let tokens = lexer.tokenize().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.message().contains("Expected single result"));
    }
}

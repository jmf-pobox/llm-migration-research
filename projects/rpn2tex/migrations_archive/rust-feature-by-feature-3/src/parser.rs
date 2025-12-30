//! Parser for RPN expressions.

use crate::ast::{ASTNode, BinaryOp, Number};
use crate::error::{Error, Result};
use crate::tokens::{Token, TokenType};

/// A parser that converts tokens into an AST.
///
/// # Examples
///
/// ```
/// use rpn2tex::lexer::Lexer;
/// use rpn2tex::parser::Parser;
///
/// let lexer = Lexer::new("42");
/// let tokens = lexer.scan_tokens().unwrap();
/// let parser = Parser::new(tokens);
/// let ast = parser.parse().unwrap();
/// assert_eq!(ast.len(), 1);
/// ```
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    /// Creates a new parser from a list of tokens.
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    /// Parses the tokens into an AST using RPN stack algorithm.
    ///
    /// Numbers are pushed onto the stack. Binary operators pop two operands,
    /// create a BinaryOp node, and push it back.
    ///
    /// # Errors
    ///
    /// Returns an error if an operator doesn't have enough operands.
    pub fn parse(self) -> Result<Vec<ASTNode>> {
        let mut stack = Vec::new();

        for token in self.tokens {
            match token.token_type() {
                TokenType::Number => {
                    let num_node =
                        Number::new(token.value().to_string(), token.line(), token.column());
                    stack.push(ASTNode::Number(num_node));
                }
                TokenType::Plus | TokenType::Minus | TokenType::Mult | TokenType::Div => {
                    if stack.len() < 2 {
                        return Err(Error::ParserError {
                            message: format!(
                                "Operator '{}' requires two operands, but only {} available",
                                token.value(),
                                stack.len()
                            ),
                            line: token.line(),
                            column: token.column(),
                        });
                    }

                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();

                    let op_node =
                        BinaryOp::new(token.value(), left, right, token.line(), token.column());
                    stack.push(ASTNode::BinaryOp(op_node));
                }
            }
        }

        Ok(stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_number() {
        let lexer = Lexer::new("42");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::Number(num) => {
                assert_eq!(num.value(), "42");
                assert_eq!(num.line(), 1);
                assert_eq!(num.column(), 1);
            }
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_parse_decimal() {
        let lexer = Lexer::new("3.14");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::Number(num) => {
                assert_eq!(num.value(), "3.14");
            }
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_parse_addition() {
        let lexer = Lexer::new("5 3 +");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "+");
                match binop.left() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "5"),
                    _ => panic!("Expected Number node for left"),
                }
                match binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "3"),
                    _ => panic!("Expected Number node for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_chained_addition() {
        let lexer = Lexer::new("1 2 + 3 +");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "+");
                // Left should be (1 + 2)
                match binop.left() {
                    ASTNode::BinaryOp(left_binop) => {
                        assert_eq!(left_binop.operator(), "+");
                        match left_binop.left() {
                            ASTNode::Number(num) => assert_eq!(num.value(), "1"),
                            _ => panic!("Expected Number"),
                        }
                        match left_binop.right() {
                            ASTNode::Number(num) => assert_eq!(num.value(), "2"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp for left"),
                }
                // Right should be 3
                match binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "3"),
                    _ => panic!("Expected Number for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_insufficient_operands() {
        let lexer = Lexer::new("5 +");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        if let Err(Error::ParserError { message, .. }) = result {
            assert!(message.contains("requires two operands"));
        } else {
            panic!("Expected ParserError");
        }
    }

    #[test]
    fn test_parse_subtraction() {
        let lexer = Lexer::new("5 3 -");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "-");
                match binop.left() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "5"),
                    _ => panic!("Expected Number node for left"),
                }
                match binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "3"),
                    _ => panic!("Expected Number node for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_chained_subtraction() {
        let lexer = Lexer::new("5 3 - 2 -");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "-");
                // Left should be (5 - 3)
                match binop.left() {
                    ASTNode::BinaryOp(left_binop) => {
                        assert_eq!(left_binop.operator(), "-");
                        match left_binop.left() {
                            ASTNode::Number(num) => assert_eq!(num.value(), "5"),
                            _ => panic!("Expected Number"),
                        }
                        match left_binop.right() {
                            ASTNode::Number(num) => assert_eq!(num.value(), "3"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp for left"),
                }
                // Right should be 2
                match binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "2"),
                    _ => panic!("Expected Number for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_negative_number() {
        let lexer = Lexer::new("-5");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::Number(num) => {
                assert_eq!(num.value(), "-5");
            }
            _ => panic!("Expected Number node"),
        }
    }

    #[test]
    fn test_parse_division() {
        let lexer = Lexer::new("10 2 /");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(binop) => {
                assert_eq!(binop.operator(), "/");
                match binop.left() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "10"),
                    _ => panic!("Expected Number node for left"),
                }
                match binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "2"),
                    _ => panic!("Expected Number node for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }

    #[test]
    fn test_parse_chained_division() {
        let lexer = Lexer::new("100 10 / 5 / 2 /");
        let tokens = lexer.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            ASTNode::BinaryOp(outer_binop) => {
                assert_eq!(outer_binop.operator(), "/");
                // Left should be ((100 / 10) / 5)
                match outer_binop.left() {
                    ASTNode::BinaryOp(mid_binop) => {
                        assert_eq!(mid_binop.operator(), "/");
                        match mid_binop.left() {
                            ASTNode::BinaryOp(inner_binop) => {
                                assert_eq!(inner_binop.operator(), "/");
                                match inner_binop.left() {
                                    ASTNode::Number(num) => assert_eq!(num.value(), "100"),
                                    _ => panic!("Expected Number"),
                                }
                                match inner_binop.right() {
                                    ASTNode::Number(num) => assert_eq!(num.value(), "10"),
                                    _ => panic!("Expected Number"),
                                }
                            }
                            _ => panic!("Expected BinaryOp"),
                        }
                        match mid_binop.right() {
                            ASTNode::Number(num) => assert_eq!(num.value(), "5"),
                            _ => panic!("Expected Number"),
                        }
                    }
                    _ => panic!("Expected BinaryOp for left"),
                }
                // Right should be 2
                match outer_binop.right() {
                    ASTNode::Number(num) => assert_eq!(num.value(), "2"),
                    _ => panic!("Expected Number for right"),
                }
            }
            _ => panic!("Expected BinaryOp node"),
        }
    }
}

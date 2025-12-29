//! Integration tests for the parser module.

use rpn2tex::{Lexer, Parser};

#[test]
fn test_parser_integration_simple_addition() {
    let mut lexer = Lexer::new("5 3 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Verify the AST structure
    match ast {
        rpn2tex::Expr::BinaryOp { operator, .. } => {
            assert_eq!(operator, "+");
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_parser_integration_precedence() {
    let mut lexer = Lexer::new("5 3 + 2 *");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Should create (5+3)*2
    match ast {
        rpn2tex::Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "*");
            match *left {
                rpn2tex::Expr::BinaryOp { operator, .. } => {
                    assert_eq!(operator, "+");
                }
                _ => panic!("Expected nested BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_parser_integration_multiple_operations() {
    let mut lexer = Lexer::new("1 2 + 3 4 + *");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Should create (1+2)*(3+4)
    match ast {
        rpn2tex::Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "*");
            assert!(matches!(*left, rpn2tex::Expr::BinaryOp { .. }));
            assert!(matches!(*right, rpn2tex::Expr::BinaryOp { .. }));
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_parser_integration_floating_point() {
    let mut lexer = Lexer::new("3.14 2 *");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        rpn2tex::Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "*");
            match *left {
                rpn2tex::Expr::Number { value, .. } => {
                    assert_eq!(value, "3.14");
                }
                _ => panic!("Expected Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_parser_integration_negative_number() {
    let mut lexer = Lexer::new("-5 3 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    match ast {
        rpn2tex::Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "+");
            match *left {
                rpn2tex::Expr::Number { value, .. } => {
                    assert_eq!(value, "-5");
                }
                _ => panic!("Expected Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_parser_integration_error_insufficient_operands() {
    let mut lexer = Lexer::new("5 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("requires two operands"));
}

#[test]
fn test_parser_integration_error_excess_operands() {
    let mut lexer = Lexer::new("5 3 2 +");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Invalid RPN"));
}

#[test]
fn test_parser_integration_error_empty_expression() {
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Empty expression"));
}

#[test]
fn test_parser_integration_all_operators() {
    let test_cases = vec![
        ("5 3 +", "+"),
        ("5 3 -", "-"),
        ("5 3 *", "*"),
        ("5 3 /", "/"),
    ];

    for (input, expected_op) in test_cases {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        match ast {
            rpn2tex::Expr::BinaryOp { operator, .. } => {
                assert_eq!(operator, expected_op, "Failed for input: {}", input);
            }
            _ => panic!("Expected BinaryOp for input: {}", input),
        }
    }
}

#[test]
fn test_parser_integration_complex_expression() {
    let mut lexer = Lexer::new("10 2 / 3 + 4 *");
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    // Should create (10/2+3)*4
    match ast {
        rpn2tex::Expr::BinaryOp { operator, .. } => {
            assert_eq!(operator, "*");
        }
        _ => panic!("Expected BinaryOp"),
    }
}

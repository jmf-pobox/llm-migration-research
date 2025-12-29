//! I/O Contract tests for the parser module.

use rpn2tex::ast::Expr;
use rpn2tex::parser::Parser;
use rpn2tex::tokens::{Token, TokenType};

#[test]
fn test_io_contract_simple_addition() {
    // "5 3 +" → BinaryOp("+", Number("5"), Number("3"))
    let tokens = vec![
        Token::new(TokenType::Number, "5".to_string(), 1, 1),
        Token::new(TokenType::Number, "3".to_string(), 1, 3),
        Token::new(TokenType::Plus, "+".to_string(), 1, 5),
        Token::new(TokenType::Eof, String::new(), 1, 6),
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_ok());

    let expr = result.unwrap();
    match expr {
        Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "+");
            match (*left, *right) {
                (Expr::Number { value: v1, .. }, Expr::Number { value: v2, .. }) => {
                    assert_eq!(v1, "5");
                    assert_eq!(v2, "3");
                }
                _ => panic!("Operands should be numbers"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_io_contract_nested_expression() {
    // "5 3 + 2 *" → BinaryOp("*", BinaryOp("+", Number("5"), Number("3")), Number("2"))
    let tokens = vec![
        Token::new(TokenType::Number, "5".to_string(), 1, 1),
        Token::new(TokenType::Number, "3".to_string(), 1, 3),
        Token::new(TokenType::Plus, "+".to_string(), 1, 5),
        Token::new(TokenType::Number, "2".to_string(), 1, 7),
        Token::new(TokenType::Mult, "*".to_string(), 1, 9),
        Token::new(TokenType::Eof, String::new(), 1, 10),
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_ok());

    let expr = result.unwrap();
    match expr {
        Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "*");
            // Left should be BinaryOp("+", ...)
            match (*left, *right) {
                (
                    Expr::BinaryOp {
                        operator: op_left, ..
                    },
                    Expr::Number { value: v2, .. },
                ) => {
                    assert_eq!(op_left, "+");
                    assert_eq!(v2, "2");
                }
                _ => panic!("Expected BinaryOp on left and Number on right"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_io_contract_operator_without_operands() {
    // "+" → Error: "Operator requires two operands"
    let tokens = vec![
        Token::new(TokenType::Plus, "+".to_string(), 1, 1),
        Token::new(TokenType::Eof, String::new(), 1, 2),
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(
        err.message.contains("requires two operands"),
        "Expected 'requires two operands' but got: {}",
        err.message
    );
}

#[test]
fn test_io_contract_too_many_values() {
    // "5 3" → Error: "Invalid RPN: 2 values remain"
    let tokens = vec![
        Token::new(TokenType::Number, "5".to_string(), 1, 1),
        Token::new(TokenType::Number, "3".to_string(), 1, 3),
        Token::new(TokenType::Eof, String::new(), 1, 5),
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(
        err.message.contains("2 values remain"),
        "Expected '2 values remain' but got: {}",
        err.message
    );
}

#[test]
fn test_io_contract_all_operators() {
    // Test all four operators work correctly
    let operators = vec![
        (TokenType::Plus, "+"),
        (TokenType::Minus, "-"),
        (TokenType::Mult, "*"),
        (TokenType::Div, "/"),
    ];

    for (token_type, op_str) in operators {
        let tokens = vec![
            Token::new(TokenType::Number, "10".to_string(), 1, 1),
            Token::new(TokenType::Number, "5".to_string(), 1, 4),
            Token::new(token_type, op_str.to_string(), 1, 6),
            Token::new(TokenType::Eof, String::new(), 1, 7),
        ];
        let parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_ok(), "Failed for operator {}", op_str);

        let expr = result.unwrap();
        match expr {
            Expr::BinaryOp { operator, .. } => {
                assert_eq!(operator, op_str);
            }
            _ => panic!("Expected BinaryOp for operator {}", op_str),
        }
    }
}

#[test]
fn test_io_contract_empty_expression() {
    // Empty input should error
    let tokens = vec![Token::new(TokenType::Eof, String::new(), 1, 1)];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());

    let err = result.unwrap_err();
    assert!(
        err.message.contains("Empty expression"),
        "Expected 'Empty expression' but got: {}",
        err.message
    );
}

#[test]
fn test_io_contract_position_information() {
    // Verify that position information is preserved in AST
    let tokens = vec![
        Token::new(TokenType::Number, "5".to_string(), 2, 3),
        Token::new(TokenType::Number, "3".to_string(), 2, 5),
        Token::new(TokenType::Plus, "+".to_string(), 2, 7),
        Token::new(TokenType::Eof, String::new(), 2, 8),
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_ok());

    let expr = result.unwrap();
    assert_eq!(expr.line(), 2);
    assert_eq!(expr.column(), 7);
}

//! Integration tests for lexer I/O contract

use rpn2tex::lexer::Lexer;
use rpn2tex::tokens::TokenType;

#[test]
fn test_basic_operators() {
    let mut lexer = Lexer::new("+ - * /");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type(), TokenType::Plus);
    assert_eq!(tokens[0].value(), "+");

    assert_eq!(tokens[1].token_type(), TokenType::Minus);
    assert_eq!(tokens[1].value(), "-");

    assert_eq!(tokens[2].token_type(), TokenType::Mult);
    assert_eq!(tokens[2].value(), "*");

    assert_eq!(tokens[3].token_type(), TokenType::Div);
    assert_eq!(tokens[3].value(), "/");
}

#[test]
fn test_numbers() {
    let mut lexer = Lexer::new("5 3.14 1.5 0.5 100");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].value(), "5");
    assert_eq!(tokens[1].value(), "3.14");
    assert_eq!(tokens[2].value(), "1.5");
    assert_eq!(tokens[3].value(), "0.5");
    assert_eq!(tokens[4].value(), "100");
}

#[test]
fn test_negative_numbers() {
    let mut lexer = Lexer::new("-3 -2");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens[0].token_type(), TokenType::Number);
    assert_eq!(tokens[0].value(), "-3");

    assert_eq!(tokens[1].token_type(), TokenType::Number);
    assert_eq!(tokens[1].value(), "-2");
}

#[test]
fn test_whitespace_delimiter() {
    let mut lexer = Lexer::new("5 3 +");
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens.len(), 4); // 5, 3, +, EOF
    assert_eq!(tokens[0].value(), "5");
    assert_eq!(tokens[1].value(), "3");
    assert_eq!(tokens[2].token_type(), TokenType::Plus);
    assert_eq!(tokens[3].token_type(), TokenType::Eof);
}

#[test]
fn test_error_on_caret() {
    let mut lexer = Lexer::new("5 ^ 3");
    let result = lexer.tokenize();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("Unexpected character '^'"));
}

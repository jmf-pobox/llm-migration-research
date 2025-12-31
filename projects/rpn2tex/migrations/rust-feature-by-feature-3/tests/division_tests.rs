//! Integration tests for division feature.

use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Test case: "10 2 /" → "$10 \\div 2$"
#[test]
fn test_simple_division() {
    let input = "10 2 /";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&expr);
    assert_eq!(latex, r"$10 \div 2$");
}

/// Test case: "100 10 / 5 / 2 /" → "$100 \\div 10 \\div 5 \\div 2$"
#[test]
fn test_chained_division() {
    let input = "100 10 / 5 / 2 /";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&expr);
    assert_eq!(latex, r"$100 \div 10 \div 5 \div 2$");
}

/// Test error case: insufficient operands for division
#[test]
fn test_division_insufficient_operands() {
    let input = "10 /";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("requires two operands"));
}

/// Test error case: no operands for division
#[test]
fn test_division_no_operands() {
    let input = "/";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("requires two operands"));
}

/// Test division with decimals
#[test]
fn test_division_with_decimals() {
    let input = "3.14 2.0 /";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&expr);
    assert_eq!(latex, r"$3.14 \div 2.0$");
}

/// Test left-associativity of division
#[test]
fn test_division_left_associativity() {
    // 10 5 / 2 / should be ((10/5)/2), not (10/(5/2))
    let input = "10 5 / 2 /";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();
    let generator = LaTeXGenerator::new();
    let latex = generator.generate(&expr);
    // Should output: 10 \div 5 \div 2
    // Which mathematically evaluates as (10/5)/2 = 2/2 = 1
    // Not as 10/(5/2) = 10/2.5 = 4
    assert_eq!(latex, r"$10 \div 5 \div 2$");
}

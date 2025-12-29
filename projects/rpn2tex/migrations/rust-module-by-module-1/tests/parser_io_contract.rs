/// I/O Contract validation for parser.rs
///
/// Tests that the parser correctly processes RPN token streams according to the specification.
use rpn2tex::ast::Expr;
use rpn2tex::parser::Parser;
use rpn2tex::tokens::{Token, TokenType};

/// Helper to create a number token
fn num(value: &str, line: u32, col: u32) -> Token {
    Token::new(TokenType::Number, value.to_string(), line, col)
}

/// Helper to create an operator token
fn op(op_type: TokenType, value: &str, line: u32, col: u32) -> Token {
    Token::new(op_type, value.to_string(), line, col)
}

/// Helper to create EOF token
fn eof(line: u32, col: u32) -> Token {
    Token::new(TokenType::Eof, String::new(), line, col)
}

#[test]
fn test_io_contract_simple_addition() {
    // Input: "5 3 +"
    // Expected: BinaryOp(+, Number(5), Number(3))
    let tokens = vec![
        num("5", 1, 1),
        num("3", 1, 3),
        op(TokenType::Plus, "+", 1, 5),
        eof(1, 6),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().expect("valid RPN");

    if let Expr::BinaryOp {
        operator,
        left,
        right,
        ..
    } = result
    {
        assert_eq!(operator, "+");
        assert!(matches!(*left, Expr::Number { value, .. } if value == "5"));
        assert!(matches!(*right, Expr::Number { value, .. } if value == "3"));
    } else {
        panic!("Expected BinaryOp, got {:?}", result);
    }
}

#[test]
fn test_io_contract_left_nested() {
    // Input: "5 3 + 2 *"
    // Expected: BinaryOp(*, BinaryOp(+, Number(5), Number(3)), Number(2))
    let tokens = vec![
        num("5", 1, 1),
        num("3", 1, 3),
        op(TokenType::Plus, "+", 1, 5),
        num("2", 1, 7),
        op(TokenType::Mult, "*", 1, 9),
        eof(1, 10),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().expect("valid RPN");

    if let Expr::BinaryOp {
        operator,
        left,
        right,
        ..
    } = result
    {
        assert_eq!(operator, "*");
        assert!(matches!(
            *left,
            Expr::BinaryOp { operator, .. } if operator == "+"
        ));
        assert!(matches!(*right, Expr::Number { value, .. } if value == "2"));
    } else {
        panic!("Expected BinaryOp, got {:?}", result);
    }
}

#[test]
fn test_io_contract_right_nested() {
    // Input: "2 3 4 * +"
    // Expected: BinaryOp(+, Number(2), BinaryOp(*, Number(3), Number(4)))
    let tokens = vec![
        num("2", 1, 1),
        num("3", 1, 3),
        num("4", 1, 5),
        op(TokenType::Mult, "*", 1, 7),
        op(TokenType::Plus, "+", 1, 9),
        eof(1, 10),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().expect("valid RPN");

    if let Expr::BinaryOp {
        operator,
        left,
        right,
        ..
    } = result
    {
        assert_eq!(operator, "+");
        assert!(matches!(*left, Expr::Number { value, .. } if value == "2"));
        assert!(matches!(
            *right,
            Expr::BinaryOp { operator, .. } if operator == "*"
        ));
    } else {
        panic!("Expected BinaryOp, got {:?}", result);
    }
}

#[test]
fn test_io_contract_complex_nested() {
    // Input: "1 2 + 3 4 + *"
    // Expected: BinaryOp(*, BinaryOp(+, Number(1), Number(2)), BinaryOp(+, Number(3), Number(4)))
    let tokens = vec![
        num("1", 1, 1),
        num("2", 1, 3),
        op(TokenType::Plus, "+", 1, 5),
        num("3", 1, 7),
        num("4", 1, 9),
        op(TokenType::Plus, "+", 1, 11),
        op(TokenType::Mult, "*", 1, 13),
        eof(1, 14),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse().expect("valid RPN");

    if let Expr::BinaryOp {
        operator,
        left,
        right,
        ..
    } = result
    {
        assert_eq!(operator, "*");
        assert!(matches!(
            *left,
            Expr::BinaryOp { operator, .. } if operator == "+"
        ));
        assert!(matches!(
            *right,
            Expr::BinaryOp { operator, .. } if operator == "+"
        ));
    } else {
        panic!("Expected BinaryOp, got {:?}", result);
    }
}

#[test]
fn test_io_contract_error_not_enough_operands() {
    // Input: "5 +"
    // Expected: Error "Operator '+' requires two operands"
    let tokens = vec![num("5", 1, 1), op(TokenType::Plus, "+", 1, 3), eof(1, 4)];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("requires two operands"));
    assert!(err.message.contains("'+'"));
}

#[test]
fn test_io_contract_error_empty_expression() {
    // Input: "" (just EOF)
    // Expected: Error "Empty expression"
    let tokens = vec![eof(1, 1)];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.message, "Empty expression");
}

#[test]
fn test_io_contract_error_too_many_values() {
    // Input: "5 3" (missing operator)
    // Expected: Error "Invalid RPN: 2 values remain on stack"
    let tokens = vec![num("5", 1, 1), num("3", 1, 3), eof(1, 4)];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("2 values remain on stack"));
    assert!(err.message.contains("missing operators"));
}

#[test]
fn test_all_io_contracts() {
    // Run all I/O contract tests and report results
    println!("\n=== Parser I/O Contract Validation ===\n");

    let test_cases = vec![
        (
            "Simple addition: 5 3 +",
            test_io_contract_simple_addition as fn(),
        ),
        (
            "Left nested: 5 3 + 2 *",
            test_io_contract_left_nested as fn(),
        ),
        (
            "Right nested: 2 3 4 * +",
            test_io_contract_right_nested as fn(),
        ),
        (
            "Complex nested: 1 2 + 3 4 + *",
            test_io_contract_complex_nested as fn(),
        ),
        (
            "Error: not enough operands",
            test_io_contract_error_not_enough_operands as fn(),
        ),
        (
            "Error: empty expression",
            test_io_contract_error_empty_expression as fn(),
        ),
        (
            "Error: too many values",
            test_io_contract_error_too_many_values as fn(),
        ),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (name, test_fn) in test_cases {
        print!("  Testing: {} ... ", name);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(test_fn));
        if result.is_ok() {
            println!("PASS");
            passed += 1;
        } else {
            println!("FAIL");
            failed += 1;
        }
    }

    println!("\n=== Results: {} passed, {} failed ===\n", passed, failed);

    assert_eq!(
        failed, 0,
        "I/O contract validation failed: {} passed, {} failed",
        passed, failed
    );
}

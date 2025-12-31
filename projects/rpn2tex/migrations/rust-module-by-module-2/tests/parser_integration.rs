//! Integration tests for the parser module.
//!
//! These tests verify that the parser works correctly with the lexer
//! to parse complete RPN expressions according to the I/O contract.

use rpn2tex::ast::Expr;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Helper function to parse an RPN expression from text.
fn parse_rpn(input: &str) -> Result<Expr, String> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;
    let mut parser = Parser::new(tokens);
    parser.parse().map_err(|e| e.to_string())
}

#[test]
fn test_simple_addition() {
    // Test case 1: "5 3 +"
    let ast = parse_rpn("5 3 +").expect("parse failed");
    match ast {
        Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "+");
            match *left {
                Expr::Number { ref value, .. } => assert_eq!(value, "5"),
                _ => panic!("Expected left to be Number"),
            }
            match *right {
                Expr::Number { ref value, .. } => assert_eq!(value, "3"),
                _ => panic!("Expected right to be Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_simple_subtraction() {
    // Test case 2: "5 3 -"
    let ast = parse_rpn("5 3 -").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "-"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_simple_multiplication() {
    // Test case 3: "4 7 *"
    let ast = parse_rpn("4 7 *").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "*"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_simple_division() {
    // Test case 4: "10 2 /"
    let ast = parse_rpn("10 2 /").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "/"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_nested_with_precedence() {
    // Test case 5: "5 3 + 2 *" -> (5 + 3) * 2
    let ast = parse_rpn("5 3 + 2 *").expect("parse failed");
    match ast {
        Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "*");
            // Left should be (5 + 3)
            match *left {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "+"),
                _ => panic!("Expected left to be BinaryOp"),
            }
            // Right should be 2
            match *right {
                Expr::Number { ref value, .. } => assert_eq!(value, "2"),
                _ => panic!("Expected right to be Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_multiplication_before_addition() {
    // Test case 6: "5 3 * 2 +" -> (5 * 3) + 2
    let ast = parse_rpn("5 3 * 2 +").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "+");
            match *left {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "*"),
                _ => panic!("Expected left to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_same_precedence_left_to_right() {
    // Test case 7: "10 2 / 5 *" -> (10 / 2) * 5
    let ast = parse_rpn("10 2 / 5 *").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "*");
            match *left {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "/"),
                _ => panic!("Expected left to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_subtraction_left_associative() {
    // Test case 8: "5 3 - 2 -" -> (5 - 3) - 2
    let ast = parse_rpn("5 3 - 2 -").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "-");
            match *left {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "-"),
                _ => panic!("Expected left to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_multiple_divisions() {
    // Test case 9: "100 10 / 5 / 2 /" -> ((100 / 10) / 5) / 2
    let ast = parse_rpn("100 10 / 5 / 2 /").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "/");
            // Left should be ((100 / 10) / 5)
            match *left {
                Expr::BinaryOp {
                    operator: ref op,
                    left: ref inner_left,
                    ..
                } => {
                    assert_eq!(op, "/");
                    // Inner left should be (100 / 10)
                    match **inner_left {
                        Expr::BinaryOp {
                            operator: ref inner_op,
                            ..
                        } => assert_eq!(inner_op, "/"),
                        _ => panic!("Expected inner left to be BinaryOp"),
                    }
                }
                _ => panic!("Expected left to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_multiple_additions() {
    // Test case 10: "1 2 + 3 + 4 +" -> ((1 + 2) + 3) + 4
    let ast = parse_rpn("1 2 + 3 + 4 +").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_floating_point_multiplication() {
    // Test case 15: "3.14 2 *"
    let ast = parse_rpn("3.14 2 *").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "*");
            match *left {
                Expr::Number { ref value, .. } => assert_eq!(value, "3.14"),
                _ => panic!("Expected left to be Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_floating_point_addition() {
    // Test case 16: "1.5 0.5 +"
    let ast = parse_rpn("1.5 0.5 +").expect("parse failed");
    match ast {
        Expr::BinaryOp {
            left,
            right,
            operator,
            ..
        } => {
            assert_eq!(operator, "+");
            match *left {
                Expr::Number { ref value, .. } => assert_eq!(value, "1.5"),
                _ => panic!("Expected left to be Number"),
            }
            match *right {
                Expr::Number { ref value, .. } => assert_eq!(value, "0.5"),
                _ => panic!("Expected right to be Number"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_complex_nested_expression() {
    // Test case 17: "1 2 + 3 4 + *" -> (1 + 2) * (3 + 4)
    let ast = parse_rpn("1 2 + 3 4 + *").expect("parse failed");
    match ast {
        Expr::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            assert_eq!(operator, "*");
            // Both left and right should be additions
            match *left {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "+"),
                _ => panic!("Expected left to be BinaryOp"),
            }
            match *right {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "+"),
                _ => panic!("Expected right to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_very_complex_expression() {
    // Test case 18: "10 2 / 3 + 4 *" -> (10 / 2 + 3) * 4
    let ast = parse_rpn("10 2 / 3 + 4 *").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, left, .. } => {
            assert_eq!(operator, "*");
            // Left should be (10 / 2 + 3)
            match *left {
                Expr::BinaryOp {
                    operator: ref op,
                    left: ref inner_left,
                    ..
                } => {
                    assert_eq!(op, "+");
                    // Inner left should be (10 / 2)
                    match **inner_left {
                        Expr::BinaryOp {
                            operator: ref inner_op,
                            ..
                        } => assert_eq!(inner_op, "/"),
                        _ => panic!("Expected inner left to be BinaryOp"),
                    }
                }
                _ => panic!("Expected left to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_error_empty_expression() {
    // Should error on empty input
    let result = parse_rpn("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Empty expression"));
}

#[test]
fn test_error_missing_operand() {
    // "5 +" - missing one operand
    let result = parse_rpn("5 +");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Too few operands"));
}

#[test]
fn test_error_missing_operator() {
    // "5 3" - missing operator
    let result = parse_rpn("5 3");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Too many values") || err.contains("missing operator"));
}

#[test]
fn test_multiline_parsing() {
    // Parser should handle tokens from multiple lines
    let ast = parse_rpn("5\n3\n+").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_whitespace_handling() {
    // Parser should handle various whitespace patterns
    let ast = parse_rpn("  5   3   +  ").expect("parse failed");
    match ast {
        Expr::BinaryOp { operator, .. } => assert_eq!(operator, "+"),
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_negative_numbers() {
    // "-5 3 +" should parse correctly
    let ast = parse_rpn("-5 3 +").expect("parse failed");
    match ast {
        Expr::BinaryOp { left, .. } => match *left {
            Expr::Number { ref value, .. } => assert_eq!(value, "-5"),
            _ => panic!("Expected left to be Number"),
        },
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_mixed_operations() {
    // "2 3 4 * +" -> 2 + (3 * 4)
    let ast = parse_rpn("2 3 4 * +").expect("parse failed");
    match ast {
        Expr::BinaryOp {
            operator, right, ..
        } => {
            assert_eq!(operator, "+");
            match *right {
                Expr::BinaryOp {
                    operator: ref op, ..
                } => assert_eq!(op, "*"),
                _ => panic!("Expected right to be BinaryOp"),
            }
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[test]
fn test_position_information_preserved() {
    // Verify that line and column information is preserved
    let ast = parse_rpn("5 3 +").expect("parse failed");
    // The root should be the + operator at position 5
    assert_eq!(ast.line(), 1);
    assert_eq!(ast.column(), 5);
}

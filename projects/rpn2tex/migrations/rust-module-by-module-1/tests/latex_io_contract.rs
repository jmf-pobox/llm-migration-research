//! I/O Contract validation tests for latex module.
//!
//! These tests validate that the LaTeX generator produces exactly the expected
//! output for the specified input cases from the migration specification.

use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Helper function to convert RPN to LaTeX using the full pipeline
fn rpn_to_latex(rpn: &str) -> String {
    let mut lexer = Lexer::new(rpn);
    let tokens = lexer.tokenize().expect("tokenization should succeed");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("parsing should succeed");
    let generator = LaTeXGenerator::new();
    generator.generate(&ast)
}

#[test]
fn test_io_contract_simple_addition() {
    // 5 3 + → $5 + 3$
    let input = "5 3 +";
    let expected = "$5 + 3$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_io_contract_addition_then_multiply() {
    // 5 3 + 2 * → $( 5 + 3 ) \times 2$
    let input = "5 3 + 2 *";
    let expected = r"$( 5 + 3 ) \times 2$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_io_contract_multiply_then_addition() {
    // 5 3 * 2 + → $5 \times 3 + 2$
    let input = "5 3 * 2 +";
    let expected = r"$5 \times 3 + 2$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_io_contract_division_multiply() {
    // 10 2 / 5 * → $10 \div 2 \times 5$
    let input = "10 2 / 5 *";
    let expected = r"$10 \div 2 \times 5$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_io_contract_left_associative_subtraction() {
    // 5 3 - 2 - → $5 - 3 - 2$
    let input = "5 3 - 2 -";
    let expected = "$5 - 3 - 2$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_io_contract_multiply_with_addition_on_right() {
    // 2 3 4 + * → $2 \times ( 3 + 4 )$
    let input = "2 3 4 + *";
    let expected = r"$2 \times ( 3 + 4 )$";
    let actual = rpn_to_latex(input);
    assert_eq!(
        actual, expected,
        "Failed for input '{}': expected '{}', got '{}'",
        input, expected, actual
    );
}

#[test]
fn test_all_io_contracts() {
    // Run all test cases in one go to get a summary
    let test_cases = vec![
        ("5 3 +", "$5 + 3$"),
        ("5 3 + 2 *", r"$( 5 + 3 ) \times 2$"),
        ("5 3 * 2 +", r"$5 \times 3 + 2$"),
        ("10 2 / 5 *", r"$10 \div 2 \times 5$"),
        ("5 3 - 2 -", "$5 - 3 - 2$"),
        ("2 3 4 + *", r"$2 \times ( 3 + 4 )$"),
    ];

    let mut passed = 0;
    let mut failed = 0;

    for (input, expected) in &test_cases {
        let actual = rpn_to_latex(input);
        if actual == *expected {
            passed += 1;
        } else {
            failed += 1;
            eprintln!(
                "FAILED: '{}' -> expected '{}', got '{}'",
                input, expected, actual
            );
        }
    }

    assert_eq!(
        failed, 0,
        "I/O contract validation failed: {} passed, {} failed",
        passed, failed
    );
}

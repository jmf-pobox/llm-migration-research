//! Integration tests validating the I/O contract from the migration specification.
//!
//! These tests verify that the complete pipeline (lexer → parser → latex generator)
//! produces the exact output specified in the I/O contract for all successful test cases.

use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Helper function to convert RPN input to LaTeX output through the complete pipeline.
fn rpn_to_latex(input: &str) -> Result<String, String> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[test]
fn test_case_01_basic_addition() {
    let result = rpn_to_latex("5 3 +").expect("should succeed");
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_case_02_basic_subtraction() {
    let result = rpn_to_latex("5 3 -").expect("should succeed");
    assert_eq!(result, "$5 - 3$");
}

#[test]
fn test_case_03_basic_multiplication() {
    let result = rpn_to_latex("4 7 *").expect("should succeed");
    assert_eq!(result, r"$4 \times 7$");
}

#[test]
fn test_case_04_basic_division() {
    let result = rpn_to_latex("10 2 /").expect("should succeed");
    assert_eq!(result, r"$10 \div 2$");
}

#[test]
fn test_case_05_addition_then_multiplication() {
    let result = rpn_to_latex("5 3 + 2 *").expect("should succeed");
    assert_eq!(result, r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_case_06_multiplication_then_addition() {
    let result = rpn_to_latex("5 3 * 2 +").expect("should succeed");
    assert_eq!(result, r"$5 \times 3 + 2$");
}

#[test]
fn test_case_07_division_then_multiplication() {
    let result = rpn_to_latex("10 2 / 5 *").expect("should succeed");
    assert_eq!(result, r"$10 \div 2 \times 5$");
}

#[test]
fn test_case_08_subtraction_chain() {
    let result = rpn_to_latex("5 3 - 2 -").expect("should succeed");
    assert_eq!(result, "$5 - 3 - 2$");
}

#[test]
fn test_case_09_division_chain() {
    let result = rpn_to_latex("100 10 / 5 / 2 /").expect("should succeed");
    assert_eq!(result, r"$100 \div 10 \div 5 \div 2$");
}

#[test]
fn test_case_10_addition_chain() {
    let result = rpn_to_latex("1 2 + 3 + 4 +").expect("should succeed");
    assert_eq!(result, "$1 + 2 + 3 + 4$");
}

#[test]
fn test_case_11_multiplication_precedence() {
    let result = rpn_to_latex("2 3 4 * +").expect("should succeed");
    assert_eq!(result, r"$2 + 3 \times 4$");
}

#[test]
fn test_case_12_addition_with_parens() {
    let result = rpn_to_latex("2 3 + 4 *").expect("should succeed");
    assert_eq!(result, r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_case_13_right_operand_addition() {
    let result = rpn_to_latex("2 3 4 + *").expect("should succeed");
    assert_eq!(result, r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_case_14_multiplication_then_addition_no_parens() {
    let result = rpn_to_latex("2 3 * 4 +").expect("should succeed");
    assert_eq!(result, r"$2 \times 3 + 4$");
}

#[test]
fn test_case_15_floating_point_multiplication() {
    let result = rpn_to_latex("3.14 2 *").expect("should succeed");
    assert_eq!(result, r"$3.14 \times 2$");
}

#[test]
fn test_case_16_floating_point_addition() {
    let result = rpn_to_latex("1.5 0.5 +").expect("should succeed");
    assert_eq!(result, "$1.5 + 0.5$");
}

#[test]
fn test_case_17_multiple_additions_with_multiplication() {
    let result = rpn_to_latex("1 2 + 3 4 + *").expect("should succeed");
    assert_eq!(result, r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_case_18_complex_expression() {
    let result = rpn_to_latex("10 2 / 3 + 4 *").expect("should succeed");
    assert_eq!(result, r"$( 10 \div 2 + 3 ) \times 4$");
}

// Error cases - these should fail appropriately
// Note: The exact error messages may differ from Python implementation

#[test]
fn test_error_case_01_exponentiation_not_implemented() {
    let result = rpn_to_latex("2 3 ^");
    assert!(result.is_err(), "Should fail with unsupported operator");
}

#[test]
fn test_error_case_02_exponentiation_in_expression() {
    let result = rpn_to_latex("2 3 ^ 4 *");
    assert!(result.is_err(), "Should fail with unsupported operator");
}

#[test]
fn test_error_case_03_multiple_exponentiation() {
    let result = rpn_to_latex("2 3 4 ^ ^");
    assert!(result.is_err(), "Should fail with unsupported operator");
}

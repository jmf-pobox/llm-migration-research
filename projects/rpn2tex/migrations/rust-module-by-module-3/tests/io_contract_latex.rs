//! Integration tests for latex generation I/O contract.
//!
//! These tests verify that the latex module produces the exact expected output
//! for all test cases in the migration specification.

use rpn2tex::ast::Expr;
use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Helper function to convert RPN string to LaTeX
fn rpn_to_latex(rpn: &str) -> Result<String, Box<dyn std::error::Error>> {
    let lexer = Lexer::new(rpn.to_string());
    let tokens = lexer.tokenize()?;
    let parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[test]
fn test_io_contract_simple_addition() {
    let result = rpn_to_latex("5 3 +").unwrap();
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_io_contract_simple_multiplication() {
    let result = rpn_to_latex("4 7 *").unwrap();
    assert_eq!(result, r"$4 \times 7$");
}

#[test]
fn test_io_contract_simple_division() {
    let result = rpn_to_latex("10 2 /").unwrap();
    assert_eq!(result, r"$10 \div 2$");
}

#[test]
fn test_io_contract_addition_times_number() {
    let result = rpn_to_latex("5 3 + 2 *").unwrap();
    assert_eq!(result, r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_io_contract_multiplication_plus_number() {
    let result = rpn_to_latex("5 3 * 2 +").unwrap();
    assert_eq!(result, r"$5 \times 3 + 2$");
}

#[test]
fn test_io_contract_left_associative_subtraction() {
    let result = rpn_to_latex("5 3 - 2 -").unwrap();
    assert_eq!(result, "$5 - 3 - 2$");
}

#[test]
fn test_io_contract_left_associative_division() {
    let result = rpn_to_latex("100 10 / 5 / 2 /").unwrap();
    assert_eq!(result, r"$100 \div 10 \div 5 \div 2$");
}

#[test]
fn test_io_contract_both_operands_need_parens() {
    let result = rpn_to_latex("1 2 + 3 4 + *").unwrap();
    assert_eq!(result, r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_decimal_number() {
    let result = rpn_to_latex("3.14 2 *").unwrap();
    assert_eq!(result, r"$3.14 \times 2$");
}

#[test]
fn test_all_io_contract_cases() {
    let test_cases = vec![
        ("5 3 +", "$5 + 3$"),
        ("4 7 *", r"$4 \times 7$"),
        ("10 2 /", r"$10 \div 2$"),
        ("5 3 + 2 *", r"$( 5 + 3 ) \times 2$"),
        ("5 3 * 2 +", r"$5 \times 3 + 2$"),
        ("5 3 - 2 -", "$5 - 3 - 2$"),
        ("100 10 / 5 / 2 /", r"$100 \div 10 \div 5 \div 2$"),
        ("1 2 + 3 4 + *", r"$( 1 + 2 ) \times ( 3 + 4 )$"),
        ("3.14 2 *", r"$3.14 \times 2$"),
    ];

    for (input, expected) in test_cases {
        let result = rpn_to_latex(input).unwrap();
        assert_eq!(
            result, expected,
            "Failed for input '{}': expected '{}', got '{}'",
            input, expected, result
        );
    }
}

// Additional edge case tests
#[test]
fn test_single_number() {
    let result = rpn_to_latex("42").unwrap();
    assert_eq!(result, "$42$");
}

#[test]
fn test_subtraction() {
    let result = rpn_to_latex("10 5 -").unwrap();
    assert_eq!(result, "$10 - 5$");
}

#[test]
fn test_nested_parentheses() {
    // ((1 + 2) * 3) + 4 = "1 2 + 3 * 4 +"
    let result = rpn_to_latex("1 2 + 3 * 4 +").unwrap();
    assert_eq!(result, r"$( 1 + 2 ) \times 3 + 4$");
}

#[test]
fn test_right_associativity_matters() {
    // 5 - (3 - 2) = "5 3 2 - -"
    let result = rpn_to_latex("5 3 2 - -").unwrap();
    assert_eq!(result, "$5 - ( 3 - 2 )$");
}

#[test]
fn test_direct_ast_generation() {
    // Test the LaTeX generator directly with AST nodes
    let generator = LaTeXGenerator::new();

    // Simple addition
    let ast = Expr::binary_op("+", Expr::number("5", 1, 1), Expr::number("3", 1, 3), 1, 2);
    assert_eq!(generator.generate(&ast), "$5 + 3$");

    // Multiplication
    let ast = Expr::binary_op("*", Expr::number("4", 1, 1), Expr::number("7", 1, 3), 1, 2);
    assert_eq!(generator.generate(&ast), r"$4 \times 7$");
}

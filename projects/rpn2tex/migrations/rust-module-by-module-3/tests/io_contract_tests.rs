/// Integration tests to verify full pipeline matches I/O contract
///
/// These tests validate that the complete pipeline (Lexer → Parser → LaTeX Generator)
/// produces exact output matching the Phase 0 I/O contract specification.
use rpn2tex::latex::LatexGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

/// Helper function to process RPN input through the full pipeline
fn process_rpn(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let generator = LatexGenerator::new();
    Ok(generator.generate(&ast))
}

// ============================================================================
// VALID TEST CASES FROM I/O CONTRACT
// ============================================================================

#[test]
fn test_simple_addition() {
    // Input: "5 3 +" → Output: "$5 + 3$"
    let result = process_rpn("5 3 +").expect("Should succeed");
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_simple_subtraction() {
    // Input: "5 3 -" → Output: "$5 - 3$"
    let result = process_rpn("5 3 -").expect("Should succeed");
    assert_eq!(result, "$5 - 3$");
}

#[test]
fn test_simple_multiplication() {
    // Input: "4 7 *" → Output: "$4 \times 7$"
    let result = process_rpn("4 7 *").expect("Should succeed");
    assert_eq!(result, "$4 \\times 7$");
}

#[test]
fn test_simple_division() {
    // Input: "10 2 /" → Output: "$10 \div 2$"
    let result = process_rpn("10 2 /").expect("Should succeed");
    assert_eq!(result, "$10 \\div 2$");
}

#[test]
fn test_precedence_addition_times_multiplication() {
    // Input: "5 3 + 2 *" → Output: "$( 5 + 3 ) \times 2$"
    // This is (5+3)*2
    let result = process_rpn("5 3 + 2 *").expect("Should succeed");
    assert_eq!(result, "$( 5 + 3 ) \\times 2$");
}

#[test]
fn test_precedence_multiplication_plus_addition() {
    // Input: "5 3 * 2 +" → Output: "$5 \times 3 + 2$"
    // This is 5*3+2 (no parentheses needed)
    let result = process_rpn("5 3 * 2 +").expect("Should succeed");
    assert_eq!(result, "$5 \\times 3 + 2$");
}

#[test]
fn test_left_to_right_division_multiplication() {
    // Input: "10 2 / 5 *" → Output: "$10 \div 2 \times 5$"
    // This is (10/2)*5, left-to-right
    let result = process_rpn("10 2 / 5 *").expect("Should succeed");
    assert_eq!(result, "$10 \\div 2 \\times 5$");
}

#[test]
fn test_left_to_right_subtraction() {
    // Input: "5 3 - 2 -" → Output: "$5 - 3 - 2$"
    // This is (5-3)-2, left-to-right
    let result = process_rpn("5 3 - 2 -").expect("Should succeed");
    assert_eq!(result, "$5 - 3 - 2$");
}

#[test]
fn test_chained_division() {
    // Input: "100 10 / 5 / 2 /" → Output: "$100 \div 10 \div 5 \div 2$"
    let result = process_rpn("100 10 / 5 / 2 /").expect("Should succeed");
    assert_eq!(result, "$100 \\div 10 \\div 5 \\div 2$");
}

#[test]
fn test_chained_addition() {
    // Input: "1 2 + 3 + 4 +" → Output: "$1 + 2 + 3 + 4$"
    let result = process_rpn("1 2 + 3 + 4 +").expect("Should succeed");
    assert_eq!(result, "$1 + 2 + 3 + 4$");
}

#[test]
fn test_precedence_addition_after_multiplication() {
    // Input: "2 3 4 * +" → Output: "$2 + 3 \times 4$"
    // This is 2+(3*4)
    let result = process_rpn("2 3 4 * +").expect("Should succeed");
    assert_eq!(result, "$2 + 3 \\times 4$");
}

#[test]
fn test_explicit_grouping_via_rpn() {
    // Input: "2 3 + 4 *" → Output: "$( 2 + 3 ) \times 4$"
    let result = process_rpn("2 3 + 4 *").expect("Should succeed");
    assert_eq!(result, "$( 2 + 3 ) \\times 4$");
}

#[test]
fn test_grouping_on_right_operand() {
    // Input: "2 3 4 + *" → Output: "$2 \times ( 3 + 4 )$"
    let result = process_rpn("2 3 4 + *").expect("Should succeed");
    assert_eq!(result, "$2 \\times ( 3 + 4 )$");
}

#[test]
fn test_multiplication_then_addition() {
    // Input: "2 3 * 4 +" → Output: "$2 \times 3 + 4$"
    let result = process_rpn("2 3 * 4 +").expect("Should succeed");
    assert_eq!(result, "$2 \\times 3 + 4$");
}

#[test]
fn test_floating_point_multiplication() {
    // Input: "3.14 2 *" → Output: "$3.14 \times 2$"
    let result = process_rpn("3.14 2 *").expect("Should succeed");
    assert_eq!(result, "$3.14 \\times 2$");
}

#[test]
fn test_floating_point_addition() {
    // Input: "1.5 0.5 +" → Output: "$1.5 + 0.5$"
    let result = process_rpn("1.5 0.5 +").expect("Should succeed");
    assert_eq!(result, "$1.5 + 0.5$");
}

#[test]
fn test_multiple_subexpressions() {
    // Input: "1 2 + 3 4 + *" → Output: "$( 1 + 2 ) \times ( 3 + 4 )$"
    let result = process_rpn("1 2 + 3 4 + *").expect("Should succeed");
    assert_eq!(result, "$( 1 + 2 ) \\times ( 3 + 4 )$");
}

#[test]
fn test_complex_expression() {
    // Input: "10 2 / 3 + 4 *" → Output: "$( 10 \div 2 + 3 ) \times 4$"
    let result = process_rpn("10 2 / 3 + 4 *").expect("Should succeed");
    assert_eq!(result, "$( 10 \\div 2 + 3 ) \\times 4$");
}

#[test]
fn test_single_number() {
    // Input: "5" → Output: "$5$"
    let result = process_rpn("5").expect("Should succeed");
    assert_eq!(result, "$5$");
}

// ============================================================================
// ERROR TEST CASES FROM I/O CONTRACT
// ============================================================================

#[test]
fn test_error_empty_expression() {
    // Input: "" → Error: "Empty expression"
    let result = process_rpn("");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Empty expression");
}

#[test]
fn test_error_missing_operator() {
    // Input: "5 3" → Error: "Invalid RPN: 2 values remain on stack (missing operators?)"
    let result = process_rpn("5 3");
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid RPN: 2 values remain on stack (missing operators?)"
    );
}

#[test]
fn test_error_insufficient_operands() {
    // Input: "5 3 + +" → Error: "Operator '+' requires two operands"
    let result = process_rpn("5 3 + +");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Operator '+' requires two operands");
}

#[test]
fn test_error_unsupported_operator_exponentiation() {
    // Input: "2 3 ^" → Error: "Unexpected character '^'"
    let result = process_rpn("2 3 ^");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}

#[test]
fn test_error_unsupported_operator_in_expression() {
    // Input: "2 3 ^ 4 *" → Error: "Unexpected character '^'"
    let result = process_rpn("2 3 ^ 4 *");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}

#[test]
fn test_error_multiple_unsupported_operators() {
    // Input: "2 3 4 ^ ^" → Error: "Unexpected character '^'"
    let result = process_rpn("2 3 4 ^ ^");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '^'"));
}

#[test]
fn test_error_unrecognized_token() {
    // Input: "invalid" → Error: "Unexpected character 'i'"
    let result = process_rpn("invalid");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character 'i'"));
}

#[test]
fn test_error_invalid_character() {
    // Input: "5 @ 3" → Error: "Unexpected character '@'"
    let result = process_rpn("5 @ 3");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unexpected character '@'"));
}

// ============================================================================
// ADDITIONAL EDGE CASES
// ============================================================================

#[test]
fn test_negative_numbers() {
    // Input: "-5 3 +" → Output: "$-5 + 3$"
    let result = process_rpn("-5 3 +").expect("Should succeed");
    assert_eq!(result, "$-5 + 3$");
}

#[test]
fn test_negative_float() {
    // Input: "-3.14 2 *" → Output: "$-3.14 \times 2$"
    let result = process_rpn("-3.14 2 *").expect("Should succeed");
    assert_eq!(result, "$-3.14 \\times 2$");
}

#[test]
fn test_zero() {
    // Input: "0" → Output: "$0$"
    let result = process_rpn("0").expect("Should succeed");
    assert_eq!(result, "$0$");
}

#[test]
fn test_large_numbers() {
    // Input: "1000 2000 +" → Output: "$1000 + 2000$"
    let result = process_rpn("1000 2000 +").expect("Should succeed");
    assert_eq!(result, "$1000 + 2000$");
}

#[test]
fn test_whitespace_variations() {
    // Multiple spaces should be handled
    let result = process_rpn("5   3   +").expect("Should succeed");
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_tabs_as_delimiters() {
    // Tabs should work as delimiters
    let result = process_rpn("5\t3\t+").expect("Should succeed");
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_newlines_as_delimiters() {
    // Newlines should work as delimiters
    let result = process_rpn("5\n3\n+").expect("Should succeed");
    assert_eq!(result, "$5 + 3$");
}

// ============================================================================
// PARENTHESIZATION EDGE CASES
// ============================================================================

#[test]
fn test_right_associative_subtraction() {
    // Input: "5 3 2 - -" → Output: "$5 - ( 3 - 2 )$"
    // This is 5-(3-2), requiring parens on the right
    let result = process_rpn("5 3 2 - -").expect("Should succeed");
    assert_eq!(result, "$5 - ( 3 - 2 )$");
}

#[test]
fn test_right_associative_division() {
    // Input: "10 5 2 / /" → Output: "$10 \div ( 5 \div 2 )$"
    // This is 10/(5/2), requiring parens on the right
    let result = process_rpn("10 5 2 / /").expect("Should succeed");
    assert_eq!(result, "$10 \\div ( 5 \\div 2 )$");
}

#[test]
fn test_no_parens_for_left_associative_addition() {
    // Input: "1 2 3 + +" → Output: "$1 + ( 2 + 3 )$"
    // Wait, this is 1+(2+3), which needs parens because + on right of +
    // But + is commutative, so actually no parens needed? Let's check...
    // Actually, mathematically 1+(2+3) = 1+2+3, but RPN structure matters
    let result = process_rpn("1 2 3 + +").expect("Should succeed");
    // RPN "1 2 3 + +" means: push 1, push 2, push 3, add (2+3), add 1+(result)
    // AST: BinaryOp("+", Number(1), BinaryOp("+", Number(2), Number(3)))
    // Since + is commutative and has equal precedence, no parens needed on right
    assert_eq!(result, "$1 + 2 + 3$");
}

#[test]
fn test_mixed_precedence_complex() {
    // Input: "2 3 + 4 5 + *" → Output: "$( 2 + 3 ) \times ( 4 + 5 )$"
    let result = process_rpn("2 3 + 4 5 + *").expect("Should succeed");
    assert_eq!(result, "$( 2 + 3 ) \\times ( 4 + 5 )$");
}

#[test]
fn test_division_with_addition_left() {
    // Input: "10 2 + 5 /" → Output: "$( 10 + 2 ) \div 5$"
    let result = process_rpn("10 2 + 5 /").expect("Should succeed");
    assert_eq!(result, "$( 10 + 2 ) \\div 5$");
}

#[test]
fn test_division_with_addition_right() {
    // Input: "10 2 5 + /" → Output: "$10 \div ( 2 + 5 )$"
    let result = process_rpn("10 2 5 + /").expect("Should succeed");
    assert_eq!(result, "$10 \\div ( 2 + 5 )$");
}

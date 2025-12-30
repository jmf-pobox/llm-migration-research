//! I/O Contract tests for rpn2tex
//!
//! These tests verify that the implementation matches the expected behavior
//! from the Python reference implementation.

use rpn2tex::latex::LaTeXGenerator;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

fn process_rpn(input: &str) -> Result<String, String> {
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize().map_err(|e| e.to_string())?;

    let parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| e.to_string())?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

#[test]
fn test_io_contract_case_1_simple_integer() {
    // Test case 1: Simple integer
    // Input: "5"
    // Expected: "$5$"
    let result = process_rpn("5").unwrap();
    assert_eq!(result, "$5$");
}

#[test]
fn test_io_contract_case_2_decimal_number() {
    // Test case 2: Decimal number
    // Input: "3.14"
    // Expected: "$3.14$"
    let result = process_rpn("3.14").unwrap();
    assert_eq!(result, "$3.14$");
}

// ============================================================================
// Feature 2: Addition
// ============================================================================

#[test]
fn test_io_contract_case_3_simple_addition() {
    // Test case 3: Simple addition
    // Input: "5 3 +"
    // Expected: "$5 + 3$"
    let result = process_rpn("5 3 +").unwrap();
    assert_eq!(result, "$5 + 3$");
}

#[test]
fn test_io_contract_case_4_chained_addition() {
    // Test case 4: Chained addition (left-associative)
    // Input: "1 2 + 3 + 4 +"
    // Expected: "$1 + 2 + 3 + 4$"
    // Verifies: ((1 + 2) + 3) + 4 without unnecessary parentheses
    let result = process_rpn("1 2 + 3 + 4 +").unwrap();
    assert_eq!(result, "$1 + 2 + 3 + 4$");
}

// ============================================================================
// Feature 3: Subtraction
// ============================================================================

#[test]
fn test_io_contract_case_5_simple_subtraction() {
    // Test case 5: Simple subtraction
    // Input: "5 3 -"
    // Expected: "$5 - 3$"
    let result = process_rpn("5 3 -").unwrap();
    assert_eq!(result, "$5 - 3$");
}

#[test]
fn test_io_contract_case_6_chained_subtraction() {
    // Test case 6: Chained subtraction (left-associative)
    // Input: "5 3 - 2 -"
    // Expected: "$5 - 3 - 2$"
    // Verifies: (5 - 3) - 2 = ((5 - 3) - 2) without unnecessary parentheses
    let result = process_rpn("5 3 - 2 -").unwrap();
    assert_eq!(result, "$5 - 3 - 2$");
}

#[test]
fn test_subtraction_right_associativity() {
    // Test case: Subtraction right associativity handling
    // Input: "5 3 2 - -"
    // Expected: "$5 - ( 3 - 2 )$"
    // Verifies: 5 - (3 - 2) requires parentheses due to non-commutativity
    // RPN: 5 3 2 - - means 5 - (3 - 2)
    let result = process_rpn("5 3 2 - -").unwrap();
    assert_eq!(result, "$5 - ( 3 - 2 )$");
}

#[test]
fn test_subtraction_with_negative_number() {
    // Test case: Subtraction with negative number
    // Input: "-5 3 -"
    // Expected: "$-5 - 3$"
    // Verifies: Lexer correctly distinguishes negative number from operator
    let result = process_rpn("-5 3 -").unwrap();
    assert_eq!(result, "$-5 - 3$");
}

#[test]
fn test_subtraction_mixed_with_addition() {
    // Test case: Mixed addition and subtraction
    // Input: "10 3 - 2 +"
    // Expected: "$10 - 3 + 2$"
    // Verifies: Same precedence operators chain correctly
    let result = process_rpn("10 3 - 2 +").unwrap();
    assert_eq!(result, "$10 - 3 + 2$");
}

// ============================================================================
// Feature 4: Multiplication
// ============================================================================

#[test]
fn test_io_contract_case_7_simple_multiplication() {
    // Test case 7: Simple multiplication
    // Input: "4 7 *"
    // Expected: "$4 \\times 7$"
    // Verifies: Basic multiplication operator
    let result = process_rpn("4 7 *").unwrap();
    assert_eq!(result, r"$4 \times 7$");
}

#[test]
fn test_io_contract_case_8_multiplication_with_addition() {
    // Test case 8: Multiplication with addition (precedence test)
    // Input: "2 3 4 * +"
    // Expected: "$2 + 3 \\times 4$"
    // AST: +(2, *(3, 4))
    // Verifies: Higher precedence multiplication doesn't need parentheses when child of addition
    let result = process_rpn("2 3 4 * +").unwrap();
    assert_eq!(result, r"$2 + 3 \times 4$");
}

#[test]
fn test_multiplication_precedence_with_addition_child() {
    // Test case: Addition as child of multiplication
    // Input: "2 3 + 4 *"
    // Expected: "$( 2 + 3 ) \\times 4$"
    // AST: *((2 + 3), 4)
    // Verifies: Lower precedence addition needs parentheses when child of multiplication
    let result = process_rpn("2 3 + 4 *").unwrap();
    assert_eq!(result, r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_multiplication_precedence_right_child() {
    // Test case: Addition as right child of multiplication
    // Input: "2 3 4 + *"
    // Expected: "$2 \\times ( 3 + 4 )$"
    // AST: *(2, (3 + 4))
    // Verifies: Lower precedence on right side also needs parentheses
    let result = process_rpn("2 3 4 + *").unwrap();
    assert_eq!(result, r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_multiplication_with_decimal() {
    // Test case: Multiplication with decimal number
    // Input: "3.14 2 *"
    // Expected: "$3.14 \\times 2$"
    // Verifies: Multiplication works with decimal numbers
    let result = process_rpn("3.14 2 *").unwrap();
    assert_eq!(result, r"$3.14 \times 2$");
}

#[test]
fn test_complex_precedence_both_children() {
    // Test case: Both children have lower precedence
    // Input: "1 2 + 3 4 + *"
    // Expected: "$( 1 + 2 ) \\times ( 3 + 4 )$"
    // AST: *((1 + 2), (3 + 4))
    // Verifies: Both children need parentheses when they have lower precedence
    let result = process_rpn("1 2 + 3 4 + *").unwrap();
    assert_eq!(result, r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_multiplication_then_addition() {
    // Test case: Multiplication followed by addition
    // Input: "5 3 * 2 +"
    // Expected: "$5 \\times 3 + 2$"
    // AST: +(*(5, 3), 2)
    // Verifies: Higher precedence multiplication doesn't need parens as left child of addition
    let result = process_rpn("5 3 * 2 +").unwrap();
    assert_eq!(result, r"$5 \times 3 + 2$");
}

#[test]
fn test_addition_then_multiplication() {
    // Test case: Addition then multiplication
    // Input: "5 3 + 2 *"
    // Expected: "$( 5 + 3 ) \\times 2$"
    // AST: *(+(5, 3), 2)
    // Verifies: Lower precedence addition needs parens when child of multiplication
    let result = process_rpn("5 3 + 2 *").unwrap();
    assert_eq!(result, r"$( 5 + 3 ) \times 2$");
}

// ============================================================================
// Feature 5: Division
// ============================================================================

#[test]
fn test_io_contract_case_9_simple_division() {
    // Test case 9: Simple division
    // Input: "10 2 /"
    // Expected: "$10 \\div 2$"
    // Verifies: Basic division operator with LaTeX \div symbol
    let result = process_rpn("10 2 /").unwrap();
    assert_eq!(result, r"$10 \div 2$");
}

#[test]
fn test_io_contract_case_10_chained_division() {
    // Test case 10: Chained division (left-associative)
    // Input: "100 10 / 5 / 2 /"
    // Expected: "$100 \\div 10 \\div 5 \\div 2$"
    // Verifies: ((100 / 10) / 5) / 2 without unnecessary parentheses
    // Division is left-associative, so no parens needed for left children
    let result = process_rpn("100 10 / 5 / 2 /").unwrap();
    assert_eq!(result, r"$100 \div 10 \div 5 \div 2$");
}

#[test]
fn test_division_right_associativity() {
    // Test case: Division right associativity handling
    // Input: "100 10 5 / /"
    // Expected: "$100 \\div ( 10 \\div 5 )$"
    // Verifies: 100 / (10 / 5) requires parentheses due to non-commutativity
    // RPN: 100 10 5 / / means 100 / (10 / 5)
    let result = process_rpn("100 10 5 / /").unwrap();
    assert_eq!(result, r"$100 \div ( 10 \div 5 )$");
}

#[test]
fn test_division_with_addition() {
    // Test case: Division with addition (precedence test)
    // Input: "10 2 / 3 +"
    // Expected: "$10 \\div 2 + 3$"
    // AST: +(/(10, 2), 3)
    // Verifies: Higher precedence division doesn't need parentheses when child of addition
    let result = process_rpn("10 2 / 3 +").unwrap();
    assert_eq!(result, r"$10 \div 2 + 3$");
}

#[test]
fn test_division_with_addition_child() {
    // Test case: Addition as child of division
    // Input: "10 2 3 + /"
    // Expected: "$10 \\div ( 2 + 3 )$"
    // AST: /(10, (2 + 3))
    // Verifies: Lower precedence addition needs parentheses when child of division
    let result = process_rpn("10 2 3 + /").unwrap();
    assert_eq!(result, r"$10 \div ( 2 + 3 )$");
}

#[test]
fn test_division_mixed_with_multiplication() {
    // Test case: Division mixed with multiplication (same precedence)
    // Input: "10 2 / 3 *"
    // Expected: "$10 \\div 2 \\times 3$"
    // AST: *(/(10, 2), 3)
    // Verifies: Same precedence operators chain correctly
    let result = process_rpn("10 2 / 3 *").unwrap();
    assert_eq!(result, r"$10 \div 2 \times 3$");
}

#[test]
fn test_complex_precedence_with_division() {
    // Test case: Complex precedence with division
    // Input: "10 2 / 3 + 4 *"
    // Expected: "$( 10 \\div 2 + 3 ) \\times 4$"
    // AST: *(+(/(10, 2), 3), 4)
    // Verifies: Complex expression with multiple precedence levels
    let result = process_rpn("10 2 / 3 + 4 *").unwrap();
    assert_eq!(result, r"$( 10 \div 2 + 3 ) \times 4$");
}

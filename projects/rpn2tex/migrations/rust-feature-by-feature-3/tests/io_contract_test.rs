//! I/O Contract Tests - Verifies exact behavior from Phase 1 specification.
//!
//! These tests ensure that the implementation matches the expected behavior
//! documented in the feature specification.

use rpn2tex::compile;

/// Test Case: Numbers - Single integer
/// Input: "5"
/// Expected: "$5$"
#[test]
fn io_contract_numbers_single_integer() {
    let input = "5";
    let expected = "$5$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Test Case: Numbers - Decimal number
/// Input: "3.14"
/// Expected: "$3.14$"
#[test]
fn io_contract_numbers_decimal() {
    let input = "3.14";
    let expected = "$3.14$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Error Case: Empty expression
/// Input: (empty)
/// Expected Error: "Error: Empty expression"
#[test]
fn io_contract_error_empty_expression() {
    let input = "";
    let result = compile(input);
    assert!(result.is_err(), "Expected Err, got Ok for empty input");
    let error = result.unwrap_err();
    assert_eq!(error, "Error: Empty expression");
}

/// Error Case: Unexpected character
/// Input: "5 @ 3"
/// Expected Error: Contains "Unexpected character '@'"
#[test]
fn io_contract_error_unexpected_character() {
    let input = "5 @";
    let result = compile(input);
    assert!(
        result.is_err(),
        "Expected Err, got Ok for input with invalid character"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("Unexpected character"),
        "Error message should mention unexpected character"
    );
    assert!(error.contains("@"), "Error message should include '@'");
}

/// Test Case: Addition - Basic addition
/// Input: "5 3 +"
/// Expected: "$5 + 3$"
#[test]
fn io_contract_addition_basic() {
    let input = "5 3 +";
    let expected = "$5 + 3$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Test Case: Addition - Chained additions (left-associative)
/// Input: "1 2 + 3 + 4 +"
/// Expected: "$1 + 2 + 3 + 4$"
#[test]
fn io_contract_addition_chained() {
    let input = "1 2 + 3 + 4 +";
    let expected = "$1 + 2 + 3 + 4$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Error Case: Addition with insufficient operands
/// Input: "5 +"
/// Expected Error: Contains "requires two operands"
#[test]
fn io_contract_error_addition_insufficient_operands() {
    let input = "5 +";
    let result = compile(input);
    assert!(
        result.is_err(),
        "Expected Err, got Ok for addition with insufficient operands"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("requires two operands"),
        "Error message should mention 'requires two operands', got: {error}"
    );
}

/// Error Case: Addition with no operands
/// Input: "+"
/// Expected Error: Contains "requires two operands"
#[test]
fn io_contract_error_addition_no_operands() {
    let input = "+";
    let result = compile(input);
    assert!(
        result.is_err(),
        "Expected Err, got Ok for addition with no operands"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("requires two operands"),
        "Error message should mention 'requires two operands', got: {error}"
    );
}

/// Test Case: Subtraction - Basic subtraction
/// Input: "5 3 -"
/// Expected: "$5 - 3$"
#[test]
fn io_contract_subtraction_basic() {
    let input = "5 3 -";
    let expected = "$5 - 3$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Test Case: Subtraction - Chained subtractions (left-associative)
/// Input: "5 3 - 2 -"
/// Expected: "$5 - 3 - 2$"
#[test]
fn io_contract_subtraction_chained() {
    let input = "5 3 - 2 -";
    let expected = "$5 - 3 - 2$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Test Case: Multiplication - Basic multiplication
/// Input: "4 7 *"
/// Expected: "$4 \times 7$"
#[test]
fn io_contract_multiplication_basic() {
    let input = "4 7 *";
    let expected = r"$4 \times 7$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Test Case: Multiplication - Multiplication with addition
/// Input: "2 3 4 * +"
/// Expected: "$2 + 3 \times 4$"
/// Note: No precedence handling yet, so no parentheses around 3*4
#[test]
fn io_contract_multiplication_with_addition() {
    let input = "2 3 4 * +";
    let expected = r"$2 + 3 \times 4$";
    let result = compile(input);
    assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result);
    assert_eq!(
        result.unwrap(),
        expected,
        "Output does not match expected for input: {input}"
    );
}

/// Error Case: Multiplication with insufficient operands
/// Input: "5 *"
/// Expected Error: Contains "requires two operands"
#[test]
fn io_contract_error_multiplication_insufficient_operands() {
    let input = "5 *";
    let result = compile(input);
    assert!(
        result.is_err(),
        "Expected Err, got Ok for multiplication with insufficient operands"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("requires two operands"),
        "Error message should mention 'requires two operands', got: {error}"
    );
}

/// Error Case: Multiplication with no operands
/// Input: "*"
/// Expected Error: Contains "requires two operands"
#[test]
fn io_contract_error_multiplication_no_operands() {
    let input = "*";
    let result = compile(input);
    assert!(
        result.is_err(),
        "Expected Err, got Ok for multiplication with no operands"
    );
    let error = result.unwrap_err();
    assert!(
        error.contains("requires two operands"),
        "Error message should mention 'requires two operands', got: {error}"
    );
}

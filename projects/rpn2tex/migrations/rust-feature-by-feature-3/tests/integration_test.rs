//! Integration tests for the numbers feature.

use rpn2tex::compile;

#[test]
fn test_feature_numbers_case_1() {
    let result = compile("5");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$5$");
}

#[test]
fn test_feature_numbers_case_2() {
    let result = compile("3.14");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$3.14$");
}

#[test]
fn test_empty_expression_error() {
    let result = compile("");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err, "Error: Empty expression");
}

#[test]
fn test_invalid_character_error() {
    let result = compile("5 @");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("Unexpected character"));
    assert!(err.contains("@"));
}

#[test]
fn test_large_number() {
    let result = compile("123456789");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$123456789$");
}

#[test]
fn test_multiple_decimal_places() {
    let result = compile("3.14159265");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$3.14159265$");
}

#[test]
fn test_zero() {
    let result = compile("0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$0$");
}

#[test]
fn test_decimal_zero() {
    let result = compile("0.0");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$0.0$");
}

// Addition feature tests

#[test]
fn test_feature_addition_case_1() {
    let result = compile("5 3 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$5 + 3$");
}

#[test]
fn test_feature_addition_case_2() {
    let result = compile("1 2 + 3 + 4 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$1 + 2 + 3 + 4$");
}

#[test]
fn test_addition_error_not_enough_operands() {
    let result = compile("5 +");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("requires two operands"));
}

#[test]
fn test_addition_with_decimals() {
    let result = compile("3.14 2.71 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$3.14 + 2.71$");
}

#[test]
fn test_addition_same_number() {
    let result = compile("5 5 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$5 + 5$");
}

// Subtraction feature tests

#[test]
fn test_feature_subtraction_case_1() {
    let result = compile("5 3 -");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$5 - 3$");
}

#[test]
fn test_feature_subtraction_case_2() {
    let result = compile("5 3 - 2 -");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$5 - 3 - 2$");
}

#[test]
fn test_subtraction_error_not_enough_operands() {
    let result = compile("5 -");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("requires two operands"));
}

#[test]
fn test_subtraction_with_decimals() {
    let result = compile("10.5 2.3 -");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$10.5 - 2.3$");
}

#[test]
fn test_subtraction_same_number() {
    let result = compile("7 7 -");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$7 - 7$");
}

#[test]
fn test_negative_number() {
    let result = compile("-5");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$-5$");
}

#[test]
fn test_negative_decimal() {
    let result = compile("-3.14");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$-3.14$");
}

#[test]
fn test_mixed_addition_subtraction() {
    let result = compile("10 5 - 3 +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$10 - 5 + 3$");
}

#[test]
fn test_mixed_subtraction_addition() {
    let result = compile("10 5 + 3 -");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "$10 + 5 - 3$");
}

// Multiplication feature tests

#[test]
fn test_feature_multiplication_case_1() {
    let result = compile("4 7 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$4 \times 7$");
}

#[test]
fn test_feature_multiplication_case_2() {
    let result = compile("2 3 4 * +");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$2 + 3 \times 4$");
}

#[test]
fn test_multiplication_error_not_enough_operands() {
    let result = compile("5 *");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.contains("requires two operands"));
}

#[test]
fn test_multiplication_with_decimals() {
    let result = compile("3.5 2.0 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$3.5 \times 2.0$");
}

#[test]
fn test_multiplication_same_number() {
    let result = compile("5 5 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$5 \times 5$");
}

#[test]
fn test_chained_multiplication() {
    let result = compile("2 3 * 4 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$2 \times 3 \times 4$");
}

// Precedence feature tests

#[test]
fn test_feature_precedence_case_1() {
    // 5 3 + 2 * => (5 + 3) * 2
    let result = compile("5 3 + 2 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_feature_precedence_case_2() {
    // 2 3 + 4 * => (2 + 3) * 4
    let result = compile("2 3 + 4 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_feature_precedence_case_3() {
    // 2 3 4 + * => 2 * (3 + 4)
    let result = compile("2 3 4 + *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_feature_precedence_case_4() {
    // 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
    let result = compile("1 2 + 3 4 + *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_feature_precedence_case_5() {
    // 10 2 / 3 + 4 * => (10 / 2 + 3) * 4
    let result = compile("10 2 / 3 + 4 *");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), r"$( 10 \div 2 + 3 ) \times 4$");
}

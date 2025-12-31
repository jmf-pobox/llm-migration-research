//! Integration tests for rpn2tex Features 1-3 (numbers, addition, subtraction).

use rpn2tex::process_input;

#[test]
fn test_io_contract_integer_5() {
    let result = process_input("5");
    assert_eq!(result.unwrap(), "$5$");
}

#[test]
fn test_io_contract_float_3_14() {
    let result = process_input("3.14");
    assert_eq!(result.unwrap(), "$3.14$");
}

#[test]
fn test_negative_number() {
    let result = process_input("-5");
    assert_eq!(result.unwrap(), "$-5$");
}

#[test]
fn test_negative_float() {
    let result = process_input("-3.14");
    assert_eq!(result.unwrap(), "$-3.14$");
}

#[test]
fn test_zero() {
    let result = process_input("0");
    assert_eq!(result.unwrap(), "$0$");
}

#[test]
fn test_decimal_point() {
    let result = process_input("1.5");
    assert_eq!(result.unwrap(), "$1.5$");
}

#[test]
fn test_string_preservation() {
    // Ensure exact string is preserved, not converted to float
    let result = process_input("3.14").unwrap();
    assert_eq!(result, "$3.14$");
    assert!(!result.contains("3.1400"));
}

#[test]
fn test_large_number() {
    let result = process_input("123456789");
    assert_eq!(result.unwrap(), "$123456789$");
}

#[test]
fn test_whitespace_handling() {
    let result = process_input("  5  ");
    assert_eq!(result.unwrap(), "$5$");
}

// Feature 3: Subtraction tests

#[test]
fn test_io_contract_subtraction_5_3() {
    let result = process_input("5 3 -");
    assert_eq!(result.unwrap(), "$5 - 3$");
}

#[test]
fn test_io_contract_chained_subtraction() {
    let result = process_input("5 3 - 2 -");
    assert_eq!(result.unwrap(), "$5 - 3 - 2$");
}

#[test]
fn test_subtraction_with_floats() {
    let result = process_input("10.5 3.2 -");
    assert_eq!(result.unwrap(), "$10.5 - 3.2$");
}

#[test]
fn test_distinguish_negative_from_subtraction() {
    // Negative number (no space after -)
    let result1 = process_input("-5");
    assert_eq!(result1.unwrap(), "$-5$");

    // Subtraction with negative number
    let result2 = process_input("5 -3 +");
    assert_eq!(result2.unwrap(), "$5 + -3$");
}

#[test]
fn test_subtraction_insufficient_operands() {
    let result = process_input("5 -");
    assert!(result.is_err());
    let err_msg = result.unwrap_err();
    assert!(err_msg.contains("requires two operands"));
}

// Feature 6: Precedence tests (I/O Contract)

#[test]
fn test_io_contract_precedence_5_3_plus_2_mult() {
    // "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
    let result = process_input("5 3 + 2 *");
    assert_eq!(result.unwrap(), r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_io_contract_precedence_2_3_plus_4_mult() {
    // "2 3 + 4 *" → "$( 2 + 3 ) \times 4$"
    let result = process_input("2 3 + 4 *");
    assert_eq!(result.unwrap(), r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_io_contract_precedence_2_3_4_plus_mult() {
    // "2 3 4 + *" → "$2 \times ( 3 + 4 )$"
    let result = process_input("2 3 4 + *");
    assert_eq!(result.unwrap(), r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_precedence_both_sides() {
    // "1 2 + 3 4 + *" → "$( 1 + 2 ) \times ( 3 + 4 )$"
    let result = process_input("1 2 + 3 4 + *");
    assert_eq!(result.unwrap(), r"$( 1 + 2 ) \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_precedence_complex_mixed() {
    // "10 2 / 3 + 4 *" → "$( 10 \div 2 + 3 ) \times 4$"
    let result = process_input("10 2 / 3 + 4 *");
    assert_eq!(result.unwrap(), r"$( 10 \div 2 + 3 ) \times 4$");
}

#[test]
fn test_precedence_no_parens_higher_precedence() {
    // Higher precedence child doesn't need parens
    let result = process_input("2 3 4 * +");
    assert_eq!(result.unwrap(), r"$2 + 3 \times 4$");
}

#[test]
fn test_precedence_mixed_mult_add() {
    let result = process_input("2 3 * 4 +");
    assert_eq!(result.unwrap(), r"$2 \times 3 + 4$");
}

#[test]
fn test_precedence_mult_mult_add() {
    let result = process_input("5 3 * 2 +");
    assert_eq!(result.unwrap(), r"$5 \times 3 + 2$");
}

#[test]
fn test_precedence_div_mult_same_level() {
    let result = process_input("10 2 / 5 *");
    assert_eq!(result.unwrap(), r"$10 \div 2 \times 5$");
}

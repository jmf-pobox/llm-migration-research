//! I/O Contract Tests for Precedence Feature
use rpn2tex::compile;

/// Test Case: "5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
#[test]
fn precedence_test_case_1() {
    let input = "5 3 + 2 *";
    let expected = r"$( 5 + 3 ) \times 2$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "Test case 1 failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

/// Test Case: "2 3 + 4 *" → "$( 2 + 3 ) \times 4$"
#[test]
fn precedence_test_case_2() {
    let input = "2 3 + 4 *";
    let expected = r"$( 2 + 3 ) \times 4$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "Test case 2 failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

/// Test Case: "2 3 4 + *" → "$2 \times ( 3 + 4 )$"
#[test]
fn precedence_test_case_3() {
    let input = "2 3 4 + *";
    let expected = r"$2 \times ( 3 + 4 )$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "Test case 3 failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

/// Test Case: "1 2 + 3 4 + *" → "$( 1 + 2 ) \times ( 3 + 4 )$"
#[test]
fn precedence_test_case_4() {
    let input = "1 2 + 3 4 + *";
    let expected = r"$( 1 + 2 ) \times ( 3 + 4 )$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "Test case 4 failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

/// Test Case: "10 2 / 3 + 4 *" → "$( 10 \div 2 + 3 ) \times 4$"
#[test]
fn precedence_test_case_5() {
    let input = "10 2 / 3 + 4 *";
    let expected = r"$( 10 \div 2 + 3 ) \times 4$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "Test case 5 failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

/// Edge Case: "2 3 4 * +" → "$2 + 3 \times 4$" (no parens needed)
#[test]
fn precedence_no_parens_needed() {
    let input = "2 3 4 * +";
    let expected = r"$2 + 3 \times 4$";
    let result = compile(input).unwrap();
    assert_eq!(
        result, expected,
        "No parens case failed:\nInput: {}\nExpected: {}\nGot: {}",
        input, expected, result
    );
}

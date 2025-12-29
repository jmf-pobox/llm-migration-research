use rpn2tex::{LaTeXGenerator, Lexer, Parser};

fn process(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer
        .tokenize()
        .map_err(|e| format!("Lexer error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {}", e))?;

    let generator = LaTeXGenerator::new();
    Ok(generator.generate(&ast))
}

// I/O Contract Tests: All 18 success cases

#[test]
fn test_io_contract_case_1_basic_addition() {
    assert_eq!(process("5 3 +").unwrap(), "$5 + 3$");
}

#[test]
fn test_io_contract_case_2_basic_subtraction() {
    assert_eq!(process("5 3 -").unwrap(), "$5 - 3$");
}

#[test]
fn test_io_contract_case_3_basic_multiplication() {
    assert_eq!(process("4 7 *").unwrap(), r"$4 \times 7$");
}

#[test]
fn test_io_contract_case_4_basic_division() {
    assert_eq!(process("10 2 /").unwrap(), r"$10 \div 2$");
}

#[test]
fn test_io_contract_case_5_precedence_parens_needed() {
    // 5 3 + 2 * → ( 5 + 3 ) * 2
    // Addition has lower precedence than multiplication, needs parens
    assert_eq!(process("5 3 + 2 *").unwrap(), r"$( 5 + 3 ) \times 2$");
}

#[test]
fn test_io_contract_case_6_precedence_no_parens() {
    // 5 3 * 2 + → 5 * 3 + 2
    // Multiplication has higher precedence, no parens needed
    assert_eq!(process("5 3 * 2 +").unwrap(), r"$5 \times 3 + 2$");
}

#[test]
fn test_io_contract_case_7_left_to_right_same_precedence() {
    // 10 2 / 5 * → 10 / 2 * 5
    // Both division and multiplication have same precedence, left-to-right
    assert_eq!(process("10 2 / 5 *").unwrap(), r"$10 \div 2 \times 5$");
}

#[test]
fn test_io_contract_case_8_left_associativity_subtraction() {
    // 5 3 - 2 - → 5 - 3 - 2
    // Left-to-right evaluation, no parens on left
    assert_eq!(process("5 3 - 2 -").unwrap(), "$5 - 3 - 2$");
}

#[test]
fn test_io_contract_case_9_multiple_divisions() {
    // 100 10 / 5 / 2 / → 100 / 10 / 5 / 2
    assert_eq!(
        process("100 10 / 5 / 2 /").unwrap(),
        r"$100 \div 10 \div 5 \div 2$"
    );
}

#[test]
fn test_io_contract_case_10_multiple_additions() {
    // 1 2 + 3 + 4 + → 1 + 2 + 3 + 4
    assert_eq!(process("1 2 + 3 + 4 +").unwrap(), "$1 + 2 + 3 + 4$");
}

#[test]
fn test_io_contract_case_11_mult_higher_precedence_right() {
    // 2 3 4 * + → 2 + 3 * 4
    // Multiplication has higher precedence, no parens needed
    assert_eq!(process("2 3 4 * +").unwrap(), r"$2 + 3 \times 4$");
}

#[test]
fn test_io_contract_case_12_addition_child_of_mult_left() {
    // 2 3 + 4 * → ( 2 + 3 ) * 4
    // Addition on left side of multiplication, needs parens
    assert_eq!(process("2 3 + 4 *").unwrap(), r"$( 2 + 3 ) \times 4$");
}

#[test]
fn test_io_contract_case_13_addition_child_of_mult_right() {
    // 2 3 4 + * → 2 * ( 3 + 4 )
    // Addition on right side of multiplication, needs parens
    assert_eq!(process("2 3 4 + *").unwrap(), r"$2 \times ( 3 + 4 )$");
}

#[test]
fn test_io_contract_case_14_mult_left_of_addition() {
    // 2 3 * 4 + → 2 * 3 + 4
    // Multiplication has higher precedence, no parens
    assert_eq!(process("2 3 * 4 +").unwrap(), r"$2 \times 3 + 4$");
}

#[test]
fn test_io_contract_case_15_floating_point_multiplication() {
    // 3.14 2 * → 3.14 * 2
    assert_eq!(process("3.14 2 *").unwrap(), r"$3.14 \times 2$");
}

#[test]
fn test_io_contract_case_16_floating_point_addition() {
    // 1.5 0.5 + → 1.5 + 0.5
    assert_eq!(process("1.5 0.5 +").unwrap(), "$1.5 + 0.5$");
}

#[test]
fn test_io_contract_case_17_both_operands_need_parens() {
    // 1 2 + 3 4 + * → ( 1 + 2 ) * ( 3 + 4 )
    // Both addition operations need parens when children of multiplication
    assert_eq!(
        process("1 2 + 3 4 + *").unwrap(),
        r"$( 1 + 2 ) \times ( 3 + 4 )$"
    );
}

#[test]
fn test_io_contract_case_18_mixed_operations_complex() {
    // 10 2 / 3 + 4 * → ( 10 / 2 + 3 ) * 4
    // The entire left expression (10/2+3) needs parens as child of multiplication
    assert_eq!(
        process("10 2 / 3 + 4 *").unwrap(),
        r"$( 10 \div 2 + 3 ) \times 4$"
    );
}

// Additional edge case tests

#[test]
fn test_single_number() {
    assert_eq!(process("42").unwrap(), "$42$");
}

#[test]
fn test_negative_number() {
    assert_eq!(process("-5").unwrap(), "$-5$");
}

#[test]
fn test_negative_in_expression() {
    assert_eq!(process("-5 3 +").unwrap(), "$-5 + 3$");
}

#[test]
fn test_preserves_number_format() {
    // Should preserve exact string representation
    assert_eq!(process("3.14159").unwrap(), "$3.14159$");
}

#[test]
fn test_deeply_nested_expression() {
    // Test a complex nested structure
    // ((1 + 2) * (3 + 4)) + 5
    // In RPN: 1 2 + 3 4 + * 5 +
    assert_eq!(
        process("1 2 + 3 4 + * 5 +").unwrap(),
        r"$( 1 + 2 ) \times ( 3 + 4 ) + 5$"
    );
}

#[test]
fn test_all_operators_together() {
    // Test with all four operators
    // 10 2 / 3 + 4 * 5 -
    // This creates: ((10/2 + 3) * 4) - 5
    assert_eq!(
        process("10 2 / 3 + 4 * 5 -").unwrap(),
        r"$( 10 \div 2 + 3 ) \times 4 - 5$"
    );
}

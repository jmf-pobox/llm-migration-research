/// Integration tests to verify Rust implementation matches Python behavior exactly
use rpn2tex::error::ErrorFormatter;
use rpn2tex::lexer::Lexer;
use rpn2tex::parser::Parser;

#[test]
fn verify_python_test_case_1() {
    // Python: formatter = ErrorFormatter('5 3 @')
    // Python: error = formatter.format_error('Unexpected character \'@\'', 1, 5)
    let formatter = ErrorFormatter::new("5 3 @");
    let error = formatter.format_error("Unexpected character '@'", 1, 5);

    // Expected output from Python:
    // Error: Unexpected character '@'
    //
    // 1 | 5 3 @
    //   |     ^
    let expected = "Error: Unexpected character '@'\n\n1 | 5 3 @\n  |     ^";
    assert_eq!(error, expected, "Output should match Python exactly");
}

#[test]
fn verify_python_test_case_2() {
    // Python: formatter = ErrorFormatter('5 3 +\n2 @ 4')
    // Python: error = formatter.format_error('Unexpected character \'@\'', 2, 3)
    let formatter = ErrorFormatter::new("5 3 +\n2 @ 4");
    let error = formatter.format_error("Unexpected character '@'", 2, 3);

    // Expected output from Python:
    // Error: Unexpected character '@'
    //
    // 1 | 5 3 +
    // 2 | 2 @ 4
    //   |   ^
    let expected = "Error: Unexpected character '@'\n\n1 | 5 3 +\n2 | 2 @ 4\n  |   ^";
    assert_eq!(
        error, expected,
        "Multiline output should match Python exactly"
    );
}

#[test]
fn verify_python_test_case_3() {
    // Python: formatter = ErrorFormatter('@test')
    // Python: error = formatter.format_error('Error at start', 1, 1)
    let formatter = ErrorFormatter::new("@test");
    let error = formatter.format_error("Error at start", 1, 1);

    // Expected output from Python:
    // Error: Error at start
    //
    // 1 | @test
    //   | ^
    let expected = "Error: Error at start\n\n1 | @test\n  | ^";
    assert_eq!(
        error, expected,
        "Column 1 output should match Python exactly"
    );
}

#[test]
fn verify_context_lines_default() {
    // Test that default context_lines=1 works correctly
    let source = "line1\nline2\nline3\nline4\nline5";
    let formatter = ErrorFormatter::new(source);
    let error = formatter.format_error("Error on line 3", 3, 1);

    // Should show lines 2-4 (1 line before, error line, 1 line after)
    assert!(error.contains("2 | line2"));
    assert!(error.contains("3 | line3"));
    assert!(error.contains("4 | line4"));
    assert!(!error.contains("1 | line1"));
    assert!(!error.contains("5 | line5"));
}

// Parser integration tests with lexer
#[test]
fn verify_parser_simple_addition() {
    // Python: Lexer("5 3 +").tokenize() then Parser(tokens).parse()
    let mut lexer = Lexer::new("5 3 +");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    // Verify AST structure
    assert_eq!(ast.as_operator(), Some("+"));
    assert_eq!(ast.left().unwrap().as_number(), Some("5"));
    assert_eq!(ast.right().unwrap().as_number(), Some("3"));
}

#[test]
fn verify_parser_nested_expression() {
    // Python: Parser(Lexer("5 3 + 2 *").tokenize()).parse()
    // Should produce: BinaryOp("*", BinaryOp("+", 5, 3), 2)
    let mut lexer = Lexer::new("5 3 + 2 *");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    // Root is multiplication
    assert_eq!(ast.as_operator(), Some("*"));

    // Left operand is addition
    let left = ast.left().unwrap();
    assert_eq!(left.as_operator(), Some("+"));
    assert_eq!(left.left().unwrap().as_number(), Some("5"));
    assert_eq!(left.right().unwrap().as_number(), Some("3"));

    // Right operand is 2
    assert_eq!(ast.right().unwrap().as_number(), Some("2"));
}

#[test]
fn verify_parser_left_associativity() {
    // Python: Parser(Lexer("5 3 - 2 -").tokenize()).parse()
    // Should produce: BinaryOp("-", BinaryOp("-", 5, 3), 2) = (5-3)-2
    let mut lexer = Lexer::new("5 3 - 2 -");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    // Root is subtraction
    assert_eq!(ast.as_operator(), Some("-"));

    // Left operand is also subtraction
    let left = ast.left().unwrap();
    assert_eq!(left.as_operator(), Some("-"));
    assert_eq!(left.left().unwrap().as_number(), Some("5"));
    assert_eq!(left.right().unwrap().as_number(), Some("3"));

    // Right operand is 2
    assert_eq!(ast.right().unwrap().as_number(), Some("2"));
}

#[test]
fn verify_parser_error_empty_expression() {
    // Python: Parser(Lexer("").tokenize()).parse() raises ParserError("Empty expression")
    let mut lexer = Lexer::new("");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Empty expression");
}

#[test]
fn verify_parser_error_missing_operator() {
    // Python: Parser(Lexer("5 3").tokenize()).parse() raises ParserError("Invalid RPN: 2 values remain...")
    let mut lexer = Lexer::new("5 3");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Invalid RPN: 2 values remain on stack (missing operators?)"
    );
}

#[test]
fn verify_parser_error_insufficient_operands() {
    // Python: Parser(Lexer("5 3 + +").tokenize()).parse() raises ParserError("Operator '+' requires two operands")
    let mut lexer = Lexer::new("5 3 + +");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Operator '+' requires two operands");
}

#[test]
fn verify_parser_complex_expression() {
    // Python: Parser(Lexer("2 3 4 * +").tokenize()).parse()
    // Should produce: BinaryOp("+", 2, BinaryOp("*", 3, 4)) = 2+(3*4)
    let mut lexer = Lexer::new("2 3 4 * +");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    // Root is addition
    assert_eq!(ast.as_operator(), Some("+"));
    assert_eq!(ast.left().unwrap().as_number(), Some("2"));

    // Right operand is multiplication
    let right = ast.right().unwrap();
    assert_eq!(right.as_operator(), Some("*"));
    assert_eq!(right.left().unwrap().as_number(), Some("3"));
    assert_eq!(right.right().unwrap().as_number(), Some("4"));
}

#[test]
fn verify_parser_single_number() {
    // Python: Parser(Lexer("5").tokenize()).parse()
    // Should produce: Number("5")
    let mut lexer = Lexer::new("5");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    assert_eq!(ast.as_number(), Some("5"));
    assert_eq!(ast.as_operator(), None);
}

#[test]
fn verify_parser_floating_point() {
    // Python: Parser(Lexer("3.14 2 *").tokenize()).parse()
    let mut lexer = Lexer::new("3.14 2 *");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    assert_eq!(ast.as_operator(), Some("*"));
    assert_eq!(ast.left().unwrap().as_number(), Some("3.14"));
    assert_eq!(ast.right().unwrap().as_number(), Some("2"));
}

#[test]
fn verify_parser_negative_numbers() {
    // Python: Parser(Lexer("-5 3 +").tokenize()).parse()
    let mut lexer = Lexer::new("-5 3 +");
    let tokens = lexer.tokenize().expect("Should tokenize");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Should parse");

    assert_eq!(ast.as_operator(), Some("+"));
    assert_eq!(ast.left().unwrap().as_number(), Some("-5"));
    assert_eq!(ast.right().unwrap().as_number(), Some("3"));
}

#[test]
fn verify_parser_all_operators() {
    // Test all four operators work correctly
    let test_cases = [
        ("5 3 +", "+"),
        ("5 3 -", "-"),
        ("5 3 *", "*"),
        ("5 3 /", "/"),
    ];

    for (input, expected_op) in test_cases {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().expect("Should tokenize");
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().expect("Should parse");
        assert_eq!(ast.as_operator(), Some(expected_op));
    }
}

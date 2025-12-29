use rpn2tex::ErrorFormatter;

#[test]
fn test_error_format_simple() {
    // Test case: Error at column 5 in "2 3 ^"
    let formatter = ErrorFormatter::new("2 3 ^".to_string());
    let error = formatter.format_error("Unexpected character '^'", 1, 5, 0);
    
    // Verify expected format
    assert!(error.contains("Error: Unexpected character '^'"));
    assert!(error.contains("1 | 2 3 ^"));
    assert!(error.contains("^"));
}

#[test]
fn test_error_format_caret_position() {
    // Test case: Verify caret points to correct column
    // Input: "5 3 + @ 2"
    // Column 7 should be the '@' character
    let formatter = ErrorFormatter::new("5 3 + @ 2".to_string());
    let error = formatter.format_error("Unexpected '@'", 1, 7, 0);
    
    let lines: Vec<&str> = error.lines().collect();
    
    // Find source and caret lines
    let source_line = lines.iter().find(|l| l.contains("5 3 + @ 2")).unwrap();
    let caret_line = lines.iter().find(|l| l.contains('^')).unwrap();
    
    // The @ should be at the same position as ^
    let at_pos = source_line.find('@').unwrap();
    let caret_pos = caret_line.find('^').unwrap();
    
    // They should be at the same index in their respective lines
    assert_eq!(at_pos, caret_pos, "Caret should align with error character");
}

#[test]
fn test_error_multiline_with_context() {
    // Test case: Error on line 2 with context lines
    let source = "5 3 +\n2 4 @\n1 2 *".to_string();
    let formatter = ErrorFormatter::new(source);
    let error = formatter.format_error("Unexpected '@'", 2, 5, 1);
    
    // Should show context: line 1, 2, 3
    assert!(error.contains("1 | 5 3 +"));
    assert!(error.contains("2 | 2 4 @"));
    assert!(error.contains("3 | 1 2 *"));
    
    // Should have caret pointing to column 5
    let lines: Vec<&str> = error.lines().collect();
    let caret_line = lines.iter().find(|l| l.contains('^')).unwrap();
    let at_pos = lines.iter()
        .find(|l| l.contains("2 | 2 4 @"))
        .and_then(|l| l.find('@'))
        .unwrap();
    let caret_pos = caret_line.find('^').unwrap();
    
    assert_eq!(at_pos, caret_pos);
}

#[test]
fn test_error_format_structure() {
    // Test format: "Error: {message}\n\n{context}"
    let formatter = ErrorFormatter::new("test".to_string());
    let error = formatter.format_error("Test message", 1, 1, 0);
    
    let lines: Vec<&str> = error.lines().collect();
    
    // First line should be "Error: Test message"
    assert_eq!(lines[0], "Error: Test message");
    
    // Second line should be blank
    assert_eq!(lines[1], "");
    
    // Third line should be source context
    assert!(lines[2].contains("1 | test"));
}

#[test]
fn test_error_one_based_line_numbers() {
    // Verify line numbers are 1-based
    let source = "first\nsecond\nthird".to_string();
    let formatter = ErrorFormatter::new(source);
    let error = formatter.format_error("Error", 2, 1, 1);
    
    // Should show line numbers 1, 2, 3 (not 0-based)
    assert!(error.contains("1 | first"));
    assert!(error.contains("2 | second"));
    assert!(error.contains("3 | third"));
}

#[test]
fn test_error_edge_case_empty_source() {
    let formatter = ErrorFormatter::new(String::new());
    let error = formatter.format_error("Error on empty", 1, 1, 1);
    
    assert!(error.contains("Error: Error on empty"));
    // Should not panic
}

#[test]
fn test_error_edge_case_out_of_bounds() {
    let formatter = ErrorFormatter::new("5 3 +".to_string());
    let error = formatter.format_error("Error beyond", 10, 100, 1);
    
    assert!(error.contains("Error: Error beyond"));
    // Should not panic
}

#[test]
fn test_error_alignment_with_numbers() {
    // Test with 100+ lines to verify alignment
    let source = (1..=100)
        .map(|i| format!("line{}", i))
        .collect::<Vec<_>>()
        .join("\n");
    
    let formatter = ErrorFormatter::new(source);
    let error = formatter.format_error("Error", 50, 1, 51);
    
    // Line numbers should be right-aligned
    // Line 50 should be " 50 | ..." (3 digit width)
    // Line 100 should be "100 | ..." (3 digit width)
    assert!(error.contains(" 50 | "));
    assert!(error.contains("100 | "));
}

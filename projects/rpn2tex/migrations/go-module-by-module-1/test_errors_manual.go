package main

import (
	"fmt"
	"testing"
)

func TestErrorMessageFormat(t *testing.T) {
	// Test: Exponentiation character error message
	err := &SyntaxError{
		Message: "Unexpected character '^'",
		Line:    1,
		Column:  5,
	}
	errMsg := err.Error()
	expected := "Line 1, column 5: Unexpected character '^'"
	if errMsg != expected {
		t.Errorf("Error message mismatch:\n  got: %q\n  want: %q", errMsg, expected)
	}
	fmt.Println("Test 1 PASS: Error message format:", errMsg)

	// Test: ErrorFormatter with exponentiation character
	source := "2 3 ^"
	formatter := NewErrorFormatter(source)
	formatted := formatter.FormatError("Unexpected character '^'", 1, 5)
	if !contains(formatted, "Unexpected character '^'") {
		t.Errorf("FormatError missing message text")
	}
	fmt.Println("Test 2 PASS: FormatError includes message")
	fmt.Println("Formatted output:")
	fmt.Println(formatted)
}

func contains(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		match := true
		for j := 0; j < len(substr); j++ {
			if s[i+j] != substr[j] {
				match = false
				break
			}
		}
		if match {
			return true
		}
	}
	return false
}

package main

import "fmt"

// ExampleErrorFormatter demonstrates the error formatting functionality.
func ExampleErrorFormatter() {
	// Example 1: Simple error in single-line input
	source1 := "5 3 ^ 2 +"
	formatter1 := NewErrorFormatter(source1)
	err1 := &SyntaxError{
		Message: "Unexpected character '^'",
		Line:    1,
		Column:  5,
	}
	fmt.Println("Example 1: Single-line error")
	fmt.Println(formatter1.FormatError(err1.Message, err1.Line, err1.Column))
	fmt.Println()

	// Example 2: Error in multi-line input
	source2 := "5 3 +\n2 4 *\n7 ^ 1"
	formatter2 := NewErrorFormatter(source2)
	err2 := &SyntaxError{
		Message: "Unexpected character '^'",
		Line:    3,
		Column:  3,
	}
	fmt.Println("Example 2: Multi-line error with context")
	fmt.Println(formatter2.FormatError(err2.Message, err2.Line, err2.Column))
	fmt.Println()

	// Example 3: Error with custom context
	source3 := "line 1\nline 2\nline 3\nline 4\nline 5"
	formatter3 := NewErrorFormatter(source3)
	fmt.Println("Example 3: Error with 2 lines of context")
	fmt.Println(formatter3.FormatErrorWithContext("Error at line 3", 3, 6, 2))
}

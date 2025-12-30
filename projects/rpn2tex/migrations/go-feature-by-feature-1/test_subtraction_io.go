package main

import (
	"fmt"
)

// Test the exact I/O contract test cases for subtraction
func TestSubtractionIOContract() {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Simple subtraction",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "Chained subtraction",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
		},
	}

	allPass := true
	for _, tt := range tests {
		output, err := processRPN(tt.input)
		if err != nil {
			fmt.Printf("FAIL: %s\n  Error: %v\n", tt.name, err)
			allPass = false
			continue
		}
		if output == tt.expected {
			fmt.Printf("PASS: %s\n  Input:    %q\n  Expected: %q\n  Got:      %q\n\n", tt.name, tt.input, tt.expected, output)
		} else {
			fmt.Printf("FAIL: %s\n  Input:    %q\n  Expected: %q\n  Got:      %q\n\n", tt.name, tt.input, tt.expected, output)
			allPass = false
		}
	}

	if allPass {
		fmt.Println("All I/O Contract tests PASSED")
	} else {
		fmt.Println("Some I/O Contract tests FAILED")
	}
}

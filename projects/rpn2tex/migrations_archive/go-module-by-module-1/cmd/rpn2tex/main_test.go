package main

import (
	"bytes"
	"strings"
	"testing"
)

// TestCLI_IOContract tests the CLI against all 21 test cases from the I/O contract.
// This verifies that the CLI behaves identically to the Python implementation.
func TestCLI_IOContract(t *testing.T) {
	tests := []struct {
		name           string
		input          string
		expectedStdout string
		expectedStderr string
		expectedExit   int
	}{
		// Successful cases (exit code 0)
		{
			name:           "simple addition",
			input:          "5 3 +",
			expectedStdout: "$5 + 3$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "simple subtraction",
			input:          "5 3 -",
			expectedStdout: "$5 - 3$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "simple multiplication",
			input:          "4 7 *",
			expectedStdout: "$4 \\times 7$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "simple division",
			input:          "10 2 /",
			expectedStdout: "$10 \\div 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "parenthesization: addition then multiplication",
			input:          "5 3 + 2 *",
			expectedStdout: "$( 5 + 3 ) \\times 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "no parentheses: multiplication then addition",
			input:          "5 3 * 2 +",
			expectedStdout: "$5 \\times 3 + 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "left-associative: division and multiplication",
			input:          "10 2 / 5 *",
			expectedStdout: "$10 \\div 2 \\times 5$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "left-associative: subtraction chain",
			input:          "5 3 - 2 -",
			expectedStdout: "$5 - 3 - 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "chain of divisions",
			input:          "100 10 / 5 / 2 /",
			expectedStdout: "$100 \\div 10 \\div 5 \\div 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "associative operator: addition chain",
			input:          "1 2 + 3 + 4 +",
			expectedStdout: "$1 + 2 + 3 + 4$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "precedence: multiplication before addition",
			input:          "2 3 4 * +",
			expectedStdout: "$2 + 3 \\times 4$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "parenthesization: addition is operand of multiplication",
			input:          "2 3 + 4 *",
			expectedStdout: "$( 2 + 3 ) \\times 4$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "right-side parenthesization",
			input:          "2 3 4 + *",
			expectedStdout: "$2 \\times ( 3 + 4 )$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "no parens: multiplication has higher precedence",
			input:          "2 3 * 4 +",
			expectedStdout: "$2 \\times 3 + 4$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "decimal numbers: floating-point literals",
			input:          "3.14 2 *",
			expectedStdout: "$3.14 \\times 2$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "decimal addition",
			input:          "1.5 0.5 +",
			expectedStdout: "$1.5 + 0.5$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "multiple parenthesizations",
			input:          "1 2 + 3 4 + *",
			expectedStdout: "$( 1 + 2 ) \\times ( 3 + 4 )$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		{
			name:           "complex precedence",
			input:          "10 2 / 3 + 4 *",
			expectedStdout: "$( 10 \\div 2 + 3 ) \\times 4$\n",
			expectedStderr: "",
			expectedExit:   0,
		},
		// Error cases (exit code 1)
		{
			name:           "error: caret operator not supported",
			input:          "2 3 ^",
			expectedStdout: "",
			expectedStderr: "Error: Unexpected character '^'\n\n1 | 2 3 ^\n  |     ^",
			expectedExit:   1,
		},
		{
			name:           "error: caret at position 5",
			input:          "2 3 ^ 4 *",
			expectedStdout: "",
			expectedStderr: "Error: Unexpected character '^'\n\n1 | 2 3 ^ 4 *\n  |     ^",
			expectedExit:   1,
		},
		{
			name:           "error: caret at position 7",
			input:          "2 3 4 ^ ^",
			expectedStdout: "",
			expectedStderr: "Error: Unexpected character '^'\n\n1 | 2 3 4 ^ ^\n  |       ^",
			expectedExit:   1,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Create buffers for stdin, stdout, and stderr
			stdin := strings.NewReader(tt.input)
			var stdout, stderr bytes.Buffer

			// Run the CLI
			exitCode := run([]string{}, stdin, &stdout, &stderr)

			// Check exit code
			if exitCode != tt.expectedExit {
				t.Errorf("exit code = %d, want %d", exitCode, tt.expectedExit)
			}

			// Check stdout
			stdoutStr := stdout.String()
			if stdoutStr != tt.expectedStdout {
				t.Errorf("stdout = %q, want %q", stdoutStr, tt.expectedStdout)
			}

			// Check stderr (ignore trailing whitespace for comparison)
			stderrStr := stderr.String()
			stderrTrimmed := strings.TrimSpace(stderrStr)
			expectedStderrTrimmed := strings.TrimSpace(tt.expectedStderr)
			if stderrTrimmed != expectedStderrTrimmed {
				t.Errorf("stderr = %q, want %q", stderrTrimmed, expectedStderrTrimmed)
			}
		})
	}
}

// TestConvert tests the convert function directly.
func TestConvert(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expected    string
		expectError bool
	}{
		{
			name:        "simple addition",
			input:       "5 3 +",
			expected:    "$5 + 3$",
			expectError: false,
		},
		{
			name:        "multiplication",
			input:       "4 7 *",
			expected:    "$4 \\times 7$",
			expectError: false,
		},
		{
			name:        "invalid character",
			input:       "2 3 ^",
			expected:    "",
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result, err := convert(tt.input)

			if tt.expectError {
				if err == nil {
					t.Errorf("convert() error = nil, want error")
				}
			} else {
				if err != nil {
					t.Errorf("convert() error = %v, want nil", err)
				}
				if result != tt.expected {
					t.Errorf("convert() = %q, want %q", result, tt.expected)
				}
			}
		})
	}
}

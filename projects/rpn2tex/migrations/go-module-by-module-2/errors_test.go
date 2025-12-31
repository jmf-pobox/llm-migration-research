package rpn2tex

import (
	"fmt"
	"strings"
	"testing"
)

func TestCompileError_Error(t *testing.T) {
	tests := []struct {
		name     string
		message  string
		source   string
		line     int
		column   int
		expected string
	}{
		{
			name:    "single character error",
			message: "Unexpected character '^'",
			source:  "2 3 ^",
			line:    1,
			column:  5,
			expected: `Error: Unexpected character '^'

1 | 2 3 ^
  |     ^`,
		},
		{
			name:    "error at beginning of line",
			message: "Invalid token",
			source:  "@foo",
			line:    1,
			column:  1,
			expected: `Error: Invalid token

1 | @foo
  | ^`,
		},
		{
			name:    "error in middle of expression",
			message: "Unexpected character '^'",
			source:  "2 3 ^ 4 *",
			line:    1,
			column:  5,
			expected: `Error: Unexpected character '^'

1 | 2 3 ^ 4 *
  |     ^`,
		},
		{
			name:    "error in multi-digit line number",
			message: "Unexpected character",
			source:  strings.Repeat("x\n", 9) + "2 3 4 ^ ^",
			line:    10,
			column:  7,
			expected: `Error: Unexpected character

10 | 2 3 4 ^ ^
   |       ^`,
		},
		{
			name:    "error at end of line",
			message: "Unexpected EOF",
			source:  "5 3",
			line:    1,
			column:  4,
			expected: `Error: Unexpected EOF

1 | 5 3
  |    ^`,
		},
		{
			name:    "multiline source first line error",
			message: "Parse error",
			source:  "5 3 +\n10 2 /",
			line:    1,
			column:  3,
			expected: `Error: Parse error

1 | 5 3 +
  |   ^`,
		},
		{
			name:    "multiline source second line error",
			message: "Parse error",
			source:  "5 3 +\n10 2 /",
			line:    2,
			column:  4,
			expected: `Error: Parse error

2 | 10 2 /
  |    ^`,
		},
		{
			name:    "empty source",
			message: "Empty input",
			source:  "",
			line:    1,
			column:  1,
			expected: `Error: Empty input

`,
		},
		{
			name:    "line out of bounds (too high)",
			message: "Line error",
			source:  "5 3",
			line:    5,
			column:  1,
			expected: `Error: Line error

`,
		},
		{
			name:    "line zero (invalid)",
			message: "Invalid line",
			source:  "5 3",
			line:    0,
			column:  1,
			expected: `Error: Invalid line

`,
		},
		{
			name:    "column zero (edge case)",
			message: "Column error",
			source:  "5 3",
			line:    1,
			column:  0,
			expected: `Error: Column error

1 | 5 3
  | ^`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := NewCompileError(tt.message, tt.source, tt.line, tt.column)
			got := err.Error()

			if got != tt.expected {
				t.Errorf("Error() output mismatch\nGot:\n%s\n\nExpected:\n%s", got, tt.expected)
			}
		})
	}
}

func TestCompileError_ErrorInterface(t *testing.T) {
	// Verify that CompileError implements the error interface
	var err error = NewCompileError("test", "source", 1, 1)
	if err == nil {
		t.Error("CompileError should implement error interface")
	}

	errStr := err.Error()
	if !strings.Contains(errStr, "test") {
		t.Errorf("Error() should contain message, got: %s", errStr)
	}
}

func TestCompileError_Fields(t *testing.T) {
	message := "Test message"
	source := "test source"
	line := 42
	column := 7

	err := NewCompileError(message, source, line, column)

	if err.Message != message {
		t.Errorf("Message = %q, want %q", err.Message, message)
	}
	if err.Source != source {
		t.Errorf("Source = %q, want %q", err.Source, source)
	}
	if err.Line != line {
		t.Errorf("Line = %d, want %d", err.Line, line)
	}
	if err.Column != column {
		t.Errorf("Column = %d, want %d", err.Column, column)
	}
}

func TestCompileError_IOContractCases(t *testing.T) {
	// Test the exact error cases from the I/O contract
	tests := []struct {
		name     string
		source   string
		line     int
		column   int
		expected string
	}{
		{
			name:   "test case 5: 2 3 ^",
			source: "2 3 ^",
			line:   1,
			column: 5,
			expected: `Error: Unexpected character '^'

1 | 2 3 ^
  |     ^`,
		},
		{
			name:   "test case 16: 2 3 ^ 4 *",
			source: "2 3 ^ 4 *",
			line:   1,
			column: 5,
			expected: `Error: Unexpected character '^'

1 | 2 3 ^ 4 *
  |     ^`,
		},
		{
			name:   "test case 17: 2 3 4 ^ ^",
			source: "2 3 4 ^ ^",
			line:   1,
			column: 7,
			expected: `Error: Unexpected character '^'

1 | 2 3 4 ^ ^
  |       ^`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := NewCompileError("Unexpected character '^'", tt.source, tt.line, tt.column)
			got := err.Error()

			if got != tt.expected {
				t.Errorf("Error() output mismatch\nGot:\n%s\n\nExpected:\n%s", got, tt.expected)

				// Show character-by-character comparison for debugging
				gotLines := strings.Split(got, "\n")
				expLines := strings.Split(tt.expected, "\n")

				t.Logf("Got %d lines, expected %d lines", len(gotLines), len(expLines))
				for i := 0; i < len(gotLines) || i < len(expLines); i++ {
					gotLine := ""
					expLine := ""
					if i < len(gotLines) {
						gotLine = gotLines[i]
					}
					if i < len(expLines) {
						expLine = expLines[i]
					}
					if gotLine != expLine {
						t.Logf("Line %d differs:", i)
						t.Logf("  Got: %q", gotLine)
						t.Logf("  Exp: %q", expLine)
					}
				}
			}
		})
	}
}

func TestCompileError_CaretAlignment(t *testing.T) {
	// Test caret alignment at various column positions
	tests := []struct {
		column int
		line   string
		caret  string // Expected position of caret relative to line
	}{
		{1, "abcd", " ^"},     // Column 1 -> no spaces before caret
		{2, "abcd", "  ^"},    // Column 2 -> 1 space
		{3, "abcd", "   ^"},   // Column 3 -> 2 spaces
		{4, "abcd", "    ^"},  // Column 4 -> 3 spaces
		{5, "abcd", "     ^"}, // Column 5 -> 4 spaces
	}

	for _, tt := range tests {
		t.Run(fmt.Sprintf("column_%d", tt.column), func(t *testing.T) {
			err := NewCompileError("test", tt.line, 1, tt.column)
			output := err.Error()
			lines := strings.Split(output, "\n")

			// The caret line should be the last line
			if len(lines) < 4 {
				t.Fatalf("Expected at least 4 lines in output, got %d", len(lines))
			}

			caretLine := lines[len(lines)-1]
			expectedCaretLine := "  |" + tt.caret

			if caretLine != expectedCaretLine {
				t.Errorf("Caret line mismatch for column %d\nGot:      %q\nExpected: %q",
					tt.column, caretLine, expectedCaretLine)
			}
		})
	}
}

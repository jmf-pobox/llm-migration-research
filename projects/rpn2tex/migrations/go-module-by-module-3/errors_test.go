package rpn2tex

import (
	"strings"
	"testing"
)

func TestNewErrorFormatter(t *testing.T) {
	source := "line1\nline2\nline3"
	formatter := NewErrorFormatter(source)

	if formatter.Source != source {
		t.Errorf("Expected Source to be %q, got %q", source, formatter.Source)
	}

	expectedLines := []string{"line1", "line2", "line3"}
	if len(formatter.Lines) != len(expectedLines) {
		t.Errorf("Expected %d lines, got %d", len(expectedLines), len(formatter.Lines))
	}

	for i, expectedLine := range expectedLines {
		if formatter.Lines[i] != expectedLine {
			t.Errorf("Line %d: expected %q, got %q", i, expectedLine, formatter.Lines[i])
		}
	}
}

func TestErrorFormatterFormatError(t *testing.T) {
	tests := []struct {
		name         string
		source       string
		message      string
		line         int
		column       int
		contextLines int
		expected     string
	}{
		{
			name:         "single line error at column 1",
			source:       "hello world",
			message:      "Unexpected character",
			line:         1,
			column:       1,
			contextLines: 1,
			expected:     "Error: Unexpected character\n\n1 | hello world\n    ^",
		},
		{
			name:         "single line error at column 7",
			source:       "hello world",
			message:      "Unexpected character",
			line:         1,
			column:       7,
			contextLines: 1,
			expected:     "Error: Unexpected character\n\n1 | hello world\n          ^",
		},
		{
			name:         "multiline source error on line 2",
			source:       "line1\nline2 error here\nline3",
			message:      "Invalid token",
			line:         2,
			column:       7,
			contextLines: 1,
			expected:     "Error: Invalid token\n\n1 | line1\n2 | line2 error here\n          ^",
		},
		{
			name:         "error with no context lines (defaults to 1)",
			source:       "single line",
			message:      "Test error",
			line:         1,
			column:       5,
			contextLines: 0, // Should default to 1
			expected:     "Error: Test error\n\n1 | single line\n        ^",
		},
		{
			name:         "error with 2 context lines",
			source:       "line1\nline2\nline3\nline4\nline5",
			message:      "Middle error",
			line:         3,
			column:       3,
			contextLines: 2,
			expected:     "Error: Middle error\n\n1 | line1\n2 | line2\n3 | line3\n      ^",
		},
		{
			name:         "RPN expression with unsupported operator",
			source:       "2 3 ^",
			message:      "Unexpected character '^'",
			line:         1,
			column:       5,
			contextLines: 1,
			expected:     "Error: Unexpected character '^'\n\n1 | 2 3 ^\n        ^",
		},
		{
			name:         "multiline RPN with error on second line",
			source:       "5 3 +\n2 3 ^ 4 *\n1 2 +",
			message:      "Unexpected character '^'",
			line:         2,
			column:       5,
			contextLines: 1,
			expected:     "Error: Unexpected character '^'\n\n1 | 5 3 +\n2 | 2 3 ^ 4 *\n        ^",
		},
		{
			name:         "error at end of line",
			source:       "test^",
			message:      "Unexpected character",
			line:         1,
			column:       5,
			contextLines: 1,
			expected:     "Error: Unexpected character\n\n1 | test^\n        ^",
		},
		{
			name:         "error on last line with context",
			source:       "line1\nline2\nline3",
			message:      "End error",
			line:         3,
			column:       2,
			contextLines: 1,
			expected:     "Error: End error\n\n2 | line2\n3 | line3\n     ^",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			formatter := NewErrorFormatter(tt.source)
			result := formatter.FormatError(tt.message, tt.line, tt.column, tt.contextLines)

			if result != tt.expected {
				t.Errorf("FormatError() mismatch\nExpected:\n%s\n\nGot:\n%s", tt.expected, result)
			}
		})
	}
}

func TestErrorFormatterGetContext(t *testing.T) {
	tests := []struct {
		name         string
		source       string
		line         int
		column       int
		contextLines int
		expected     string
	}{
		{
			name:         "single line context",
			source:       "hello",
			line:         1,
			column:       3,
			contextLines: 1,
			expected:     "1 | hello\n      ^",
		},
		{
			name:         "multiple lines with context before and after",
			source:       "a\nb\nc\nd\ne",
			line:         3,
			column:       1,
			contextLines: 1,
			expected:     "2 | b\n3 | c\n    ^",
		},
		{
			name:         "caret at beginning of line",
			source:       "test line",
			line:         1,
			column:       1,
			contextLines: 0,
			expected:     "1 | test line\n    ^",
		},
		{
			name:         "double digit line numbers",
			source:       strings.Repeat("line\n", 12) + "error here",
			line:         13,
			column:       7,
			contextLines: 1,
			expected:     "12 | line\n13 | error here\n           ^",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			formatter := NewErrorFormatter(tt.source)
			result := formatter.getContext(tt.line, tt.column, tt.contextLines)

			if result != tt.expected {
				t.Errorf("getContext() mismatch\nExpected:\n%s\n\nGot:\n%s", tt.expected, result)
			}
		})
	}
}

func TestMaxMin(t *testing.T) {
	tests := []struct {
		name     string
		a        int
		b        int
		expected int
		function string
	}{
		{"max positive", 5, 3, 5, "max"},
		{"max negative", -5, -3, -3, "max"},
		{"max equal", 7, 7, 7, "max"},
		{"min positive", 5, 3, 3, "min"},
		{"min negative", -5, -3, -5, "min"},
		{"min equal", 7, 7, 7, "min"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			var result int
			if tt.function == "max" {
				result = max(tt.a, tt.b)
			} else {
				result = min(tt.a, tt.b)
			}

			if result != tt.expected {
				t.Errorf("%s(%d, %d) = %d, expected %d", tt.function, tt.a, tt.b, result, tt.expected)
			}
		})
	}
}

func TestErrorFormatterEdgeCases(t *testing.T) {
	t.Run("empty source", func(t *testing.T) {
		formatter := NewErrorFormatter("")
		result := formatter.FormatError("Error", 1, 1, 1)
		// Should not panic
		if !strings.Contains(result, "Error: Error") {
			t.Errorf("Expected error message to contain 'Error: Error', got: %s", result)
		}
	})

	t.Run("single character source", func(t *testing.T) {
		formatter := NewErrorFormatter("x")
		result := formatter.FormatError("Test", 1, 1, 1)
		expected := "Error: Test\n\n1 | x\n    ^"
		if result != expected {
			t.Errorf("Expected:\n%s\n\nGot:\n%s", expected, result)
		}
	})

	t.Run("very large context lines", func(t *testing.T) {
		source := "line1\nline2\nline3"
		formatter := NewErrorFormatter(source)
		// Request 100 context lines, should clamp to available lines
		result := formatter.FormatError("Test", 2, 1, 100)
		// Should show lines up to the error line (line1 and line2), plus context after
		if !strings.Contains(result, "line1") || !strings.Contains(result, "line2") {
			t.Errorf("Expected lines 1-3 to be shown, got: %s", result)
		}
	})

	t.Run("line beyond source length", func(t *testing.T) {
		formatter := NewErrorFormatter("line1\nline2")
		// Requesting line 10 (beyond source) should not panic
		result := formatter.FormatError("Test", 10, 1, 1)
		if !strings.Contains(result, "Error: Test") {
			t.Errorf("Expected error message, got: %s", result)
		}
	})
}

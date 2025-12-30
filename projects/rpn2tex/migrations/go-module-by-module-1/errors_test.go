package main

import (
	"strings"
	"testing"
)

func TestSyntaxError_Error(t *testing.T) {
	tests := []struct {
		name     string
		err      *SyntaxError
		expected string
	}{
		{
			name: "basic error",
			err: &SyntaxError{
				Message: "Unexpected character",
				Line:    1,
				Column:  5,
			},
			expected: "Line 1, column 5: Unexpected character",
		},
		{
			name: "multi-digit line and column",
			err: &SyntaxError{
				Message: "Invalid token",
				Line:    42,
				Column:  137,
			},
			expected: "Line 42, column 137: Invalid token",
		},
		{
			name: "error with special characters",
			err: &SyntaxError{
				Message: "Unexpected character '^'",
				Line:    3,
				Column:  8,
			},
			expected: "Line 3, column 8: Unexpected character '^'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := tt.err.Error()
			if got != tt.expected {
				t.Errorf("Error() = %q, want %q", got, tt.expected)
			}
		})
	}
}

func TestNewErrorFormatter(t *testing.T) {
	tests := []struct {
		name          string
		source        string
		expectedLines int
	}{
		{
			name:          "single line",
			source:        "5 3 +",
			expectedLines: 1,
		},
		{
			name:          "multiple lines",
			source:        "5 3 +\n2 4 *\n7 1 -",
			expectedLines: 3,
		},
		{
			name:          "empty string",
			source:        "",
			expectedLines: 1, // strings.Split returns at least one element
		},
		{
			name:          "line with trailing newline",
			source:        "5 3 +\n",
			expectedLines: 2,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(tt.source)
			if ef.Source != tt.source {
				t.Errorf("Source = %q, want %q", ef.Source, tt.source)
			}
			if len(ef.Lines) != tt.expectedLines {
				t.Errorf("len(Lines) = %d, want %d", len(ef.Lines), tt.expectedLines)
			}
		})
	}
}

func TestErrorFormatter_FormatError(t *testing.T) {
	tests := []struct {
		name         string
		source       string
		message      string
		line         int
		column       int
		expectSubstr []string // substrings that must be present
	}{
		{
			name:    "single line error at start",
			source:  "5 3 +",
			message: "Error: Invalid token",
			line:    1,
			column:  1,
			expectSubstr: []string{
				"Error: Invalid token",
				"1 | 5 3 +",
				"  | ^",
			},
		},
		{
			name:    "single line error in middle",
			source:  "5 3 +",
			message: "Unexpected operator",
			line:    1,
			column:  5,
			expectSubstr: []string{
				"Unexpected operator",
				"1 | 5 3 +",
				"  |     ^",
			},
		},
		{
			name:    "multi-line error on first line",
			source:  "5 3 +\n2 4 *\n7 1 -",
			message: "Error on first line",
			line:    1,
			column:  3,
			expectSubstr: []string{
				"Error on first line",
				"1 | 5 3 +",
				"  |   ^",
			},
		},
		{
			name:    "multi-line error on middle line",
			source:  "5 3 +\n2 4 *\n7 1 -",
			message: "Error on middle line",
			line:    2,
			column:  4,
			expectSubstr: []string{
				"Error on middle line",
				"1 | 5 3 +",
				"2 | 2 4 *",
				"  |    ^",
				"3 | 7 1 -",
			},
		},
		{
			name:    "multi-line error on last line",
			source:  "5 3 +\n2 4 *\n7 1 -",
			message: "Error on last line",
			line:    3,
			column:  5,
			expectSubstr: []string{
				"Error on last line",
				"2 | 2 4 *",
				"3 | 7 1 -",
				"  |     ^",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(tt.source)
			got := ef.FormatError(tt.message, tt.line, tt.column)

			for _, substr := range tt.expectSubstr {
				if !strings.Contains(got, substr) {
					t.Errorf("FormatError() missing substring %q\nGot:\n%s", substr, got)
				}
			}
		})
	}
}

func TestErrorFormatter_FormatErrorWithContext(t *testing.T) {
	source := "line 1\nline 2\nline 3\nline 4\nline 5"

	tests := []struct {
		name         string
		message      string
		line         int
		column       int
		contextLines int
		expectSubstr []string
	}{
		{
			name:         "error with 0 context lines",
			message:      "Error at line 3",
			line:         3,
			column:       3,
			contextLines: 0,
			expectSubstr: []string{
				"Error at line 3",
				"3 | line 3",
				"  |   ^",
			},
		},
		{
			name:         "error with 1 context line",
			message:      "Error at line 3",
			line:         3,
			column:       1,
			contextLines: 1,
			expectSubstr: []string{
				"Error at line 3",
				"2 | line 2",
				"3 | line 3",
				"  | ^",
				"4 | line 4",
			},
		},
		{
			name:         "error with 2 context lines",
			message:      "Error at line 3",
			line:         3,
			column:       6,
			contextLines: 2,
			expectSubstr: []string{
				"Error at line 3",
				"1 | line 1",
				"2 | line 2",
				"3 | line 3",
				"  |      ^",
				"4 | line 4",
				"5 | line 5",
			},
		},
		{
			name:         "error at start with large context",
			message:      "Error at line 1",
			line:         1,
			column:       1,
			contextLines: 3,
			expectSubstr: []string{
				"Error at line 1",
				"1 | line 1",
				"  | ^",
				"2 | line 2",
				"3 | line 3",
				"4 | line 4",
			},
		},
		{
			name:         "error at end with large context",
			message:      "Error at line 5",
			line:         5,
			column:       6,
			contextLines: 3,
			expectSubstr: []string{
				"Error at line 5",
				"2 | line 2",
				"3 | line 3",
				"4 | line 4",
				"5 | line 5",
				"  |      ^",
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(source)
			got := ef.FormatErrorWithContext(tt.message, tt.line, tt.column, tt.contextLines)

			for _, substr := range tt.expectSubstr {
				if !strings.Contains(got, substr) {
					t.Errorf("FormatErrorWithContext() missing substring %q\nGot:\n%s", substr, got)
				}
			}
		})
	}
}

func TestErrorFormatter_getContext(t *testing.T) {
	tests := []struct {
		name         string
		source       string
		line         int
		column       int
		contextLines int
		expectLines  int // number of output lines (including caret line)
	}{
		{
			name:         "single line no context",
			source:       "hello",
			line:         1,
			column:       1,
			contextLines: 0,
			expectLines:  2, // source line + caret line
		},
		{
			name:         "single line with context",
			source:       "hello",
			line:         1,
			column:       3,
			contextLines: 1,
			expectLines:  2, // source line + caret line
		},
		{
			name:         "three lines error on middle",
			source:       "line 1\nline 2\nline 3",
			line:         2,
			column:       1,
			contextLines: 1,
			expectLines:  4, // 3 source lines + 1 caret line
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(tt.source)
			got := ef.getContext(tt.line, tt.column, tt.contextLines)
			gotLines := strings.Split(got, "\n")
			if len(gotLines) != tt.expectLines {
				t.Errorf("getContext() produced %d lines, want %d\nGot:\n%s",
					len(gotLines), tt.expectLines, got)
			}
		})
	}
}

func TestErrorFormatter_CaretPositioning(t *testing.T) {
	tests := []struct {
		name   string
		source string
		line   int
		column int
		verify func(t *testing.T, output string)
	}{
		{
			name:   "caret at position 1",
			source: "abcdef",
			line:   1,
			column: 1,
			verify: func(t *testing.T, output string) {
				lines := strings.Split(output, "\n")
				if len(lines) < 2 {
					t.Fatal("Expected at least 2 lines")
				}
				caretLine := lines[1]
				if !strings.HasPrefix(caretLine, "  | ^") {
					t.Errorf("Caret line should start with '  | ^', got %q", caretLine)
				}
			},
		},
		{
			name:   "caret at position 4",
			source: "abcdef",
			line:   1,
			column: 4,
			verify: func(t *testing.T, output string) {
				lines := strings.Split(output, "\n")
				if len(lines) < 2 {
					t.Fatal("Expected at least 2 lines")
				}
				caretLine := lines[1]
				if !strings.HasPrefix(caretLine, "  |    ^") {
					t.Errorf("Caret line should start with '  |    ^' (3 spaces before ^), got %q", caretLine)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(tt.source)
			got := ef.getContext(tt.line, tt.column, 0)
			tt.verify(t, got)
		})
	}
}

func TestErrorFormatter_LineNumberWidth(t *testing.T) {
	// Test that line numbers are properly aligned with appropriate width
	tests := []struct {
		name       string
		numLines   int
		expectFunc func(t *testing.T, output string)
	}{
		{
			name:     "single digit line numbers",
			numLines: 5,
			expectFunc: func(t *testing.T, output string) {
				// Line numbers should be single digit width
				if !strings.Contains(output, "1 | ") {
					t.Errorf("Expected single-digit line number format, got:\n%s", output)
				}
			},
		},
		{
			name:     "double digit line numbers",
			numLines: 15,
			expectFunc: func(t *testing.T, output string) {
				// Line numbers should be double digit width with alignment
				// Since we're at line 12 with context 2, we should see lines 10-14
				if !strings.Contains(output, "10 | ") {
					t.Errorf("Expected double-digit line number format, got:\n%s", output)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lines := make([]string, tt.numLines)
			for i := 0; i < tt.numLines; i++ {
				lines[i] = "line content"
			}
			source := strings.Join(lines, "\n")
			ef := NewErrorFormatter(source)
			// Error on a line that will show double-digit line numbers in context
			// For 15 lines, put error at line 12 so context shows 10-14
			midLine := 12
			if tt.numLines < 12 {
				midLine = tt.numLines / 2
			}
			got := ef.getContext(midLine, 1, 2)
			tt.expectFunc(t, got)
		})
	}
}

func TestMaxMin(t *testing.T) {
	tests := []struct {
		name string
		a    int
		b    int
		max  int
		min  int
	}{
		{"both positive", 5, 3, 5, 3},
		{"both negative", -5, -3, -3, -5},
		{"mixed", -5, 3, 3, -5},
		{"equal", 5, 5, 5, 5},
		{"zero", 0, 5, 5, 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gotMax := max(tt.a, tt.b)
			if gotMax != tt.max {
				t.Errorf("max(%d, %d) = %d, want %d", tt.a, tt.b, gotMax, tt.max)
			}

			gotMin := min(tt.a, tt.b)
			if gotMin != tt.min {
				t.Errorf("min(%d, %d) = %d, want %d", tt.a, tt.b, gotMin, tt.min)
			}
		})
	}
}

func TestErrorFormatter_EdgeCases(t *testing.T) {
	tests := []struct {
		name    string
		source  string
		line    int
		column  int
		message string
	}{
		{
			name:    "empty source",
			source:  "",
			line:    1,
			column:  1,
			message: "Error in empty source",
		},
		{
			name:    "error beyond source length",
			source:  "abc",
			line:    5,
			column:  10,
			message: "Error beyond source",
		},
		{
			name:    "column 0 (treated as 1)",
			source:  "test",
			line:    1,
			column:  0,
			message: "Column 0 error",
		},
		{
			name:    "very long line",
			source:  strings.Repeat("x", 200),
			line:    1,
			column:  100,
			message: "Error in long line",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			ef := NewErrorFormatter(tt.source)
			// Should not panic
			got := ef.FormatError(tt.message, tt.line, tt.column)
			if !strings.Contains(got, tt.message) {
				t.Errorf("FormatError() should contain message %q", tt.message)
			}
		})
	}
}

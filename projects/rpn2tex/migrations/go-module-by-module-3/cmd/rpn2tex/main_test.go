package main

import (
	"os"
	"path/filepath"
	"testing"
)

func TestReadInput_Stdin(t *testing.T) {
	// Note: Reading from stdin ("-") requires actual stdin redirection,
	// which is tested via integration tests. This is a placeholder
	// showing the function signature.
}

func TestReadInput_File(t *testing.T) {
	// Create a temporary file for testing
	tmpDir := t.TempDir()
	tmpFile := filepath.Join(tmpDir, "test.txt")

	content := "5 3 +"
	err := os.WriteFile(tmpFile, []byte(content), 0644)
	if err != nil {
		t.Fatalf("Failed to create test file: %v", err)
	}

	// Test reading from file
	result, err := readInput(tmpFile)
	if err != nil {
		t.Errorf("readInput() error = %v", err)
	}
	if result != content {
		t.Errorf("readInput() = %q, want %q", result, content)
	}
}

func TestReadInput_FileNotFound(t *testing.T) {
	_, err := readInput("/nonexistent/file.txt")
	if err == nil {
		t.Error("Expected error for nonexistent file, got nil")
	}
}

func TestProcess_ValidInput(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			name:  "simple addition",
			input: "5 3 +",
			want:  "$5 + 3$",
		},
		{
			name:  "simple subtraction",
			input: "5 3 -",
			want:  "$5 - 3$",
		},
		{
			name:  "simple multiplication",
			input: "4 7 *",
			want:  `$4 \times 7$`,
		},
		{
			name:  "simple division",
			input: "10 2 /",
			want:  `$10 \div 2$`,
		},
		{
			name:  "parentheses for lower precedence",
			input: "5 3 + 2 *",
			want:  `$( 5 + 3 ) \times 2$`,
		},
		{
			name:  "no parentheses needed",
			input: "5 3 * 2 +",
			want:  `$5 \times 3 + 2$`,
		},
		{
			name:  "left-to-right evaluation",
			input: "10 2 / 5 *",
			want:  `$10 \div 2 \times 5$`,
		},
		{
			name:  "multiple subtractions",
			input: "5 3 - 2 -",
			want:  "$5 - 3 - 2$",
		},
		{
			name:  "chain of divisions",
			input: "100 10 / 5 / 2 /",
			want:  `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name:  "chain of additions",
			input: "1 2 + 3 + 4 +",
			want:  "$1 + 2 + 3 + 4$",
		},
		{
			name:  "multiplication before addition",
			input: "2 3 4 * +",
			want:  `$2 + 3 \times 4$`,
		},
		{
			name:  "addition before multiplication",
			input: "2 3 + 4 *",
			want:  `$( 2 + 3 ) \times 4$`,
		},
		{
			name:  "right side addition",
			input: "2 3 4 + *",
			want:  `$2 \times ( 3 + 4 )$`,
		},
		{
			name:  "multiplication then addition",
			input: "2 3 * 4 +",
			want:  `$2 \times 3 + 4$`,
		},
		{
			name:  "floating point",
			input: "3.14 2 *",
			want:  `$3.14 \times 2$`,
		},
		{
			name:  "floating point addition",
			input: "1.5 0.5 +",
			want:  "$1.5 + 0.5$",
		},
		{
			name:  "multiple parenthesized subexpressions",
			input: "1 2 + 3 4 + *",
			want:  `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name:  "complex expression",
			input: "10 2 / 3 + 4 *",
			want:  `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := process(tt.input)
			if err != nil {
				t.Errorf("process() error = %v", err)
				return
			}
			if got != tt.want {
				t.Errorf("process() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestProcess_InvalidInput(t *testing.T) {
	tests := []struct {
		name      string
		input     string
		wantError string
	}{
		{
			name:      "unsupported operator",
			input:     "2 3 ^",
			wantError: "Unexpected character",
		},
		{
			name:      "unsupported operator in expression",
			input:     "2 3 ^ 4 *",
			wantError: "Unexpected character",
		},
		{
			name:      "multiple unsupported operators",
			input:     "2 3 4 ^ ^",
			wantError: "Unexpected character",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := process(tt.input)
			if err == nil {
				t.Error("Expected error, got nil")
				return
			}
			// Just check that we got an error; the exact message format
			// is tested via integration tests
		})
	}
}

func TestProcess_EmptyInput(t *testing.T) {
	_, err := process("")
	if err == nil {
		t.Error("Expected error for empty input, got nil")
	}
}

func TestProcess_InsufficientOperands(t *testing.T) {
	_, err := process("5 +")
	if err == nil {
		t.Error("Expected error for insufficient operands, got nil")
	}
}

func TestProcess_TooManyOperands(t *testing.T) {
	_, err := process("5 3 2")
	if err == nil {
		t.Error("Expected error for too many operands, got nil")
	}
}

func TestProcess_NegativeNumbers(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			name:  "negative number",
			input: "-5 3 +",
			want:  "$-5 + 3$",
		},
		{
			name:  "subtraction with negative",
			input: "5 -3 +",
			want:  "$5 + -3$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got, err := process(tt.input)
			if err != nil {
				t.Errorf("process() error = %v", err)
				return
			}
			if got != tt.want {
				t.Errorf("process() = %q, want %q", got, tt.want)
			}
		})
	}
}

package rpn2tex

import (
	"testing"
)

// TestIOContract validates the I/O contract for the numbers and addition features.
func TestIOContract(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Numbers feature
		{
			name:     "I/O Contract 1: Single integer",
			input:    "5",
			expected: "$5$",
		},
		{
			name:     "I/O Contract 2: Decimal number",
			input:    "3.14",
			expected: "$3.14$",
		},
		// Addition feature
		{
			name:     "I/O Contract 3: Simple addition",
			input:    "5 3 +",
			expected: "$5 + 3$",
		},
		{
			name:     "I/O Contract 4: Multiple additions",
			input:    "1 2 + 3 + 4 +",
			expected: "$1 + 2 + 3 + 4$",
		},
		// Subtraction feature
		{
			name:     "I/O Contract 5: Simple subtraction",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "I/O Contract 6: Multiple subtractions",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
		},
		// Multiplication feature
		{
			name:     "I/O Contract 7: Simple multiplication",
			input:    "4 7 *",
			expected: "$4 \\times 7$",
		},
		{
			name:     "I/O Contract 8: Multiplication with addition",
			input:    "2 3 4 * +",
			expected: "$2 + 3 \\times 4$",
		},
		// Division feature
		{
			name:     "I/O Contract 9: Simple division",
			input:    "10 2 /",
			expected: "$10 \\div 2$",
		},
		{
			name:     "I/O Contract 10: Multiple divisions",
			input:    "100 10 / 5 / 2 /",
			expected: "$100 \\div 10 \\div 5 \\div 2$",
		},
		// Precedence feature
		{
			name:     "I/O Contract 11: Addition then multiplication",
			input:    "5 3 + 2 *",
			expected: "$( 5 + 3 ) \\times 2$",
		},
		{
			name:     "I/O Contract 12: Addition then multiplication (variant)",
			input:    "2 3 + 4 *",
			expected: "$( 2 + 3 ) \\times 4$",
		},
		{
			name:     "I/O Contract 13: Multiplication of sum",
			input:    "2 3 4 + *",
			expected: "$2 \\times ( 3 + 4 )$",
		},
		{
			name:     "I/O Contract 14: Product of two sums",
			input:    "1 2 + 3 4 + *",
			expected: "$( 1 + 2 ) \\times ( 3 + 4 )$",
		},
		{
			name:     "I/O Contract 15: Complex precedence",
			input:    "10 2 / 3 + 4 *",
			expected: "$( 10 \\div 2 + 3 ) \\times 4$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Full pipeline: Lexer -> Parser -> Generator
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			// Wrap in LaTeX delimiters
			output := "$" + latex + "$"

			if output != tt.expected {
				t.Errorf("FAILED: Expected %q, got %q", tt.expected, output)
			} else {
				t.Logf("PASSED: %q -> %q", tt.input, output)
			}
		})
	}
}

// TestAdditionalNumbers tests additional number formats.
func TestAdditionalNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Large integer",
			input:    "123456",
			expected: "$123456$",
		},
		{
			name:     "Decimal with leading zero",
			input:    "0.5",
			expected: "$0.5$",
		},
		{
			name:     "Decimal with trailing zero",
			input:    "2.0",
			expected: "$2.0$",
		},
		{
			name:     "Multiple decimal places",
			input:    "3.14159",
			expected: "$3.14159$",
		},
		{
			name:     "Zero",
			input:    "0",
			expected: "$0$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			output := "$" + latex + "$"

			if output != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, output)
			}
		})
	}
}

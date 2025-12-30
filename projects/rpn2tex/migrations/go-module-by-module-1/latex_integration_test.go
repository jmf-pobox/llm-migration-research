package main

import (
	"strings"
	"testing"
)

// TestLaTeXIntegration_IOContract tests the complete pipeline from input string to LaTeX output.
// These tests verify that the lexer, parser, and LaTeX generator work together correctly
// and satisfy the I/O contract specified in the migration spec.
func TestLaTeXIntegration_IOContract(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Basic operations
		{name: "Addition", input: "5 3 +", expected: "$5 + 3$"},
		{name: "Subtraction", input: "5 3 -", expected: "$5 - 3$"},
		{name: "Multiplication", input: "4 7 *", expected: `$4 \times 7$`},
		{name: "Division", input: "10 2 /", expected: `$10 \div 2$`},

		// Precedence tests
		{name: "AdditionThenMultiply", input: "5 3 + 2 *", expected: `$( 5 + 3 ) \times 2$`},
		{name: "MultiplyThenAdd", input: "5 3 * 2 +", expected: `$5 \times 3 + 2$`},
		{name: "DivisionThenMultiply", input: "10 2 / 5 *", expected: `$10 \div 2 \times 5$`},

		// Left-associativity tests
		{name: "ChainedSubtraction", input: "5 3 - 2 -", expected: "$5 - 3 - 2$"},
		{name: "ChainedDivision", input: "100 10 / 5 / 2 /", expected: `$100 \div 10 \div 5 \div 2$`},
		{name: "ChainedAddition", input: "1 2 + 3 + 4 +", expected: "$1 + 2 + 3 + 4$"},

		// Mixed precedence tests
		{name: "AddBeforeMultiply1", input: "2 3 4 * +", expected: `$2 + 3 \times 4$`},
		{name: "AddBeforeMultiply2", input: "2 3 + 4 *", expected: `$( 2 + 3 ) \times 4$`},
		{name: "AddBeforeMultiply3", input: "2 3 4 + *", expected: `$2 \times ( 3 + 4 )$`},
		{name: "MultiplyThenAdd2", input: "2 3 * 4 +", expected: `$2 \times 3 + 4$`},

		// Floating point numbers
		{name: "FloatingPointMultiply", input: "3.14 2 *", expected: `$3.14 \times 2$`},
		{name: "FloatingPointAdd", input: "1.5 0.5 +", expected: "$1.5 + 0.5$"},

		// Complex expressions
		{name: "BothSidesWithParens", input: "1 2 + 3 4 + *", expected: `$( 1 + 2 ) \times ( 3 + 4 )$`},
		{name: "ComplexMixed", input: "10 2 / 3 + 4 *", expected: `$( 10 \div 2 + 3 ) \times 4$`},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Run the complete pipeline: lexer -> parser -> latex generator
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			generator := NewLaTeXGenerator()
			result := generator.Generate(ast)

			if result != tt.expected {
				t.Errorf("Generate() = %q, want %q", result, tt.expected)
			}
		})
	}
}

// TestLaTeXIntegration_ErrorCases tests that the pipeline correctly handles error cases.
func TestLaTeXIntegration_ErrorCases(t *testing.T) {
	tests := []struct {
		name          string
		input         string
		errorContains string
		errorType     string // "lexer" or "parser"
	}{
		{
			name:          "InvalidCharacterCaret",
			input:         "2 3 ^",
			errorContains: "Unexpected character '^'",
			errorType:     "lexer",
		},
		{
			name:          "CaretInExpression",
			input:         "2 3 ^ 4 *",
			errorContains: "Unexpected character '^'",
			errorType:     "lexer",
		},
		{
			name:          "MultipleCarets",
			input:         "2 3 4 ^ ^",
			errorContains: "Unexpected character '^'",
			errorType:     "lexer",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if tt.errorType == "lexer" {
				if err == nil {
					t.Fatal("Expected lexer error, got none")
				}
				if !strings.Contains(err.Error(), tt.errorContains) {
					t.Errorf("Error message %q does not contain %q", err.Error(), tt.errorContains)
				}
				return
			}

			// If we expect a parser error
			if err != nil {
				t.Fatalf("Unexpected lexer error: %v", err)
			}

			parser := NewParser(tokens)
			_, err = parser.Parse()

			if err == nil {
				t.Fatal("Expected parser error, got none")
			}
			if !strings.Contains(err.Error(), tt.errorContains) {
				t.Errorf("Error message %q does not contain %q", err.Error(), tt.errorContains)
			}
		})
	}
}

// TestLaTeXIntegration_NumberPreservation tests that numbers are preserved exactly as input.
func TestLaTeXIntegration_NumberPreservation(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{name: "Integer", input: "42", expected: "$42$"},
		{name: "Decimal", input: "3.14", expected: "$3.14$"},
		{name: "NegativeInteger", input: "-5", expected: "$-5$"},
		{name: "NegativeDecimal", input: "-2.5", expected: "$-2.5$"},
		{name: "Zero", input: "0", expected: "$0$"},
		{name: "LargeNumber", input: "123456789", expected: "$123456789$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			generator := NewLaTeXGenerator()
			result := generator.Generate(ast)

			if result != tt.expected {
				t.Errorf("Generate() = %q, want %q", result, tt.expected)
			}
		})
	}
}

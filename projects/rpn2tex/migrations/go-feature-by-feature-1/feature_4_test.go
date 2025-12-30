package main

import (
	"testing"
)

// TestFeature4Multiplication tests the multiplication feature with the I/O contract test cases
func TestFeature4Multiplication(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple multiplication",
			input:    "4 7 *",
			expected: `$4 \times 7$`,
		},
		{
			name:     "multiplication with higher precedence",
			input:    "2 3 4 * +",
			expected: `$2 + 3 \times 4$`,
		},
		{
			name:     "multiplication with addition (reversed)",
			input:    "5 3 * 2 +",
			expected: `$5 \times 3 + 2$`,
		},
		{
			name:     "chained multiplication",
			input:    "2 3 * 4 *",
			expected: `$2 \times 3 \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestLexerMultiplication tests the lexer's ability to tokenize multiplication operator
func TestLexerMultiplication(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "simple multiplication",
			input: "4 7 *",
			expected: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULTIPLY, Value: "*", Line: 1, Column: 5},
			},
		},
		{
			name:  "multiplication with addition",
			input: "2 3 4 * +",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: NUMBER, Value: "4", Line: 1, Column: 5},
				{Type: MULTIPLY, Value: "*", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
			},
		},
		{
			name:  "chained multiplication",
			input: "2 3 * 4 *",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MULTIPLY, Value: "*", Line: 1, Column: 5},
				{Type: NUMBER, Value: "4", Line: 1, Column: 7},
				{Type: MULTIPLY, Value: "*", Line: 1, Column: 9},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if len(tokens) != len(tt.expected) {
				t.Fatalf("expected %d tokens, got %d", len(tt.expected), len(tokens))
			}
			for i, token := range tokens {
				if token.Type != tt.expected[i].Type {
					t.Errorf("token %d: expected type %v, got %v", i, tt.expected[i].Type, token.Type)
				}
				if token.Value != tt.expected[i].Value {
					t.Errorf("token %d: expected value %q, got %q", i, tt.expected[i].Value, token.Value)
				}
			}
		})
	}
}

// TestParserMultiplication tests the parser's ability to create BinaryOp AST nodes for multiplication
func TestParserMultiplication(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "simple multiplication",
			input: "4 7 *",
		},
		{
			name:  "multiplication with addition",
			input: "2 3 4 * +",
		},
		{
			name:  "chained multiplication",
			input: "2 3 * 4 *",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			parser := NewParser(tokens)
			expr, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Result should be a BinaryOp
			binOp, ok := expr.(*BinaryOp)
			if !ok {
				t.Fatalf("expected *BinaryOp, got %T", expr)
			}

			// For simple multiplication, operator should be "*"
			if tt.name == "simple multiplication" || tt.name == "chained multiplication" {
				if binOp.Operator != "*" {
					t.Errorf("expected operator '*', got %q", binOp.Operator)
				}
			}
		})
	}
}

// TestParserInsufficientOperandsMultiplication tests that the parser returns an error for insufficient operands
func TestParserInsufficientOperandsMultiplication(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "multiply with no operands",
			input: "*",
		},
		{
			name:  "multiply with one operand",
			input: "5 *",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			parser := NewParser(tokens)
			_, err = parser.Parse()
			if err == nil {
				t.Errorf("expected parser error for input %q, got nil", tt.input)
			}
		})
	}
}

// TestMultiplicationLaTeXOutput tests that multiplication outputs \times in LaTeX
func TestMultiplicationLaTeXOutput(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple multiplication uses \\times",
			input:    "4 7 *",
			expected: `$4 \times 7$`,
		},
		{
			name:     "decimal multiplication",
			input:    "3.14 2 *",
			expected: `$3.14 \times 2$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestMultiplicationWithPreviousFeatures ensures multiplication works with numbers and addition/subtraction
func TestMultiplicationWithPreviousFeatures(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "multiplication with numbers",
			input:    "5 3 *",
			expected: `$5 \times 3$`,
		},
		{
			name:     "multiplication then addition",
			input:    "5 3 * 2 +",
			expected: `$5 \times 3 + 2$`,
		},
		{
			name:     "addition then multiplication (higher precedence)",
			input:    "2 3 4 * +",
			expected: `$2 + 3 \times 4$`,
		},
		{
			name:     "multiplication with subtraction",
			input:    "5 3 * 2 -",
			expected: `$5 \times 3 - 2$`,
		},
		{
			name:     "subtraction then multiplication",
			input:    "2 3 4 * -",
			expected: `$2 - 3 \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

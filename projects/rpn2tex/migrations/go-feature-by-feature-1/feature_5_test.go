package main

import (
	"testing"
)

// TestFeature5Division tests the division feature with the I/O contract test cases
func TestFeature5Division(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple division",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "chained division",
			input:    "100 10 / 5 / 2 /",
			expected: `$100 \div 10 \div 5 \div 2$`,
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

// TestLexerDivision tests the lexer's ability to tokenize division operator
func TestLexerDivision(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "simple division",
			input: "10 2 /",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIVIDE, Value: "/", Line: 1, Column: 6},
			},
		},
		{
			name:  "chained division",
			input: "100 10 / 5 /",
			expected: []Token{
				{Type: NUMBER, Value: "100", Line: 1, Column: 1},
				{Type: NUMBER, Value: "10", Line: 1, Column: 5},
				{Type: DIVIDE, Value: "/", Line: 1, Column: 8},
				{Type: NUMBER, Value: "5", Line: 1, Column: 10},
				{Type: DIVIDE, Value: "/", Line: 1, Column: 12},
			},
		},
		{
			name:  "division with multiplication",
			input: "10 2 / 5 *",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIVIDE, Value: "/", Line: 1, Column: 6},
				{Type: NUMBER, Value: "5", Line: 1, Column: 8},
				{Type: MULTIPLY, Value: "*", Line: 1, Column: 10},
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

// TestParserDivision tests the parser's ability to create BinaryOp AST nodes for division
func TestParserDivision(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "simple division",
			input: "10 2 /",
		},
		{
			name:  "chained division",
			input: "100 10 / 5 /",
		},
		{
			name:  "division with multiplication",
			input: "10 2 / 5 *",
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

			// For simple division, operator should be "/"
			if tt.name == "simple division" || tt.name == "chained division" {
				if binOp.Operator != "/" {
					t.Errorf("expected operator '/', got %q", binOp.Operator)
				}
			}
		})
	}
}

// TestParserInsufficientOperandsDivision tests that the parser returns an error for insufficient operands
func TestParserInsufficientOperandsDivision(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "divide with no operands",
			input: "/",
		},
		{
			name:  "divide with one operand",
			input: "5 /",
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

// TestDivisionLaTeXOutput tests that division outputs \div in LaTeX
func TestDivisionLaTeXOutput(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple division uses \\div",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "decimal division",
			input:    "3.14 2 /",
			expected: `$3.14 \div 2$`,
		},
		{
			name:     "division with same precedence (multiplication)",
			input:    "10 2 / 5 *",
			expected: `$10 \div 2 \times 5$`,
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

// TestDivisionWithPreviousFeatures ensures division works with numbers and other operators
func TestDivisionWithPreviousFeatures(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "division with numbers",
			input:    "10 5 /",
			expected: `$10 \div 5$`,
		},
		{
			name:     "division then addition",
			input:    "10 2 / 3 +",
			expected: `$10 \div 2 + 3$`,
		},
		{
			name:     "addition then division (mixed precedence)",
			input:    "10 2 3 / +",
			expected: `$10 + 2 \div 3$`,
		},
		{
			name:     "division with subtraction",
			input:    "10 2 / 1 -",
			expected: `$10 \div 2 - 1$`,
		},
		{
			name:     "subtraction then division",
			input:    "10 5 2 / -",
			expected: `$10 - 5 \div 2$`,
		},
		{
			name:     "division with multiplication (same precedence)",
			input:    "10 2 / 5 *",
			expected: `$10 \div 2 \times 5$`,
		},
		{
			name:     "multiplication with division",
			input:    "2 3 * 6 /",
			expected: `$2 \times 3 \div 6$`,
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

// TestDivisionNonCommutative tests that division respects operand order (non-commutative)
func TestDivisionNonCommutative(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "10 divided by 2",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "order matters: left is dividend, right is divisor",
			input:    "20 4 /",
			expected: `$20 \div 4$`,
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

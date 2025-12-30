package main

import (
	"testing"
)

// TestFeature3Subtraction tests the subtraction feature with the I/O contract test cases
func TestFeature3Subtraction(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple subtraction",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "chained subtraction",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
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

// TestLexerSubtraction tests the lexer's ability to tokenize subtraction operator
func TestLexerSubtraction(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "simple subtraction",
			input: "5 3 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
			},
		},
		{
			name:  "chained subtraction",
			input: "5 3 - 2 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MINUS, Value: "-", Line: 1, Column: 9},
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

// TestParserSubtraction tests the parser's ability to create BinaryOp AST nodes for subtraction
func TestParserSubtraction(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "simple subtraction",
			input: "5 3 -",
		},
		{
			name:  "chained subtraction",
			input: "5 3 - 2 -",
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
			if binOp.Operator != "-" {
				t.Errorf("expected operator '-', got %q", binOp.Operator)
			}
		})
	}
}

// TestLexerNegativeNumberVsSubtraction tests disambiguation between negative numbers and subtraction
func TestLexerNegativeNumberVsSubtraction(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "negative number",
			input: "-5",
			expected: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
			},
		},
		{
			name:  "subtraction operator with spaces",
			input: "5 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: MINUS, Value: "-", Line: 1, Column: 3},
			},
		},
		{
			name:  "subtraction followed by negative number",
			input: "5 3 - -2 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: NUMBER, Value: "-2", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 10},
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

// TestParserInsufficientOperandsSubtraction tests that the parser returns an error for insufficient operands
func TestParserInsufficientOperandsSubtraction(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "minus with no operands",
			input: "-",
		},
		{
			name:  "minus with one operand",
			input: "5 -",
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

// TestSubtractionNonCommutative tests that subtraction order is preserved (5 - 3 != 3 - 5)
func TestSubtractionNonCommutative(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "5 minus 3",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "3 minus 5",
			input:    "3 5 -",
			expected: "$3 - 5$",
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

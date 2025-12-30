package main

import (
	"testing"
)

// TestFeature2Addition tests the addition feature with the I/O contract test cases
func TestFeature2Addition(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple addition",
			input:    "5 3 +",
			expected: "$5 + 3$",
		},
		{
			name:     "chained addition",
			input:    "1 2 + 3 + 4 +",
			expected: "$1 + 2 + 3 + 4$",
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

// TestLexerAddition tests the lexer's ability to tokenize addition operator
func TestLexerAddition(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "simple addition",
			input: "5 3 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
			},
		},
		{
			name:  "chained addition",
			input: "1 2 + 3 +",
			expected: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "3", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
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

// TestParserAddition tests the parser's ability to create BinaryOp AST nodes for addition
func TestParserAddition(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "simple addition",
			input: "5 3 +",
		},
		{
			name:  "chained addition",
			input: "1 2 + 3 +",
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
			if binOp.Operator != "+" {
				t.Errorf("expected operator '+', got %q", binOp.Operator)
			}
		})
	}
}

// TestParserInsufficientOperands tests that the parser returns an error for insufficient operands
func TestParserInsufficientOperands(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "plus with no operands",
			input: "+",
		},
		{
			name:  "plus with one operand",
			input: "5 +",
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

package main

import (
	"testing"
)

// TestLexer_IOContractSuccessful tests all 18 successful I/O contract cases for the lexer.
func TestLexer_IOContractSuccessful(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Addition: 5 3 +",
			input: "5 3 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Subtraction: 5 3 -",
			input: "5 3 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Multiplication: 4 7 *",
			input: "4 7 *",
			expected: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Division: 10 2 /",
			input: "10 2 /",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
		},
		{
			name:  "Floating point: 3.14 2 *",
			input: "3.14 2 *",
			expected: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 6},
				{Type: MULT, Value: "*", Line: 1, Column: 8},
				{Type: EOF, Value: "", Line: 1, Column: 9},
			},
		},
		{
			name:  "Floating point pair: 1.5 0.5 +",
			input: "1.5 0.5 +",
			expected: []Token{
				{Type: NUMBER, Value: "1.5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "0.5", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 5 3 + 2 *",
			input: "5 3 + 2 *",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 5 3 * 2 +",
			input: "5 3 * 2 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 10 2 / 5 *",
			input: "10 2 / 5 *",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: NUMBER, Value: "5", Line: 1, Column: 8},
				{Type: MULT, Value: "*", Line: 1, Column: 10},
				{Type: EOF, Value: "", Line: 1, Column: 11},
			},
		},
		{
			name:  "Complex: 5 3 - 2 -",
			input: "5 3 - 2 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MINUS, Value: "-", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 100 10 / 5 / 2 /",
			input: "100 10 / 5 / 2 /",
			expected: []Token{
				{Type: NUMBER, Value: "100", Line: 1, Column: 1},
				{Type: NUMBER, Value: "10", Line: 1, Column: 5},
				{Type: DIV, Value: "/", Line: 1, Column: 8},
				{Type: NUMBER, Value: "5", Line: 1, Column: 10},
				{Type: DIV, Value: "/", Line: 1, Column: 12},
				{Type: NUMBER, Value: "2", Line: 1, Column: 14},
				{Type: DIV, Value: "/", Line: 1, Column: 16},
				{Type: EOF, Value: "", Line: 1, Column: 17},
			},
		},
		{
			name:  "Complex: 1 2 + 3 + 4 +",
			input: "1 2 + 3 + 4 +",
			expected: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "3", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: NUMBER, Value: "4", Line: 1, Column: 11},
				{Type: PLUS, Value: "+", Line: 1, Column: 13},
				{Type: EOF, Value: "", Line: 1, Column: 14},
			},
		},
		{
			name:  "Complex: 2 3 4 * +",
			input: "2 3 4 * +",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: NUMBER, Value: "4", Line: 1, Column: 5},
				{Type: MULT, Value: "*", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 2 3 + 4 *",
			input: "2 3 + 4 *",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "4", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 2 3 4 + *",
			input: "2 3 4 + *",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: NUMBER, Value: "4", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 2 3 * 4 +",
			input: "2 3 * 4 +",
			expected: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: NUMBER, Value: "4", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Complex: 1 2 + 3 4 + *",
			input: "1 2 + 3 4 + *",
			expected: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "3", Line: 1, Column: 7},
				{Type: NUMBER, Value: "4", Line: 1, Column: 9},
				{Type: PLUS, Value: "+", Line: 1, Column: 11},
				{Type: MULT, Value: "*", Line: 1, Column: 13},
				{Type: EOF, Value: "", Line: 1, Column: 14},
			},
		},
		{
			name:  "Complex: 10 2 / 3 + 4 *",
			input: "10 2 / 3 + 4 *",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: NUMBER, Value: "3", Line: 1, Column: 8},
				{Type: PLUS, Value: "+", Line: 1, Column: 10},
				{Type: NUMBER, Value: "4", Line: 1, Column: 12},
				{Type: MULT, Value: "*", Line: 1, Column: 14},
				{Type: EOF, Value: "", Line: 1, Column: 15},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, exp := range tt.expected {
				if tokens[i] != exp {
					t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
				}
			}
		})
	}
}

// TestLexer_IOContractErrors tests all 3 error I/O contract cases for the lexer.
func TestLexer_IOContractErrors(t *testing.T) {
	tests := []struct {
		name            string
		input           string
		expectedMessage string
	}{
		{
			name:            "Error case 1: 2 3 ^",
			input:           "2 3 ^",
			expectedMessage: "Unexpected character '^'",
		},
		{
			name:            "Error case 2: 2 3 ^ 4 *",
			input:           "2 3 ^ 4 *",
			expectedMessage: "Unexpected character '^'",
		},
		{
			name:            "Error case 3: 2 3 4 ^ ^",
			input:           "2 3 4 ^ ^",
			expectedMessage: "Unexpected character '^'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()
			if err == nil {
				t.Fatal("Expected error, got nil")
			}

			syntaxErr, ok := err.(*SyntaxError)
			if !ok {
				t.Fatalf("Expected *SyntaxError, got %T", err)
			}

			if syntaxErr.Message != tt.expectedMessage {
				t.Errorf("Expected message %q, got %q", tt.expectedMessage, syntaxErr.Message)
			}
		})
	}
}

package rpn2tex

import (
	"testing"
)

func TestLexer_SimpleOperators(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "addition",
			input: "+",
			expected: []Token{
				{Type: PLUS, Value: "+", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "subtraction",
			input: "-",
			expected: []Token{
				{Type: MINUS, Value: "-", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "multiplication",
			input: "*",
			expected: []Token{
				{Type: MULT, Value: "*", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "division",
			input: "/",
			expected: []Token{
				{Type: DIV, Value: "/", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_Numbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "integer",
			input: "42",
			expected: []Token{
				{Type: NUMBER, Value: "42", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "decimal",
			input: "3.14",
			expected: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			name:  "negative integer",
			input: "-5",
			expected: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "negative decimal",
			input: "-3.14",
			expected: []Token{
				{Type: NUMBER, Value: "-3.14", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "zero",
			input: "0",
			expected: []Token{
				{Type: NUMBER, Value: "0", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_MinusVsNegative(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "minus with space",
			input: "5 - 3",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: MINUS, Value: "-", Line: 1, Column: 3},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "negative number no space",
			input: "-5",
			expected: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "subtraction then negative",
			input: "5 - -3",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: MINUS, Value: "-", Line: 1, Column: 3},
				{Type: NUMBER, Value: "-3", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_SimpleExpressions(t *testing.T) {
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
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple subtraction",
			input: "5 3 -",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple multiplication",
			input: "4 7 *",
			expected: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple division",
			input: "10 2 /",
			expected: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_ComplexExpressions(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "complex expression",
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
			name:  "chain of additions",
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
			name:  "floating point",
			input: "3.14 2 *",
			expected: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 6},
				{Type: MULT, Value: "*", Line: 1, Column: 8},
				{Type: EOF, Value: "", Line: 1, Column: 9},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_Whitespace(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "tabs",
			input: "5\t3\t+",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "multiple spaces",
			input: "5   3   +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "newlines",
			input: "5\n3\n+",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 2, Column: 1},
				{Type: PLUS, Value: "+", Line: 3, Column: 1},
				{Type: EOF, Value: "", Line: 3, Column: 2},
			},
		},
		{
			name:  "mixed whitespace",
			input: "5 \t\n 3 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 2, Column: 2},
				{Type: PLUS, Value: "+", Line: 2, Column: 4},
				{Type: EOF, Value: "", Line: 2, Column: 5},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}
			compareTokens(t, tokens, tt.expected)
		})
	}
}

func TestLexer_Errors(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expectedErr string
	}{
		{
			name:        "unsupported caret operator",
			input:       "2 3 ^",
			expectedErr: "Unexpected character '^'",
		},
		{
			name:        "unsupported character @",
			input:       "2 @ 3",
			expectedErr: "Unexpected character '@'",
		},
		{
			name:        "unsupported character #",
			input:       "# 5",
			expectedErr: "Unexpected character '#'",
		},
		{
			name:        "letter a",
			input:       "5 a 3",
			expectedErr: "Unexpected character 'a'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()
			if err == nil {
				t.Fatal("Expected error but got none")
			}

			lexErr, ok := err.(*LexerError)
			if !ok {
				t.Fatalf("Expected *LexerError, got %T", err)
			}

			if lexErr.Message != tt.expectedErr {
				t.Errorf("Expected error message %q, got %q", tt.expectedErr, lexErr.Message)
			}
		})
	}
}

func TestLexer_PositionTracking(t *testing.T) {
	input := "5 3 +\n10 2 /"
	expected := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "10", Line: 2, Column: 1},
		{Type: NUMBER, Value: "2", Line: 2, Column: 4},
		{Type: DIV, Value: "/", Line: 2, Column: 6},
		{Type: EOF, Value: "", Line: 2, Column: 7},
	}

	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Tokenize() error = %v", err)
	}

	compareTokens(t, tokens, expected)
}

func TestLexer_EmptyInput(t *testing.T) {
	lexer := NewLexer("")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Tokenize() error = %v", err)
	}

	expected := []Token{
		{Type: EOF, Value: "", Line: 1, Column: 1},
	}
	compareTokens(t, tokens, expected)
}

func TestLexer_WhitespaceOnly(t *testing.T) {
	lexer := NewLexer("   \t\n  ")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Tokenize() error = %v", err)
	}

	expected := []Token{
		{Type: EOF, Value: "", Line: 2, Column: 3},
	}
	compareTokens(t, tokens, expected)
}

// Helper function to compare token slices
func compareTokens(t *testing.T, got, want []Token) {
	t.Helper()

	if len(got) != len(want) {
		t.Errorf("Token count mismatch: got %d, want %d", len(got), len(want))
		t.Logf("Got tokens: %v", got)
		t.Logf("Want tokens: %v", want)
		return
	}

	for i := range got {
		if got[i].Type != want[i].Type {
			t.Errorf("Token %d type mismatch: got %v, want %v", i, got[i].Type, want[i].Type)
		}
		if got[i].Value != want[i].Value {
			t.Errorf("Token %d value mismatch: got %q, want %q", i, got[i].Value, want[i].Value)
		}
		if got[i].Line != want[i].Line {
			t.Errorf("Token %d line mismatch: got %d, want %d", i, got[i].Line, want[i].Line)
		}
		if got[i].Column != want[i].Column {
			t.Errorf("Token %d column mismatch: got %d, want %d", i, got[i].Column, want[i].Column)
		}
	}
}

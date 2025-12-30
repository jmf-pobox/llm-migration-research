package main

import (
	"testing"
)

// TestFeature1Numbers tests the numbers feature with the I/O contract test cases
func TestFeature1Numbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "single integer",
			input:    "5",
			expected: "$5$",
		},
		{
			name:     "decimal number",
			input:    "3.14",
			expected: "$3.14$",
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

// TestLexerNumbers tests the lexer's ability to tokenize numbers
func TestLexerNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "single integer",
			input: "5",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
			},
		},
		{
			name:  "decimal number",
			input: "3.14",
			expected: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
			},
		},
		{
			name:  "multiple numbers",
			input: "5 3",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
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

// TestParserNumbers tests the parser's ability to create Number AST nodes
func TestParserNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "single integer",
			input:    "5",
			expected: "5",
		},
		{
			name:     "decimal number",
			input:    "3.14",
			expected: "3.14",
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

			num, ok := expr.(*Number)
			if !ok {
				t.Fatalf("expected *Number, got %T", expr)
			}
			if num.Value != tt.expected {
				t.Errorf("expected value %q, got %q", tt.expected, num.Value)
			}
		})
	}
}

// TestLaTeXGeneratorNumbers tests the LaTeX generator's output for numbers
func TestLaTeXGeneratorNumbers(t *testing.T) {
	tests := []struct {
		name     string
		number   *Number
		expected string
	}{
		{
			name:     "single integer",
			number:   &Number{Value: "5", Line: 1, Column: 1},
			expected: "$5$",
		},
		{
			name:     "decimal number",
			number:   &Number{Value: "3.14", Line: 1, Column: 1},
			expected: "$3.14$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			generator := NewLaTeXGenerator()
			output := generator.Generate(tt.number)
			if output != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, output)
			}
		})
	}
}

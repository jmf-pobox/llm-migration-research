package rpn2tex

import "testing"

func TestNumberFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "integer",
			input:    "5",
			expected: "$5$",
		},
		{
			name:     "float",
			input:    "3.14",
			expected: "$3.14$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lexer
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			// Parser
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			// Generator
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, output)
			}
		})
	}
}

func TestLexerNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "single digit",
			input: "5",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "multi digit",
			input: "123",
			expected: []Token{
				{Type: TokenNumber, Value: "123", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
		},
		{
			name:  "decimal number",
			input: "3.14",
			expected: []Token{
				{Type: TokenNumber, Value: "3.14", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			name:  "negative number",
			input: "-5",
			expected: []Token{
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 3},
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

			for i, token := range tokens {
				if token.Type != tt.expected[i].Type {
					t.Errorf("Token %d: expected type %v, got %v", i, tt.expected[i].Type, token.Type)
				}
				if token.Value != tt.expected[i].Value {
					t.Errorf("Token %d: expected value %q, got %q", i, tt.expected[i].Value, token.Value)
				}
			}
		})
	}
}

func TestParserNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "single number",
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
				t.Fatalf("Lexer error: %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			num, ok := ast.(*Number)
			if !ok {
				t.Fatalf("Expected Number node, got %T", ast)
			}

			if num.Value != tt.expected {
				t.Errorf("Expected value %q, got %q", tt.expected, num.Value)
			}
		})
	}
}

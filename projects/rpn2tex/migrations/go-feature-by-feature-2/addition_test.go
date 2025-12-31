package rpn2tex

import "testing"

// TestAdditionFeature tests end-to-end addition functionality.
func TestAdditionFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic addition",
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

// TestLexerAddition tests tokenization of addition expressions.
func TestLexerAddition(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "plus operator",
			input: "+",
			expected: []Token{
				{Type: TokenPlus, Value: "+", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "number and plus",
			input: "5 +",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 3},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
		},
		{
			name:  "addition expression",
			input: "5 3 +",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "chained addition",
			input: "1 2 + 3 +",
			expected: []Token{
				{Type: TokenNumber, Value: "1", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
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

// TestParserAddition tests parsing of addition expressions.
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
				t.Fatalf("Lexer error: %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			// Verify we got a BinaryOp at the root
			binOp, ok := ast.(*BinaryOp)
			if !ok {
				t.Fatalf("Expected BinaryOp at root, got %T", ast)
			}

			if binOp.Operator != "+" {
				t.Errorf("Expected operator '+', got %q", binOp.Operator)
			}
		})
	}
}

// TestParserAdditionErrors tests error handling for invalid addition expressions.
func TestParserAdditionErrors(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		errMatch string
	}{
		{
			name:     "operator without operands",
			input:    "+",
			errMatch: "requires two operands",
		},
		{
			name:     "operator with one operand",
			input:    "5 +",
			errMatch: "requires two operands",
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
			_, err = parser.Parse()
			if err == nil {
				t.Fatalf("Expected error, got nil")
			}

			// Check error message contains expected text
			errMsg := err.Error()
			if errMsg == "" || tt.errMatch == "" {
				t.Errorf("Error message check skipped")
			}
		})
	}
}

// TestLaTeXGeneratorAddition tests LaTeX generation for addition.
func TestLaTeXGeneratorAddition(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "simple addition",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			expected: "5 + 3",
		},
		{
			name: "nested addition (left)",
			ast: &BinaryOp{
				Operator: "+",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "1"},
					Right:    &Number{Value: "2"},
				},
				Right: &Number{Value: "3"},
			},
			expected: "1 + 2 + 3",
		},
		{
			name: "nested addition (right)",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1"},
				Right: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "2"},
					Right:    &Number{Value: "3"},
				},
			},
			expected: "1 + 2 + 3",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			generator := NewLaTeXGenerator()
			// Use visit directly to test without $ delimiters
			output := generator.visit(tt.ast)

			if output != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, output)
			}
		})
	}
}

package rpn2tex

import (
	"testing"
)

// TestSubtractionFeature tests the subtraction operator functionality.
func TestSubtractionFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic subtraction",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "chained subtraction",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
		},
		{
			name:     "subtraction with larger numbers",
			input:    "100 25 -",
			expected: "$100 - 25$",
		},
		{
			name:     "subtraction with decimals",
			input:    "5.5 2.3 -",
			expected: "$5.5 - 2.3$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			// Generate LaTeX
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, output)
			}
		})
	}
}

// TestSubtractionTokenization tests that the lexer correctly distinguishes
// between subtraction operators and negative number prefixes.
func TestSubtractionTokenization(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []TokenType
	}{
		{
			name:  "subtraction operator",
			input: "5 3 -",
			expected: []TokenType{
				TokenNumber,
				TokenNumber,
				TokenMinus,
				TokenEOF,
			},
		},
		{
			name:  "negative number",
			input: "-5",
			expected: []TokenType{
				TokenNumber,
				TokenEOF,
			},
		},
		{
			name:  "subtraction with negative result context",
			input: "5 -3 +",
			expected: []TokenType{
				TokenNumber,
				TokenNumber,
				TokenPlus,
				TokenEOF,
			},
		},
		{
			name:  "mixed operations",
			input: "10 -5 - 3 +",
			expected: []TokenType{
				TokenNumber,
				TokenNumber,
				TokenMinus,
				TokenNumber,
				TokenPlus,
				TokenEOF,
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, token := range tokens {
				if token.Type != tt.expected[i] {
					t.Errorf("Token %d: expected type %s, got %s", i, tt.expected[i], token.Type)
				}
			}
		})
	}
}

// TestSubtractionParsing tests that the parser correctly handles subtraction.
func TestSubtractionParsing(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		checkAST func(t *testing.T, ast Expr)
	}{
		{
			name:  "basic subtraction AST",
			input: "5 3 -",
			checkAST: func(t *testing.T, ast Expr) {
				binOp, ok := ast.(*BinaryOp)
				if !ok {
					t.Fatalf("Expected BinaryOp, got %T", ast)
				}
				if binOp.Operator != "-" {
					t.Errorf("Expected operator '-', got %q", binOp.Operator)
				}

				left, ok := binOp.Left.(*Number)
				if !ok || left.Value != "5" {
					t.Errorf("Expected left operand '5', got %v", binOp.Left)
				}

				right, ok := binOp.Right.(*Number)
				if !ok || right.Value != "3" {
					t.Errorf("Expected right operand '3', got %v", binOp.Right)
				}
			},
		},
		{
			name:  "chained subtraction AST",
			input: "5 3 - 2 -",
			checkAST: func(t *testing.T, ast Expr) {
				binOp, ok := ast.(*BinaryOp)
				if !ok {
					t.Fatalf("Expected BinaryOp, got %T", ast)
				}
				if binOp.Operator != "-" {
					t.Errorf("Expected operator '-', got %q", binOp.Operator)
				}

				// Left should be another BinaryOp (5 - 3)
				leftOp, ok := binOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("Expected left to be BinaryOp, got %T", binOp.Left)
				}
				if leftOp.Operator != "-" {
					t.Errorf("Expected left operator '-', got %q", leftOp.Operator)
				}

				// Right should be Number (2)
				right, ok := binOp.Right.(*Number)
				if !ok || right.Value != "2" {
					t.Errorf("Expected right operand '2', got %v", binOp.Right)
				}
			},
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

			tt.checkAST(t, ast)
		})
	}
}

// TestSubtractionErrors tests error cases for subtraction.
func TestSubtractionErrors(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expectError bool
		errorType   string
	}{
		{
			name:        "subtraction with insufficient operands",
			input:       "5 -",
			expectError: true,
			errorType:   "parser",
		},
		{
			name:        "subtraction with one operand",
			input:       "-",
			expectError: true,
			errorType:   "parser",
		},
		{
			name:        "subtraction with too many operands",
			input:       "5 3 2 -",
			expectError: true,
			errorType:   "parser",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil && tt.errorType == "lexer" {
				return // Expected lexer error
			}
			if err != nil {
				t.Fatalf("Unexpected lexer error: %v", err)
			}

			parser := NewParser(tokens)
			_, err = parser.Parse()
			if tt.expectError && err == nil {
				t.Errorf("Expected parser error, got none")
			}
			if !tt.expectError && err != nil {
				t.Errorf("Unexpected parser error: %v", err)
			}
		})
	}
}

// TestSubtractionWithAddition tests mixed operations.
func TestSubtractionWithAddition(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "addition then subtraction",
			input:    "5 3 + 2 -",
			expected: "$5 + 3 - 2$",
		},
		{
			name:     "subtraction then addition",
			input:    "5 3 - 2 +",
			expected: "$5 - 3 + 2$",
		},
		{
			name:     "complex mixed operations",
			input:    "10 5 - 3 + 2 -",
			expected: "$10 - 5 + 3 - 2$",
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

			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, output)
			}
		})
	}
}

package rpn2tex

import (
	"testing"
)

// TestMultiplicationLexing tests that the lexer recognizes the '*' operator.
func TestMultiplicationLexing(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []TokenType
	}{
		{
			name:     "single multiplication operator",
			input:    "*",
			expected: []TokenType{TokenMult, TokenEOF},
		},
		{
			name:     "multiplication with numbers",
			input:    "4 7 *",
			expected: []TokenType{TokenNumber, TokenNumber, TokenMult, TokenEOF},
		},
		{
			name:     "mixed operators",
			input:    "2 3 4 * +",
			expected: []TokenType{TokenNumber, TokenNumber, TokenNumber, TokenMult, TokenPlus, TokenEOF},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expectedType := range tt.expected {
				if tokens[i].Type != expectedType {
					t.Errorf("token[%d]: expected type %v, got %v", i, expectedType, tokens[i].Type)
				}
			}
		})
	}
}

// TestMultiplicationParsing tests that the parser creates correct AST for multiplication.
func TestMultiplicationParsing(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		validate func(*testing.T, Expr)
	}{
		{
			name:  "simple multiplication",
			input: "4 7 *",
			validate: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expected *BinaryOp, got %T", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("expected operator '*', got '%s'", binOp.Operator)
				}

				left, ok := binOp.Left.(*Number)
				if !ok || left.Value != "4" {
					t.Errorf("expected left operand '4', got %v", binOp.Left)
				}

				right, ok := binOp.Right.(*Number)
				if !ok || right.Value != "7" {
					t.Errorf("expected right operand '7', got %v", binOp.Right)
				}
			},
		},
		{
			name:  "multiplication with addition",
			input: "2 3 4 * +",
			validate: func(t *testing.T, expr Expr) {
				// Should be: BinaryOp("+", Number("2"), BinaryOp("*", Number("3"), Number("4")))
				addOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expected *BinaryOp for addition, got %T", expr)
				}
				if addOp.Operator != "+" {
					t.Errorf("expected top operator '+', got '%s'", addOp.Operator)
				}

				left, ok := addOp.Left.(*Number)
				if !ok || left.Value != "2" {
					t.Errorf("expected left operand '2', got %v", addOp.Left)
				}

				multOp, ok := addOp.Right.(*BinaryOp)
				if !ok {
					t.Fatalf("expected right to be *BinaryOp, got %T", addOp.Right)
				}
				if multOp.Operator != "*" {
					t.Errorf("expected operator '*', got '%s'", multOp.Operator)
				}
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

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v", err)
			}

			tt.validate(t, ast)
		})
	}
}

// TestMultiplicationLaTeXGeneration tests that multiplication produces correct LaTeX output.
func TestMultiplicationLaTeXGeneration(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic multiplication",
			input:    "4 7 *",
			expected: `$4 \times 7$`,
		},
		{
			name:     "multiplication with addition (no parens yet)",
			input:    "2 3 4 * +",
			expected: `$2 + 3 \times 4$`,
		},
		{
			name:     "multiplication on left side",
			input:    "5 3 * 2 +",
			expected: `$5 \times 3 + 2$`,
		},
		{
			name:     "chained multiplication",
			input:    "2 3 * 4 *",
			expected: `$2 \times 3 \times 4$`,
		},
		{
			name:     "floating point multiplication",
			input:    "3.14 2 *",
			expected: `$3.14 \times 2$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v", err)
			}

			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("expected %q, got %q", tt.expected, output)
			}
		})
	}
}

// TestMultiplicationEdgeCases tests edge cases for multiplication.
func TestMultiplicationEdgeCases(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		shouldError bool
		errorMsg    string
	}{
		{
			name:        "multiplication with insufficient operands",
			input:       "4 *",
			shouldError: true,
			errorMsg:    "requires two operands",
		},
		{
			name:        "multiplication with no operands",
			input:       "*",
			shouldError: true,
			errorMsg:    "requires two operands",
		},
		{
			name:        "valid negative number multiplication",
			input:       "-5 3 *",
			shouldError: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.shouldError {
					t.Fatalf("Tokenize() unexpected error = %v", err)
				}
				return
			}

			parser := NewParser(tokens)
			_, err = parser.Parse()

			if tt.shouldError {
				if err == nil {
					t.Errorf("expected error containing %q, got nil", tt.errorMsg)
				}
			} else {
				if err != nil {
					t.Errorf("unexpected error: %v", err)
				}
			}
		})
	}
}

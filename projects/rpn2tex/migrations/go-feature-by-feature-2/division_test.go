package rpn2tex

import (
	"testing"
)

// TestDivisionLexing tests that the lexer recognizes the '/' operator.
func TestDivisionLexing(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []TokenType
	}{
		{
			name:     "single division operator",
			input:    "/",
			expected: []TokenType{TokenDiv, TokenEOF},
		},
		{
			name:     "division with numbers",
			input:    "10 2 /",
			expected: []TokenType{TokenNumber, TokenNumber, TokenDiv, TokenEOF},
		},
		{
			name:     "mixed operators with division",
			input:    "10 2 / 3 +",
			expected: []TokenType{TokenNumber, TokenNumber, TokenDiv, TokenNumber, TokenPlus, TokenEOF},
		},
		{
			name:     "chained division",
			input:    "100 10 / 5 / 2 /",
			expected: []TokenType{TokenNumber, TokenNumber, TokenDiv, TokenNumber, TokenDiv, TokenNumber, TokenDiv, TokenEOF},
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

// TestDivisionParsing tests that the parser creates correct AST for division.
func TestDivisionParsing(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		validate func(*testing.T, Expr)
	}{
		{
			name:  "simple division",
			input: "10 2 /",
			validate: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expected *BinaryOp, got %T", expr)
				}
				if binOp.Operator != "/" {
					t.Errorf("expected operator '/', got '%s'", binOp.Operator)
				}

				left, ok := binOp.Left.(*Number)
				if !ok || left.Value != "10" {
					t.Errorf("expected left operand '10', got %v", binOp.Left)
				}

				right, ok := binOp.Right.(*Number)
				if !ok || right.Value != "2" {
					t.Errorf("expected right operand '2', got %v", binOp.Right)
				}
			},
		},
		{
			name:  "chained division",
			input: "100 10 / 5 /",
			validate: func(t *testing.T, expr Expr) {
				// Should be: BinaryOp("/", BinaryOp("/", Number("100"), Number("10")), Number("5"))
				divOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expected *BinaryOp for division, got %T", expr)
				}
				if divOp.Operator != "/" {
					t.Errorf("expected top operator '/', got '%s'", divOp.Operator)
				}

				innerDivOp, ok := divOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("expected left to be *BinaryOp, got %T", divOp.Left)
				}
				if innerDivOp.Operator != "/" {
					t.Errorf("expected inner operator '/', got '%s'", innerDivOp.Operator)
				}

				right, ok := divOp.Right.(*Number)
				if !ok || right.Value != "5" {
					t.Errorf("expected right operand '5', got %v", divOp.Right)
				}
			},
		},
		{
			name:  "division with addition",
			input: "10 2 / 3 +",
			validate: func(t *testing.T, expr Expr) {
				// Should be: BinaryOp("+", BinaryOp("/", Number("10"), Number("2")), Number("3"))
				addOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expected *BinaryOp for addition, got %T", expr)
				}
				if addOp.Operator != "+" {
					t.Errorf("expected top operator '+', got '%s'", addOp.Operator)
				}

				divOp, ok := addOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("expected left to be *BinaryOp, got %T", addOp.Left)
				}
				if divOp.Operator != "/" {
					t.Errorf("expected left operator '/', got '%s'", divOp.Operator)
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

// TestDivisionLaTeXGeneration tests that division produces correct LaTeX output.
func TestDivisionLaTeXGeneration(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "basic division",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "chained division",
			input:    "100 10 / 5 / 2 /",
			expected: `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name:     "division with addition",
			input:    "10 2 / 3 +",
			expected: `$10 \div 2 + 3$`,
		},
		{
			name:     "division on right side",
			input:    "3 10 2 / +",
			expected: `$3 + 10 \div 2$`,
		},
		{
			name:     "floating point division",
			input:    "15.5 3.1 /",
			expected: `$15.5 \div 3.1$`,
		},
		{
			name:     "division with multiplication",
			input:    "10 2 / 3 *",
			expected: `$10 \div 2 \times 3$`,
		},
		{
			name:     "multiplication with division",
			input:    "10 2 * 3 /",
			expected: `$10 \times 2 \div 3$`,
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

// TestDivisionEdgeCases tests edge cases for division.
func TestDivisionEdgeCases(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		shouldError bool
		errorMsg    string
	}{
		{
			name:        "division with insufficient operands",
			input:       "10 /",
			shouldError: true,
			errorMsg:    "requires two operands",
		},
		{
			name:        "division with no operands",
			input:       "/",
			shouldError: true,
			errorMsg:    "requires two operands",
		},
		{
			name:        "valid negative number division",
			input:       "-10 2 /",
			shouldError: false,
		},
		{
			name:        "division with negative divisor",
			input:       "10 -2 /",
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

// TestDivisionIOContract tests the I/O contract for division feature.
func TestDivisionIOContract(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "I/O contract: 10 2 /",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "I/O contract: 100 10 / 5 / 2 /",
			input:    "100 10 / 5 / 2 /",
			expected: `$100 \div 10 \div 5 \div 2$`,
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
				t.Errorf("I/O contract mismatch:\n  input:    %q\n  expected: %q\n  got:      %q", tt.input, tt.expected, output)
			}
		})
	}
}

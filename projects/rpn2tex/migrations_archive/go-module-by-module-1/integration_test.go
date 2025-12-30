package rpn2tex

import (
	"testing"
)

// TestIntegration_LexerAndParser tests the integration between the lexer and parser.
// This verifies that the complete pipeline from text input to AST works correctly.
func TestIntegration_LexerAndParser(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(t *testing.T, expr Expr)
	}{
		{
			name:  "simple addition: 5 3 +",
			input: "5 3 +",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("operator = %q, want %q", binOp.Operator, "+")
				}
				left, ok := binOp.Left.(*Number)
				if !ok || left.Value != "5" {
					t.Errorf("left = %v, want Number{Value: \"5\"}", binOp.Left)
				}
				right, ok := binOp.Right.(*Number)
				if !ok || right.Value != "3" {
					t.Errorf("right = %v, want Number{Value: \"3\"}", binOp.Right)
				}
			},
		},
		{
			name:  "simple subtraction: 5 3 -",
			input: "5 3 -",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "-" {
					t.Errorf("operator = %q, want %q", binOp.Operator, "-")
				}
			},
		},
		{
			name:  "simple multiplication: 4 7 *",
			input: "4 7 *",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("operator = %q, want %q", binOp.Operator, "*")
				}
			},
		},
		{
			name:  "simple division: 10 2 /",
			input: "10 2 /",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "/" {
					t.Errorf("operator = %q, want %q", binOp.Operator, "/")
				}
			},
		},
		{
			name:  "nested: 5 3 + 2 *",
			input: "5 3 + 2 *",
			check: func(t *testing.T, expr Expr) {
				// Root should be multiplication
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "*")
				}
				// Left should be addition
				leftOp, ok := binOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("left type = %T, want *BinaryOp", binOp.Left)
				}
				if leftOp.Operator != "+" {
					t.Errorf("left operator = %q, want %q", leftOp.Operator, "+")
				}
				// Right should be number 2
				rightNum, ok := binOp.Right.(*Number)
				if !ok || rightNum.Value != "2" {
					t.Errorf("right = %v, want Number{Value: \"2\"}", binOp.Right)
				}
			},
		},
		{
			name:  "no parens needed: 5 3 * 2 +",
			input: "5 3 * 2 +",
			check: func(t *testing.T, expr Expr) {
				// Root should be addition
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "+")
				}
				// Left should be multiplication
				leftOp, ok := binOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("left type = %T, want *BinaryOp", binOp.Left)
				}
				if leftOp.Operator != "*" {
					t.Errorf("left operator = %q, want %q", leftOp.Operator, "*")
				}
			},
		},
		{
			name:  "decimal numbers: 3.14 2 *",
			input: "3.14 2 *",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				left, ok := binOp.Left.(*Number)
				if !ok || left.Value != "3.14" {
					t.Errorf("left value = %v, want \"3.14\"", left.Value)
				}
			},
		},
		{
			name:  "chain of operations: 1 2 + 3 + 4 +",
			input: "1 2 + 3 + 4 +",
			check: func(t *testing.T, expr Expr) {
				// Root should be addition
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "+")
				}
				// Right should be number 4
				rightNum, ok := binOp.Right.(*Number)
				if !ok || rightNum.Value != "4" {
					t.Errorf("right = %v, want Number{Value: \"4\"}", binOp.Right)
				}
			},
		},
		{
			name:  "both sides parens: 1 2 + 3 4 + *",
			input: "1 2 + 3 4 + *",
			check: func(t *testing.T, expr Expr) {
				// Root should be multiplication
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "*")
				}
				// Both left and right should be BinaryOp
				_, leftOk := binOp.Left.(*BinaryOp)
				_, rightOk := binOp.Right.(*BinaryOp)
				if !leftOk || !rightOk {
					t.Errorf("left type = %T, right type = %T, both want *BinaryOp",
						binOp.Left, binOp.Right)
				}
			},
		},
		{
			name:  "right side parens: 2 3 4 + *",
			input: "2 3 4 + *",
			check: func(t *testing.T, expr Expr) {
				// Root should be multiplication
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "*")
				}
				// Left should be a number
				leftNum, ok := binOp.Left.(*Number)
				if !ok || leftNum.Value != "2" {
					t.Errorf("left = %v, want Number{Value: \"2\"}", binOp.Left)
				}
				// Right should be BinaryOp
				rightOp, ok := binOp.Right.(*BinaryOp)
				if !ok {
					t.Fatalf("right type = %T, want *BinaryOp", binOp.Right)
				}
				if rightOp.Operator != "+" {
					t.Errorf("right operator = %q, want %q", rightOp.Operator, "+")
				}
			},
		},
		{
			name:  "left-associative subtraction: 5 3 - 2 -",
			input: "5 3 - 2 -",
			check: func(t *testing.T, expr Expr) {
				// Root should be subtraction
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "-" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "-")
				}
				// Left should be subtraction
				leftOp, ok := binOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("left type = %T, want *BinaryOp", binOp.Left)
				}
				if leftOp.Operator != "-" {
					t.Errorf("left operator = %q, want %q", leftOp.Operator, "-")
				}
				// Right should be number 2
				rightNum, ok := binOp.Right.(*Number)
				if !ok || rightNum.Value != "2" {
					t.Errorf("right = %v, want Number{Value: \"2\"}", binOp.Right)
				}
			},
		},
		{
			name:  "long division chain: 100 10 / 5 / 2 /",
			input: "100 10 / 5 / 2 /",
			check: func(t *testing.T, expr Expr) {
				// Root should be division
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "/" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "/")
				}
				// Left should be nested divisions
				_, ok = binOp.Left.(*BinaryOp)
				if !ok {
					t.Errorf("left type = %T, want *BinaryOp", binOp.Left)
				}
			},
		},
		{
			name:  "complex precedence: 10 2 / 3 + 4 *",
			input: "10 2 / 3 + 4 *",
			check: func(t *testing.T, expr Expr) {
				// Root should be multiplication
				binOp, ok := expr.(*BinaryOp)
				if !ok {
					t.Fatalf("expr type = %T, want *BinaryOp", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("root operator = %q, want %q", binOp.Operator, "*")
				}
				// Left should be addition
				leftOp, ok := binOp.Left.(*BinaryOp)
				if !ok {
					t.Fatalf("left type = %T, want *BinaryOp", binOp.Left)
				}
				if leftOp.Operator != "+" {
					t.Errorf("left operator = %q, want %q", leftOp.Operator, "+")
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lexer phase
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v, want nil", err)
			}

			// Parser phase
			parser := NewParser(tokens)
			expr, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v, want nil", err)
			}

			// Check the AST structure
			tt.check(t, expr)
		})
	}
}

// TestIntegration_FullPipeline tests the complete pipeline from RPN input to LaTeX output.
// This verifies that all components work together correctly.
func TestIntegration_FullPipeline(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "simple addition: 5 3 +",
			input:    "5 3 +",
			expected: "$5 + 3$",
		},
		{
			name:     "simple subtraction: 5 3 -",
			input:    "5 3 -",
			expected: "$5 - 3$",
		},
		{
			name:     "simple multiplication: 4 7 *",
			input:    "4 7 *",
			expected: `$4 \times 7$`,
		},
		{
			name:     "simple division: 10 2 /",
			input:    "10 2 /",
			expected: `$10 \div 2$`,
		},
		{
			name:     "addition with multiplication: 5 3 + 2 *",
			input:    "5 3 + 2 *",
			expected: `$( 5 + 3 ) \times 2$`,
		},
		{
			name:     "multiplication with addition: 5 3 * 2 +",
			input:    "5 3 * 2 +",
			expected: `$5 \times 3 + 2$`,
		},
		{
			name:     "division chain: 10 2 / 5 *",
			input:    "10 2 / 5 *",
			expected: `$10 \div 2 \times 5$`,
		},
		{
			name:     "subtraction chain: 5 3 - 2 -",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
		},
		{
			name:     "long division chain: 100 10 / 5 / 2 /",
			input:    "100 10 / 5 / 2 /",
			expected: `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name:     "addition chain: 1 2 + 3 + 4 +",
			input:    "1 2 + 3 + 4 +",
			expected: "$1 + 2 + 3 + 4$",
		},
		{
			name:     "precedence: 2 3 4 * +",
			input:    "2 3 4 * +",
			expected: `$2 + 3 \times 4$`,
		},
		{
			name:     "precedence with parens: 2 3 + 4 *",
			input:    "2 3 + 4 *",
			expected: `$( 2 + 3 ) \times 4$`,
		},
		{
			name:     "right side parens: 2 3 4 + *",
			input:    "2 3 4 + *",
			expected: `$2 \times ( 3 + 4 )$`,
		},
		{
			name:     "no parens for higher precedence: 2 3 * 4 +",
			input:    "2 3 * 4 +",
			expected: `$2 \times 3 + 4$`,
		},
		{
			name:     "decimal multiplication: 3.14 2 *",
			input:    "3.14 2 *",
			expected: `$3.14 \times 2$`,
		},
		{
			name:     "decimal addition: 1.5 0.5 +",
			input:    "1.5 0.5 +",
			expected: "$1.5 + 0.5$",
		},
		{
			name:     "both sides parens: 1 2 + 3 4 + *",
			input:    "1 2 + 3 4 + *",
			expected: `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name:     "complex precedence: 10 2 / 3 + 4 *",
			input:    "10 2 / 3 + 4 *",
			expected: `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lexer phase
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v, want nil", err)
			}

			// Parser phase
			parser := NewParser(tokens)
			expr, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v, want nil", err)
			}

			// LaTeX generation phase
			generator := NewLaTeXGenerator()
			result := generator.Generate(expr)

			if result != tt.expected {
				t.Errorf("Generate() = %q, want %q", result, tt.expected)
			}
		})
	}
}

// TestIntegration_ErrorCases tests error handling in the complete pipeline.
func TestIntegration_ErrorCases(t *testing.T) {
	tests := []struct {
		name           string
		input          string
		expectLexErr   bool
		expectParseErr bool
	}{
		{
			name:         "invalid character: ^",
			input:        "2 3 ^",
			expectLexErr: true,
		},
		{
			name:           "insufficient operands",
			input:          "5 +",
			expectParseErr: true,
		},
		{
			name:           "too many values",
			input:          "5 3 2 +",
			expectParseErr: true,
		},
		{
			name:           "empty expression",
			input:          "",
			expectParseErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lexer phase
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if tt.expectLexErr {
				if err == nil {
					t.Fatalf("Tokenize() error = nil, want error")
				}
				// Check it's a LexerError
				if _, ok := err.(*LexerError); !ok {
					t.Errorf("error type = %T, want *LexerError", err)
				}
				return
			}

			if err != nil {
				t.Fatalf("Tokenize() error = %v, want nil", err)
			}

			// Parser phase
			parser := NewParser(tokens)
			expr, err := parser.Parse()

			if tt.expectParseErr {
				if err == nil {
					t.Fatalf("Parse() error = nil, want error")
				}
				// Check it's a ParserError
				if _, ok := err.(*ParserError); !ok {
					t.Errorf("error type = %T, want *ParserError", err)
				}
				if expr != nil {
					t.Errorf("expr = %v, want nil", expr)
				}
				return
			}

			if err != nil {
				t.Fatalf("Parse() error = %v, want nil", err)
			}
		})
	}
}

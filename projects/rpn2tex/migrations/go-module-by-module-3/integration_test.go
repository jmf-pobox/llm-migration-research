package rpn2tex

import (
	"testing"
)

// TestIntegration_LexerAndParser tests the integration between lexer and parser.
func TestIntegration_LexerAndParser(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expectError bool
		validateAST func(t *testing.T, ast Expr)
	}{
		{
			name:        "Simple addition",
			input:       "5 3 +",
			expectError: false,
			validateAST: func(t *testing.T, ast Expr) {
				binOp, ok := ast.(*BinaryOp)
				if !ok {
					t.Fatalf("Expected BinaryOp, got: %T", ast)
				}
				if binOp.Operator != "+" {
					t.Errorf("Expected '+', got: %s", binOp.Operator)
				}
				left := binOp.Left.(*Number)
				right := binOp.Right.(*Number)
				if left.Value != "5" || right.Value != "3" {
					t.Errorf("Expected '5 + 3', got: %s + %s", left.Value, right.Value)
				}
			},
		},
		{
			name:        "Nested expression",
			input:       "5 3 + 2 *",
			expectError: false,
			validateAST: func(t *testing.T, ast Expr) {
				mult, ok := ast.(*BinaryOp)
				if !ok || mult.Operator != "*" {
					t.Fatalf("Expected root to be '*', got: %T", ast)
				}
				plus, ok := mult.Left.(*BinaryOp)
				if !ok || plus.Operator != "+" {
					t.Fatalf("Expected left child to be '+', got: %T", mult.Left)
				}
				num2, ok := mult.Right.(*Number)
				if !ok || num2.Value != "2" {
					t.Fatalf("Expected right child to be '2', got: %T", mult.Right)
				}
			},
		},
		{
			name:        "Floating point",
			input:       "3.14 2 *",
			expectError: false,
			validateAST: func(t *testing.T, ast Expr) {
				binOp := ast.(*BinaryOp)
				left := binOp.Left.(*Number)
				if left.Value != "3.14" {
					t.Errorf("Expected '3.14', got: %s", left.Value)
				}
			},
		},
		{
			name:        "Negative numbers",
			input:       "-5 3 +",
			expectError: false,
			validateAST: func(t *testing.T, ast Expr) {
				binOp := ast.(*BinaryOp)
				left := binOp.Left.(*Number)
				if left.Value != "-5" {
					t.Errorf("Expected '-5', got: %s", left.Value)
				}
			},
		},
		{
			name:        "Complex expression",
			input:       "1 2 + 3 4 + *",
			expectError: false,
			validateAST: func(t *testing.T, ast Expr) {
				mult := ast.(*BinaryOp)
				if mult.Operator != "*" {
					t.Errorf("Expected '*', got: %s", mult.Operator)
				}
				leftPlus := mult.Left.(*BinaryOp)
				rightPlus := mult.Right.(*BinaryOp)
				if leftPlus.Operator != "+" || rightPlus.Operator != "+" {
					t.Error("Expected both children to be '+'")
				}
			},
		},
		{
			name:        "Insufficient operands",
			input:       "5 +",
			expectError: true,
		},
		{
			name:        "Incomplete expression",
			input:       "5 3 2 +",
			expectError: true,
		},
		{
			name:        "Empty expression",
			input:       "",
			expectError: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil && !tt.expectError {
				t.Fatalf("Unexpected lexer error: %v", err)
			}
			if err != nil && tt.expectError {
				return // Expected error in lexer
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()

			if tt.expectError {
				if err == nil {
					t.Fatal("Expected error, got nil")
				}
				return
			}

			if err != nil {
				t.Fatalf("Unexpected parser error: %v", err)
			}

			if tt.validateAST != nil {
				tt.validateAST(t, ast)
			}
		})
	}
}

// TestIntegration_StackOrderCorrectness verifies the critical stack order requirement.
func TestIntegration_StackOrderCorrectness(t *testing.T) {
	// Test case: 5 3 - should be (5 - 3) = 2, NOT (3 - 5) = -2
	input := "5 3 -"

	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Lexer error: %v", err)
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("Parser error: %v", err)
	}

	binOp, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp, got: %T", ast)
	}

	left := binOp.Left.(*Number)
	right := binOp.Right.(*Number)

	// CRITICAL: First operand encountered (5) should be LEFT
	// Second operand encountered (3) should be RIGHT
	if left.Value != "5" {
		t.Errorf("Expected left='5', got: %s", left.Value)
	}
	if right.Value != "3" {
		t.Errorf("Expected right='3', got: %s", right.Value)
	}

	// Verify position tracking is preserved
	if left.Line != 1 || left.Column != 1 {
		t.Errorf("Expected left at 1:1, got: %d:%d", left.Line, left.Column)
	}
	if right.Line != 1 || right.Column != 3 {
		t.Errorf("Expected right at 1:3, got: %d:%d", right.Line, right.Column)
	}
}

package rpn2tex

import "testing"

// TestPrecedenceFeature tests end-to-end precedence functionality.
func TestPrecedenceFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "addition before multiplication (left)",
			input:    "5 3 + 2 *",
			expected: "$( 5 + 3 ) \\times 2$",
		},
		{
			name:     "addition before multiplication (left, variant)",
			input:    "2 3 + 4 *",
			expected: "$( 2 + 3 ) \\times 4$",
		},
		{
			name:     "addition before multiplication (right)",
			input:    "2 3 4 + *",
			expected: "$2 \\times ( 3 + 4 )$",
		},
		{
			name:     "both operands need parens",
			input:    "1 2 + 3 4 + *",
			expected: "$( 1 + 2 ) \\times ( 3 + 4 )$",
		},
		{
			name:     "mixed operators with division",
			input:    "10 2 / 3 + 4 *",
			expected: "$( 10 \\div 2 + 3 ) \\times 4$",
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

// TestPrecedenceMultiplicationNoParens tests that multiplication binds tighter than addition.
func TestPrecedenceMultiplicationNoParens(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "addition with multiplication (no parens needed)",
			input:    "2 3 4 * +",
			expected: "$2 + 3 \\times 4$",
		},
		{
			name:     "multiplication on left",
			input:    "5 3 * 2 +",
			expected: "$5 \\times 3 + 2$",
		},
		{
			name:     "division on left of addition",
			input:    "10 2 / 3 +",
			expected: "$10 \\div 2 + 3$",
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

// TestPrecedenceSamePrecedence tests equal precedence operators (left-associativity).
func TestPrecedenceSamePrecedence(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "chained subtraction (left-associative)",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
		},
		{
			name:     "chained division (left-associative)",
			input:    "100 10 / 5 / 2 /",
			expected: "$100 \\div 10 \\div 5 \\div 2$",
		},
		{
			name:     "chained addition (no parens)",
			input:    "1 2 + 3 + 4 +",
			expected: "$1 + 2 + 3 + 4$",
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

// TestNeedsParens tests the needsParens logic directly.
func TestNeedsParens(t *testing.T) {
	tests := []struct {
		name             string
		child            Expr
		parentPrecedence int
		isRight          bool
		expected         bool
	}{
		{
			name:             "number never needs parens",
			child:            &Number{Value: "5"},
			parentPrecedence: 2,
			isRight:          false,
			expected:         false,
		},
		{
			name: "lower precedence on left needs parens",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 2,
			isRight:          false,
			expected:         true,
		},
		{
			name: "lower precedence on right needs parens",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 2,
			isRight:          true,
			expected:         true,
		},
		{
			name: "equal precedence + on left (no parens)",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
		{
			name: "equal precedence + on right (no parens)",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          true,
			expected:         false,
		},
		{
			name: "equal precedence - on left (no parens)",
			child: &BinaryOp{
				Operator: "-",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
		{
			name: "equal precedence - on right (needs parens)",
			child: &BinaryOp{
				Operator: "-",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          true,
			expected:         true,
		},
		{
			name: "equal precedence / on right (needs parens)",
			child: &BinaryOp{
				Operator: "/",
				Left:     &Number{Value: "10"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 2,
			isRight:          true,
			expected:         true,
		},
		{
			name: "higher precedence never needs parens",
			child: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "3"},
				Right:    &Number{Value: "4"},
			},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			generator := NewLaTeXGenerator()
			result := generator.needsParens(tt.child, tt.parentPrecedence, tt.isRight)

			if result != tt.expected {
				t.Errorf("Expected %v, got %v", tt.expected, result)
			}
		})
	}
}

// TestLaTeXGeneratorPrecedence tests LaTeX generation with precedence.
func TestLaTeXGeneratorPrecedence(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "addition before multiplication (left)",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "5"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "2"},
			},
			expected: "( 5 + 3 ) \\times 2",
		},
		{
			name: "addition before multiplication (right)",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "2"},
				Right: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "3"},
					Right:    &Number{Value: "4"},
				},
			},
			expected: "2 \\times ( 3 + 4 )",
		},
		{
			name: "multiplication and addition (no parens needed)",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "2"},
				Right: &BinaryOp{
					Operator: "*",
					Left:     &Number{Value: "3"},
					Right:    &Number{Value: "4"},
				},
			},
			expected: "2 + 3 \\times 4",
		},
		{
			name: "complex nested expression",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left: &BinaryOp{
						Operator: "/",
						Left:     &Number{Value: "10"},
						Right:    &Number{Value: "2"},
					},
					Right: &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			expected: "( 10 \\div 2 + 3 ) \\times 4",
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

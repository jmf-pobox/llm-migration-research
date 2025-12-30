package rpn2tex

import "testing"

// TestLaTeXGenerator_VisitNumber tests the VisitNumber method directly.
func TestLaTeXGenerator_VisitNumber(t *testing.T) {
	gen := NewLaTeXGenerator()

	tests := []struct {
		name     string
		node     *Number
		expected string
	}{
		{
			name:     "integer",
			node:     &Number{Line: 1, Column: 1, Value: "5"},
			expected: "5",
		},
		{
			name:     "decimal",
			node:     &Number{Line: 1, Column: 1, Value: "3.14"},
			expected: "3.14",
		},
		{
			name:     "negative integer",
			node:     &Number{Line: 1, Column: 1, Value: "-10"},
			expected: "-10",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := gen.VisitNumber(tt.node)
			if result != tt.expected {
				t.Errorf("VisitNumber() = %q, want %q", result, tt.expected)
			}
		})
	}
}

// TestLaTeXGenerator_Generate tests the full Generate method with various AST structures.
func TestLaTeXGenerator_Generate(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name:     "simple addition: 5 3 +",
			ast:      &BinaryOp{Line: 1, Column: 3, Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			expected: "$5 + 3$",
		},
		{
			name:     "simple subtraction: 5 3 -",
			ast:      &BinaryOp{Line: 1, Column: 3, Operator: "-", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			expected: "$5 - 3$",
		},
		{
			name:     "simple multiplication: 4 7 *",
			ast:      &BinaryOp{Line: 1, Column: 3, Operator: "*", Left: &Number{Value: "4"}, Right: &Number{Value: "7"}},
			expected: `$4 \times 7$`,
		},
		{
			name:     "simple division: 10 2 /",
			ast:      &BinaryOp{Line: 1, Column: 4, Operator: "/", Left: &Number{Value: "10"}, Right: &Number{Value: "2"}},
			expected: `$10 \div 2$`,
		},
		{
			name: "addition with multiplication: 5 3 + 2 *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &BinaryOp{Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
				Right:    &Number{Value: "2"},
			},
			expected: `$( 5 + 3 ) \times 2$`,
		},
		{
			name: "multiplication with addition: 5 3 * 2 +",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &BinaryOp{Operator: "*", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
				Right:    &Number{Value: "2"},
			},
			expected: `$5 \times 3 + 2$`,
		},
		{
			name: "division chain: 10 2 / 5 *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &BinaryOp{Operator: "/", Left: &Number{Value: "10"}, Right: &Number{Value: "2"}},
				Right:    &Number{Value: "5"},
			},
			expected: `$10 \div 2 \times 5$`,
		},
		{
			name: "subtraction chain: 5 3 - 2 -",
			ast: &BinaryOp{
				Operator: "-",
				Left:     &BinaryOp{Operator: "-", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
				Right:    &Number{Value: "2"},
			},
			expected: "$5 - 3 - 2$",
		},
		{
			name: "long division chain: 100 10 / 5 / 2 /",
			ast: &BinaryOp{
				Operator: "/",
				Left: &BinaryOp{
					Operator: "/",
					Left:     &BinaryOp{Operator: "/", Left: &Number{Value: "100"}, Right: &Number{Value: "10"}},
					Right:    &Number{Value: "5"},
				},
				Right: &Number{Value: "2"},
			},
			expected: `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name: "addition chain: 1 2 + 3 + 4 +",
			ast: &BinaryOp{
				Operator: "+",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			expected: "$1 + 2 + 3 + 4$",
		},
		{
			name: "precedence: 2 3 4 * +",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "2"},
				Right:    &BinaryOp{Operator: "*", Left: &Number{Value: "3"}, Right: &Number{Value: "4"}},
			},
			expected: `$2 + 3 \times 4$`,
		},
		{
			name: "precedence with parens: 2 3 + 4 *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &BinaryOp{Operator: "+", Left: &Number{Value: "2"}, Right: &Number{Value: "3"}},
				Right:    &Number{Value: "4"},
			},
			expected: `$( 2 + 3 ) \times 4$`,
		},
		{
			name: "right side parens: 2 3 4 + *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "2"},
				Right:    &BinaryOp{Operator: "+", Left: &Number{Value: "3"}, Right: &Number{Value: "4"}},
			},
			expected: `$2 \times ( 3 + 4 )$`,
		},
		{
			name: "no parens for higher precedence: 2 3 * 4 +",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &BinaryOp{Operator: "*", Left: &Number{Value: "2"}, Right: &Number{Value: "3"}},
				Right:    &Number{Value: "4"},
			},
			expected: `$2 \times 3 + 4$`,
		},
		{
			name: "decimal multiplication: 3.14 2 *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "3.14"},
				Right:    &Number{Value: "2"},
			},
			expected: `$3.14 \times 2$`,
		},
		{
			name: "decimal addition: 1.5 0.5 +",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1.5"},
				Right:    &Number{Value: "0.5"},
			},
			expected: "$1.5 + 0.5$",
		},
		{
			name: "both sides parens: 1 2 + 3 4 + *",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
				Right:    &BinaryOp{Operator: "+", Left: &Number{Value: "3"}, Right: &Number{Value: "4"}},
			},
			expected: `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name: "complex precedence: 10 2 / 3 + 4 *",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &BinaryOp{Operator: "/", Left: &Number{Value: "10"}, Right: &Number{Value: "2"}},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			expected: `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, want %q", result, tt.expected)
			}
		})
	}
}

// TestLaTeXGenerator_NeedsParens tests the parenthesization logic directly.
func TestLaTeXGenerator_NeedsParens(t *testing.T) {
	gen := NewLaTeXGenerator()

	tests := []struct {
		name             string
		child            Expr
		parentPrecedence int
		isRight          bool
		expected         bool
	}{
		{
			name:             "Number never needs parens",
			child:            &Number{Value: "5"},
			parentPrecedence: PrecedenceHigh,
			isRight:          true,
			expected:         false,
		},
		{
			name:             "Lower precedence child needs parens",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceHigh,
			isRight:          false,
			expected:         true,
		},
		{
			name:             "Equal precedence on left, no parens",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceLow,
			isRight:          false,
			expected:         false,
		},
		{
			name:             "Equal precedence on right with subtraction, needs parens",
			child:            &BinaryOp{Operator: "-", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceLow,
			isRight:          true,
			expected:         true,
		},
		{
			name:             "Equal precedence on right with division, needs parens",
			child:            &BinaryOp{Operator: "/", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceHigh,
			isRight:          true,
			expected:         true,
		},
		{
			name:             "Equal precedence on right with addition, no parens",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceLow,
			isRight:          true,
			expected:         false,
		},
		{
			name:             "Equal precedence on right with multiplication, no parens",
			child:            &BinaryOp{Operator: "*", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceHigh,
			isRight:          true,
			expected:         false,
		},
		{
			name:             "Higher precedence child, no parens",
			child:            &BinaryOp{Operator: "*", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: PrecedenceLow,
			isRight:          false,
			expected:         false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := gen.needsParens(tt.child, tt.parentPrecedence, tt.isRight)
			if result != tt.expected {
				t.Errorf("needsParens() = %v, want %v", result, tt.expected)
			}
		})
	}
}

// TestLaTeXGenerator_OperatorMappings verifies the operator mappings are correct.
func TestLaTeXGenerator_OperatorMappings(t *testing.T) {
	tests := []struct {
		operator string
		latex    string
		prec     int
	}{
		{"+", "+", PrecedenceLow},
		{"-", "-", PrecedenceLow},
		{"*", `\times`, PrecedenceHigh},
		{"/", `\div`, PrecedenceHigh},
	}

	for _, tt := range tests {
		t.Run(tt.operator, func(t *testing.T) {
			if latex := BinaryOps[tt.operator]; latex != tt.latex {
				t.Errorf("BinaryOps[%q] = %q, want %q", tt.operator, latex, tt.latex)
			}
			if prec := Precedence[tt.operator]; prec != tt.prec {
				t.Errorf("Precedence[%q] = %d, want %d", tt.operator, prec, tt.prec)
			}
		})
	}
}

// TestLaTeXGenerator_BackslashInLaTeXCommands verifies that LaTeX commands contain single backslash.
func TestLaTeXGenerator_BackslashInLaTeXCommands(t *testing.T) {
	gen := NewLaTeXGenerator()

	// Test multiplication produces \times (single backslash)
	multAST := &BinaryOp{
		Operator: "*",
		Left:     &Number{Value: "2"},
		Right:    &Number{Value: "3"},
	}
	multResult := gen.Generate(multAST)
	expectedMult := `$2 \times 3$`
	if multResult != expectedMult {
		t.Errorf("Multiplication: got %q, want %q", multResult, expectedMult)
	}

	// Test division produces \div (single backslash)
	divAST := &BinaryOp{
		Operator: "/",
		Left:     &Number{Value: "10"},
		Right:    &Number{Value: "2"},
	}
	divResult := gen.Generate(divAST)
	expectedDiv := `$10 \div 2$`
	if divResult != expectedDiv {
		t.Errorf("Division: got %q, want %q", divResult, expectedDiv)
	}
}

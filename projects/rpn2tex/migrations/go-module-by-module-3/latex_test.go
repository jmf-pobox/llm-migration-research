package rpn2tex

import "testing"

func TestLaTeXGenerator_Generate_SimpleOperations(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "simple addition",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			expected: "$5 + 3$",
		},
		{
			name: "simple subtraction",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "-",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			expected: "$5 - 3$",
		},
		{
			name: "simple multiplication",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "*",
				Left:     &Number{Line: 1, Column: 1, Value: "4"},
				Right:    &Number{Line: 1, Column: 3, Value: "7"},
			},
			expected: `$4 \times 7$`,
		},
		{
			name: "simple division",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "/",
				Left:     &Number{Line: 1, Column: 1, Value: "10"},
				Right:    &Number{Line: 1, Column: 4, Value: "2"},
			},
			expected: `$10 \div 2$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_Generate_Precedence(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "addition before multiplication (needs parens)",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "*",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 1, Value: "5"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "2"},
			},
			expected: `$( 5 + 3 ) \times 2$`,
		},
		{
			name: "multiplication before addition (no parens needed)",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "+",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "*",
					Left:     &Number{Line: 1, Column: 1, Value: "5"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "2"},
			},
			expected: `$5 \times 3 + 2$`,
		},
		{
			name: "mixed operators: 2 + 3 * 4",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "2"},
				Right: &BinaryOp{
					Line:     1,
					Column:   5,
					Operator: "*",
					Left:     &Number{Line: 1, Column: 3, Value: "3"},
					Right:    &Number{Line: 1, Column: 5, Value: "4"},
				},
			},
			expected: `$2 + 3 \times 4$`,
		},
		{
			name: "mixed operators: (2 + 3) * 4",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "*",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 1, Value: "2"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "4"},
			},
			expected: `$( 2 + 3 ) \times 4$`,
		},
		{
			name: "mixed operators: 2 * (3 + 4)",
			ast: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "*",
				Left:     &Number{Line: 1, Column: 1, Value: "2"},
				Right: &BinaryOp{
					Line:     1,
					Column:   5,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 3, Value: "3"},
					Right:    &Number{Line: 1, Column: 5, Value: "4"},
				},
			},
			expected: `$2 \times ( 3 + 4 )$`,
		},
		{
			name: "mixed operators: 2 * 3 + 4",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "+",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "*",
					Left:     &Number{Line: 1, Column: 1, Value: "2"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "4"},
			},
			expected: `$2 \times 3 + 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_Generate_LeftAssociativity(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "left-to-right evaluation for same precedence: 10 / 2 * 5",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "*",
				Left: &BinaryOp{
					Line:     1,
					Column:   4,
					Operator: "/",
					Left:     &Number{Line: 1, Column: 1, Value: "10"},
					Right:    &Number{Line: 1, Column: 4, Value: "2"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "5"},
			},
			expected: `$10 \div 2 \times 5$`,
		},
		{
			name: "multiple subtractions: 5 - 3 - 2",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "-",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "-",
					Left:     &Number{Line: 1, Column: 1, Value: "5"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "2"},
			},
			expected: `$5 - 3 - 2$`,
		},
		{
			name: "chain of divisions: 100 / 10 / 5 / 2",
			ast: &BinaryOp{
				Line:     1,
				Column:   13,
				Operator: "/",
				Left: &BinaryOp{
					Line:     1,
					Column:   9,
					Operator: "/",
					Left: &BinaryOp{
						Line:     1,
						Column:   5,
						Operator: "/",
						Left:     &Number{Line: 1, Column: 1, Value: "100"},
						Right:    &Number{Line: 1, Column: 5, Value: "10"},
					},
					Right: &Number{Line: 1, Column: 9, Value: "5"},
				},
				Right: &Number{Line: 1, Column: 13, Value: "2"},
			},
			expected: `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name: "chain of additions: 1 + 2 + 3 + 4",
			ast: &BinaryOp{
				Line:     1,
				Column:   11,
				Operator: "+",
				Left: &BinaryOp{
					Line:     1,
					Column:   7,
					Operator: "+",
					Left: &BinaryOp{
						Line:     1,
						Column:   3,
						Operator: "+",
						Left:     &Number{Line: 1, Column: 1, Value: "1"},
						Right:    &Number{Line: 1, Column: 3, Value: "2"},
					},
					Right: &Number{Line: 1, Column: 7, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 11, Value: "4"},
			},
			expected: `$1 + 2 + 3 + 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_Generate_FloatingPoint(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "floating point multiplication: 3.14 * 2",
			ast: &BinaryOp{
				Line:     1,
				Column:   6,
				Operator: "*",
				Left:     &Number{Line: 1, Column: 1, Value: "3.14"},
				Right:    &Number{Line: 1, Column: 6, Value: "2"},
			},
			expected: `$3.14 \times 2$`,
		},
		{
			name: "floating point addition: 1.5 + 0.5",
			ast: &BinaryOp{
				Line:     1,
				Column:   5,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "1.5"},
				Right:    &Number{Line: 1, Column: 5, Value: "0.5"},
			},
			expected: `$1.5 + 0.5$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_Generate_ComplexExpressions(t *testing.T) {
	tests := []struct {
		name     string
		ast      Expr
		expected string
	}{
		{
			name: "multiple parenthesized subexpressions: (1 + 2) * (3 + 4)",
			ast: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "*",
				Left: &BinaryOp{
					Line:     1,
					Column:   3,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 1, Value: "1"},
					Right:    &Number{Line: 1, Column: 3, Value: "2"},
				},
				Right: &BinaryOp{
					Line:     1,
					Column:   11,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 9, Value: "3"},
					Right:    &Number{Line: 1, Column: 11, Value: "4"},
				},
			},
			expected: `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name: "complex expression: (10 / 2 + 3) * 4",
			ast: &BinaryOp{
				Line:     1,
				Column:   13,
				Operator: "*",
				Left: &BinaryOp{
					Line:     1,
					Column:   9,
					Operator: "+",
					Left: &BinaryOp{
						Line:     1,
						Column:   4,
						Operator: "/",
						Left:     &Number{Line: 1, Column: 1, Value: "10"},
						Right:    &Number{Line: 1, Column: 4, Value: "2"},
					},
					Right: &Number{Line: 1, Column: 9, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 13, Value: "4"},
			},
			expected: `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.Generate(tt.ast)
			if result != tt.expected {
				t.Errorf("Generate() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_visitNumber(t *testing.T) {
	tests := []struct {
		name     string
		number   *Number
		expected string
	}{
		{
			name:     "integer",
			number:   &Number{Line: 1, Column: 1, Value: "42"},
			expected: "42",
		},
		{
			name:     "float",
			number:   &Number{Line: 1, Column: 1, Value: "3.14"},
			expected: "3.14",
		},
		{
			name:     "negative integer",
			number:   &Number{Line: 1, Column: 1, Value: "-10"},
			expected: "-10",
		},
		{
			name:     "negative float",
			number:   &Number{Line: 1, Column: 1, Value: "-2.5"},
			expected: "-2.5",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewLaTeXGenerator()
			result := gen.visitNumber(tt.number)
			if result != tt.expected {
				t.Errorf("visitNumber() = %q, expected %q", result, tt.expected)
			}
		})
	}
}

func TestLaTeXGenerator_needsParens(t *testing.T) {
	gen := NewLaTeXGenerator()

	tests := []struct {
		name             string
		child            Expr
		parentPrecedence int
		isRight          bool
		expected         bool
	}{
		{
			name:             "number never needs parens",
			child:            &Number{Line: 1, Column: 1, Value: "5"},
			parentPrecedence: 2,
			isRight:          false,
			expected:         false,
		},
		{
			name: "lower precedence child needs parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 2,
			isRight:          false,
			expected:         true,
		},
		{
			name: "equal precedence on left side (addition) - no parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
		{
			name: "equal precedence on right side (addition) - no parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          true,
			expected:         false,
		},
		{
			name: "equal precedence on right side (subtraction) - needs parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "-",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          true,
			expected:         true,
		},
		{
			name: "equal precedence on right side (division) - needs parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "/",
				Left:     &Number{Line: 1, Column: 1, Value: "10"},
				Right:    &Number{Line: 1, Column: 3, Value: "2"},
			},
			parentPrecedence: 2,
			isRight:          true,
			expected:         true,
		},
		{
			name: "equal precedence on right side (multiplication) - no parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "*",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 2,
			isRight:          true,
			expected:         false,
		},
		{
			name: "higher precedence child - no parens",
			child: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "*",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := gen.needsParens(tt.child, tt.parentPrecedence, tt.isRight)
			if result != tt.expected {
				t.Errorf("needsParens() = %v, expected %v", result, tt.expected)
			}
		})
	}
}

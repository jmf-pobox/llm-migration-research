package rpn2tex

import (
	"testing"
)

func TestLaTeXGenerator_Generate(t *testing.T) {
	tests := []struct {
		name string
		ast  Expr
		want string
	}{
		{
			name: "simple number",
			ast:  &Number{Value: "5"},
			want: "$5$",
		},
		{
			name: "decimal number",
			ast:  &Number{Value: "3.14"},
			want: "$3.14$",
		},
		{
			name: "basic addition",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			want: "$5 + 3$",
		},
		{
			name: "basic subtraction",
			ast: &BinaryOp{
				Operator: "-",
				Left:     &Number{Value: "5"},
				Right:    &Number{Value: "3"},
			},
			want: "$5 - 3$",
		},
		{
			name: "basic multiplication",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "4"},
				Right:    &Number{Value: "7"},
			},
			want: `$4 \times 7$`,
		},
		{
			name: "basic division",
			ast: &BinaryOp{
				Operator: "/",
				Left:     &Number{Value: "10"},
				Right:    &Number{Value: "2"},
			},
			want: `$10 \div 2$`,
		},
		{
			name: "addition then multiplication (needs parens on left)",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "5"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "2"},
			},
			want: `$( 5 + 3 ) \times 2$`,
		},
		{
			name: "multiplication then addition (no parens needed)",
			ast: &BinaryOp{
				Operator: "+",
				Left: &BinaryOp{
					Operator: "*",
					Left:     &Number{Value: "5"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "2"},
			},
			want: `$5 \times 3 + 2$`,
		},
		{
			name: "division then multiplication (same precedence, no parens)",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "/",
					Left:     &Number{Value: "10"},
					Right:    &Number{Value: "2"},
				},
				Right: &Number{Value: "5"},
			},
			want: `$10 \div 2 \times 5$`,
		},
		{
			name: "left-associative subtraction (no parens)",
			ast: &BinaryOp{
				Operator: "-",
				Left: &BinaryOp{
					Operator: "-",
					Left:     &Number{Value: "5"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "2"},
			},
			want: "$5 - 3 - 2$",
		},
		{
			name: "multiple divisions (left-associative)",
			ast: &BinaryOp{
				Operator: "/",
				Left: &BinaryOp{
					Operator: "/",
					Left: &BinaryOp{
						Operator: "/",
						Left:     &Number{Value: "100"},
						Right:    &Number{Value: "10"},
					},
					Right: &Number{Value: "5"},
				},
				Right: &Number{Value: "2"},
			},
			want: `$100 \div 10 \div 5 \div 2$`,
		},
		{
			name: "multiple additions (associative)",
			ast: &BinaryOp{
				Operator: "+",
				Left: &BinaryOp{
					Operator: "+",
					Left: &BinaryOp{
						Operator: "+",
						Left:     &Number{Value: "1"},
						Right:    &Number{Value: "2"},
					},
					Right: &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			want: "$1 + 2 + 3 + 4$",
		},
		{
			name: "addition on right of multiplication (needs parens)",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "2"},
				Right: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "3"},
					Right:    &Number{Value: "4"},
				},
			},
			want: `$2 \times ( 3 + 4 )$`,
		},
		{
			name: "multiplication on right of addition (no parens)",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "2"},
				Right: &BinaryOp{
					Operator: "*",
					Left:     &Number{Value: "3"},
					Right:    &Number{Value: "4"},
				},
			},
			want: `$2 + 3 \times 4$`,
		},
		{
			name: "addition on left of multiplication (needs parens)",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "2"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			want: `$( 2 + 3 ) \times 4$`,
		},
		{
			name: "multiplication on left of addition (no parens)",
			ast: &BinaryOp{
				Operator: "+",
				Left: &BinaryOp{
					Operator: "*",
					Left:     &Number{Value: "2"},
					Right:    &Number{Value: "3"},
				},
				Right: &Number{Value: "4"},
			},
			want: `$2 \times 3 + 4$`,
		},
		{
			name: "decimal multiplication",
			ast: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "3.14"},
				Right:    &Number{Value: "2"},
			},
			want: `$3.14 \times 2$`,
		},
		{
			name: "decimal addition",
			ast: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1.5"},
				Right:    &Number{Value: "0.5"},
			},
			want: "$1.5 + 0.5$",
		},
		{
			name: "two additions multiplied (both need parens)",
			ast: &BinaryOp{
				Operator: "*",
				Left: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "1"},
					Right:    &Number{Value: "2"},
				},
				Right: &BinaryOp{
					Operator: "+",
					Left:     &Number{Value: "3"},
					Right:    &Number{Value: "4"},
				},
			},
			want: `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name: "complex expression",
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
			want: `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			g := NewLaTeXGenerator()
			got := g.Generate(tt.ast)
			if got != tt.want {
				t.Errorf("Generate() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestLaTeXGenerator_OperatorMapping(t *testing.T) {
	tests := []struct {
		operator string
		want     string
	}{
		{"+", "+"},
		{"-", "-"},
		{"*", `\times`},
		{"/", `\div`},
	}

	g := NewLaTeXGenerator()
	for _, tt := range tests {
		t.Run(tt.operator, func(t *testing.T) {
			got := g.binaryOps[tt.operator]
			if got != tt.want {
				t.Errorf("binaryOps[%q] = %q, want %q", tt.operator, got, tt.want)
			}
		})
	}
}

func TestLaTeXGenerator_Precedence(t *testing.T) {
	tests := []struct {
		operator   string
		precedence int
	}{
		{"+", 1},
		{"-", 1},
		{"*", 2},
		{"/", 2},
	}

	g := NewLaTeXGenerator()
	for _, tt := range tests {
		t.Run(tt.operator, func(t *testing.T) {
			got := g.precedence[tt.operator]
			if got != tt.precedence {
				t.Errorf("precedence[%q] = %d, want %d", tt.operator, got, tt.precedence)
			}
		})
	}
}

func TestLaTeXGenerator_NeedsParens(t *testing.T) {
	g := NewLaTeXGenerator()

	tests := []struct {
		name             string
		child            Expr
		parentPrecedence int
		isRight          bool
		want             bool
	}{
		{
			name:             "number never needs parens",
			child:            &Number{Value: "5"},
			parentPrecedence: 2,
			isRight:          true,
			want:             false,
		},
		{
			name: "lower precedence needs parens",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 2,
			isRight:          false,
			want:             true,
		},
		{
			name: "equal precedence on left doesn't need parens",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 1,
			isRight:          false,
			want:             false,
		},
		{
			name: "equal precedence subtraction on right needs parens",
			child: &BinaryOp{
				Operator: "-",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 1,
			isRight:          true,
			want:             true,
		},
		{
			name: "equal precedence division on right needs parens",
			child: &BinaryOp{
				Operator: "/",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 2,
			isRight:          true,
			want:             true,
		},
		{
			name: "equal precedence addition on right doesn't need parens",
			child: &BinaryOp{
				Operator: "+",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 1,
			isRight:          true,
			want:             false,
		},
		{
			name: "equal precedence multiplication on right doesn't need parens",
			child: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 2,
			isRight:          true,
			want:             false,
		},
		{
			name: "higher precedence never needs parens",
			child: &BinaryOp{
				Operator: "*",
				Left:     &Number{Value: "1"},
				Right:    &Number{Value: "2"},
			},
			parentPrecedence: 1,
			isRight:          true,
			want:             false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := g.needsParens(tt.child, tt.parentPrecedence, tt.isRight)
			if got != tt.want {
				t.Errorf("needsParens() = %v, want %v", got, tt.want)
			}
		})
	}
}

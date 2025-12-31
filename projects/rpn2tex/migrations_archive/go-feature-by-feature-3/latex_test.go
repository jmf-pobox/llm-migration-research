package rpn2tex

import "testing"

func TestGeneratorNumbers(t *testing.T) {
	tests := []struct {
		name string
		expr Expr
		want string
	}{
		{
			name: "single integer",
			expr: &Number{Line: 1, Column: 1, Value: "5"},
			want: "$5$",
		},
		{
			name: "floating point",
			expr: &Number{Line: 1, Column: 1, Value: "3.14"},
			want: "$3.14$",
		},
		{
			name: "large number",
			expr: &Number{Line: 1, Column: 1, Value: "123456"},
			want: "$123456$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewGenerator()
			got := gen.Generate(tt.expr)
			if got != tt.want {
				t.Errorf("Generate() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestGeneratorAddition(t *testing.T) {
	tests := []struct {
		name string
		expr Expr
		want string
	}{
		{
			name: "simple addition",
			expr: &BinaryOp{
				Line:     1,
				Column:   5,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			want: "$5 + 3$",
		},
		{
			name: "chained addition",
			expr: &BinaryOp{
				Line:     1,
				Column:   9,
				Operator: "+",
				Left: &BinaryOp{
					Line:     1,
					Column:   5,
					Operator: "+",
					Left:     &Number{Line: 1, Column: 1, Value: "1"},
					Right:    &Number{Line: 1, Column: 3, Value: "2"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "3"},
			},
			want: "$1 + 2 + 3$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewGenerator()
			got := gen.Generate(tt.expr)
			if got != tt.want {
				t.Errorf("Generate() = %q, want %q", got, tt.want)
			}
		})
	}
}

func TestGeneratorSubtraction(t *testing.T) {
	tests := []struct {
		name string
		expr Expr
		want string
	}{
		{
			name: "simple subtraction",
			expr: &BinaryOp{
				Line:     1,
				Column:   5,
				Operator: "-",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 3, Value: "3"},
			},
			want: "$5 - 3$",
		},
		{
			name: "chained subtraction",
			expr: &BinaryOp{
				Line:     1,
				Column:   9,
				Operator: "-",
				Left: &BinaryOp{
					Line:     1,
					Column:   5,
					Operator: "-",
					Left:     &Number{Line: 1, Column: 1, Value: "5"},
					Right:    &Number{Line: 1, Column: 3, Value: "3"},
				},
				Right: &Number{Line: 1, Column: 7, Value: "2"},
			},
			want: "$5 - 3 - 2$",
		},
		{
			name: "subtraction with negative number",
			expr: &BinaryOp{
				Line:     1,
				Column:   7,
				Operator: "-",
				Left:     &Number{Line: 1, Column: 1, Value: "10"},
				Right:    &Number{Line: 1, Column: 4, Value: "-5"},
			},
			want: "$10 - -5$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			gen := NewGenerator()
			got := gen.Generate(tt.expr)
			if got != tt.want {
				t.Errorf("Generate() = %q, want %q", got, tt.want)
			}
		})
	}
}

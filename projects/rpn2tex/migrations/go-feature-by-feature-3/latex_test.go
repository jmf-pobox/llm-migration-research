package rpn2tex

import (
	"testing"
)

func TestLaTeXGenerator_Number(t *testing.T) {
	node := &Number{Value: "5", Line: 1, Column: 1}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$5$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_Float(t *testing.T) {
	node := &Number{Value: "3.14", Line: 1, Column: 1}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$3.14$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_Addition(t *testing.T) {
	node := &BinaryOp{
		Operator: "+",
		Left:     &Number{Value: "5", Line: 1, Column: 1},
		Right:    &Number{Value: "3", Line: 1, Column: 3},
		Line:     1,
		Column:   5,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$5 + 3$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_ChainedAddition(t *testing.T) {
	// (1 + 2) + 3
	node := &BinaryOp{
		Operator: "+",
		Left: &BinaryOp{
			Operator: "+",
			Left:     &Number{Value: "1", Line: 1, Column: 1},
			Right:    &Number{Value: "2", Line: 1, Column: 3},
			Line:     1,
			Column:   5,
		},
		Right:  &Number{Value: "3", Line: 1, Column: 7},
		Line:   1,
		Column: 9,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$1 + 2 + 3$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_Subtraction(t *testing.T) {
	node := &BinaryOp{
		Operator: "-",
		Left:     &Number{Value: "5", Line: 1, Column: 1},
		Right:    &Number{Value: "3", Line: 1, Column: 3},
		Line:     1,
		Column:   5,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$5 - 3$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_ChainedSubtraction(t *testing.T) {
	// (5 - 3) - 2
	node := &BinaryOp{
		Operator: "-",
		Left: &BinaryOp{
			Operator: "-",
			Left:     &Number{Value: "5", Line: 1, Column: 1},
			Right:    &Number{Value: "3", Line: 1, Column: 3},
			Line:     1,
			Column:   5,
		},
		Right:  &Number{Value: "2", Line: 1, Column: 7},
		Line:   1,
		Column: 9,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$5 - 3 - 2$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_Multiplication(t *testing.T) {
	node := &BinaryOp{
		Operator: "*",
		Left:     &Number{Value: "4", Line: 1, Column: 1},
		Right:    &Number{Value: "7", Line: 1, Column: 3},
		Line:     1,
		Column:   5,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$4 \\times 7$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_MultiplicationWithAddition(t *testing.T) {
	// 2 + (3 * 4)
	node := &BinaryOp{
		Operator: "+",
		Left:     &Number{Value: "2", Line: 1, Column: 1},
		Right: &BinaryOp{
			Operator: "*",
			Left:     &Number{Value: "3", Line: 1, Column: 3},
			Right:    &Number{Value: "4", Line: 1, Column: 5},
			Line:     1,
			Column:   7,
		},
		Line:   1,
		Column: 9,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	// Note: At this stage (feature 4), we don't have precedence handling yet
	// So this will be rendered as "2 + 3 \times 4" without parentheses
	expected := "$2 + 3 \\times 4$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_Division(t *testing.T) {
	node := &BinaryOp{
		Operator: "/",
		Left:     &Number{Value: "10", Line: 1, Column: 1},
		Right:    &Number{Value: "2", Line: 1, Column: 4},
		Line:     1,
		Column:   6,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$10 \\div 2$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

func TestLaTeXGenerator_ChainedDivision(t *testing.T) {
	// ((100 / 10) / 5) / 2
	node := &BinaryOp{
		Operator: "/",
		Left: &BinaryOp{
			Operator: "/",
			Left: &BinaryOp{
				Operator: "/",
				Left:     &Number{Value: "100", Line: 1, Column: 1},
				Right:    &Number{Value: "10", Line: 1, Column: 5},
				Line:     1,
				Column:   8,
			},
			Right:  &Number{Value: "5", Line: 1, Column: 11},
			Line:   1,
			Column: 13,
		},
		Right:  &Number{Value: "2", Line: 1, Column: 16},
		Line:   1,
		Column: 18,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$100 \\div 10 \\div 5 \\div 2$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

// Test precedence helper function
func TestPrecedenceOf(t *testing.T) {
	generator := NewLaTeXGenerator()

	tests := []struct {
		name     string
		operator string
		want     int
	}{
		{"addition", "+", 1},
		{"subtraction", "-", 1},
		{"multiplication", "*", 2},
		{"division", "/", 2},
		{"unknown", "^", 0},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := generator.precedenceOf(tt.operator)
			if got != tt.want {
				t.Errorf("precedenceOf(%q) = %d, want %d", tt.operator, got, tt.want)
			}
		})
	}
}

// Test needsParens function
func TestNeedsParens(t *testing.T) {
	generator := NewLaTeXGenerator()

	tests := []struct {
		name       string
		child      Expr
		parentPrec int
		isRight    bool
		want       bool
	}{
		{
			name:       "number never needs parens",
			child:      &Number{Value: "5"},
			parentPrec: 2,
			isRight:    false,
			want:       false,
		},
		{
			name:       "lower precedence child needs parens (left)",
			child:      &BinaryOp{Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 2,
			isRight:    false,
			want:       true,
		},
		{
			name:       "lower precedence child needs parens (right)",
			child:      &BinaryOp{Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 2,
			isRight:    true,
			want:       true,
		},
		{
			name:       "equal precedence on left, addition - no parens",
			child:      &BinaryOp{Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 1,
			isRight:    false,
			want:       false,
		},
		{
			name:       "equal precedence on right, addition - no parens",
			child:      &BinaryOp{Operator: "+", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 1,
			isRight:    true,
			want:       false,
		},
		{
			name:       "equal precedence on left, subtraction - no parens",
			child:      &BinaryOp{Operator: "-", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 1,
			isRight:    false,
			want:       false,
		},
		{
			name:       "equal precedence on right, subtraction - needs parens",
			child:      &BinaryOp{Operator: "-", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 1,
			isRight:    true,
			want:       true,
		},
		{
			name:       "equal precedence on right, division - needs parens",
			child:      &BinaryOp{Operator: "/", Left: &Number{Value: "10"}, Right: &Number{Value: "2"}},
			parentPrec: 2,
			isRight:    true,
			want:       true,
		},
		{
			name:       "equal precedence on right, multiplication - no parens",
			child:      &BinaryOp{Operator: "*", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 2,
			isRight:    true,
			want:       false,
		},
		{
			name:       "higher precedence child - no parens",
			child:      &BinaryOp{Operator: "*", Left: &Number{Value: "5"}, Right: &Number{Value: "3"}},
			parentPrec: 1,
			isRight:    false,
			want:       false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := generator.needsParens(tt.child, tt.parentPrec, tt.isRight)
			if got != tt.want {
				t.Errorf("needsParens() = %v, want %v", got, tt.want)
			}
		})
	}
}

// Test precedence with parenthesization - addition on left of multiplication
func TestLaTeXGenerator_PrecedenceAdditionLeftOfMultiplication(t *testing.T) {
	// (5 + 3) * 2
	node := &BinaryOp{
		Operator: "*",
		Left: &BinaryOp{
			Operator: "+",
			Left:     &Number{Value: "5", Line: 1, Column: 1},
			Right:    &Number{Value: "3", Line: 1, Column: 3},
			Line:     1,
			Column:   5,
		},
		Right:  &Number{Value: "2", Line: 1, Column: 7},
		Line:   1,
		Column: 9,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$( 5 + 3 ) \\times 2$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

// Test precedence with parenthesization - addition on right of multiplication
func TestLaTeXGenerator_PrecedenceAdditionRightOfMultiplication(t *testing.T) {
	// 2 * (3 + 4)
	node := &BinaryOp{
		Operator: "*",
		Left:     &Number{Value: "2", Line: 1, Column: 1},
		Right: &BinaryOp{
			Operator: "+",
			Left:     &Number{Value: "3", Line: 1, Column: 3},
			Right:    &Number{Value: "4", Line: 1, Column: 5},
			Line:     1,
			Column:   7,
		},
		Line:   1,
		Column: 9,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$2 \\times ( 3 + 4 )$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

// Test precedence with both sides having lower precedence
func TestLaTeXGenerator_PrecedenceBothSidesLowerPrecedence(t *testing.T) {
	// (1 + 2) * (3 + 4)
	node := &BinaryOp{
		Operator: "*",
		Left: &BinaryOp{
			Operator: "+",
			Left:     &Number{Value: "1", Line: 1, Column: 1},
			Right:    &Number{Value: "2", Line: 1, Column: 3},
			Line:     1,
			Column:   5,
		},
		Right: &BinaryOp{
			Operator: "+",
			Left:     &Number{Value: "3", Line: 1, Column: 7},
			Right:    &Number{Value: "4", Line: 1, Column: 9},
			Line:     1,
			Column:   11,
		},
		Line:   1,
		Column: 13,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$( 1 + 2 ) \\times ( 3 + 4 )$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

// Test complex precedence with division and addition
func TestLaTeXGenerator_PrecedenceComplexDivisionAddition(t *testing.T) {
	// (10 / 2 + 3) * 4
	node := &BinaryOp{
		Operator: "*",
		Left: &BinaryOp{
			Operator: "+",
			Left: &BinaryOp{
				Operator: "/",
				Left:     &Number{Value: "10", Line: 1, Column: 1},
				Right:    &Number{Value: "2", Line: 1, Column: 4},
				Line:     1,
				Column:   6,
			},
			Right:  &Number{Value: "3", Line: 1, Column: 8},
			Line:   1,
			Column: 10,
		},
		Right:  &Number{Value: "4", Line: 1, Column: 12},
		Line:   1,
		Column: 14,
	}
	generator := NewLaTeXGenerator()
	output := generator.Generate(node)

	expected := "$( 10 \\div 2 + 3 ) \\times 4$"
	if output != expected {
		t.Errorf("expected '%s', got '%s'", expected, output)
	}
}

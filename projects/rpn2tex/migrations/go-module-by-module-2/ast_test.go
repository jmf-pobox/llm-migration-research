package rpn2tex

import (
	"testing"
)

func TestNewNumber(t *testing.T) {
	tests := []struct {
		name   string
		line   int
		column int
		value  string
	}{
		{
			name:   "integer number",
			line:   1,
			column: 1,
			value:  "42",
		},
		{
			name:   "decimal number",
			line:   1,
			column: 5,
			value:  "3.14",
		},
		{
			name:   "negative number",
			line:   2,
			column: 3,
			value:  "-5",
		},
		{
			name:   "zero",
			line:   1,
			column: 1,
			value:  "0",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			num := NewNumber(tt.line, tt.column, tt.value)

			if num.Line != tt.line {
				t.Errorf("NewNumber() Line = %d, want %d", num.Line, tt.line)
			}
			if num.Column != tt.column {
				t.Errorf("NewNumber() Column = %d, want %d", num.Column, tt.column)
			}
			if num.Value != tt.value {
				t.Errorf("NewNumber() Value = %q, want %q", num.Value, tt.value)
			}
		})
	}
}

func TestNewBinaryOp(t *testing.T) {
	tests := []struct {
		name     string
		line     int
		column   int
		operator string
		left     Expr
		right    Expr
	}{
		{
			name:     "addition",
			line:     1,
			column:   5,
			operator: "+",
			left:     NewNumber(1, 1, "5"),
			right:    NewNumber(1, 3, "3"),
		},
		{
			name:     "subtraction",
			line:     1,
			column:   5,
			operator: "-",
			left:     NewNumber(1, 1, "10"),
			right:    NewNumber(1, 3, "2"),
		},
		{
			name:     "multiplication",
			line:     1,
			column:   5,
			operator: "*",
			left:     NewNumber(1, 1, "4"),
			right:    NewNumber(1, 3, "7"),
		},
		{
			name:     "division",
			line:     1,
			column:   5,
			operator: "/",
			left:     NewNumber(1, 1, "10"),
			right:    NewNumber(1, 3, "2"),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			op := NewBinaryOp(tt.line, tt.column, tt.operator, tt.left, tt.right)

			if op.Line != tt.line {
				t.Errorf("NewBinaryOp() Line = %d, want %d", op.Line, tt.line)
			}
			if op.Column != tt.column {
				t.Errorf("NewBinaryOp() Column = %d, want %d", op.Column, tt.column)
			}
			if op.Operator != tt.operator {
				t.Errorf("NewBinaryOp() Operator = %q, want %q", op.Operator, tt.operator)
			}
			if op.Left != tt.left {
				t.Errorf("NewBinaryOp() Left = %v, want %v", op.Left, tt.left)
			}
			if op.Right != tt.right {
				t.Errorf("NewBinaryOp() Right = %v, want %v", op.Right, tt.right)
			}
		})
	}
}

func TestExprInterface(t *testing.T) {
	t.Run("Number implements Expr", func(t *testing.T) {
		var _ Expr = &Number{}
		var _ Expr = NewNumber(1, 1, "42")
	})

	t.Run("BinaryOp implements Expr", func(t *testing.T) {
		var _ Expr = &BinaryOp{}
		var _ Expr = NewBinaryOp(1, 1, "+", nil, nil)
	})
}

func TestRecursiveTree(t *testing.T) {
	// Build: 5 3 + 2 * => (5 + 3) * 2
	five := NewNumber(1, 1, "5")
	three := NewNumber(1, 3, "3")
	addition := NewBinaryOp(1, 5, "+", five, three)
	two := NewNumber(1, 7, "2")
	multiplication := NewBinaryOp(1, 9, "*", addition, two)

	// Verify structure
	if multiplication.Operator != "*" {
		t.Errorf("Root operator = %q, want %q", multiplication.Operator, "*")
	}

	// Verify left subtree
	leftOp, ok := multiplication.Left.(*BinaryOp)
	if !ok {
		t.Fatal("Left operand is not a BinaryOp")
	}
	if leftOp.Operator != "+" {
		t.Errorf("Left operator = %q, want %q", leftOp.Operator, "+")
	}

	// Verify left subtree operands
	leftNum, ok := leftOp.Left.(*Number)
	if !ok {
		t.Fatal("Left.Left is not a Number")
	}
	if leftNum.Value != "5" {
		t.Errorf("Left.Left.Value = %q, want %q", leftNum.Value, "5")
	}

	rightNum, ok := leftOp.Right.(*Number)
	if !ok {
		t.Fatal("Left.Right is not a Number")
	}
	if rightNum.Value != "3" {
		t.Errorf("Left.Right.Value = %q, want %q", rightNum.Value, "3")
	}

	// Verify right operand
	rightOperand, ok := multiplication.Right.(*Number)
	if !ok {
		t.Fatal("Right operand is not a Number")
	}
	if rightOperand.Value != "2" {
		t.Errorf("Right.Value = %q, want %q", rightOperand.Value, "2")
	}
}

func TestTypeAssertions(t *testing.T) {
	t.Run("Number type assertion", func(t *testing.T) {
		var expr Expr = NewNumber(1, 1, "42")

		num, ok := expr.(*Number)
		if !ok {
			t.Fatal("Type assertion to *Number failed")
		}
		if num.Value != "42" {
			t.Errorf("Number.Value = %q, want %q", num.Value, "42")
		}
	})

	t.Run("BinaryOp type assertion", func(t *testing.T) {
		var expr Expr = NewBinaryOp(1, 1, "+",
			NewNumber(1, 1, "5"),
			NewNumber(1, 3, "3"))

		op, ok := expr.(*BinaryOp)
		if !ok {
			t.Fatal("Type assertion to *BinaryOp failed")
		}
		if op.Operator != "+" {
			t.Errorf("BinaryOp.Operator = %q, want %q", op.Operator, "+")
		}
	})

	t.Run("incorrect type assertion", func(t *testing.T) {
		var expr Expr = NewNumber(1, 1, "42")

		_, ok := expr.(*BinaryOp)
		if ok {
			t.Error("Type assertion to *BinaryOp should have failed for Number")
		}
	})
}

func TestPositionTracking(t *testing.T) {
	t.Run("Number position", func(t *testing.T) {
		num := NewNumber(5, 10, "42")
		if num.Line != 5 {
			t.Errorf("Number.Line = %d, want %d", num.Line, 5)
		}
		if num.Column != 10 {
			t.Errorf("Number.Column = %d, want %d", num.Column, 10)
		}
	})

	t.Run("BinaryOp position", func(t *testing.T) {
		op := NewBinaryOp(3, 7, "*", nil, nil)
		if op.Line != 3 {
			t.Errorf("BinaryOp.Line = %d, want %d", op.Line, 3)
		}
		if op.Column != 7 {
			t.Errorf("BinaryOp.Column = %d, want %d", op.Column, 7)
		}
	})
}

func TestComplexExpression(t *testing.T) {
	// Build: 2 3 4 * + => 2 + (3 * 4)
	two := NewNumber(1, 1, "2")
	three := NewNumber(1, 3, "3")
	four := NewNumber(1, 5, "4")
	multiply := NewBinaryOp(1, 7, "*", three, four)
	add := NewBinaryOp(1, 9, "+", two, multiply)

	// Verify root
	if add.Operator != "+" {
		t.Errorf("Root operator = %q, want %q", add.Operator, "+")
	}

	// Verify left is Number
	leftNum, ok := add.Left.(*Number)
	if !ok {
		t.Fatal("Left operand is not a Number")
	}
	if leftNum.Value != "2" {
		t.Errorf("Left.Value = %q, want %q", leftNum.Value, "2")
	}

	// Verify right is BinaryOp
	rightOp, ok := add.Right.(*BinaryOp)
	if !ok {
		t.Fatal("Right operand is not a BinaryOp")
	}
	if rightOp.Operator != "*" {
		t.Errorf("Right.Operator = %q, want %q", rightOp.Operator, "*")
	}

	// Verify right's operands
	rightLeft, ok := rightOp.Left.(*Number)
	if !ok {
		t.Fatal("Right.Left is not a Number")
	}
	if rightLeft.Value != "3" {
		t.Errorf("Right.Left.Value = %q, want %q", rightLeft.Value, "3")
	}

	rightRight, ok := rightOp.Right.(*Number)
	if !ok {
		t.Fatal("Right.Right is not a Number")
	}
	if rightRight.Value != "4" {
		t.Errorf("Right.Right.Value = %q, want %q", rightRight.Value, "4")
	}
}

func TestDecimalPreservation(t *testing.T) {
	tests := []struct {
		name  string
		value string
	}{
		{"pi approximation", "3.14"},
		{"decimal with trailing zero", "1.50"},
		{"leading zero", "0.5"},
		{"large decimal", "123.456789"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			num := NewNumber(1, 1, tt.value)
			if num.Value != tt.value {
				t.Errorf("Number.Value = %q, want %q (decimal not preserved)", num.Value, tt.value)
			}
		})
	}
}

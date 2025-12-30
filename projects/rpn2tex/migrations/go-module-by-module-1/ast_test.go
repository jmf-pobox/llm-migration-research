package main

import (
	"testing"
)

// TestNumberImplementsExpr verifies that Number implements the Expr interface.
func TestNumberImplementsExpr(t *testing.T) {
	var _ Expr = &Number{}
}

// TestBinaryOpImplementsExpr verifies that BinaryOp implements the Expr interface.
func TestBinaryOpImplementsExpr(t *testing.T) {
	var _ Expr = &BinaryOp{}
}

// TestNumberCreation tests creation of Number nodes.
func TestNumberCreation(t *testing.T) {
	tests := []struct {
		name   string
		line   int
		column int
		value  string
	}{
		{"integer", 1, 1, "42"},
		{"negative", 1, 5, "-3"},
		{"decimal", 2, 10, "3.14"},
		{"zero", 3, 1, "0"},
		{"large number", 4, 15, "123456.789"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			num := &Number{
				Line:   tt.line,
				Column: tt.column,
				Value:  tt.value,
			}

			if num.Line != tt.line {
				t.Errorf("Line = %d, want %d", num.Line, tt.line)
			}
			if num.Column != tt.column {
				t.Errorf("Column = %d, want %d", num.Column, tt.column)
			}
			if num.Value != tt.value {
				t.Errorf("Value = %q, want %q", num.Value, tt.value)
			}
		})
	}
}

// TestNumberGetters tests the getter methods of Number.
func TestNumberGetters(t *testing.T) {
	num := &Number{
		Line:   5,
		Column: 12,
		Value:  "99",
	}

	if got := num.GetLine(); got != 5 {
		t.Errorf("GetLine() = %d, want 5", got)
	}
	if got := num.GetColumn(); got != 12 {
		t.Errorf("GetColumn() = %d, want 12", got)
	}
}

// TestBinaryOpCreation tests creation of BinaryOp nodes.
func TestBinaryOpCreation(t *testing.T) {
	left := &Number{Line: 1, Column: 1, Value: "5"}
	right := &Number{Line: 1, Column: 3, Value: "3"}

	tests := []struct {
		name     string
		line     int
		column   int
		operator string
		left     Expr
		right    Expr
	}{
		{"addition", 1, 5, "+", left, right},
		{"subtraction", 1, 5, "-", left, right},
		{"multiplication", 1, 5, "*", left, right},
		{"division", 1, 5, "/", left, right},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			op := &BinaryOp{
				Line:     tt.line,
				Column:   tt.column,
				Operator: tt.operator,
				Left:     tt.left,
				Right:    tt.right,
			}

			if op.Line != tt.line {
				t.Errorf("Line = %d, want %d", op.Line, tt.line)
			}
			if op.Column != tt.column {
				t.Errorf("Column = %d, want %d", op.Column, tt.column)
			}
			if op.Operator != tt.operator {
				t.Errorf("Operator = %q, want %q", op.Operator, tt.operator)
			}
			if op.Left != tt.left {
				t.Errorf("Left node mismatch")
			}
			if op.Right != tt.right {
				t.Errorf("Right node mismatch")
			}
		})
	}
}

// TestBinaryOpGetters tests the getter methods of BinaryOp.
func TestBinaryOpGetters(t *testing.T) {
	left := &Number{Line: 1, Column: 1, Value: "10"}
	right := &Number{Line: 1, Column: 4, Value: "2"}

	op := &BinaryOp{
		Line:     1,
		Column:   6,
		Operator: "/",
		Left:     left,
		Right:    right,
	}

	if got := op.GetLine(); got != 1 {
		t.Errorf("GetLine() = %d, want 1", got)
	}
	if got := op.GetColumn(); got != 6 {
		t.Errorf("GetColumn() = %d, want 6", got)
	}
}

// TestNestedBinaryOp tests nested binary operations.
func TestNestedBinaryOp(t *testing.T) {
	// Build AST for: (5 + 3) * 2
	// RPN: 5 3 + 2 *
	num5 := &Number{Line: 1, Column: 1, Value: "5"}
	num3 := &Number{Line: 1, Column: 3, Value: "3"}
	num2 := &Number{Line: 1, Column: 7, Value: "2"}

	// 5 + 3
	add := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     num5,
		Right:    num3,
	}

	// (5 + 3) * 2
	mult := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left:     add,
		Right:    num2,
	}

	// Verify structure
	if mult.Operator != "*" {
		t.Errorf("Root operator = %q, want \"*\"", mult.Operator)
	}

	leftOp, ok := mult.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("Left child is not BinaryOp")
	}
	if leftOp.Operator != "+" {
		t.Errorf("Left operator = %q, want \"+\"", leftOp.Operator)
	}

	rightNum, ok := mult.Right.(*Number)
	if !ok {
		t.Fatalf("Right child is not Number")
	}
	if rightNum.Value != "2" {
		t.Errorf("Right value = %q, want \"2\"", rightNum.Value)
	}
}

// TestExprInterface tests that both types can be used through the Expr interface.
func TestExprInterface(t *testing.T) {
	exprs := []Expr{
		&Number{Line: 1, Column: 1, Value: "42"},
		&BinaryOp{
			Line:     1,
			Column:   5,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 1, Value: "1"},
			Right:    &Number{Line: 1, Column: 3, Value: "2"},
		},
	}

	for i, expr := range exprs {
		if expr.GetLine() != 1 {
			t.Errorf("expr[%d].GetLine() = %d, want 1", i, expr.GetLine())
		}
		// Verify exprNode() can be called (marker method)
		expr.exprNode()
	}
}

// TestImmutability tests that struct fields can be read and assigned but structs are immutable.
func TestImmutability(t *testing.T) {
	// Create a Number
	num := &Number{Line: 1, Column: 1, Value: "5"}

	// Reading fields works
	if num.Value != "5" {
		t.Errorf("Value = %q, want \"5\"", num.Value)
	}

	// Create a BinaryOp
	op := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     num,
		Right:    &Number{Line: 1, Column: 5, Value: "3"},
	}

	// Reading fields works
	if op.Operator != "+" {
		t.Errorf("Operator = %q, want \"+\"", op.Operator)
	}

	// Verify position information is preserved
	if op.Left.GetLine() != 1 {
		t.Errorf("Left.GetLine() = %d, want 1", op.Left.GetLine())
	}
}

// TestPositionTracking tests that position information is properly tracked.
func TestPositionTracking(t *testing.T) {
	tests := []struct {
		name   string
		expr   Expr
		line   int
		column int
	}{
		{
			name:   "number at position 1:1",
			expr:   &Number{Line: 1, Column: 1, Value: "42"},
			line:   1,
			column: 1,
		},
		{
			name:   "number at position 5:10",
			expr:   &Number{Line: 5, Column: 10, Value: "3.14"},
			line:   5,
			column: 10,
		},
		{
			name: "binary op at position 2:7",
			expr: &BinaryOp{
				Line:     2,
				Column:   7,
				Operator: "*",
				Left:     &Number{Line: 2, Column: 1, Value: "4"},
				Right:    &Number{Line: 2, Column: 3, Value: "7"},
			},
			line:   2,
			column: 7,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.expr.GetLine(); got != tt.line {
				t.Errorf("GetLine() = %d, want %d", got, tt.line)
			}
			if got := tt.expr.GetColumn(); got != tt.column {
				t.Errorf("GetColumn() = %d, want %d", got, tt.column)
			}
		})
	}
}

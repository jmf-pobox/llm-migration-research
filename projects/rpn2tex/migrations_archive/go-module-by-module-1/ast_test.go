package rpn2tex

import "testing"

// MockVisitor is a simple visitor implementation for testing.
type MockVisitor struct{}

func (m *MockVisitor) VisitNumber(n *Number) string {
	return n.Value
}

func (m *MockVisitor) VisitBinaryOp(b *BinaryOp) string {
	left := b.Left.Accept(m)
	right := b.Right.Accept(m)
	return left + " " + b.Operator + " " + right
}

func TestNumberNode(t *testing.T) {
	num := &Number{
		Line:   1,
		Column: 1,
		Value:  "42",
	}

	if num.GetLine() != 1 {
		t.Errorf("Expected line 1, got %d", num.GetLine())
	}

	if num.GetColumn() != 1 {
		t.Errorf("Expected column 1, got %d", num.GetColumn())
	}

	if num.Value != "42" {
		t.Errorf("Expected value '42', got '%s'", num.Value)
	}
}

func TestBinaryOpNode(t *testing.T) {
	left := &Number{Line: 1, Column: 1, Value: "5"}
	right := &Number{Line: 1, Column: 3, Value: "3"}

	op := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     left,
		Right:    right,
	}

	if op.GetLine() != 1 {
		t.Errorf("Expected line 1, got %d", op.GetLine())
	}

	if op.GetColumn() != 5 {
		t.Errorf("Expected column 5, got %d", op.GetColumn())
	}

	if op.Operator != "+" {
		t.Errorf("Expected operator '+', got '%s'", op.Operator)
	}
}

func TestVisitorPattern(t *testing.T) {
	visitor := &MockVisitor{}

	// Test simple number
	num := &Number{Line: 1, Column: 1, Value: "42"}
	result := num.Accept(visitor)
	if result != "42" {
		t.Errorf("Expected '42', got '%s'", result)
	}

	// Test binary operation: 5 + 3
	left := &Number{Line: 1, Column: 1, Value: "5"}
	right := &Number{Line: 1, Column: 3, Value: "3"}
	op := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     left,
		Right:    right,
	}
	result = op.Accept(visitor)
	if result != "5 + 3" {
		t.Errorf("Expected '5 + 3', got '%s'", result)
	}
}

func TestNestedBinaryOp(t *testing.T) {
	visitor := &MockVisitor{}

	// Test nested operation: (5 + 3) * 2
	// AST: BinaryOp(*, BinaryOp(+, 5, 3), 2)
	innerLeft := &Number{Line: 1, Column: 1, Value: "5"}
	innerRight := &Number{Line: 1, Column: 3, Value: "3"}
	innerOp := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     innerLeft,
		Right:    innerRight,
	}

	outerRight := &Number{Line: 1, Column: 7, Value: "2"}
	outerOp := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left:     innerOp,
		Right:    outerRight,
	}

	result := outerOp.Accept(visitor)
	expected := "5 + 3 * 2"
	if result != expected {
		t.Errorf("Expected '%s', got '%s'", expected, result)
	}
}

func TestDecimalNumberValue(t *testing.T) {
	// Test that decimal values are preserved as strings
	num := &Number{
		Line:   1,
		Column: 1,
		Value:  "3.14",
	}

	if num.Value != "3.14" {
		t.Errorf("Expected value '3.14', got '%s'", num.Value)
	}

	visitor := &MockVisitor{}
	result := num.Accept(visitor)
	if result != "3.14" {
		t.Errorf("Expected '3.14', got '%s'", result)
	}
}

func TestExprInterface(t *testing.T) {
	// Verify that both Number and BinaryOp implement Expr interface
	var _ Expr = &Number{}
	var _ Expr = &BinaryOp{}

	// Test using Expr interface
	var expr Expr
	expr = &Number{Line: 1, Column: 1, Value: "10"}

	if expr.GetLine() != 1 {
		t.Errorf("Expected line 1, got %d", expr.GetLine())
	}

	// Test type assertion
	if num, ok := expr.(*Number); ok {
		if num.Value != "10" {
			t.Errorf("Expected value '10', got '%s'", num.Value)
		}
	} else {
		t.Error("Type assertion to *Number failed")
	}
}

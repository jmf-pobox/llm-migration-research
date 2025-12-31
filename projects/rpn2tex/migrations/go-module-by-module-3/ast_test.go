package rpn2tex

import "testing"

func TestNumber_GetLine(t *testing.T) {
	tests := []struct {
		name string
		num  *Number
		want int
	}{
		{
			name: "line 1",
			num:  &Number{Line: 1, Column: 1, Value: "42"},
			want: 1,
		},
		{
			name: "line 10",
			num:  &Number{Line: 10, Column: 5, Value: "3.14"},
			want: 10,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.num.GetLine(); got != tt.want {
				t.Errorf("Number.GetLine() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestNumber_GetColumn(t *testing.T) {
	tests := []struct {
		name string
		num  *Number
		want int
	}{
		{
			name: "column 1",
			num:  &Number{Line: 1, Column: 1, Value: "42"},
			want: 1,
		},
		{
			name: "column 5",
			num:  &Number{Line: 10, Column: 5, Value: "3.14"},
			want: 5,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.num.GetColumn(); got != tt.want {
				t.Errorf("Number.GetColumn() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestNumber_ExprInterface(t *testing.T) {
	num := &Number{Line: 1, Column: 1, Value: "42"}
	var _ Expr = num // Compile-time check that Number implements Expr
}

func TestNumber_ValuePreservation(t *testing.T) {
	tests := []struct {
		name  string
		value string
	}{
		{
			name:  "integer",
			value: "42",
		},
		{
			name:  "decimal",
			value: "3.14",
		},
		{
			name:  "negative integer",
			value: "-5",
		},
		{
			name:  "negative decimal",
			value: "-1.5",
		},
		{
			name:  "zero",
			value: "0",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			num := &Number{Line: 1, Column: 1, Value: tt.value}
			if num.Value != tt.value {
				t.Errorf("Number.Value = %v, want %v", num.Value, tt.value)
			}
		})
	}
}

func TestBinaryOp_GetLine(t *testing.T) {
	tests := []struct {
		name string
		op   *BinaryOp
		want int
	}{
		{
			name: "line 1",
			op: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 2, Value: "3"},
			},
			want: 1,
		},
		{
			name: "line 5",
			op: &BinaryOp{
				Line:     5,
				Column:   10,
				Operator: "*",
				Left:     &Number{Line: 5, Column: 8, Value: "2"},
				Right:    &Number{Line: 5, Column: 9, Value: "4"},
			},
			want: 5,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.op.GetLine(); got != tt.want {
				t.Errorf("BinaryOp.GetLine() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestBinaryOp_GetColumn(t *testing.T) {
	tests := []struct {
		name string
		op   *BinaryOp
		want int
	}{
		{
			name: "column 3",
			op: &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 2, Value: "3"},
			},
			want: 3,
		},
		{
			name: "column 10",
			op: &BinaryOp{
				Line:     5,
				Column:   10,
				Operator: "*",
				Left:     &Number{Line: 5, Column: 8, Value: "2"},
				Right:    &Number{Line: 5, Column: 9, Value: "4"},
			},
			want: 10,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.op.GetColumn(); got != tt.want {
				t.Errorf("BinaryOp.GetColumn() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestBinaryOp_ExprInterface(t *testing.T) {
	op := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 2, Value: "3"},
	}
	var _ Expr = op // Compile-time check that BinaryOp implements Expr
}

func TestBinaryOp_Operators(t *testing.T) {
	tests := []struct {
		name     string
		operator string
	}{
		{
			name:     "addition",
			operator: "+",
		},
		{
			name:     "subtraction",
			operator: "-",
		},
		{
			name:     "multiplication",
			operator: "*",
		},
		{
			name:     "division",
			operator: "/",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			op := &BinaryOp{
				Line:     1,
				Column:   3,
				Operator: tt.operator,
				Left:     &Number{Line: 1, Column: 1, Value: "5"},
				Right:    &Number{Line: 1, Column: 2, Value: "3"},
			}
			if op.Operator != tt.operator {
				t.Errorf("BinaryOp.Operator = %v, want %v", op.Operator, tt.operator)
			}
		})
	}
}

func TestBinaryOp_RecursiveStructure(t *testing.T) {
	// Test nested BinaryOp: (5 + 3) * 2
	innerOp := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 2, Value: "3"},
	}

	outerOp := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "*",
		Left:     innerOp,
		Right:    &Number{Line: 1, Column: 4, Value: "2"},
	}

	// Verify the structure
	if outerOp.Operator != "*" {
		t.Errorf("outerOp.Operator = %v, want *", outerOp.Operator)
	}

	leftOp, ok := outerOp.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("outerOp.Left is not a BinaryOp")
	}

	if leftOp.Operator != "+" {
		t.Errorf("leftOp.Operator = %v, want +", leftOp.Operator)
	}

	// Verify left operand of inner operation
	leftNum, ok := leftOp.Left.(*Number)
	if !ok {
		t.Fatalf("leftOp.Left is not a Number")
	}
	if leftNum.Value != "5" {
		t.Errorf("leftNum.Value = %v, want 5", leftNum.Value)
	}

	// Verify right operand of inner operation
	rightNum, ok := leftOp.Right.(*Number)
	if !ok {
		t.Fatalf("leftOp.Right is not a Number")
	}
	if rightNum.Value != "3" {
		t.Errorf("rightNum.Value = %v, want 3", rightNum.Value)
	}

	// Verify right operand of outer operation
	outerRightNum, ok := outerOp.Right.(*Number)
	if !ok {
		t.Fatalf("outerOp.Right is not a Number")
	}
	if outerRightNum.Value != "2" {
		t.Errorf("outerRightNum.Value = %v, want 2", outerRightNum.Value)
	}
}

func TestBinaryOp_BothOperandsAreBinaryOps(t *testing.T) {
	// Test: (1 + 2) * (3 + 4)
	left := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "1"},
		Right:    &Number{Line: 1, Column: 2, Value: "2"},
	}

	right := &BinaryOp{
		Line:     1,
		Column:   6,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 4, Value: "3"},
		Right:    &Number{Line: 1, Column: 5, Value: "4"},
	}

	root := &BinaryOp{
		Line:     1,
		Column:   7,
		Operator: "*",
		Left:     left,
		Right:    right,
	}

	// Verify structure
	if root.Operator != "*" {
		t.Errorf("root.Operator = %v, want *", root.Operator)
	}

	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Left is not a BinaryOp")
	}
	if leftOp.Operator != "+" {
		t.Errorf("leftOp.Operator = %v, want +", leftOp.Operator)
	}

	rightOp, ok := root.Right.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Right is not a BinaryOp")
	}
	if rightOp.Operator != "+" {
		t.Errorf("rightOp.Operator = %v, want +", rightOp.Operator)
	}
}

func TestExpr_TypeAssertion(t *testing.T) {
	// Test that we can use type assertions to distinguish between Number and BinaryOp
	var expr Expr

	// Test Number
	expr = &Number{Line: 1, Column: 1, Value: "42"}
	if _, ok := expr.(*Number); !ok {
		t.Error("Type assertion to *Number failed")
	}
	if _, ok := expr.(*BinaryOp); ok {
		t.Error("Type assertion to *BinaryOp should have failed for Number")
	}

	// Test BinaryOp
	expr = &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 2, Value: "3"},
	}
	if _, ok := expr.(*BinaryOp); !ok {
		t.Error("Type assertion to *BinaryOp failed")
	}
	if _, ok := expr.(*Number); ok {
		t.Error("Type assertion to *Number should have failed for BinaryOp")
	}
}

func TestExpr_TypeSwitch(t *testing.T) {
	// Test that we can use type switches to distinguish between Number and BinaryOp
	exprs := []Expr{
		&Number{Line: 1, Column: 1, Value: "42"},
		&BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 1, Value: "5"},
			Right:    &Number{Line: 1, Column: 2, Value: "3"},
		},
	}

	for i, expr := range exprs {
		switch e := expr.(type) {
		case *Number:
			if i != 0 {
				t.Errorf("Expected BinaryOp at index %d, got Number", i)
			}
			if e.Value != "42" {
				t.Errorf("Number.Value = %v, want 42", e.Value)
			}
		case *BinaryOp:
			if i != 1 {
				t.Errorf("Expected Number at index %d, got BinaryOp", i)
			}
			if e.Operator != "+" {
				t.Errorf("BinaryOp.Operator = %v, want +", e.Operator)
			}
		default:
			t.Errorf("Unexpected type: %T", e)
		}
	}
}

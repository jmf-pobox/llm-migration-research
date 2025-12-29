package rpn2tex

import (
	"testing"
)

// TestParserSimpleNumber tests parsing a single number.
func TestParserSimpleNumber(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "42", Line: 1, Column: 1},
		{Type: EOF, Value: "", Line: 1, Column: 3},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	num, ok := expr.(*Number)
	if !ok {
		t.Fatalf("expr type = %T, want *Number", expr)
	}

	if num.Value != "42" {
		t.Errorf("num.Value = %q, want %q", num.Value, "42")
	}
	if num.Line != 1 {
		t.Errorf("num.Line = %d, want %d", num.Line, 1)
	}
	if num.Column != 1 {
		t.Errorf("num.Column = %d, want %d", num.Column, 1)
	}
}

// TestParserSimpleAddition tests parsing "5 3 +".
func TestParserSimpleAddition(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	binOp, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	if binOp.Operator != "+" {
		t.Errorf("binOp.Operator = %q, want %q", binOp.Operator, "+")
	}

	// Check left operand
	left, ok := binOp.Left.(*Number)
	if !ok {
		t.Fatalf("binOp.Left type = %T, want *Number", binOp.Left)
	}
	if left.Value != "5" {
		t.Errorf("left.Value = %q, want %q", left.Value, "5")
	}

	// Check right operand
	right, ok := binOp.Right.(*Number)
	if !ok {
		t.Fatalf("binOp.Right type = %T, want *Number", binOp.Right)
	}
	if right.Value != "3" {
		t.Errorf("right.Value = %q, want %q", right.Value, "3")
	}
}

// TestParserAllOperators tests all four operators.
func TestParserAllOperators(t *testing.T) {
	tests := []struct {
		name     string
		tokens   []Token
		operator string
	}{
		{
			name: "addition",
			tokens: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			operator: "+",
		},
		{
			name: "subtraction",
			tokens: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			operator: "-",
		},
		{
			name: "multiplication",
			tokens: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			operator: "*",
		},
		{
			name: "division",
			tokens: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
			operator: "/",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			expr, err := parser.Parse()

			if err != nil {
				t.Fatalf("Parse() error = %v, want nil", err)
			}

			binOp, ok := expr.(*BinaryOp)
			if !ok {
				t.Fatalf("expr type = %T, want *BinaryOp", expr)
			}

			if binOp.Operator != tt.operator {
				t.Errorf("binOp.Operator = %q, want %q", binOp.Operator, tt.operator)
			}
		})
	}
}

// TestParserNestedExpression tests parsing "5 3 + 2 *" which should create:
// BinaryOp(*, BinaryOp(+, 5, 3), 2)
func TestParserNestedExpression(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "2", Line: 1, Column: 7},
		{Type: MULT, Value: "*", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	// Root should be multiplication
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}
	if root.Operator != "*" {
		t.Errorf("root.Operator = %q, want %q", root.Operator, "*")
	}

	// Left child should be addition
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Left type = %T, want *BinaryOp", root.Left)
	}
	if leftOp.Operator != "+" {
		t.Errorf("leftOp.Operator = %q, want %q", leftOp.Operator, "+")
	}

	// Left.Left should be 5
	leftLeft, ok := leftOp.Left.(*Number)
	if !ok {
		t.Fatalf("leftOp.Left type = %T, want *Number", leftOp.Left)
	}
	if leftLeft.Value != "5" {
		t.Errorf("leftLeft.Value = %q, want %q", leftLeft.Value, "5")
	}

	// Left.Right should be 3
	leftRight, ok := leftOp.Right.(*Number)
	if !ok {
		t.Fatalf("leftOp.Right type = %T, want *Number", leftOp.Right)
	}
	if leftRight.Value != "3" {
		t.Errorf("leftRight.Value = %q, want %q", leftRight.Value, "3")
	}

	// Right child should be 2
	right, ok := root.Right.(*Number)
	if !ok {
		t.Fatalf("root.Right type = %T, want *Number", root.Right)
	}
	if right.Value != "2" {
		t.Errorf("right.Value = %q, want %q", right.Value, "2")
	}
}

// TestParserComplexExpression tests parsing "2 3 + 4 *" which creates:
// BinaryOp(*, BinaryOp(+, 2, 3), 4)
func TestParserComplexExpression(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "2", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "4", Line: 1, Column: 7},
		{Type: MULT, Value: "*", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	if root.Operator != "*" {
		t.Errorf("root.Operator = %q, want %q", root.Operator, "*")
	}
}

// TestParserDecimalNumbers tests parsing decimal numbers.
func TestParserDecimalNumbers(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 6},
		{Type: MULT, Value: "*", Line: 1, Column: 8},
		{Type: EOF, Value: "", Line: 1, Column: 9},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	binOp, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	left, ok := binOp.Left.(*Number)
	if !ok {
		t.Fatalf("binOp.Left type = %T, want *Number", binOp.Left)
	}

	// Verify decimal is preserved as string
	if left.Value != "3.14" {
		t.Errorf("left.Value = %q, want %q", left.Value, "3.14")
	}
}

// TestParserEmptyExpression tests parsing an empty expression.
func TestParserEmptyExpression(t *testing.T) {
	tokens := []Token{
		{Type: EOF, Value: "", Line: 1, Column: 1},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err == nil {
		t.Fatalf("Parse() error = nil, want error")
	}

	if expr != nil {
		t.Errorf("expr = %v, want nil", expr)
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("err type = %T, want *ParserError", err)
	}

	if parserErr.Message != "Empty expression" {
		t.Errorf("err.Message = %q, want %q", parserErr.Message, "Empty expression")
	}
}

// TestParserInsufficientOperands tests parsing with insufficient operands.
func TestParserInsufficientOperands(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: PLUS, Value: "+", Line: 1, Column: 3},
		{Type: EOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err == nil {
		t.Fatalf("Parse() error = nil, want error")
	}

	if expr != nil {
		t.Errorf("expr = %v, want nil", expr)
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("err type = %T, want *ParserError", err)
	}

	expectedMsg := "Operator '+' requires two operands"
	if parserErr.Message != expectedMsg {
		t.Errorf("err.Message = %q, want %q", parserErr.Message, expectedMsg)
	}
}

// TestParserTooManyValues tests parsing with too many values.
func TestParserTooManyValues(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "2", Line: 1, Column: 5},
		{Type: PLUS, Value: "+", Line: 1, Column: 7},
		{Type: EOF, Value: "", Line: 1, Column: 8},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err == nil {
		t.Fatalf("Parse() error = nil, want error")
	}

	if expr != nil {
		t.Errorf("expr = %v, want nil", expr)
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("err type = %T, want *ParserError", err)
	}

	expectedMsg := "Invalid RPN: 2 values remain on stack (missing operators?)"
	if parserErr.Message != expectedMsg {
		t.Errorf("err.Message = %q, want %q", parserErr.Message, expectedMsg)
	}
}

// TestParserChainedOperations tests parsing "1 2 + 3 + 4 +" which creates
// nested BinaryOp nodes for left-associative operations.
func TestParserChainedOperations(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "1", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "3", Line: 1, Column: 7},
		{Type: PLUS, Value: "+", Line: 1, Column: 9},
		{Type: NUMBER, Value: "4", Line: 1, Column: 11},
		{Type: PLUS, Value: "+", Line: 1, Column: 13},
		{Type: EOF, Value: "", Line: 1, Column: 14},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	// Verify it's a BinaryOp at the root
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	if root.Operator != "+" {
		t.Errorf("root.Operator = %q, want %q", root.Operator, "+")
	}

	// Verify the structure has nested BinaryOp on the left
	_, ok = root.Left.(*BinaryOp)
	if !ok {
		t.Errorf("root.Left type = %T, want *BinaryOp", root.Left)
	}

	// Verify the right is a number
	rightNum, ok := root.Right.(*Number)
	if !ok {
		t.Fatalf("root.Right type = %T, want *Number", root.Right)
	}
	if rightNum.Value != "4" {
		t.Errorf("rightNum.Value = %q, want %q", rightNum.Value, "4")
	}
}

// TestParserRightSideParens tests "2 3 4 + *" which creates:
// BinaryOp(*, 2, BinaryOp(+, 3, 4))
func TestParserRightSideParens(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "2", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "4", Line: 1, Column: 5},
		{Type: PLUS, Value: "+", Line: 1, Column: 7},
		{Type: MULT, Value: "*", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	if root.Operator != "*" {
		t.Errorf("root.Operator = %q, want %q", root.Operator, "*")
	}

	// Left should be a number
	leftNum, ok := root.Left.(*Number)
	if !ok {
		t.Fatalf("root.Left type = %T, want *Number", root.Left)
	}
	if leftNum.Value != "2" {
		t.Errorf("leftNum.Value = %q, want %q", leftNum.Value, "2")
	}

	// Right should be a BinaryOp
	rightOp, ok := root.Right.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Right type = %T, want *BinaryOp", root.Right)
	}
	if rightOp.Operator != "+" {
		t.Errorf("rightOp.Operator = %q, want %q", rightOp.Operator, "+")
	}
}

// TestParserBothSidesParens tests "1 2 + 3 4 + *" which creates:
// BinaryOp(*, BinaryOp(+, 1, 2), BinaryOp(+, 3, 4))
func TestParserBothSidesParens(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "1", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "3", Line: 1, Column: 7},
		{Type: NUMBER, Value: "4", Line: 1, Column: 9},
		{Type: PLUS, Value: "+", Line: 1, Column: 11},
		{Type: MULT, Value: "*", Line: 1, Column: 13},
		{Type: EOF, Value: "", Line: 1, Column: 14},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	if root.Operator != "*" {
		t.Errorf("root.Operator = %q, want %q", root.Operator, "*")
	}

	// Both left and right should be BinaryOp
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Left type = %T, want *BinaryOp", root.Left)
	}
	if leftOp.Operator != "+" {
		t.Errorf("leftOp.Operator = %q, want %q", leftOp.Operator, "+")
	}

	rightOp, ok := root.Right.(*BinaryOp)
	if !ok {
		t.Fatalf("root.Right type = %T, want *BinaryOp", root.Right)
	}
	if rightOp.Operator != "+" {
		t.Errorf("rightOp.Operator = %q, want %q", rightOp.Operator, "+")
	}
}

// TestParserPositionTracking tests that position information is preserved.
func TestParserPositionTracking(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	binOp, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("expr type = %T, want *BinaryOp", expr)
	}

	// Check operator position
	if binOp.Line != 1 || binOp.Column != 5 {
		t.Errorf("binOp position = (%d, %d), want (1, 5)", binOp.Line, binOp.Column)
	}

	// Check left operand position
	left, ok := binOp.Left.(*Number)
	if !ok {
		t.Fatalf("binOp.Left type = %T, want *Number", binOp.Left)
	}
	if left.Line != 1 || left.Column != 1 {
		t.Errorf("left position = (%d, %d), want (1, 1)", left.Line, left.Column)
	}

	// Check right operand position
	right, ok := binOp.Right.(*Number)
	if !ok {
		t.Fatalf("binOp.Right type = %T, want *Number", binOp.Right)
	}
	if right.Line != 1 || right.Column != 3 {
		t.Errorf("right position = (%d, %d), want (1, 3)", right.Line, right.Column)
	}
}

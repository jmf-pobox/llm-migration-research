package rpn2tex

import (
	"strings"
	"testing"
)

// TestParserSimpleAddition tests parsing a simple addition expression.
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

	// Check the AST structure
	binOp, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}

	if binOp.Operator != "+" {
		t.Errorf("Operator = %q, want %q", binOp.Operator, "+")
	}

	// Check left operand
	left, ok := binOp.Left.(*Number)
	if !ok {
		t.Fatalf("Left operand is %T, want *Number", binOp.Left)
	}
	if left.Value != "5" {
		t.Errorf("Left value = %q, want %q", left.Value, "5")
	}

	// Check right operand
	right, ok := binOp.Right.(*Number)
	if !ok {
		t.Fatalf("Right operand is %T, want *Number", binOp.Right)
	}
	if right.Value != "3" {
		t.Errorf("Right value = %q, want %q", right.Value, "3")
	}
}

// TestParserAllOperators tests parsing expressions with all operator types.
func TestParserAllOperators(t *testing.T) {
	tests := []struct {
		name     string
		tokens   []Token
		operator string
	}{
		{
			name: "addition",
			tokens: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
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
				t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
			}

			if binOp.Operator != tt.operator {
				t.Errorf("Operator = %q, want %q", binOp.Operator, tt.operator)
			}
		})
	}
}

// TestParserComplexExpression tests parsing a complex nested expression.
func TestParserComplexExpression(t *testing.T) {
	// Parse: "5 3 + 2 *" which should produce: (5 + 3) * 2
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
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}
	if root.Operator != "*" {
		t.Errorf("Root operator = %q, want %q", root.Operator, "*")
	}

	// Left child should be addition
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("Left child is %T, want *BinaryOp", root.Left)
	}
	if leftOp.Operator != "+" {
		t.Errorf("Left child operator = %q, want %q", leftOp.Operator, "+")
	}

	// Right child should be Number(2)
	rightNum, ok := root.Right.(*Number)
	if !ok {
		t.Fatalf("Right child is %T, want *Number", root.Right)
	}
	if rightNum.Value != "2" {
		t.Errorf("Right child value = %q, want %q", rightNum.Value, "2")
	}

	// Check the addition's operands
	leftNum, ok := leftOp.Left.(*Number)
	if !ok || leftNum.Value != "5" {
		t.Errorf("Addition left operand = %v, want Number(5)", leftOp.Left)
	}

	rightNum2, ok := leftOp.Right.(*Number)
	if !ok || rightNum2.Value != "3" {
		t.Errorf("Addition right operand = %v, want Number(3)", leftOp.Right)
	}
}

// TestParserLeftAssociative tests left-associative parsing.
func TestParserLeftAssociative(t *testing.T) {
	// Parse: "5 3 - 2 -" which should produce: (5 - 3) - 2
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: MINUS, Value: "-", Line: 1, Column: 5},
		{Type: NUMBER, Value: "2", Line: 1, Column: 7},
		{Type: MINUS, Value: "-", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	// Root should be subtraction
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}
	if root.Operator != "-" {
		t.Errorf("Root operator = %q, want %q", root.Operator, "-")
	}

	// Left child should be subtraction (5 - 3)
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("Left child is %T, want *BinaryOp", root.Left)
	}
	if leftOp.Operator != "-" {
		t.Errorf("Left child operator = %q, want %q", leftOp.Operator, "-")
	}

	// Right child should be Number(2)
	rightNum, ok := root.Right.(*Number)
	if !ok || rightNum.Value != "2" {
		t.Errorf("Right child = %v, want Number(2)", root.Right)
	}
}

// TestParserMultipleAdditions tests parsing multiple additions.
func TestParserMultipleAdditions(t *testing.T) {
	// Parse: "1 2 + 3 + 4 +" which should produce: ((1 + 2) + 3) + 4
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

	// Root should be addition
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}
	if root.Operator != "+" {
		t.Errorf("Root operator = %q, want %q", root.Operator, "+")
	}

	// Should have nested structure with right child being Number(4)
	rightNum, ok := root.Right.(*Number)
	if !ok || rightNum.Value != "4" {
		t.Errorf("Right child = %v, want Number(4)", root.Right)
	}
}

// TestParserDecimalNumbers tests parsing with decimal numbers.
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
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "3.14" {
		t.Errorf("Left value = %v, want Number(3.14)", binOp.Left)
	}
}

// TestParserEmptyExpression tests error handling for empty expressions.
func TestParserEmptyExpression(t *testing.T) {
	tokens := []Token{
		{Type: EOF, Value: "", Line: 1, Column: 1},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Parse() error = nil, want error for empty expression")
	}

	if !strings.Contains(err.Error(), "Empty expression") {
		t.Errorf("Error message = %q, want message containing 'Empty expression'", err.Error())
	}
}

// TestParserInsufficientOperands tests error handling for insufficient operands.
func TestParserInsufficientOperands(t *testing.T) {
	tests := []struct {
		name   string
		tokens []Token
	}{
		{
			name: "operator only",
			tokens: []Token{
				{Type: PLUS, Value: "+", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name: "one operand",
			tokens: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: PLUS, Value: "+", Line: 1, Column: 3},
				{Type: EOF, Value: "", Line: 1, Column: 4},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			_, err := parser.Parse()

			if err == nil {
				t.Fatal("Parse() error = nil, want error for insufficient operands")
			}

			if !strings.Contains(err.Error(), "requires two operands") {
				t.Errorf("Error message = %q, want message containing 'requires two operands'", err.Error())
			}
		})
	}
}

// TestParserExtraOperands tests error handling for too many operands.
func TestParserExtraOperands(t *testing.T) {
	// Three numbers with only one operator - should have 2 values on stack at end
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "2", Line: 1, Column: 5},
		{Type: PLUS, Value: "+", Line: 1, Column: 7},
		{Type: EOF, Value: "", Line: 1, Column: 8},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Parse() error = nil, want error for extra operands")
	}

	if !strings.Contains(err.Error(), "Invalid RPN") {
		t.Errorf("Error message = %q, want message containing 'Invalid RPN'", err.Error())
	}

	if !strings.Contains(err.Error(), "2 values remain on stack") {
		t.Errorf("Error message = %q, want message containing '2 values remain on stack'", err.Error())
	}
}

// TestParserPositionPreservation tests that AST nodes preserve token positions.
func TestParserPositionPreservation(t *testing.T) {
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

	binOp := expr.(*BinaryOp)

	// Check operator position
	if binOp.Line != 1 || binOp.Column != 5 {
		t.Errorf("BinaryOp position = (%d, %d), want (1, 5)", binOp.Line, binOp.Column)
	}

	// Check left operand position
	left := binOp.Left.(*Number)
	if left.Line != 1 || left.Column != 1 {
		t.Errorf("Left Number position = (%d, %d), want (1, 1)", left.Line, left.Column)
	}

	// Check right operand position
	right := binOp.Right.(*Number)
	if right.Line != 1 || right.Column != 3 {
		t.Errorf("Right Number position = (%d, %d), want (1, 3)", right.Line, right.Column)
	}
}

// TestParserMixedPrecedence tests parsing expressions with mixed operator precedence.
func TestParserMixedPrecedence(t *testing.T) {
	// Parse: "2 3 4 * +" which should produce: 2 + (3 * 4)
	tokens := []Token{
		{Type: NUMBER, Value: "2", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "4", Line: 1, Column: 5},
		{Type: MULT, Value: "*", Line: 1, Column: 7},
		{Type: PLUS, Value: "+", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	// Root should be addition
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}
	if root.Operator != "+" {
		t.Errorf("Root operator = %q, want %q", root.Operator, "+")
	}

	// Left child should be Number(2)
	leftNum, ok := root.Left.(*Number)
	if !ok || leftNum.Value != "2" {
		t.Errorf("Left child = %v, want Number(2)", root.Left)
	}

	// Right child should be multiplication (3 * 4)
	rightOp, ok := root.Right.(*BinaryOp)
	if !ok {
		t.Fatalf("Right child is %T, want *BinaryOp", root.Right)
	}
	if rightOp.Operator != "*" {
		t.Errorf("Right child operator = %q, want %q", rightOp.Operator, "*")
	}
}

// TestParserBothOperandsAdditions tests parsing with both operands being additions.
func TestParserBothOperandsAdditions(t *testing.T) {
	// Parse: "1 2 + 3 4 + *" which should produce: (1 + 2) * (3 + 4)
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

	// Root should be multiplication
	root, ok := expr.(*BinaryOp)
	if !ok {
		t.Fatalf("Parse() returned %T, want *BinaryOp", expr)
	}
	if root.Operator != "*" {
		t.Errorf("Root operator = %q, want %q", root.Operator, "*")
	}

	// Both children should be additions
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok || leftOp.Operator != "+" {
		t.Errorf("Left child = %v, want addition", root.Left)
	}

	rightOp, ok := root.Right.(*BinaryOp)
	if !ok || rightOp.Operator != "+" {
		t.Errorf("Right child = %v, want addition", root.Right)
	}
}

// TestParserComplexNested tests a complex nested expression.
func TestParserComplexNested(t *testing.T) {
	// Parse: "10 2 / 3 + 4 *" which should produce: ((10 / 2) + 3) * 4
	tokens := []Token{
		{Type: NUMBER, Value: "10", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 4},
		{Type: DIV, Value: "/", Line: 1, Column: 6},
		{Type: NUMBER, Value: "3", Line: 1, Column: 8},
		{Type: PLUS, Value: "+", Line: 1, Column: 10},
		{Type: NUMBER, Value: "4", Line: 1, Column: 12},
		{Type: MULT, Value: "*", Line: 1, Column: 14},
		{Type: EOF, Value: "", Line: 1, Column: 15},
	}

	parser := NewParser(tokens)
	expr, err := parser.Parse()

	if err != nil {
		t.Fatalf("Parse() error = %v, want nil", err)
	}

	// Root should be multiplication
	root, ok := expr.(*BinaryOp)
	if !ok || root.Operator != "*" {
		t.Errorf("Root = %v, want multiplication", expr)
	}

	// Left child should be addition
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok || leftOp.Operator != "+" {
		t.Errorf("Left child = %v, want addition", root.Left)
	}

	// Left child of addition should be division
	leftLeftOp, ok := leftOp.Left.(*BinaryOp)
	if !ok || leftLeftOp.Operator != "/" {
		t.Errorf("Left-left child = %v, want division", leftOp.Left)
	}

	// Right child should be Number(4)
	rightNum, ok := root.Right.(*Number)
	if !ok || rightNum.Value != "4" {
		t.Errorf("Right child = %v, want Number(4)", root.Right)
	}
}

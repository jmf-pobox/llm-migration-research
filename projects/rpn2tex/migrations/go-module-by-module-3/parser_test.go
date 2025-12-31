package rpn2tex

import (
	"testing"
)

func TestParser_SimpleAddition(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp, got: %T", result)
	}

	if binOp.Operator != "+" {
		t.Errorf("Expected operator '+', got: %s", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("Expected left operand '5', got: %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("Expected right operand '3', got: %v", binOp.Right)
	}
}

func TestParser_SimpleSubtraction(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: MINUS, Value: "-", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp, got: %T", result)
	}

	if binOp.Operator != "-" {
		t.Errorf("Expected operator '-', got: %s", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("Expected left operand '5', got: %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("Expected right operand '3', got: %v", binOp.Right)
	}
}

func TestParser_SimpleMultiplication(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "4", Line: 1, Column: 1},
		{Type: NUMBER, Value: "7", Line: 1, Column: 3},
		{Type: MULT, Value: "*", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp, got: %T", result)
	}

	if binOp.Operator != "*" {
		t.Errorf("Expected operator '*', got: %s", binOp.Operator)
	}
}

func TestParser_SimpleDivision(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "10", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 4},
		{Type: DIV, Value: "/", Line: 1, Column: 6},
		{Type: EOF, Value: "", Line: 1, Column: 7},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp, got: %T", result)
	}

	if binOp.Operator != "/" {
		t.Errorf("Expected operator '/', got: %s", binOp.Operator)
	}
}

func TestParser_StackOrder(t *testing.T) {
	// Test: 5 3 - should be (5 - 3), NOT (3 - 5)
	// This tests that first pop = RIGHT, second pop = LEFT
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: MINUS, Value: "-", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp := result.(*BinaryOp)
	left := binOp.Left.(*Number)
	right := binOp.Right.(*Number)

	// CRITICAL: Verify correct order
	if left.Value != "5" {
		t.Errorf("Expected left='5', got: %s", left.Value)
	}
	if right.Value != "3" {
		t.Errorf("Expected right='3', got: %s", right.Value)
	}
}

func TestParser_NestedExpression(t *testing.T) {
	// Test: 5 3 + 2 * => (5 + 3) * 2
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "2", Line: 1, Column: 7},
		{Type: MULT, Value: "*", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be multiplication
	mult, ok := result.(*BinaryOp)
	if !ok || mult.Operator != "*" {
		t.Fatalf("Expected root to be '*', got: %v", result)
	}

	// Left child should be addition (5 + 3)
	plus, ok := mult.Left.(*BinaryOp)
	if !ok || plus.Operator != "+" {
		t.Fatalf("Expected left child to be '+', got: %v", mult.Left)
	}

	// Right child should be number 2
	num, ok := mult.Right.(*Number)
	if !ok || num.Value != "2" {
		t.Fatalf("Expected right child to be '2', got: %v", mult.Right)
	}

	// Verify addition operands
	left, ok := plus.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("Expected addition left='5', got: %v", plus.Left)
	}

	right, ok := plus.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("Expected addition right='3', got: %v", plus.Right)
	}
}

func TestParser_ChainedOperations(t *testing.T) {
	// Test: 1 2 + 3 + 4 + => ((1 + 2) + 3) + 4
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
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be the final addition
	root, ok := result.(*BinaryOp)
	if !ok || root.Operator != "+" {
		t.Fatalf("Expected root to be '+', got: %v", result)
	}

	// Right should be 4
	num4, ok := root.Right.(*Number)
	if !ok || num4.Value != "4" {
		t.Errorf("Expected root right='4', got: %v", root.Right)
	}
}

func TestParser_FloatingPoint(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
		{Type: NUMBER, Value: "2", Line: 1, Column: 6},
		{Type: MULT, Value: "*", Line: 1, Column: 8},
		{Type: EOF, Value: "", Line: 1, Column: 9},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp := result.(*BinaryOp)
	left := binOp.Left.(*Number)

	// Verify exact string preservation
	if left.Value != "3.14" {
		t.Errorf("Expected '3.14', got: %s", left.Value)
	}
}

func TestParser_NegativeNumbers(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 4},
		{Type: PLUS, Value: "+", Line: 1, Column: 6},
		{Type: EOF, Value: "", Line: 1, Column: 7},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp := result.(*BinaryOp)
	left := binOp.Left.(*Number)

	// Verify negative number preserved
	if left.Value != "-5" {
		t.Errorf("Expected '-5', got: %s", left.Value)
	}
}

func TestParser_ErrorEmptyExpression(t *testing.T) {
	tokens := []Token{
		{Type: EOF, Value: "", Line: 1, Column: 1},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Expected error for empty expression, got nil")
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("Expected ParserError, got: %T", err)
	}

	if parserErr.Message != "Empty expression" {
		t.Errorf("Expected 'Empty expression', got: %s", parserErr.Message)
	}
}

func TestParser_ErrorInsufficientOperands(t *testing.T) {
	// Only one number, but trying to apply operator
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: PLUS, Value: "+", Line: 1, Column: 3},
		{Type: EOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Expected error for insufficient operands, got nil")
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("Expected ParserError, got: %T", err)
	}

	if parserErr.Message != "Insufficient operands for operator" {
		t.Errorf("Expected 'Insufficient operands for operator', got: %s", parserErr.Message)
	}
}

func TestParser_ErrorIncompleteExpression(t *testing.T) {
	// Too many operands, not enough operators
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
		t.Fatal("Expected error for incomplete expression, got nil")
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("Expected ParserError, got: %T", err)
	}

	if parserErr.Message != "Incomplete expression: too many operands" {
		t.Errorf("Expected 'Incomplete expression: too many operands', got: %s", parserErr.Message)
	}
}

func TestParser_ComplexExpression(t *testing.T) {
	// Test: 2 3 + 4 5 + * => (2 + 3) * (4 + 5)
	tokens := []Token{
		{Type: NUMBER, Value: "2", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: NUMBER, Value: "4", Line: 1, Column: 7},
		{Type: NUMBER, Value: "5", Line: 1, Column: 9},
		{Type: PLUS, Value: "+", Line: 1, Column: 11},
		{Type: MULT, Value: "*", Line: 1, Column: 13},
		{Type: EOF, Value: "", Line: 1, Column: 14},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be multiplication
	mult, ok := result.(*BinaryOp)
	if !ok || mult.Operator != "*" {
		t.Fatalf("Expected root to be '*', got: %v", result)
	}

	// Both children should be additions
	leftPlus, ok := mult.Left.(*BinaryOp)
	if !ok || leftPlus.Operator != "+" {
		t.Fatalf("Expected left child to be '+', got: %v", mult.Left)
	}

	rightPlus, ok := mult.Right.(*BinaryOp)
	if !ok || rightPlus.Operator != "+" {
		t.Fatalf("Expected right child to be '+', got: %v", mult.Right)
	}

	// Verify left addition: 2 + 3
	num2 := leftPlus.Left.(*Number)
	num3 := leftPlus.Right.(*Number)
	if num2.Value != "2" || num3.Value != "3" {
		t.Errorf("Expected left addition '2 + 3', got: %s + %s", num2.Value, num3.Value)
	}

	// Verify right addition: 4 + 5
	num4 := rightPlus.Left.(*Number)
	num5 := rightPlus.Right.(*Number)
	if num4.Value != "4" || num5.Value != "5" {
		t.Errorf("Expected right addition '4 + 5', got: %s + %s", num4.Value, num5.Value)
	}
}

func TestParser_PositionTracking(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	binOp := result.(*BinaryOp)

	// Verify position tracking
	if binOp.Line != 1 || binOp.Column != 5 {
		t.Errorf("Expected BinaryOp at 1:5, got: %d:%d", binOp.Line, binOp.Column)
	}

	left := binOp.Left.(*Number)
	if left.Line != 1 || left.Column != 1 {
		t.Errorf("Expected left Number at 1:1, got: %d:%d", left.Line, left.Column)
	}

	right := binOp.Right.(*Number)
	if right.Line != 1 || right.Column != 3 {
		t.Errorf("Expected right Number at 1:3, got: %d:%d", right.Line, right.Column)
	}
}

func TestParserError_Error(t *testing.T) {
	token := &Token{
		Type:   PLUS,
		Value:  "+",
		Line:   5,
		Column: 10,
	}

	err := &ParserError{
		Message: "Test error",
		Token:   token,
	}

	expected := "Test error at line 5, column 10"
	if err.Error() != expected {
		t.Errorf("Expected '%s', got: '%s'", expected, err.Error())
	}
}

func TestParser_AllOperators(t *testing.T) {
	tests := []struct {
		name     string
		tokens   []Token
		expected string
	}{
		{
			name: "PLUS",
			tokens: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			expected: "+",
		},
		{
			name: "MINUS",
			tokens: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			expected: "-",
		},
		{
			name: "MULT",
			tokens: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			expected: "*",
		},
		{
			name: "DIV",
			tokens: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: DIV, Value: "/", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			expected: "/",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			result, err := parser.Parse()

			if err != nil {
				t.Fatalf("Expected no error, got: %v", err)
			}

			binOp, ok := result.(*BinaryOp)
			if !ok {
				t.Fatalf("Expected BinaryOp, got: %T", result)
			}

			if binOp.Operator != tt.expected {
				t.Errorf("Expected operator '%s', got: '%s'", tt.expected, binOp.Operator)
			}
		})
	}
}

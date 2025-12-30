package main

import (
	"testing"
)

// TestParserSingleNumber tests parsing a single number.
func TestParserSingleNumber(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "42", Line: 1, Column: 1},
		{Type: EOF, Value: "", Line: 1, Column: 3},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	num, ok := result.(*Number)
	if !ok {
		t.Fatalf("Expected Number node, got: %T", result)
	}

	if num.Value != "42" {
		t.Errorf("Expected value '42', got: '%s'", num.Value)
	}
}

// TestParserSimpleAddition tests parsing a simple addition operation.
func TestParserSimpleAddition(t *testing.T) {
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
		t.Fatalf("Expected BinaryOp node, got: %T", result)
	}

	if binOp.Operator != "+" {
		t.Errorf("Expected operator '+', got: '%s'", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("Expected left operand to be Number with value '5'")
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("Expected right operand to be Number with value '3'")
	}
}

// TestParserAllOperators tests parsing with all operators.
func TestParserAllOperators(t *testing.T) {
	tests := []struct {
		name     string
		tokens   []Token
		operator string
		leftVal  string
		rightVal string
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
			leftVal:  "1",
			rightVal: "2",
		},
		{
			name: "subtraction",
			tokens: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 4},
				{Type: MINUS, Value: "-", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
			operator: "-",
			leftVal:  "10",
			rightVal: "3",
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
			leftVal:  "4",
			rightVal: "7",
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
			leftVal:  "10",
			rightVal: "2",
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
				t.Fatalf("Expected BinaryOp node, got: %T", result)
			}

			if binOp.Operator != tt.operator {
				t.Errorf("Expected operator '%s', got: '%s'", tt.operator, binOp.Operator)
			}

			left, ok := binOp.Left.(*Number)
			if !ok || left.Value != tt.leftVal {
				t.Errorf("Expected left operand '%s', got: %v", tt.leftVal, binOp.Left)
			}

			right, ok := binOp.Right.(*Number)
			if !ok || right.Value != tt.rightVal {
				t.Errorf("Expected right operand '%s', got: %v", tt.rightVal, binOp.Right)
			}
		})
	}
}

// TestParserChainedOperations tests parsing chained operations.
func TestParserChainedOperations(t *testing.T) {
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
	root, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected root to be BinaryOp, got: %T", result)
	}

	if root.Operator != "*" {
		t.Errorf("Expected root operator '*', got: '%s'", root.Operator)
	}

	// Left child should be addition (5 + 3)
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected left child to be BinaryOp, got: %T", root.Left)
	}

	if leftOp.Operator != "+" {
		t.Errorf("Expected left operator '+', got: '%s'", leftOp.Operator)
	}

	// Right child should be number 2
	rightNum, ok := root.Right.(*Number)
	if !ok || rightNum.Value != "2" {
		t.Errorf("Expected right operand to be Number '2'")
	}
}

// TestParserMultipleChainedOperations tests longer chains of operations.
func TestParserMultipleChainedOperations(t *testing.T) {
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
	if !ok {
		t.Fatalf("Expected root to be BinaryOp, got: %T", result)
	}

	if root.Operator != "+" {
		t.Errorf("Expected root operator '+', got: '%s'", root.Operator)
	}

	// Right child should be number 4
	rightNum, ok := root.Right.(*Number)
	if !ok || rightNum.Value != "4" {
		t.Errorf("Expected right operand to be Number '4'")
	}

	// Left should be another BinaryOp
	_, ok = root.Left.(*BinaryOp)
	if !ok {
		t.Errorf("Expected left child to be BinaryOp")
	}
}

// TestParserNotEnoughOperands tests error when operator has insufficient operands.
func TestParserNotEnoughOperands(t *testing.T) {
	tests := []struct {
		name   string
		tokens []Token
	}{
		{
			name: "operator with no operands",
			tokens: []Token{
				{Type: PLUS, Value: "+", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name: "operator with one operand",
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
				t.Fatal("Expected error for insufficient operands, got nil")
			}

			parserErr, ok := err.(*ParserError)
			if !ok {
				t.Fatalf("Expected ParserError, got: %T", err)
			}

			if parserErr.Message == "" {
				t.Error("Expected error message to be non-empty")
			}
		})
	}
}

// TestParserTooManyOperands tests error when there are too many operands.
func TestParserTooManyOperands(t *testing.T) {
	// Test: 5 3 2 (missing operator)
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "2", Line: 1, Column: 5},
		{Type: EOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Expected error for too many operands, got nil")
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("Expected ParserError, got: %T", err)
	}

	if parserErr.Message == "" {
		t.Error("Expected error message to be non-empty")
	}
}

// TestParserEmptyExpression tests error for empty input.
func TestParserEmptyExpression(t *testing.T) {
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

	if parserErr.Message == "" {
		t.Error("Expected error message to be non-empty")
	}
}

// TestParserFloatingPointNumbers tests parsing floating-point numbers.
func TestParserFloatingPointNumbers(t *testing.T) {
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

	binOp, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected BinaryOp node, got: %T", result)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "3.14" {
		t.Errorf("Expected left operand to be '3.14', got: %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "2" {
		t.Errorf("Expected right operand to be '2', got: %v", binOp.Right)
	}
}

// TestParserErrorContainsTokenInfo tests that parser errors include token information.
func TestParserErrorContainsTokenInfo(t *testing.T) {
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 2, Column: 5},
		{Type: PLUS, Value: "+", Line: 2, Column: 7},
		{Type: EOF, Value: "", Line: 2, Column: 8},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()

	if err == nil {
		t.Fatal("Expected error, got nil")
	}

	parserErr, ok := err.(*ParserError)
	if !ok {
		t.Fatalf("Expected ParserError, got: %T", err)
	}

	if parserErr.Token.Line != 2 {
		t.Errorf("Expected error token line 2, got: %d", parserErr.Token.Line)
	}

	if parserErr.Token.Column != 7 {
		t.Errorf("Expected error token column 7, got: %d", parserErr.Token.Column)
	}
}

// TestParserComplexExpression tests a complex nested expression.
func TestParserComplexExpression(t *testing.T) {
	// Test: 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
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
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be multiplication
	root, ok := result.(*BinaryOp)
	if !ok {
		t.Fatalf("Expected root to be BinaryOp, got: %T", result)
	}

	if root.Operator != "*" {
		t.Errorf("Expected root operator '*', got: '%s'", root.Operator)
	}

	// Both children should be BinaryOps (additions)
	leftOp, ok := root.Left.(*BinaryOp)
	if !ok || leftOp.Operator != "+" {
		t.Errorf("Expected left child to be addition BinaryOp")
	}

	rightOp, ok := root.Right.(*BinaryOp)
	if !ok || rightOp.Operator != "+" {
		t.Errorf("Expected right child to be addition BinaryOp")
	}
}

package main

import (
	"testing"
)

// TestParserIOContract_Addition tests the parser with addition from the I/O contract.
func TestParserIOContract_Addition(t *testing.T) {
	// Test case 1: 5 3 +
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
		t.Errorf("Expected operator '+', got: '%s'", binOp.Operator)
	}
}

// TestParserIOContract_Subtraction tests the parser with subtraction from the I/O contract.
func TestParserIOContract_Subtraction(t *testing.T) {
	// Test case 2: 5 3 -
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
		t.Errorf("Expected operator '-', got: '%s'", binOp.Operator)
	}
}

// TestParserIOContract_Multiplication tests the parser with multiplication from the I/O contract.
func TestParserIOContract_Multiplication(t *testing.T) {
	// Test case 3: 4 7 *
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
		t.Errorf("Expected operator '*', got: '%s'", binOp.Operator)
	}
}

// TestParserIOContract_Division tests the parser with division from the I/O contract.
func TestParserIOContract_Division(t *testing.T) {
	// Test case 4: 10 2 /
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
		t.Errorf("Expected operator '/', got: '%s'", binOp.Operator)
	}
}

// TestParserIOContract_PrecedenceCase1 tests case 5: 5 3 + 2 *
func TestParserIOContract_PrecedenceCase1(t *testing.T) {
	// Test case 5: 5 3 + 2 * => (5 + 3) * 2
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
	if !ok || root.Operator != "*" {
		t.Fatalf("Expected root to be multiplication")
	}

	// Left should be addition
	left, ok := root.Left.(*BinaryOp)
	if !ok || left.Operator != "+" {
		t.Errorf("Expected left child to be addition")
	}
}

// TestParserIOContract_PrecedenceCase2 tests case 6: 5 3 * 2 +
func TestParserIOContract_PrecedenceCase2(t *testing.T) {
	// Test case 6: 5 3 * 2 + => (5 * 3) + 2
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: MULT, Value: "*", Line: 1, Column: 5},
		{Type: NUMBER, Value: "2", Line: 1, Column: 7},
		{Type: PLUS, Value: "+", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be addition
	root, ok := result.(*BinaryOp)
	if !ok || root.Operator != "+" {
		t.Fatalf("Expected root to be addition")
	}

	// Left should be multiplication
	left, ok := root.Left.(*BinaryOp)
	if !ok || left.Operator != "*" {
		t.Errorf("Expected left child to be multiplication")
	}
}

// TestParserIOContract_ChainedSubtraction tests case 8: 5 3 - 2 -
func TestParserIOContract_ChainedSubtraction(t *testing.T) {
	// Test case 8: 5 3 - 2 - => (5 - 3) - 2
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: MINUS, Value: "-", Line: 1, Column: 5},
		{Type: NUMBER, Value: "2", Line: 1, Column: 7},
		{Type: MINUS, Value: "-", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be subtraction
	root, ok := result.(*BinaryOp)
	if !ok || root.Operator != "-" {
		t.Fatalf("Expected root to be subtraction")
	}

	// Left should also be subtraction
	left, ok := root.Left.(*BinaryOp)
	if !ok || left.Operator != "-" {
		t.Errorf("Expected left child to be subtraction")
	}

	// Right should be number 2
	right, ok := root.Right.(*Number)
	if !ok || right.Value != "2" {
		t.Errorf("Expected right child to be number '2'")
	}
}

// TestParserIOContract_ChainedAddition tests case 10: 1 2 + 3 + 4 +
func TestParserIOContract_ChainedAddition(t *testing.T) {
	// Test case 10: 1 2 + 3 + 4 + => ((1 + 2) + 3) + 4
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

	// Root should be addition with right operand 4
	root, ok := result.(*BinaryOp)
	if !ok || root.Operator != "+" {
		t.Fatalf("Expected root to be addition")
	}

	right, ok := root.Right.(*Number)
	if !ok || right.Value != "4" {
		t.Errorf("Expected right child to be number '4'")
	}
}

// TestParserIOContract_MixedOperators tests case 11: 2 3 4 * +
func TestParserIOContract_MixedOperators(t *testing.T) {
	// Test case 11: 2 3 4 * + => 2 + (3 * 4)
	tokens := []Token{
		{Type: NUMBER, Value: "2", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: NUMBER, Value: "4", Line: 1, Column: 5},
		{Type: MULT, Value: "*", Line: 1, Column: 7},
		{Type: PLUS, Value: "+", Line: 1, Column: 9},
		{Type: EOF, Value: "", Line: 1, Column: 10},
	}

	parser := NewParser(tokens)
	result, err := parser.Parse()

	if err != nil {
		t.Fatalf("Expected no error, got: %v", err)
	}

	// Root should be addition
	root, ok := result.(*BinaryOp)
	if !ok || root.Operator != "+" {
		t.Fatalf("Expected root to be addition")
	}

	// Left should be number 2
	left, ok := root.Left.(*Number)
	if !ok || left.Value != "2" {
		t.Errorf("Expected left child to be number '2'")
	}

	// Right should be multiplication
	right, ok := root.Right.(*BinaryOp)
	if !ok || right.Operator != "*" {
		t.Errorf("Expected right child to be multiplication")
	}
}

// TestParserIOContract_FloatingPoint tests case 15: 3.14 2 *
func TestParserIOContract_FloatingPoint(t *testing.T) {
	// Test case 15: 3.14 2 *
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
		t.Fatalf("Expected BinaryOp, got: %T", result)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "3.14" {
		t.Errorf("Expected left operand '3.14', got: %v", binOp.Left)
	}
}

// TestParserIOContract_ComplexNested tests case 17: 1 2 + 3 4 + *
func TestParserIOContract_ComplexNested(t *testing.T) {
	// Test case 17: 1 2 + 3 4 + * => (1 + 2) * (3 + 4)
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
	if !ok || root.Operator != "*" {
		t.Fatalf("Expected root to be multiplication")
	}

	// Both children should be additions
	left, ok := root.Left.(*BinaryOp)
	if !ok || left.Operator != "+" {
		t.Errorf("Expected left child to be addition")
	}

	right, ok := root.Right.(*BinaryOp)
	if !ok || right.Operator != "+" {
		t.Errorf("Expected right child to be addition")
	}
}

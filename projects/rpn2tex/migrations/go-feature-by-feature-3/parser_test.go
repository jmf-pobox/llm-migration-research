package rpn2tex

import (
	"testing"
)

func TestParser_SingleNumber(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenEOF, Value: "", Line: 1, Column: 2},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	num, ok := ast.(*Number)
	if !ok {
		t.Fatalf("expected *Number, got %T", ast)
	}
	if num.Value != "5" {
		t.Errorf("expected value '5', got '%s'", num.Value)
	}
}

func TestParser_Float(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "3.14", Line: 1, Column: 1},
		{Type: TokenEOF, Value: "", Line: 1, Column: 5},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	num, ok := ast.(*Number)
	if !ok {
		t.Fatalf("expected *Number, got %T", ast)
	}
	if num.Value != "3.14" {
		t.Errorf("expected value '3.14', got '%s'", num.Value)
	}
}

func TestParser_EmptyExpression(t *testing.T) {
	tokens := []Token{
		{Type: TokenEOF, Value: "", Line: 1, Column: 1},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for empty expression, got nil")
	}
}

func TestParser_TooManyValues(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
		{Type: TokenEOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for too many values on stack, got nil")
	}
}

func TestParser_Addition(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
		{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
		{Type: TokenEOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	binOp, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("expected *BinaryOp, got %T", ast)
	}
	if binOp.Operator != "+" {
		t.Errorf("expected operator '+', got '%s'", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("expected left operand '5', got %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("expected right operand '3', got %v", binOp.Right)
	}
}

func TestParser_AdditionUnderflow(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenPlus, Value: "+", Line: 1, Column: 3},
		{Type: TokenEOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for insufficient operands, got nil")
	}
}

func TestParser_Subtraction(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
		{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
		{Type: TokenEOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	binOp, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("expected *BinaryOp, got %T", ast)
	}
	if binOp.Operator != "-" {
		t.Errorf("expected operator '-', got '%s'", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("expected left operand '5', got %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("expected right operand '3', got %v", binOp.Right)
	}
}

func TestParser_SubtractionUnderflow(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenMinus, Value: "-", Line: 1, Column: 3},
		{Type: TokenEOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for insufficient operands, got nil")
	}
}

func TestParser_Multiplication(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "4", Line: 1, Column: 1},
		{Type: TokenNumber, Value: "7", Line: 1, Column: 3},
		{Type: TokenStar, Value: "*", Line: 1, Column: 5},
		{Type: TokenEOF, Value: "", Line: 1, Column: 6},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	binOp, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("expected *BinaryOp, got %T", ast)
	}
	if binOp.Operator != "*" {
		t.Errorf("expected operator '*', got '%s'", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "4" {
		t.Errorf("expected left operand '4', got %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "7" {
		t.Errorf("expected right operand '7', got %v", binOp.Right)
	}
}

func TestParser_MultiplicationUnderflow(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
		{Type: TokenStar, Value: "*", Line: 1, Column: 3},
		{Type: TokenEOF, Value: "", Line: 1, Column: 4},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for insufficient operands, got nil")
	}
}

func TestParser_Division(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
		{Type: TokenNumber, Value: "2", Line: 1, Column: 4},
		{Type: TokenSlash, Value: "/", Line: 1, Column: 6},
		{Type: TokenEOF, Value: "", Line: 1, Column: 7},
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	binOp, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("expected *BinaryOp, got %T", ast)
	}
	if binOp.Operator != "/" {
		t.Errorf("expected operator '/', got '%s'", binOp.Operator)
	}

	left, ok := binOp.Left.(*Number)
	if !ok || left.Value != "10" {
		t.Errorf("expected left operand '10', got %v", binOp.Left)
	}

	right, ok := binOp.Right.(*Number)
	if !ok || right.Value != "2" {
		t.Errorf("expected right operand '2', got %v", binOp.Right)
	}
}

func TestParser_DivisionUnderflow(t *testing.T) {
	tokens := []Token{
		{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
		{Type: TokenSlash, Value: "/", Line: 1, Column: 4},
		{Type: TokenEOF, Value: "", Line: 1, Column: 5},
	}

	parser := NewParser(tokens)
	_, err := parser.Parse()
	if err == nil {
		t.Fatal("expected error for insufficient operands, got nil")
	}
}

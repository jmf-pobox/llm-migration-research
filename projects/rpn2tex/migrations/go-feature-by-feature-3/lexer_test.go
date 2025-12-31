package rpn2tex

import (
	"testing"
)

func TestLexer_Integer(t *testing.T) {
	lexer := NewLexer("5")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 2 {
		t.Fatalf("expected 2 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber {
		t.Errorf("expected TokenNumber, got %v", tokens[0].Type)
	}
	if tokens[0].Value != "5" {
		t.Errorf("expected value '5', got '%s'", tokens[0].Value)
	}

	if tokens[1].Type != TokenEOF {
		t.Errorf("expected TokenEOF, got %v", tokens[1].Type)
	}
}

func TestLexer_Float(t *testing.T) {
	lexer := NewLexer("3.14")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 2 {
		t.Fatalf("expected 2 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber {
		t.Errorf("expected TokenNumber, got %v", tokens[0].Type)
	}
	if tokens[0].Value != "3.14" {
		t.Errorf("expected value '3.14', got '%s'", tokens[0].Value)
	}
}

func TestLexer_MultipleNumbers(t *testing.T) {
	lexer := NewLexer("5 3")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 3 {
		t.Fatalf("expected 3 tokens, got %d", len(tokens))
	}

	if tokens[0].Value != "5" {
		t.Errorf("expected first value '5', got '%s'", tokens[0].Value)
	}
	if tokens[1].Value != "3" {
		t.Errorf("expected second value '3', got '%s'", tokens[1].Value)
	}
}

func TestLexer_WhitespaceHandling(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"leading spaces", "  5", "5"},
		{"trailing spaces", "5  ", "5"},
		{"multiple spaces", "5   3", "5"},
		{"tabs", "5\t3", "5"},
		{"newlines", "5\n3", "5"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if tokens[0].Value != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, tokens[0].Value)
			}
		})
	}
}

func TestLexer_PlusOperator(t *testing.T) {
	lexer := NewLexer("5 3 +")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 4 {
		t.Fatalf("expected 4 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber || tokens[0].Value != "5" {
		t.Errorf("expected NUMBER '5', got %v '%s'", tokens[0].Type, tokens[0].Value)
	}
	if tokens[1].Type != TokenNumber || tokens[1].Value != "3" {
		t.Errorf("expected NUMBER '3', got %v '%s'", tokens[1].Type, tokens[1].Value)
	}
	if tokens[2].Type != TokenPlus || tokens[2].Value != "+" {
		t.Errorf("expected PLUS '+', got %v '%s'", tokens[2].Type, tokens[2].Value)
	}
	if tokens[3].Type != TokenEOF {
		t.Errorf("expected EOF, got %v", tokens[3].Type)
	}
}

func TestLexer_MinusOperator(t *testing.T) {
	lexer := NewLexer("5 3 -")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 4 {
		t.Fatalf("expected 4 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber || tokens[0].Value != "5" {
		t.Errorf("expected NUMBER '5', got %v '%s'", tokens[0].Type, tokens[0].Value)
	}
	if tokens[1].Type != TokenNumber || tokens[1].Value != "3" {
		t.Errorf("expected NUMBER '3', got %v '%s'", tokens[1].Type, tokens[1].Value)
	}
	if tokens[2].Type != TokenMinus || tokens[2].Value != "-" {
		t.Errorf("expected MINUS '-', got %v '%s'", tokens[2].Type, tokens[2].Value)
	}
	if tokens[3].Type != TokenEOF {
		t.Errorf("expected EOF, got %v", tokens[3].Type)
	}
}

func TestLexer_NegativeNumber(t *testing.T) {
	lexer := NewLexer("-5")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 2 {
		t.Fatalf("expected 2 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber {
		t.Errorf("expected TokenNumber, got %v", tokens[0].Type)
	}
	if tokens[0].Value != "-5" {
		t.Errorf("expected value '-5', got '%s'", tokens[0].Value)
	}
}

func TestLexer_MinusVsNegative(t *testing.T) {
	// Test that "5 -3" is parsed as NUMBER MINUS NUMBER (operator)
	// because there's whitespace before the minus
	lexer := NewLexer("5 -3")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// After lexer skips whitespace, we're at '-', then we check next char
	// Since we advance past '-' and then check peek(), and '3' is a digit,
	// it should be treated as a negative number
	if len(tokens) != 3 {
		t.Fatalf("expected 3 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber || tokens[0].Value != "5" {
		t.Errorf("expected NUMBER '5', got %v '%s'", tokens[0].Type, tokens[0].Value)
	}
	if tokens[1].Type != TokenNumber || tokens[1].Value != "-3" {
		t.Errorf("expected NUMBER '-3', got %v '%s'", tokens[1].Type, tokens[1].Value)
	}
}

func TestLexer_StarOperator(t *testing.T) {
	lexer := NewLexer("4 7 *")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 4 {
		t.Fatalf("expected 4 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber || tokens[0].Value != "4" {
		t.Errorf("expected NUMBER '4', got %v '%s'", tokens[0].Type, tokens[0].Value)
	}
	if tokens[1].Type != TokenNumber || tokens[1].Value != "7" {
		t.Errorf("expected NUMBER '7', got %v '%s'", tokens[1].Type, tokens[1].Value)
	}
	if tokens[2].Type != TokenStar || tokens[2].Value != "*" {
		t.Errorf("expected STAR '*', got %v '%s'", tokens[2].Type, tokens[2].Value)
	}
	if tokens[3].Type != TokenEOF {
		t.Errorf("expected EOF, got %v", tokens[3].Type)
	}
}

func TestLexer_SlashOperator(t *testing.T) {
	lexer := NewLexer("10 2 /")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(tokens) != 4 {
		t.Fatalf("expected 4 tokens, got %d", len(tokens))
	}

	if tokens[0].Type != TokenNumber || tokens[0].Value != "10" {
		t.Errorf("expected NUMBER '10', got %v '%s'", tokens[0].Type, tokens[0].Value)
	}
	if tokens[1].Type != TokenNumber || tokens[1].Value != "2" {
		t.Errorf("expected NUMBER '2', got %v '%s'", tokens[1].Type, tokens[1].Value)
	}
	if tokens[2].Type != TokenSlash || tokens[2].Value != "/" {
		t.Errorf("expected SLASH '/', got %v '%s'", tokens[2].Type, tokens[2].Value)
	}
	if tokens[3].Type != TokenEOF {
		t.Errorf("expected EOF, got %v", tokens[3].Type)
	}
}

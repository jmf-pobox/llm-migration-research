package rpn2tex

import (
	"testing"
)

func TestParserNumbers(t *testing.T) {
	tests := []struct {
		name   string
		tokens []Token
		want   *Number
		hasErr bool
	}{
		{
			name: "single integer",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 2},
			},
			want:   &Number{Line: 1, Column: 1, Value: "5"},
			hasErr: false,
		},
		{
			name: "floating point",
			tokens: []Token{
				{Type: TokenNumber, Value: "3.14", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 5},
			},
			want:   &Number{Line: 1, Column: 1, Value: "3.14"},
			hasErr: false,
		},
		{
			name: "invalid - multiple operands",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
			want:   nil,
			hasErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			got, err := parser.Parse()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Parse() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Parse() unexpected error: %v", err)
				return
			}

			num, ok := got.(*Number)
			if !ok {
				t.Errorf("Parse() returned %T, want *Number", got)
				return
			}

			if num.Line != tt.want.Line || num.Column != tt.want.Column || num.Value != tt.want.Value {
				t.Errorf("Parse() = {Line:%d, Column:%d, Value:%q}, want {Line:%d, Column:%d, Value:%q}",
					num.Line, num.Column, num.Value,
					tt.want.Line, tt.want.Column, tt.want.Value)
			}
		})
	}
}

func TestParserAddition(t *testing.T) {
	tests := []struct {
		name   string
		tokens []Token
		hasErr bool
	}{
		{
			name: "simple addition",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name: "chained addition",
			tokens: []Token{
				{Type: TokenNumber, Value: "1", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
			hasErr: false,
		},
		{
			name: "insufficient operands",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 3},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
			hasErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			got, err := parser.Parse()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Parse() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Parse() unexpected error: %v", err)
				return
			}

			binOp, ok := got.(*BinaryOp)
			if !ok {
				t.Errorf("Parse() returned %T, want *BinaryOp", got)
				return
			}

			if binOp.Operator != "+" {
				t.Errorf("Parse() operator = %q, want %q", binOp.Operator, "+")
			}
		})
	}
}

func TestParserSubtraction(t *testing.T) {
	tests := []struct {
		name   string
		tokens []Token
		hasErr bool
	}{
		{
			name: "simple subtraction",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name: "chained subtraction",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 7},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
			hasErr: false,
		},
		{
			name: "insufficient operands",
			tokens: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 3},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
			hasErr: true,
		},
		{
			name: "negative number operand",
			tokens: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 4},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 7},
				{Type: TokenEOF, Value: "", Line: 1, Column: 8},
			},
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			parser := NewParser(tt.tokens)
			got, err := parser.Parse()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Parse() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Parse() unexpected error: %v", err)
				return
			}

			binOp, ok := got.(*BinaryOp)
			if !ok {
				t.Errorf("Parse() returned %T, want *BinaryOp", got)
				return
			}

			if binOp.Operator != "-" {
				t.Errorf("Parse() operator = %q, want %q", binOp.Operator, "-")
			}
		})
	}
}

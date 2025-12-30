package rpn2tex

import "testing"

func TestTokenType_String(t *testing.T) {
	tests := []struct {
		name string
		tt   TokenType
		want string
	}{
		{"NUMBER", NUMBER, "NUMBER"},
		{"PLUS", PLUS, "PLUS"},
		{"MINUS", MINUS, "MINUS"},
		{"MULT", MULT, "MULT"},
		{"DIV", DIV, "DIV"},
		{"EOF", EOF, "EOF"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.tt.String(); got != tt.want {
				t.Errorf("TokenType.String() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestToken_String(t *testing.T) {
	tests := []struct {
		name  string
		token Token
		want  string
	}{
		{
			name:  "NUMBER token",
			token: Token{Type: NUMBER, Value: "42", Line: 1, Column: 1},
			want:  `Token(NUMBER, "42", 1, 1)`,
		},
		{
			name:  "PLUS token",
			token: Token{Type: PLUS, Value: "+", Line: 1, Column: 5},
			want:  `Token(PLUS, "+", 1, 5)`,
		},
		{
			name:  "Decimal number",
			token: Token{Type: NUMBER, Value: "3.14", Line: 2, Column: 10},
			want:  `Token(NUMBER, "3.14", 2, 10)`,
		},
		{
			name:  "EOF token",
			token: Token{Type: EOF, Value: "", Line: 1, Column: 10},
			want:  `Token(EOF, "", 1, 10)`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := tt.token.String(); got != tt.want {
				t.Errorf("Token.String() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestToken_Creation(t *testing.T) {
	// Test that Token struct can be created with all fields
	token := Token{
		Type:   MINUS,
		Value:  "-",
		Line:   3,
		Column: 7,
	}

	if token.Type != MINUS {
		t.Errorf("Token.Type = %v, want MINUS", token.Type)
	}
	if token.Value != "-" {
		t.Errorf("Token.Value = %v, want '-'", token.Value)
	}
	if token.Line != 3 {
		t.Errorf("Token.Line = %v, want 3", token.Line)
	}
	if token.Column != 7 {
		t.Errorf("Token.Column = %v, want 7", token.Column)
	}
}

func TestTokenType_Values(t *testing.T) {
	// Verify that token types have distinct values
	types := []TokenType{NUMBER, PLUS, MINUS, MULT, DIV, EOF}
	seen := make(map[TokenType]bool)

	for _, tt := range types {
		if seen[tt] {
			t.Errorf("Duplicate TokenType value: %v", tt)
		}
		seen[tt] = true
	}

	// Verify expected ordering (iota increments from 0)
	if NUMBER != 0 {
		t.Errorf("NUMBER = %d, want 0", NUMBER)
	}
	if EOF != 5 {
		t.Errorf("EOF = %d, want 5", EOF)
	}
}

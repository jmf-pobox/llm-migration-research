package rpn2tex

import (
	"testing"
)

func TestTokenTypeString(t *testing.T) {
	tests := []struct {
		name     string
		tokType  TokenType
		expected string
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
			got := tt.tokType.String()
			if got != tt.expected {
				t.Errorf("TokenType.String() = %q, want %q", got, tt.expected)
			}
		})
	}
}

func TestTokenCreation(t *testing.T) {
	tests := []struct {
		name       string
		token      Token
		wantType   TokenType
		wantValue  string
		wantLine   int
		wantColumn int
	}{
		{
			name: "NUMBER token",
			token: Token{
				Type:   NUMBER,
				Value:  "42",
				Line:   1,
				Column: 5,
			},
			wantType:   NUMBER,
			wantValue:  "42",
			wantLine:   1,
			wantColumn: 5,
		},
		{
			name: "PLUS token",
			token: Token{
				Type:   PLUS,
				Value:  "+",
				Line:   2,
				Column: 3,
			},
			wantType:   PLUS,
			wantValue:  "+",
			wantLine:   2,
			wantColumn: 3,
		},
		{
			name: "EOF token",
			token: Token{
				Type:   EOF,
				Value:  "",
				Line:   3,
				Column: 10,
			},
			wantType:   EOF,
			wantValue:  "",
			wantLine:   3,
			wantColumn: 10,
		},
		{
			name: "Decimal number token",
			token: Token{
				Type:   NUMBER,
				Value:  "3.14",
				Line:   1,
				Column: 1,
			},
			wantType:   NUMBER,
			wantValue:  "3.14",
			wantLine:   1,
			wantColumn: 1,
		},
		{
			name: "Negative number token",
			token: Token{
				Type:   NUMBER,
				Value:  "-5",
				Line:   1,
				Column: 1,
			},
			wantType:   NUMBER,
			wantValue:  "-5",
			wantLine:   1,
			wantColumn: 1,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.token.Type != tt.wantType {
				t.Errorf("Token.Type = %v, want %v", tt.token.Type, tt.wantType)
			}
			if tt.token.Value != tt.wantValue {
				t.Errorf("Token.Value = %q, want %q", tt.token.Value, tt.wantValue)
			}
			if tt.token.Line != tt.wantLine {
				t.Errorf("Token.Line = %d, want %d", tt.token.Line, tt.wantLine)
			}
			if tt.token.Column != tt.wantColumn {
				t.Errorf("Token.Column = %d, want %d", tt.token.Column, tt.wantColumn)
			}
		})
	}
}

func TestTokenString(t *testing.T) {
	tests := []struct {
		name     string
		token    Token
		expected string
	}{
		{
			name: "NUMBER token string",
			token: Token{
				Type:   NUMBER,
				Value:  "42",
				Line:   1,
				Column: 5,
			},
			expected: `Token(NUMBER, "42", 1:5)`,
		},
		{
			name: "PLUS token string",
			token: Token{
				Type:   PLUS,
				Value:  "+",
				Line:   2,
				Column: 3,
			},
			expected: `Token(PLUS, "+", 2:3)`,
		},
		{
			name: "MINUS token string",
			token: Token{
				Type:   MINUS,
				Value:  "-",
				Line:   1,
				Column: 7,
			},
			expected: `Token(MINUS, "-", 1:7)`,
		},
		{
			name: "MULT token string",
			token: Token{
				Type:   MULT,
				Value:  "*",
				Line:   1,
				Column: 9,
			},
			expected: `Token(MULT, "*", 1:9)`,
		},
		{
			name: "DIV token string",
			token: Token{
				Type:   DIV,
				Value:  "/",
				Line:   1,
				Column: 11,
			},
			expected: `Token(DIV, "/", 1:11)`,
		},
		{
			name: "EOF token string",
			token: Token{
				Type:   EOF,
				Value:  "",
				Line:   3,
				Column: 10,
			},
			expected: `Token(EOF, "", 3:10)`,
		},
		{
			name: "Decimal number token string",
			token: Token{
				Type:   NUMBER,
				Value:  "3.14",
				Line:   1,
				Column: 1,
			},
			expected: `Token(NUMBER, "3.14", 1:1)`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := tt.token.String()
			if got != tt.expected {
				t.Errorf("Token.String() = %q, want %q", got, tt.expected)
			}
		})
	}
}

func TestTokenTypeComparison(t *testing.T) {
	tests := []struct {
		name  string
		tok1  TokenType
		tok2  TokenType
		equal bool
	}{
		{"NUMBER equals NUMBER", NUMBER, NUMBER, true},
		{"PLUS equals PLUS", PLUS, PLUS, true},
		{"NUMBER not equals PLUS", NUMBER, PLUS, false},
		{"MULT not equals DIV", MULT, DIV, false},
		{"EOF equals EOF", EOF, EOF, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := tt.tok1 == tt.tok2
			if got != tt.equal {
				t.Errorf("(%v == %v) = %v, want %v", tt.tok1, tt.tok2, got, tt.equal)
			}
		})
	}
}

func TestTokenPositionTracking(t *testing.T) {
	// Verify that tokens correctly preserve their position information
	// This is critical for error reporting
	token := Token{
		Type:   NUMBER,
		Value:  "123",
		Line:   5,
		Column: 12,
	}

	if token.Line != 5 {
		t.Errorf("Position tracking failed: Line = %d, want 5", token.Line)
	}
	if token.Column != 12 {
		t.Errorf("Position tracking failed: Column = %d, want 12", token.Column)
	}
}

func TestAllOperatorTypes(t *testing.T) {
	// Test that all operator types are correctly represented
	operators := []struct {
		tokType TokenType
		value   string
		name    string
	}{
		{PLUS, "+", "PLUS"},
		{MINUS, "-", "MINUS"},
		{MULT, "*", "MULT"},
		{DIV, "/", "DIV"},
	}

	for _, op := range operators {
		t.Run(op.name, func(t *testing.T) {
			token := Token{
				Type:   op.tokType,
				Value:  op.value,
				Line:   1,
				Column: 1,
			}

			if token.Type != op.tokType {
				t.Errorf("Expected type %v, got %v", op.tokType, token.Type)
			}
			if token.Value != op.value {
				t.Errorf("Expected value %q, got %q", op.value, token.Value)
			}
			if token.Type.String() != op.name {
				t.Errorf("Expected name %q, got %q", op.name, token.Type.String())
			}
		})
	}
}

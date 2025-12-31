package rpn2tex

import (
	"strings"
	"testing"
)

func TestTokenTypeString(t *testing.T) {
	tests := []struct {
		name     string
		tokType  TokenType
		expected string
	}{
		{"NUMBER type", NUMBER, "NUMBER"},
		{"PLUS type", PLUS, "PLUS"},
		{"MINUS type", MINUS, "MINUS"},
		{"MULT type", MULT, "MULT"},
		{"DIV type", DIV, "DIV"},
		{"EOF type", EOF, "EOF"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := tt.tokType.String()
			if result != tt.expected {
				t.Errorf("TokenType.String() = %q, want %q", result, tt.expected)
			}
		})
	}
}

func TestTokenTypeConstants(t *testing.T) {
	// Verify that TokenType constants are distinct
	types := []TokenType{NUMBER, PLUS, MINUS, MULT, DIV, EOF}
	seen := make(map[TokenType]bool)

	for _, typ := range types {
		if seen[typ] {
			t.Errorf("Duplicate TokenType value: %v", typ)
		}
		seen[typ] = true
	}

	// Verify that they start from 0 and increment (iota behavior)
	if NUMBER != 0 {
		t.Errorf("NUMBER should be 0, got %d", NUMBER)
	}
	if PLUS != 1 {
		t.Errorf("PLUS should be 1, got %d", PLUS)
	}
	if MINUS != 2 {
		t.Errorf("MINUS should be 2, got %d", MINUS)
	}
	if MULT != 3 {
		t.Errorf("MULT should be 3, got %d", MULT)
	}
	if DIV != 4 {
		t.Errorf("DIV should be 4, got %d", DIV)
	}
	if EOF != 5 {
		t.Errorf("EOF should be 5, got %d", EOF)
	}
}

func TestTokenCreation(t *testing.T) {
	tests := []struct {
		name     string
		tokType  TokenType
		value    string
		line     int
		column   int
		expected Token
	}{
		{
			name:     "Number token",
			tokType:  NUMBER,
			value:    "42",
			line:     1,
			column:   1,
			expected: Token{Type: NUMBER, Value: "42", Line: 1, Column: 1},
		},
		{
			name:     "Plus operator",
			tokType:  PLUS,
			value:    "+",
			line:     1,
			column:   3,
			expected: Token{Type: PLUS, Value: "+", Line: 1, Column: 3},
		},
		{
			name:     "Minus operator",
			tokType:  MINUS,
			value:    "-",
			line:     2,
			column:   5,
			expected: Token{Type: MINUS, Value: "-", Line: 2, Column: 5},
		},
		{
			name:     "Multiplication operator",
			tokType:  MULT,
			value:    "*",
			line:     3,
			column:   7,
			expected: Token{Type: MULT, Value: "*", Line: 3, Column: 7},
		},
		{
			name:     "Division operator",
			tokType:  DIV,
			value:    "/",
			line:     4,
			column:   9,
			expected: Token{Type: DIV, Value: "/", Line: 4, Column: 9},
		},
		{
			name:     "EOF token",
			tokType:  EOF,
			value:    "",
			line:     5,
			column:   11,
			expected: Token{Type: EOF, Value: "", Line: 5, Column: 11},
		},
		{
			name:     "Decimal number",
			tokType:  NUMBER,
			value:    "3.14",
			line:     1,
			column:   1,
			expected: Token{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
		},
		{
			name:     "Negative number",
			tokType:  NUMBER,
			value:    "-42",
			line:     1,
			column:   1,
			expected: Token{Type: NUMBER, Value: "-42", Line: 1, Column: 1},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			token := Token{
				Type:   tt.tokType,
				Value:  tt.value,
				Line:   tt.line,
				Column: tt.column,
			}

			if token != tt.expected {
				t.Errorf("Token creation failed:\ngot  %+v\nwant %+v", token, tt.expected)
			}

			// Verify individual fields
			if token.Type != tt.expected.Type {
				t.Errorf("Type = %v, want %v", token.Type, tt.expected.Type)
			}
			if token.Value != tt.expected.Value {
				t.Errorf("Value = %q, want %q", token.Value, tt.expected.Value)
			}
			if token.Line != tt.expected.Line {
				t.Errorf("Line = %d, want %d", token.Line, tt.expected.Line)
			}
			if token.Column != tt.expected.Column {
				t.Errorf("Column = %d, want %d", token.Column, tt.expected.Column)
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
			name:     "Number token representation",
			token:    Token{Type: NUMBER, Value: "42", Line: 1, Column: 1},
			expected: "Token(NUMBER, '42', 1:1)",
		},
		{
			name:     "Plus operator representation",
			token:    Token{Type: PLUS, Value: "+", Line: 1, Column: 5},
			expected: "Token(PLUS, '+', 1:5)",
		},
		{
			name:     "Minus operator representation",
			token:    Token{Type: MINUS, Value: "-", Line: 2, Column: 3},
			expected: "Token(MINUS, '-', 2:3)",
		},
		{
			name:     "Multiplication operator representation",
			token:    Token{Type: MULT, Value: "*", Line: 3, Column: 7},
			expected: "Token(MULT, '*', 3:7)",
		},
		{
			name:     "Division operator representation",
			token:    Token{Type: DIV, Value: "/", Line: 4, Column: 9},
			expected: "Token(DIV, '/', 4:9)",
		},
		{
			name:     "EOF token representation",
			token:    Token{Type: EOF, Value: "", Line: 5, Column: 11},
			expected: "Token(EOF, '', 5:11)",
		},
		{
			name:     "Decimal number representation",
			token:    Token{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
			expected: "Token(NUMBER, '3.14', 1:1)",
		},
		{
			name:     "Negative number representation",
			token:    Token{Type: NUMBER, Value: "-5", Line: 2, Column: 1},
			expected: "Token(NUMBER, '-5', 2:1)",
		},
		{
			name:     "Multi-line position",
			token:    Token{Type: NUMBER, Value: "100", Line: 10, Column: 25},
			expected: "Token(NUMBER, '100', 10:25)",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := tt.token.String()
			if result != tt.expected {
				t.Errorf("Token.String() = %q, want %q", result, tt.expected)
			}
		})
	}
}

func TestTokenStringFormat(t *testing.T) {
	// Test that the string format matches Python's __repr__ format
	token := Token{Type: NUMBER, Value: "42", Line: 1, Column: 5}
	str := token.String()

	// Check format components
	if !strings.HasPrefix(str, "Token(") {
		t.Errorf("String should start with 'Token(', got %q", str)
	}
	if !strings.HasSuffix(str, ")") {
		t.Errorf("String should end with ')', got %q", str)
	}
	if !strings.Contains(str, "NUMBER") {
		t.Errorf("String should contain token type 'NUMBER', got %q", str)
	}
	if !strings.Contains(str, "'42'") {
		t.Errorf("String should contain quoted value '42', got %q", str)
	}
	if !strings.Contains(str, "1:5") {
		t.Errorf("String should contain position '1:5', got %q", str)
	}
}

func TestTokenPositionTracking(t *testing.T) {
	// Test that position tracking is 1-based as per specification
	tests := []struct {
		name   string
		token  Token
		line   int
		column int
	}{
		{"First position", Token{Type: NUMBER, Value: "1", Line: 1, Column: 1}, 1, 1},
		{"Second line start", Token{Type: NUMBER, Value: "2", Line: 2, Column: 1}, 2, 1},
		{"Mid line", Token{Type: NUMBER, Value: "3", Line: 1, Column: 10}, 1, 10},
		{"Large position", Token{Type: NUMBER, Value: "4", Line: 100, Column: 50}, 100, 50},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.token.Line != tt.line {
				t.Errorf("Line = %d, want %d (should be 1-based)", tt.token.Line, tt.line)
			}
			if tt.token.Column != tt.column {
				t.Errorf("Column = %d, want %d (should be 1-based)", tt.token.Column, tt.column)
			}

			// Verify positions are positive (1-based means >= 1)
			if tt.token.Line < 1 {
				t.Errorf("Line should be >= 1, got %d", tt.token.Line)
			}
			if tt.token.Column < 1 {
				t.Errorf("Column should be >= 1, got %d", tt.token.Column)
			}
		})
	}
}

func TestTokenValuePreservation(t *testing.T) {
	// Test that token values preserve exact string representation
	tests := []struct {
		name  string
		value string
	}{
		{"Integer", "42"},
		{"Decimal", "3.14"},
		{"Zero", "0"},
		{"Negative integer", "-5"},
		{"Negative decimal", "-3.14"},
		{"Large number", "999999"},
		{"Multiple decimals preserved", "1.5"},
		{"Leading zero decimal", "0.5"},
		{"Trailing zero preserved", "1.0"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			token := Token{Type: NUMBER, Value: tt.value, Line: 1, Column: 1}
			if token.Value != tt.value {
				t.Errorf("Value not preserved: got %q, want %q", token.Value, tt.value)
			}
		})
	}
}

func TestTokenImmutability(t *testing.T) {
	// Test that tokens can be created and their values are accessible
	// Note: Go doesn't enforce immutability at compile time, but we test
	// that the API doesn't provide mutation methods
	token := Token{Type: NUMBER, Value: "42", Line: 1, Column: 1}

	// Make a copy to verify value semantics
	tokenCopy := token

	// Verify both have the same values
	if token != tokenCopy {
		t.Errorf("Token copy should be equal to original")
	}

	// Verify individual fields
	if token.Type != tokenCopy.Type {
		t.Errorf("Type mismatch after copy")
	}
	if token.Value != tokenCopy.Value {
		t.Errorf("Value mismatch after copy")
	}
	if token.Line != tokenCopy.Line {
		t.Errorf("Line mismatch after copy")
	}
	if token.Column != tokenCopy.Column {
		t.Errorf("Column mismatch after copy")
	}
}

func TestOperatorTokenValues(t *testing.T) {
	// Test that operator tokens preserve their string values correctly
	tests := []struct {
		name    string
		tokType TokenType
		value   string
	}{
		{"Plus preserves +", PLUS, "+"},
		{"Minus preserves -", MINUS, "-"},
		{"Mult preserves *", MULT, "*"},
		{"Div preserves /", DIV, "/"},
		{"EOF has empty value", EOF, ""},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			token := Token{Type: tt.tokType, Value: tt.value, Line: 1, Column: 1}
			if token.Value != tt.value {
				t.Errorf("Value = %q, want %q", token.Value, tt.value)
			}
		})
	}
}

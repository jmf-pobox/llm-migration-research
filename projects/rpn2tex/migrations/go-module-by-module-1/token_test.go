package main

import (
	"strings"
	"testing"
)

// TestTokenTypeConstants verifies that all TokenType constants are defined correctly.
func TestTokenTypeConstants(t *testing.T) {
	tests := []struct {
		tokenType TokenType
		expected  string
	}{
		{NUMBER, "NUMBER"},
		{PLUS, "PLUS"},
		{MINUS, "MINUS"},
		{MULT, "MULT"},
		{DIV, "DIV"},
		{EOF, "EOF"},
	}

	for _, tt := range tests {
		t.Run(tt.expected, func(t *testing.T) {
			if got := tt.tokenType.String(); got != tt.expected {
				t.Errorf("TokenType.String() = %q, want %q", got, tt.expected)
			}
		})
	}
}

// TestTokenTypeString verifies String() method for TokenType.
func TestTokenTypeString(t *testing.T) {
	// Test valid token types
	if got := NUMBER.String(); got != "NUMBER" {
		t.Errorf("NUMBER.String() = %q, want %q", got, "NUMBER")
	}

	// Test invalid token type (default case)
	invalidType := TokenType(999)
	got := invalidType.String()
	if !strings.Contains(got, "TokenType(999)") {
		t.Errorf("invalid TokenType.String() = %q, want to contain %q", got, "TokenType(999)")
	}
}

// TestTokenCreation verifies Token struct creation and field access.
func TestTokenCreation(t *testing.T) {
	token := Token{
		Type:   NUMBER,
		Value:  "42",
		Line:   1,
		Column: 5,
	}

	if token.Type != NUMBER {
		t.Errorf("Token.Type = %v, want %v", token.Type, NUMBER)
	}
	if token.Value != "42" {
		t.Errorf("Token.Value = %q, want %q", token.Value, "42")
	}
	if token.Line != 1 {
		t.Errorf("Token.Line = %d, want %d", token.Line, 1)
	}
	if token.Column != 5 {
		t.Errorf("Token.Column = %d, want %d", token.Column, 5)
	}
}

// TestTokenString verifies the String() method for Token.
func TestTokenString(t *testing.T) {
	tests := []struct {
		name     string
		token    Token
		expected string
	}{
		{
			name: "number token",
			token: Token{
				Type:   NUMBER,
				Value:  "42",
				Line:   1,
				Column: 5,
			},
			expected: `Token(NUMBER, "42", 1:5)`,
		},
		{
			name: "plus operator",
			token: Token{
				Type:   PLUS,
				Value:  "+",
				Line:   1,
				Column: 8,
			},
			expected: `Token(PLUS, "+", 1:8)`,
		},
		{
			name: "minus operator",
			token: Token{
				Type:   MINUS,
				Value:  "-",
				Line:   2,
				Column: 3,
			},
			expected: `Token(MINUS, "-", 2:3)`,
		},
		{
			name: "multiplication operator",
			token: Token{
				Type:   MULT,
				Value:  "*",
				Line:   1,
				Column: 10,
			},
			expected: `Token(MULT, "*", 1:10)`,
		},
		{
			name: "division operator",
			token: Token{
				Type:   DIV,
				Value:  "/",
				Line:   3,
				Column: 7,
			},
			expected: `Token(DIV, "/", 3:7)`,
		},
		{
			name: "EOF token",
			token: Token{
				Type:   EOF,
				Value:  "",
				Line:   1,
				Column: 15,
			},
			expected: `Token(EOF, "", 1:15)`,
		},
		{
			name: "floating point number",
			token: Token{
				Type:   NUMBER,
				Value:  "3.14",
				Line:   1,
				Column: 1,
			},
			expected: `Token(NUMBER, "3.14", 1:1)`,
		},
		{
			name: "negative number",
			token: Token{
				Type:   NUMBER,
				Value:  "-5",
				Line:   2,
				Column: 10,
			},
			expected: `Token(NUMBER, "-5", 2:10)`,
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

// TestTokenImmutability verifies that Token is immutable by design.
// In Go, structs are value types, so copies are independent.
func TestTokenImmutability(t *testing.T) {
	original := Token{
		Type:   NUMBER,
		Value:  "42",
		Line:   1,
		Column: 5,
	}

	// Create a copy
	copy := original

	// Modify the copy
	copy.Value = "99"
	copy.Line = 10

	// Original should be unchanged
	if original.Value != "42" {
		t.Errorf("Original token was modified: Value = %q, want %q", original.Value, "42")
	}
	if original.Line != 1 {
		t.Errorf("Original token was modified: Line = %d, want %d", original.Line, 1)
	}
}

// TestTokenFieldAccess verifies all fields are accessible.
func TestTokenFieldAccess(t *testing.T) {
	token := Token{
		Type:   PLUS,
		Value:  "+",
		Line:   3,
		Column: 7,
	}

	// All fields should be accessible
	_ = token.Type
	_ = token.Value
	_ = token.Line
	_ = token.Column
}

// TestTokenTypesAreDistinct verifies that all token types have unique values.
func TestTokenTypesAreDistinct(t *testing.T) {
	types := []TokenType{NUMBER, PLUS, MINUS, MULT, DIV, EOF}
	seen := make(map[TokenType]bool)

	for _, tt := range types {
		if seen[tt] {
			t.Errorf("Duplicate TokenType value: %v", tt)
		}
		seen[tt] = true
	}

	if len(seen) != 6 {
		t.Errorf("Expected 6 distinct token types, got %d", len(seen))
	}
}

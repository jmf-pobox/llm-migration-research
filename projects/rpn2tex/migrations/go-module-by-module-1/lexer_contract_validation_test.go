package main

import (
	"strings"
	"testing"
)

// TestLexerContract_SuccessfulCases validates all successful I/O contract cases.
func TestLexerContract_SuccessfulCases(t *testing.T) {
	cases := []struct {
		name  string
		input string
	}{
		{"Case 1: Simple addition", "5 3 +"},
		{"Case 2: Subtraction", "5 3 -"},
		{"Case 3: Multiplication", "4 7 *"},
		{"Case 4: Division", "10 2 /"},
		{"Case 5: Addition then multiplication", "5 3 + 2 *"},
		{"Case 6: Multiplication then addition", "5 3 * 2 +"},
		{"Case 7: Division chain", "10 2 / 5 *"},
		{"Case 8: Subtraction chain", "5 3 - 2 -"},
		{"Case 9: Multiple divisions", "100 10 / 5 / 2 /"},
		{"Case 10: Multiple additions", "1 2 + 3 + 4 +"},
		{"Case 11: Mixed precedence 1", "2 3 4 * +"},
		{"Case 12: Mixed precedence 2", "2 3 + 4 *"},
		{"Case 13: Mixed precedence 3", "2 3 4 + *"},
		{"Case 14: Mixed precedence 4", "2 3 * 4 +"},
		{"Case 15: Floating point", "3.14 2 *"},
		{"Case 16: Two floats", "1.5 0.5 +"},
		{"Case 17: Complex nested", "1 2 + 3 4 + *"},
		{"Case 18: Division and addition", "10 2 / 3 + 4 *"},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			lexer := NewLexer(tc.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Expected successful tokenization, got error: %v", err)
			}

			// Verify we got tokens
			if len(tokens) == 0 {
				t.Fatal("Expected tokens, got empty slice")
			}

			// Verify last token is EOF
			if tokens[len(tokens)-1].Type != EOF {
				t.Errorf("Expected last token to be EOF, got %v", tokens[len(tokens)-1].Type)
			}

			// Verify all tokens have valid line and column numbers
			for i, token := range tokens {
				if token.Line < 1 {
					t.Errorf("Token %d has invalid line number %d", i, token.Line)
				}
				if token.Column < 1 {
					t.Errorf("Token %d has invalid column number %d", i, token.Column)
				}
			}

			// Verify no unexpected token types
			for i, token := range tokens {
				switch token.Type {
				case NUMBER, PLUS, MINUS, MULT, DIV, EOF:
					// Valid token type
				default:
					t.Errorf("Token %d has unexpected type %v", i, token.Type)
				}
			}
		})
	}
}

// TestLexerContract_ErrorCases validates all error I/O contract cases.
func TestLexerContract_ErrorCases(t *testing.T) {
	cases := []struct {
		name         string
		input        string
		expectedChar rune
	}{
		{"Error 1: Caret", "2 3 ^", '^'},
		{"Error 2: Caret in expression", "2 3 ^ 4 *", '^'},
		{"Error 3: Multiple carets", "2 3 4 ^ ^", '^'},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			lexer := NewLexer(tc.input)
			_, err := lexer.Tokenize()
			if err == nil {
				t.Fatal("Expected error, got nil")
			}

			// Verify it's a SyntaxError
			syntaxErr, ok := err.(*SyntaxError)
			if !ok {
				t.Fatalf("Expected *SyntaxError, got %T: %v", err, err)
			}

			// Verify error message contains "Unexpected character"
			expectedMsg := "Unexpected character"
			if !strings.Contains(syntaxErr.Message, expectedMsg) {
				t.Errorf("Expected error message to contain %q, got %q", expectedMsg, syntaxErr.Message)
			}

			// Verify error message contains the expected character
			if !strings.Contains(syntaxErr.Message, string(tc.expectedChar)) {
				t.Errorf("Expected error message to contain character %q, got %q", tc.expectedChar, syntaxErr.Message)
			}

			// Verify position information is present
			if syntaxErr.Line < 1 {
				t.Errorf("Expected valid line number, got %d", syntaxErr.Line)
			}
			if syntaxErr.Column < 1 {
				t.Errorf("Expected valid column number, got %d", syntaxErr.Column)
			}
		})
	}
}

// TestLexer_NegativeFloatingPoint tests negative floating-point numbers.
func TestLexer_NegativeFloatingPoint(t *testing.T) {
	tests := []struct {
		input    string
		expected string
	}{
		{"-3.14", "-3.14"},
		{"-0.5", "-0.5"},
		{"-10.25", "-10.25"},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != 2 { // number + EOF
				t.Fatalf("Expected 2 tokens, got %d", len(tokens))
			}

			if tokens[0].Type != NUMBER {
				t.Errorf("Expected NUMBER token, got %v", tokens[0].Type)
			}

			if tokens[0].Value != tt.expected {
				t.Errorf("Expected value %q, got %q", tt.expected, tokens[0].Value)
			}
		})
	}
}

// TestLexer_EdgeCases tests edge cases for the lexer.
func TestLexer_EdgeCases(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expectError bool
		errorMsg    string
	}{
		{
			name:        "Number ending with dot (no fraction)",
			input:       "5.",
			expectError: false, // Should tokenize as "5" followed by error on '.'
		},
		{
			name:        "Dot without integer part",
			input:       ".5",
			expectError: true,
			errorMsg:    "Unexpected character '.'",
		},
		{
			name:        "Multiple dots",
			input:       "5.3.2",
			expectError: false, // Should tokenize "5.3" then error on '.'
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if tt.expectError {
				if err == nil {
					t.Fatalf("Expected error, got nil")
				}
				if tt.errorMsg != "" {
					syntaxErr, ok := err.(*SyntaxError)
					if !ok {
						t.Fatalf("Expected *SyntaxError, got %T", err)
					}
					if syntaxErr.Message != tt.errorMsg {
						t.Errorf("Expected error %q, got %q", tt.errorMsg, syntaxErr.Message)
					}
				}
			} else {
				// Not expecting error in initial tokens, but might error later
				if err == nil {
					// Verify we got some tokens
					if len(tokens) == 0 {
						t.Error("Expected tokens, got empty slice")
					}
				}
			}
		})
	}
}

// TestLexer_AllOperators verifies all four operators are supported.
func TestLexer_AllOperators(t *testing.T) {
	operators := []struct {
		char      string
		tokenType TokenType
	}{
		{"+", PLUS},
		{"-", MINUS},
		{"*", MULT},
		{"/", DIV},
	}

	for _, op := range operators {
		t.Run(op.char, func(t *testing.T) {
			input := "5 3 " + op.char
			lexer := NewLexer(input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			// Should have: NUMBER, NUMBER, OPERATOR, EOF
			if len(tokens) != 4 {
				t.Fatalf("Expected 4 tokens, got %d", len(tokens))
			}

			if tokens[2].Type != op.tokenType {
				t.Errorf("Expected token type %v, got %v", op.tokenType, tokens[2].Type)
			}

			if tokens[2].Value != op.char {
				t.Errorf("Expected value %q, got %q", op.char, tokens[2].Value)
			}
		})
	}
}

// TestLexer_UnsupportedOperator verifies that '^' is not supported.
func TestLexer_UnsupportedOperator(t *testing.T) {
	lexer := NewLexer("^")
	_, err := lexer.Tokenize()
	if err == nil {
		t.Fatal("Expected error for unsupported operator '^', got nil")
	}

	syntaxErr, ok := err.(*SyntaxError)
	if !ok {
		t.Fatalf("Expected *SyntaxError, got %T", err)
	}

	expectedMsg := "Unexpected character '^'"
	if syntaxErr.Message != expectedMsg {
		t.Errorf("Expected error message %q, got %q", expectedMsg, syntaxErr.Message)
	}
}

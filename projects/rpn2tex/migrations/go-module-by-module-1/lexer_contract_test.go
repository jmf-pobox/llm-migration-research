package rpn2tex

import (
	"testing"
)

// TestLexer_IOContract verifies the lexer matches all I/O contract requirements
func TestLexer_IOContract_SuccessCases(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{"simple addition", "5 3 +"},
		{"simple subtraction", "5 3 -"},
		{"simple multiplication", "4 7 *"},
		{"simple division", "10 2 /"},
		{"precedence 1", "5 3 + 2 *"},
		{"precedence 2", "5 3 * 2 +"},
		{"left-associative division", "10 2 / 5 *"},
		{"left-associative subtraction", "5 3 - 2 -"},
		{"chain of divisions", "100 10 / 5 / 2 /"},
		{"associative addition", "1 2 + 3 + 4 +"},
		{"precedence 3", "2 3 4 * +"},
		{"precedence 4", "2 3 + 4 *"},
		{"precedence 5", "2 3 4 + *"},
		{"precedence 6", "2 3 * 4 +"},
		{"decimal numbers 1", "3.14 2 *"},
		{"decimal numbers 2", "1.5 0.5 +"},
		{"multiple parenthesizations", "1 2 + 3 4 + *"},
		{"complex precedence", "10 2 / 3 + 4 *"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if err != nil {
				t.Errorf("Tokenize() unexpected error for valid input %q: %v", tt.input, err)
				return
			}

			if len(tokens) == 0 {
				t.Errorf("Tokenize() returned empty token list for input %q", tt.input)
				return
			}

			// Verify last token is EOF
			lastToken := tokens[len(tokens)-1]
			if lastToken.Type != EOF {
				t.Errorf("Last token should be EOF, got %v", lastToken.Type)
			}

			// Verify all tokens have valid position information (1-based)
			for i, token := range tokens {
				if token.Line < 1 {
					t.Errorf("Token %d has invalid line number: %d", i, token.Line)
				}
				if token.Column < 1 {
					t.Errorf("Token %d has invalid column number: %d", i, token.Column)
				}
			}
		})
	}
}

func TestLexer_IOContract_ErrorCases(t *testing.T) {
	tests := []struct {
		name           string
		input          string
		expectedLine   int
		expectedColumn int
	}{
		{
			name:           "exponentiation operator",
			input:          "2 3 ^",
			expectedLine:   1,
			expectedColumn: 5,
		},
		{
			name:           "exponentiation in middle",
			input:          "2 3 ^ 4 *",
			expectedLine:   1,
			expectedColumn: 5,
		},
		{
			name:           "multiple exponentiation",
			input:          "2 3 4 ^ ^",
			expectedLine:   1,
			expectedColumn: 7,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()

			if err == nil {
				t.Errorf("Tokenize() expected error for input %q but got none", tt.input)
				return
			}

			lexErr, ok := err.(*LexerError)
			if !ok {
				t.Errorf("Tokenize() error is not LexerError: %v", err)
				return
			}

			if lexErr.Line != tt.expectedLine {
				t.Errorf("LexerError line = %d, want %d", lexErr.Line, tt.expectedLine)
			}

			if lexErr.Column != tt.expectedColumn {
				t.Errorf("LexerError column = %d, want %d", lexErr.Column, tt.expectedColumn)
			}

			// Verify error message mentions the unexpected character
			if lexErr.Message != "Unexpected character '^'" {
				t.Errorf("LexerError message = %q, want %q", lexErr.Message, "Unexpected character '^'")
			}
		})
	}
}

func TestLexer_IOContract_ErrorFormatting(t *testing.T) {
	// Test that errors can be properly formatted with ErrorFormatter
	input := "2 3 ^"
	lexer := NewLexer(input)
	_, err := lexer.Tokenize()

	if err == nil {
		t.Fatal("Expected error for exponentiation operator")
	}

	lexErr, ok := err.(*LexerError)
	if !ok {
		t.Fatalf("Error is not LexerError: %v", err)
	}

	// Create error formatter and format the error
	formatter := NewErrorFormatter(input)
	formatted := formatter.FormatError("Error: "+lexErr.Message, lexErr.Line, lexErr.Column, 1)

	// Verify formatted error contains expected components
	expectedComponents := []string{
		"Error: Unexpected character '^'",
		"1 | 2 3 ^",
		"  |     ^",
	}

	for _, component := range expectedComponents {
		if !contains(formatted, component) {
			t.Errorf("Formatted error missing component %q\nGot:\n%s", component, formatted)
		}
	}
}

func TestLexer_IOContract_DecimalNumberPreservation(t *testing.T) {
	// Verify that decimal numbers are preserved exactly as input
	tests := []struct {
		input    string
		expected string
	}{
		{"3.14", "3.14"},
		{"1.5", "1.5"},
		{"0.5", "0.5"},
		{"10.0", "10.0"},
		{"0.123", "0.123"},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}

			if len(tokens) < 1 {
				t.Fatal("No tokens returned")
			}

			numberToken := tokens[0]
			if numberToken.Type != NUMBER {
				t.Errorf("First token should be NUMBER, got %v", numberToken.Type)
			}

			if numberToken.Value != tt.expected {
				t.Errorf("Number value = %q, want %q", numberToken.Value, tt.expected)
			}
		})
	}
}

func TestLexer_IOContract_OperatorSupport(t *testing.T) {
	// Verify that only the four supported operators work
	supportedOps := []struct {
		op       string
		expected TokenType
	}{
		{"+", PLUS},
		{"-", MINUS},
		{"*", MULT},
		{"/", DIV},
	}

	for _, tt := range supportedOps {
		t.Run("operator "+tt.op, func(t *testing.T) {
			lexer := NewLexer(tt.op)
			tokens, err := lexer.Tokenize()

			if err != nil {
				t.Errorf("Tokenize() error for supported operator %q: %v", tt.op, err)
				return
			}

			if len(tokens) < 1 {
				t.Fatal("No tokens returned")
			}

			opToken := tokens[0]
			if opToken.Type != tt.expected {
				t.Errorf("Operator %q token type = %v, want %v", tt.op, opToken.Type, tt.expected)
			}
		})
	}

	// Verify that unsupported operator '^' causes error
	t.Run("unsupported operator ^", func(t *testing.T) {
		lexer := NewLexer("^")
		_, err := lexer.Tokenize()

		if err == nil {
			t.Error("Expected error for unsupported operator '^'")
		}
	})
}

func TestLexer_IOContract_WhitespaceNormalization(t *testing.T) {
	// Verify that various whitespace inputs produce identical token sequences
	inputs := []string{
		"5 3 +",
		"5  3  +",
		"5   3   +",
		"5\t3\t+",
		"  5 3 +",
		"5 3 +  ",
		" 5  3  + ",
	}

	var expectedTokenTypes []TokenType
	for _, input := range inputs {
		lexer := NewLexer(input)
		tokens, err := lexer.Tokenize()

		if err != nil {
			t.Errorf("Tokenize() error for input %q: %v", input, err)
			continue
		}

		// Extract token types
		var tokenTypes []TokenType
		for _, token := range tokens {
			tokenTypes = append(tokenTypes, token.Type)
		}

		if expectedTokenTypes == nil {
			expectedTokenTypes = tokenTypes
		} else {
			if !tokenTypesEqual(tokenTypes, expectedTokenTypes) {
				t.Errorf("Input %q produced different tokens than expected", input)
			}
		}
	}

	// Verify the expected sequence
	want := []TokenType{NUMBER, NUMBER, PLUS, EOF}
	if !tokenTypesEqual(expectedTokenTypes, want) {
		t.Errorf("Token types = %v, want %v", expectedTokenTypes, want)
	}
}

// Helper function to check if a string contains a substring
func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr || len(s) > len(substr) &&
		(s[:len(substr)] == substr || contains(s[1:], substr)))
}

// Helper function to compare token type slices
func tokenTypesEqual(a, b []TokenType) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}

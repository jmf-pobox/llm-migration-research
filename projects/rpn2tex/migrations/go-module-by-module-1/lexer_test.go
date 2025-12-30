package main

import (
	"testing"
)

// TestLexer_SimpleNumber tests lexing a single number.
func TestLexer_SimpleNumber(t *testing.T) {
	lexer := NewLexer("42")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	expected := []Token{
		{Type: NUMBER, Value: "42", Line: 1, Column: 1},
		{Type: EOF, Value: "", Line: 1, Column: 3},
	}

	if len(tokens) != len(expected) {
		t.Fatalf("Expected %d tokens, got %d", len(expected), len(tokens))
	}

	for i, exp := range expected {
		if tokens[i] != exp {
			t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
		}
	}
}

// TestLexer_FloatingPoint tests lexing decimal numbers.
func TestLexer_FloatingPoint(t *testing.T) {
	tests := []struct {
		input    string
		expected []Token
	}{
		{
			input: "3.14",
			expected: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			input: "1.5 0.5",
			expected: []Token{
				{Type: NUMBER, Value: "1.5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "0.5", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 8},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, exp := range tt.expected {
				if tokens[i] != exp {
					t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
				}
			}
		})
	}
}

// TestLexer_NegativeNumber tests lexing negative numbers.
func TestLexer_NegativeNumber(t *testing.T) {
	lexer := NewLexer("-5")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	expected := []Token{
		{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
		{Type: EOF, Value: "", Line: 1, Column: 3},
	}

	if len(tokens) != len(expected) {
		t.Fatalf("Expected %d tokens, got %d", len(expected), len(tokens))
	}

	for i, exp := range expected {
		if tokens[i] != exp {
			t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
		}
	}
}

// TestLexer_Operators tests lexing all operators.
func TestLexer_Operators(t *testing.T) {
	tests := []struct {
		input    string
		expected TokenType
		value    string
	}{
		{"+", PLUS, "+"},
		{"-", MINUS, "-"},
		{"*", MULT, "*"},
		{"/", DIV, "/"},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != 2 { // operator + EOF
				t.Fatalf("Expected 2 tokens, got %d", len(tokens))
			}

			if tokens[0].Type != tt.expected {
				t.Errorf("Expected type %v, got %v", tt.expected, tokens[0].Type)
			}

			if tokens[0].Value != tt.value {
				t.Errorf("Expected value %q, got %q", tt.value, tokens[0].Value)
			}
		})
	}
}

// TestLexer_MinusDisambiguation tests minus as operator vs negative number.
func TestLexer_MinusDisambiguation(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Minus operator with space",
			input: "5 - 3",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: MINUS, Value: "-", Line: 1, Column: 3},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Negative number",
			input: "5 -3",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "-3", Line: 1, Column: 3},
				{Type: EOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			name:  "Minus followed by non-digit",
			input: "5 - +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: MINUS, Value: "-", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, exp := range tt.expected {
				if tokens[i] != exp {
					t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
				}
			}
		})
	}
}

// TestLexer_ComplexExpression tests lexing a complete RPN expression.
func TestLexer_ComplexExpression(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Simple addition",
			input: "5 3 +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Mixed operators",
			input: "5 3 + 2 *",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Multiple whitespace",
			input: "5   3    +",
			expected: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 10},
				{Type: EOF, Value: "", Line: 1, Column: 11},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, exp := range tt.expected {
				if tokens[i] != exp {
					t.Errorf("Token %d: expected %v, got %v", i, exp, tokens[i])
				}
			}
		})
	}
}

// TestLexer_InvalidCharacter tests error handling for invalid characters.
func TestLexer_InvalidCharacter(t *testing.T) {
	tests := []struct {
		name           string
		input          string
		expectedChar   rune
		expectedLine   int
		expectedColumn int
	}{
		{
			name:           "Caret operator",
			input:          "2 3 ^",
			expectedChar:   '^',
			expectedLine:   1,
			expectedColumn: 5,
		},
		{
			name:           "Invalid character at start",
			input:          "^ 2 3",
			expectedChar:   '^',
			expectedLine:   1,
			expectedColumn: 1,
		},
		{
			name:           "Letter",
			input:          "5 a 3",
			expectedChar:   'a',
			expectedLine:   1,
			expectedColumn: 3,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()
			if err == nil {
				t.Fatal("Expected error, got nil")
			}

			syntaxErr, ok := err.(*SyntaxError)
			if !ok {
				t.Fatalf("Expected *SyntaxError, got %T", err)
			}

			expectedMsg := "Unexpected character"
			if len(syntaxErr.Message) < len(expectedMsg) || syntaxErr.Message[:len(expectedMsg)] != expectedMsg {
				t.Errorf("Expected error message to start with %q, got %q", expectedMsg, syntaxErr.Message)
			}

			if syntaxErr.Line != tt.expectedLine {
				t.Errorf("Expected line %d, got %d", tt.expectedLine, syntaxErr.Line)
			}

			if syntaxErr.Column != tt.expectedColumn {
				t.Errorf("Expected column %d, got %d", tt.expectedColumn, syntaxErr.Column)
			}
		})
	}
}

// TestLexer_EmptyInput tests lexing empty input.
func TestLexer_EmptyInput(t *testing.T) {
	lexer := NewLexer("")
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if len(tokens) != 1 {
		t.Fatalf("Expected 1 token (EOF), got %d", len(tokens))
	}

	if tokens[0].Type != EOF {
		t.Errorf("Expected EOF token, got %v", tokens[0].Type)
	}
}

// TestLexer_WhitespaceOnly tests lexing whitespace-only input.
func TestLexer_WhitespaceOnly(t *testing.T) {
	inputs := []string{
		" ",
		"   ",
		"\t",
		"\n",
		" \t\n ",
	}

	for _, input := range inputs {
		t.Run("whitespace", func(t *testing.T) {
			lexer := NewLexer(input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != 1 {
				t.Fatalf("Expected 1 token (EOF), got %d", len(tokens))
			}

			if tokens[0].Type != EOF {
				t.Errorf("Expected EOF token, got %v", tokens[0].Type)
			}
		})
	}
}

// TestLexer_IOContract tests specific I/O contract cases.
func TestLexer_IOContract(t *testing.T) {
	tests := []struct {
		name  string
		input string
		valid bool // true if should succeed, false if should error
	}{
		{"Addition", "5 3 +", true},
		{"Subtraction", "5 3 -", true},
		{"Multiplication", "4 7 *", true},
		{"Division", "10 2 /", true},
		{"Floating point", "3.14 2 *", true},
		{"Complex expression", "5 3 + 2 *", true},
		{"Invalid caret", "2 3 ^", false},
		{"Caret in expression", "2 3 ^ 4 *", false},
		{"Multiple carets", "2 3 4 ^ ^", false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if tt.valid {
				if err != nil {
					t.Fatalf("Expected success, got error: %v", err)
				}
				if len(tokens) == 0 {
					t.Fatal("Expected tokens, got empty slice")
				}
				// Last token should be EOF
				if tokens[len(tokens)-1].Type != EOF {
					t.Errorf("Expected last token to be EOF, got %v", tokens[len(tokens)-1].Type)
				}
			} else {
				if err == nil {
					t.Fatal("Expected error, got nil")
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
		})
	}
}

// TestLexer_PositionTracking tests that line and column numbers are correct.
func TestLexer_PositionTracking(t *testing.T) {
	input := "5 3 +"
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	expectedPositions := []struct {
		line   int
		column int
	}{
		{1, 1}, // 5
		{1, 3}, // 3
		{1, 5}, // +
		{1, 6}, // EOF
	}

	if len(tokens) != len(expectedPositions) {
		t.Fatalf("Expected %d tokens, got %d", len(expectedPositions), len(tokens))
	}

	for i, pos := range expectedPositions {
		if tokens[i].Line != pos.line {
			t.Errorf("Token %d: expected line %d, got %d", i, pos.line, tokens[i].Line)
		}
		if tokens[i].Column != pos.column {
			t.Errorf("Token %d: expected column %d, got %d", i, pos.column, tokens[i].Column)
		}
	}
}

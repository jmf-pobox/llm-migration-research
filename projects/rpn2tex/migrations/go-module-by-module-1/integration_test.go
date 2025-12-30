package main

import (
	"strings"
	"testing"
)

// TestIOContract_AllSuccessfulCases tests all 18 successful test cases from the I/O contract.
// These tests validate that the complete pipeline (lexer → parser → latex generator) produces
// the exact LaTeX output specified in the I/O contract.
func TestIOContract_AllSuccessfulCases(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{"basic addition", "5 3 +", "$5 + 3$"},
		{"basic subtraction", "5 3 -", "$5 - 3$"},
		{"basic multiplication", "4 7 *", "$4 \\times 7$"},
		{"basic division", "10 2 /", "$10 \\div 2$"},
		{"precedence: addition then multiply", "5 3 + 2 *", "$( 5 + 3 ) \\times 2$"},
		{"precedence: multiply then addition", "5 3 * 2 +", "$5 \\times 3 + 2$"},
		{"precedence: division and multiply left-to-right", "10 2 / 5 *", "$10 \\div 2 \\times 5$"},
		{"chained subtraction left-to-right", "5 3 - 2 -", "$5 - 3 - 2$"},
		{"chained division left-to-right", "100 10 / 5 / 2 /", "$100 \\div 10 \\div 5 \\div 2$"},
		{"chained addition", "1 2 + 3 + 4 +", "$1 + 2 + 3 + 4$"},
		{"mixed precedence: multiply before addition", "2 3 4 * +", "$2 + 3 \\times 4$"},
		{"parentheses around addition due to multiplication", "2 3 + 4 *", "$( 2 + 3 ) \\times 4$"},
		{"parentheses around addition as right operand of multiply", "2 3 4 + *", "$2 \\times ( 3 + 4 )$"},
		{"no parentheses for multiply before addition", "2 3 * 4 +", "$2 \\times 3 + 4$"},
		{"floating point numbers", "3.14 2 *", "$3.14 \\times 2$"},
		{"floating point addition", "1.5 0.5 +", "$1.5 + 0.5$"},
		{"multiple expressions with precedence", "1 2 + 3 4 + *", "$( 1 + 2 ) \\times ( 3 + 4 )$"},
		{"complex precedence: division/add/multiply", "10 2 / 3 + 4 *", "$( 10 \\div 2 + 3 ) \\times 4$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Execute the pipeline
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v", err)
			}

			generator := NewLaTeXGenerator()
			got := generator.Generate(ast)

			if got != tt.expected {
				t.Errorf("Generate() = %q, want %q", got, tt.expected)
			}
		})
	}
}

// TestIOContract_AllErrorCases tests all 3 error test cases from the I/O contract.
// These tests validate that unsupported operators produce appropriate error messages.
func TestIOContract_AllErrorCases(t *testing.T) {
	tests := []struct {
		name         string
		input        string
		errorMessage string
	}{
		{"exponentiation not supported", "2 3 ^", "Unexpected character '^'"},
		{"exponentiation in complex expression", "2 3 ^ 4 *", "Unexpected character '^'"},
		{"multiple exponentiation operators", "2 3 4 ^ ^", "Unexpected character '^'"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := run(tt.input)
			if err == nil {
				t.Fatalf("run() expected error, got nil")
			}

			errMsg := err.Error()
			if !strings.Contains(errMsg, tt.errorMessage) {
				t.Errorf("run() error = %q, should contain %q", errMsg, tt.errorMessage)
			}
		})
	}
}

// TestIntegration_EndToEnd tests the complete system integration.
func TestIntegration_EndToEnd(t *testing.T) {
	testCases := []struct {
		name    string
		input   string
		wantErr bool
		wantOut string
	}{
		{
			name:    "simple expression",
			input:   "5 3 +",
			wantErr: false,
			wantOut: "$5 + 3$",
		},
		{
			name:    "complex expression",
			input:   "1 2 + 3 4 + *",
			wantErr: false,
			wantOut: "$( 1 + 2 ) \\times ( 3 + 4 )$",
		},
		{
			name:    "error handling",
			input:   "2 3 ^",
			wantErr: true,
			wantOut: "Unexpected character '^'",
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Step 1: Lexer
			lexer := NewLexer(tc.input)
			tokens, err := lexer.Tokenize()

			if tc.wantErr {
				if err == nil {
					t.Fatal("expected lexer error, got nil")
				}
				if !strings.Contains(err.Error(), tc.wantOut) {
					t.Errorf("error message = %q, should contain %q", err.Error(), tc.wantOut)
				}
				return
			}

			if err != nil {
				t.Fatalf("Tokenize() unexpected error: %v", err)
			}

			// Step 2: Parser
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() unexpected error: %v", err)
			}

			// Step 3: LaTeX Generator
			generator := NewLaTeXGenerator()
			latex := generator.Generate(ast)

			if latex != tc.wantOut {
				t.Errorf("Generate() = %q, want %q", latex, tc.wantOut)
			}
		})
	}
}

// TestIntegration_ErrorPropagation tests that errors propagate correctly through the pipeline.
func TestIntegration_ErrorPropagation(t *testing.T) {
	tests := []struct {
		name      string
		input     string
		errorType string // "lexer" or "parser"
		contains  string
	}{
		{
			name:      "lexer error: invalid character",
			input:     "2 3 ^",
			errorType: "lexer",
			contains:  "Unexpected character '^'",
		},
		{
			name:      "parser error: empty expression",
			input:     "",
			errorType: "parser",
			contains:  "Empty expression",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := run(tt.input)
			if err == nil {
				t.Fatal("expected error, got nil")
			}

			errMsg := err.Error()
			if !strings.Contains(errMsg, tt.contains) {
				t.Errorf("error = %q, should contain %q", errMsg, tt.contains)
			}
		})
	}
}

// TestIntegration_PositionTracking verifies that position information is correctly tracked
// throughout the pipeline and used in error reporting.
func TestIntegration_PositionTracking(t *testing.T) {
	input := "5 3 ^"

	err := run(input)
	if err == nil {
		t.Fatal("expected error, got nil")
	}

	errMsg := err.Error()

	// Should contain the line number
	if !strings.Contains(errMsg, "1 |") {
		t.Errorf("error message missing line number: %q", errMsg)
	}

	// Should contain a caret pointing to the error
	if !strings.Contains(errMsg, "^") {
		t.Errorf("error message missing caret: %q", errMsg)
	}

	// Should show the source context
	if !strings.Contains(errMsg, "5 3 ^") {
		t.Errorf("error message missing source context: %q", errMsg)
	}
}

// TestIntegration_PrecedenceRules tests that operator precedence is correctly implemented
// across the entire pipeline.
func TestIntegration_PrecedenceRules(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		note     string
	}{
		{
			name:     "addition and multiplication: multiply binds tighter",
			input:    "2 3 4 * +",
			expected: "$2 + 3 \\times 4$",
			note:     "no parentheses because * has higher precedence",
		},
		{
			name:     "multiplication and addition: parens needed",
			input:    "2 3 + 4 *",
			expected: "$( 2 + 3 ) \\times 4$",
			note:     "parentheses needed because + has lower precedence",
		},
		{
			name:     "left associativity: subtraction",
			input:    "5 3 - 2 -",
			expected: "$5 - 3 - 2$",
			note:     "left-to-right: (5 - 3) - 2",
		},
		{
			name:     "left associativity: division",
			input:    "100 10 / 5 /",
			expected: "$100 \\div 10 \\div 5$",
			note:     "left-to-right: (100 / 10) / 5",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}

			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v", err)
			}

			generator := NewLaTeXGenerator()
			got := generator.Generate(ast)

			if got != tt.expected {
				t.Errorf("Generate() = %q, want %q\nNote: %s", got, tt.expected, tt.note)
			}
		})
	}
}

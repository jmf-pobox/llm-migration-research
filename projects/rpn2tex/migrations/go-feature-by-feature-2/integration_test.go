package rpn2tex

import "testing"

// TestEndToEndIOContract verifies the I/O contract for all implemented features.
func TestEndToEndIOContract(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		feature  string
	}{
		// Feature 1: Numbers
		{name: "integer", input: "5", expected: "$5$", feature: "numbers"},
		{name: "float", input: "3.14", expected: "$3.14$", feature: "numbers"},

		// Feature 2: Addition
		{name: "basic addition", input: "5 3 +", expected: "$5 + 3$", feature: "addition"},
		{name: "chained addition", input: "1 2 + 3 + 4 +", expected: "$1 + 2 + 3 + 4$", feature: "addition"},

		// Feature 3: Subtraction
		{name: "basic subtraction", input: "5 3 -", expected: "$5 - 3$", feature: "subtraction"},
		{name: "chained subtraction", input: "5 3 - 2 -", expected: "$5 - 3 - 2$", feature: "subtraction"},

		// Feature 4: Multiplication
		{name: "basic multiplication", input: "4 7 *", expected: `$4 \times 7$`, feature: "multiplication"},
		{name: "mult with addition (right)", input: "2 3 4 * +", expected: `$2 + 3 \times 4$`, feature: "multiplication"},
		{name: "mult with addition (left)", input: "5 3 * 2 +", expected: `$5 \times 3 + 2$`, feature: "multiplication"},
		{name: "chained multiplication", input: "2 3 * 4 *", expected: `$2 \times 3 \times 4$`, feature: "multiplication"},
		{name: "float multiplication", input: "3.14 2 *", expected: `$3.14 \times 2$`, feature: "multiplication"},
		{name: "negative multiplication", input: "-5 3 *", expected: `$-5 \times 3$`, feature: "multiplication"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lexer
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Tokenize() error = %v", err)
			}

			// Parser
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse() error = %v", err)
			}

			// Generator
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("Feature: %s\nInput:    %q\nExpected: %q\nGot:      %q",
					tt.feature, tt.input, tt.expected, output)
			}
		})
	}
}

// TestMultiplicationInteractions tests how multiplication interacts with other operators.
func TestMultiplicationInteractions(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
		note     string
	}{
		{
			name:     "mult before add",
			input:    "2 3 * 4 +",
			expected: `$2 \times 3 + 4$`,
			note:     "Multiplication evaluated first in RPN, output left-to-right",
		},
		{
			name:     "add before mult",
			input:    "2 3 + 4 *",
			expected: `$( 2 + 3 ) \times 4$`,
			note:     "Addition evaluated first in RPN and needs parens due to lower precedence",
		},
		{
			name:     "mult between adds",
			input:    "1 2 + 3 4 * +",
			expected: `$1 + 2 + 3 \times 4$`,
			note:     "Mixed operations show natural precedence in output",
		},
		{
			name:     "mult with subtraction",
			input:    "10 2 * 5 -",
			expected: `$10 \times 2 - 5$`,
			note:     "Multiplication has higher precedence than subtraction",
		},
		{
			name:     "subtraction with mult",
			input:    "10 5 - 2 *",
			expected: `$( 10 - 5 ) \times 2$`,
			note:     "Subtraction needs parens when used as operand to multiplication",
		},
		{
			name:     "complex expression",
			input:    "1 2 + 3 * 4 5 * +",
			expected: `$( 1 + 2 ) \times 3 + 4 \times 5$`,
			note:     "Complex expression with multiple operations and precedence",
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
			output := generator.Generate(ast)

			if output != tt.expected {
				t.Errorf("%s\nInput:    %q\nExpected: %q\nGot:      %q\nNote:     %s",
					tt.name, tt.input, tt.expected, output, tt.note)
			}
		})
	}
}

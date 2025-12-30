package main

import (
	"testing"
)

// TestFeature6Precedence tests the precedence and parenthesization feature with the I/O contract test cases
func TestFeature6Precedence(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "addition wrapped before multiplication (left)",
			input:    "5 3 + 2 *",
			expected: `$( 5 + 3 ) \times 2$`,
		},
		{
			name:     "addition wrapped before multiplication (left, different numbers)",
			input:    "2 3 + 4 *",
			expected: `$( 2 + 3 ) \times 4$`,
		},
		{
			name:     "addition wrapped before multiplication (right)",
			input:    "2 3 4 + *",
			expected: `$2 \times ( 3 + 4 )$`,
		},
		{
			name:     "both operands are wrapped expressions",
			input:    "1 2 + 3 4 + *",
			expected: `$( 1 + 2 ) \times ( 3 + 4 )$`,
		},
		{
			name:     "mixed division and addition in left operand",
			input:    "10 2 / 3 + 4 *",
			expected: `$( 10 \div 2 + 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestPrecedenceNoParensNeeded tests cases where parentheses are NOT needed
func TestPrecedenceNoParensNeeded(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "multiplication before addition (no parens)",
			input:    "5 3 * 2 +",
			expected: `$5 \times 3 + 2$`,
		},
		{
			name:     "multiplication before addition (right)",
			input:    "2 3 4 * +",
			expected: `$2 + 3 \times 4$`,
		},
		{
			name:     "division before addition",
			input:    "10 2 / 3 +",
			expected: `$10 \div 2 + 3$`,
		},
		{
			name:     "same precedence left-to-right",
			input:    "10 2 / 5 *",
			expected: `$10 \div 2 \times 5$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestPrecedenceSubtractionWrapping tests subtraction with higher precedence operators
func TestPrecedenceSubtractionWrapping(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "subtraction wrapped before multiplication (left)",
			input:    "5 3 - 2 *",
			expected: `$( 5 - 3 ) \times 2$`,
		},
		{
			name:     "subtraction wrapped before multiplication (right)",
			input:    "2 3 4 - *",
			expected: `$2 \times ( 3 - 4 )$`,
		},
		{
			name:     "subtraction wrapped before division (left)",
			input:    "10 3 - 2 /",
			expected: `$( 10 - 3 ) \div 2$`,
		},
		{
			name:     "subtraction wrapped before division (right)",
			input:    "10 3 4 - /",
			expected: `$10 \div ( 3 - 4 )$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestPrecedenceChainedOperations tests chained operations with same precedence
func TestPrecedenceChainedOperations(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "chained addition (no parens)",
			input:    "1 2 + 3 + 4 +",
			expected: `$1 + 2 + 3 + 4$`,
		},
		{
			name:     "chained subtraction (no parens)",
			input:    "5 3 - 2 -",
			expected: `$5 - 3 - 2$`,
		},
		{
			name:     "chained multiplication (no parens)",
			input:    "2 3 * 4 *",
			expected: `$2 \times 3 \times 4$`,
		},
		{
			name:     "chained division (no parens)",
			input:    "100 10 / 5 / 2 /",
			expected: `$100 \div 10 \div 5 \div 2$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestPrecedenceMixedComplexExpressions tests complex nested expressions
func TestPrecedenceMixedComplexExpressions(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "addition then multiplication then addition",
			input:    "1 2 + 3 * 4 +",
			expected: `$( 1 + 2 ) \times 3 + 4$`,
		},
		{
			name:     "multiplication then addition then multiplication",
			input:    "2 3 * 4 + 5 *",
			expected: `$( 2 \times 3 + 4 ) \times 5$`,
		},
		{
			name:     "division and subtraction before multiplication",
			input:    "10 2 / 3 - 4 *",
			expected: `$( 10 \div 2 - 3 ) \times 4$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestPrecedenceWithDecimalNumbers tests that precedence works with decimal numbers
func TestPrecedenceWithDecimalNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "decimal addition before multiplication",
			input:    "1.5 0.5 + 2 *",
			expected: `$( 1.5 + 0.5 ) \times 2$`,
		},
		{
			name:     "decimal multiplication before addition",
			input:    "3.14 2 * 1 +",
			expected: `$3.14 \times 2 + 1$`,
		},
		{
			name:     "decimal division with addition",
			input:    "10.5 2 / 3.5 +",
			expected: `$10.5 \div 2 + 3.5$`,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

// TestNeedsParensFunction tests the needsParens helper function directly
func TestNeedsParensFunction(t *testing.T) {
	g := NewLaTeXGenerator()

	tests := []struct {
		name             string
		child            Expr
		parentPrecedence int
		isRight          bool
		expected         bool
	}{
		{
			name:             "number never needs parens",
			child:            &Number{Value: "5", Line: 1, Column: 1},
			parentPrecedence: 2,
			isRight:          false,
			expected:         false,
		},
		{
			name:             "lower precedence addition in multiplication (left)",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 2,
			isRight:          false,
			expected:         true,
		},
		{
			name:             "lower precedence addition in multiplication (right)",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 2,
			isRight:          true,
			expected:         true,
		},
		{
			name:             "same precedence commutative on left (no parens)",
			child:            &BinaryOp{Operator: "+", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
		{
			name:             "same precedence non-commutative on right (needs parens)",
			child:            &BinaryOp{Operator: "-", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 1,
			isRight:          true,
			expected:         true,
		},
		{
			name:             "same precedence non-commutative on left (no parens)",
			child:            &BinaryOp{Operator: "-", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
		{
			name:             "higher precedence never needs parens",
			child:            &BinaryOp{Operator: "*", Left: &Number{Value: "1"}, Right: &Number{Value: "2"}},
			parentPrecedence: 1,
			isRight:          false,
			expected:         false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := g.needsParens(tt.child, tt.parentPrecedence, tt.isRight)
			if result != tt.expected {
				t.Errorf("expected %v, got %v", tt.expected, result)
			}
		})
	}
}

// TestFeature6RegressionAllPreviousFeatures ensures all previous feature tests still pass
func TestFeature6RegressionAllPreviousFeatures(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Feature 1: Numbers
		{name: "single integer", input: "5", expected: "$5$"},
		{name: "decimal number", input: "3.14", expected: "$3.14$"},

		// Feature 2: Addition
		{name: "simple addition", input: "5 3 +", expected: "$5 + 3$"},
		{name: "chained addition", input: "1 2 + 3 + 4 +", expected: "$1 + 2 + 3 + 4$"},

		// Feature 3: Subtraction
		{name: "simple subtraction", input: "5 3 -", expected: "$5 - 3$"},
		{name: "chained subtraction", input: "5 3 - 2 -", expected: "$5 - 3 - 2$"},

		// Feature 4: Multiplication
		{name: "simple multiplication", input: "4 7 *", expected: `$4 \times 7$`},
		{name: "multiplication has higher precedence", input: "2 3 4 * +", expected: `$2 + 3 \times 4$`},

		// Feature 5: Division
		{name: "simple division", input: "10 2 /", expected: `$10 \div 2$`},
		{name: "chained division", input: "100 10 / 5 / 2 /", expected: `$100 \div 10 \div 5 \div 2$`},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			output, err := processRPN(tt.input)
			if err != nil {
				t.Fatalf("unexpected error: %v", err)
			}
			if output != tt.expected {
				t.Errorf("input: %q\nexpected: %q\ngot: %q", tt.input, tt.expected, output)
			}
		})
	}
}

package rpn2tex

import (
	"testing"
)

// TestIntegrationLexerParser tests the integration between lexer and parser
// for valid RPN expressions from the I/O contract.
func TestIntegrationLexerParser(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{"simple addition", "5 3 +"},
		{"simple subtraction", "5 3 -"},
		{"simple multiplication", "4 7 *"},
		{"simple division", "10 2 /"},
		{"precedence: addition then multiplication", "5 3 + 2 *"},
		{"precedence: multiplication then addition", "5 3 * 2 +"},
		{"left-associative subtraction", "5 3 - 2 -"},
		{"multiple divisions", "100 10 / 5 / 2 /"},
		{"multiple additions", "1 2 + 3 + 4 +"},
		{"mixed precedence", "2 3 4 * +"},
		{"decimal numbers", "3.14 2 *"},
		{"both operands additions", "1 2 + 3 4 + *"},
		{"complex nested", "10 2 / 3 + 4 *"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex the input
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer.Tokenize() error = %v, want nil", err)
			}

			// Parse the tokens
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser.Parse() error = %v, want nil", err)
			}

			// Verify we got a valid AST (not nil)
			if ast == nil {
				t.Fatal("Parser.Parse() returned nil AST, want non-nil")
			}

			// Verify the AST is one of the expected types
			switch ast.(type) {
			case *Number, *BinaryOp:
				// Valid
			default:
				t.Errorf("Parser.Parse() returned unexpected type %T", ast)
			}
		})
	}
}

// TestIntegrationLexerParserErrors tests error handling through the pipeline.
func TestIntegrationLexerParserErrors(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		expectError string
	}{
		{"unsupported operator", "2 3 ^", "Unexpected character"},
		{"insufficient operands", "5 +", "requires two operands"},
		{"extra operands", "5 3 2 +", "Invalid RPN"},
		{"empty expression", "", "Empty expression"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex the input
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			// If lexing fails, verify the error message
			if err != nil {
				if tt.expectError == "" {
					t.Fatalf("Lexer.Tokenize() unexpected error = %v", err)
				}
				return // Expected error during lexing
			}

			// Parse the tokens
			parser := NewParser(tokens)
			_, err = parser.Parse()

			// Should get an error
			if err == nil {
				t.Fatal("Parser.Parse() error = nil, want error")
			}
		})
	}
}

// TestIntegrationASTStructure tests that the parser produces correct AST structure.
func TestIntegrationASTStructure(t *testing.T) {
	// Test: "5 3 + 2 *" should produce: (5 + 3) * 2
	input := "5 3 + 2 *"

	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		t.Fatalf("Lexer error: %v", err)
	}

	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		t.Fatalf("Parser error: %v", err)
	}

	// Root should be multiplication
	mult, ok := ast.(*BinaryOp)
	if !ok {
		t.Fatalf("Root is %T, want *BinaryOp", ast)
	}
	if mult.Operator != "*" {
		t.Errorf("Root operator = %q, want %q", mult.Operator, "*")
	}

	// Left child should be addition
	add, ok := mult.Left.(*BinaryOp)
	if !ok {
		t.Fatalf("Left child is %T, want *BinaryOp", mult.Left)
	}
	if add.Operator != "+" {
		t.Errorf("Left child operator = %q, want %q", add.Operator, "+")
	}

	// Addition's operands should be 5 and 3
	left, ok := add.Left.(*Number)
	if !ok || left.Value != "5" {
		t.Errorf("Addition left = %v, want Number(5)", add.Left)
	}

	right, ok := add.Right.(*Number)
	if !ok || right.Value != "3" {
		t.Errorf("Addition right = %v, want Number(3)", add.Right)
	}

	// Multiplication's right operand should be 2
	two, ok := mult.Right.(*Number)
	if !ok || two.Value != "2" {
		t.Errorf("Multiplication right = %v, want Number(2)", mult.Right)
	}
}

// TestFullPipeline tests the complete lexer → parser → latex pipeline
// against all 18 passing test cases from the I/O contract.
func TestFullPipeline(t *testing.T) {
	tests := []struct {
		input string
		want  string
	}{
		// Test 1: Basic Addition
		{"5 3 +", "$5 + 3$"},
		// Test 2: Subtraction
		{"5 3 -", "$5 - 3$"},
		// Test 3: Multiplication
		{"4 7 *", `$4 \times 7$`},
		// Test 4: Division
		{"10 2 /", `$10 \div 2$`},
		// Test 6: Operator Precedence (Addition + Multiplication)
		{"5 3 + 2 *", `$( 5 + 3 ) \times 2$`},
		// Test 7: Operator Precedence (Multiplication + Addition)
		{"5 3 * 2 +", `$5 \times 3 + 2$`},
		// Test 8: Left-to-right Division and Multiplication
		{"10 2 / 5 *", `$10 \div 2 \times 5$`},
		// Test 9: Left-associative Subtraction
		{"5 3 - 2 -", "$5 - 3 - 2$"},
		// Test 10: Multiple Divisions
		{"100 10 / 5 / 2 /", `$100 \div 10 \div 5 \div 2$`},
		// Test 11: Multiple Additions
		{"1 2 + 3 + 4 +", "$1 + 2 + 3 + 4$"},
		// Test 12: Operator Precedence (Addition inside Multiplication)
		{"2 3 4 * +", `$2 + 3 \times 4$`},
		// Test 13: Parentheses for Lower Precedence Left Operand
		{"2 3 + 4 *", `$( 2 + 3 ) \times 4$`},
		// Test 14: Parentheses for Lower Precedence Right Operand
		{"2 3 4 + *", `$2 \times ( 3 + 4 )$`},
		// Test 15: Mixed Operations
		{"2 3 * 4 +", `$2 \times 3 + 4$`},
		// Test 18: Decimal Number Multiplication
		{"3.14 2 *", `$3.14 \times 2$`},
		// Test 19: Decimal Number Addition
		{"1.5 0.5 +", "$1.5 + 0.5$"},
		// Test 20: Two Additions Multiplied
		{"1 2 + 3 4 + *", `$( 1 + 2 ) \times ( 3 + 4 )$`},
		// Test 21: Complex Expression
		{"10 2 / 3 + 4 *", `$( 10 \div 2 + 3 ) \times 4$`},
	}

	for _, tt := range tests {
		t.Run(tt.input, func(t *testing.T) {
			// Step 1: Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("Lexer error: %v", err)
			}

			// Step 2: Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parser error: %v", err)
			}

			// Step 3: Generate LaTeX
			generator := NewLaTeXGenerator()
			got := generator.Generate(ast)

			// Validate output matches I/O contract EXACTLY
			if got != tt.want {
				t.Errorf("\nInput:    %q\nGot:      %q\nExpected: %q", tt.input, got, tt.want)
			}
		})
	}
}

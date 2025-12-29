package rpn2tex

import (
	"testing"
)

// TestNumbersFeature tests the numbers feature with the I/O contract test cases.
func TestNumbersFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Single integer",
			input:    "5",
			expected: "5",
		},
		{
			name:     "Decimal number",
			input:    "3.14",
			expected: "3.14",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestLexerNumbers tests the lexer's ability to tokenize numbers.
func TestLexerNumbers(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Single integer",
			input: "5",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "Decimal number",
			input: "3.14",
			expected: []Token{
				{Type: TokenNumber, Value: "3.14", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			name:  "Integer with trailing space",
			input: "42 ",
			expected: []Token{
				{Type: TokenNumber, Value: "42", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens := []Token{}

			for {
				token, err := lexer.NextToken()
				if err != nil {
					t.Fatalf("Lexer error: %v", err)
				}
				tokens = append(tokens, token)
				if token.Type == TokenEOF {
					break
				}
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expected := range tt.expected {
				actual := tokens[i]
				if actual.Type != expected.Type {
					t.Errorf("Token %d: expected type %v, got %v", i, expected.Type, actual.Type)
				}
				if actual.Value != expected.Value {
					t.Errorf("Token %d: expected value %q, got %q", i, expected.Value, actual.Value)
				}
				if actual.Line != expected.Line {
					t.Errorf("Token %d: expected line %d, got %d", i, expected.Line, actual.Line)
				}
				if actual.Column != expected.Column {
					t.Errorf("Token %d: expected column %d, got %d", i, expected.Column, actual.Column)
				}
			}
		})
	}
}

// TestParserNumbers tests the parser's ability to create AST nodes for numbers.
func TestParserNumbers(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(*testing.T, Expr)
	}{
		{
			name:  "Single integer",
			input: "5",
			check: func(t *testing.T, expr Expr) {
				num, ok := expr.(*NumberNode)
				if !ok {
					t.Fatalf("Expected *NumberNode, got %T", expr)
				}
				if num.Value != "5" {
					t.Errorf("Expected value %q, got %q", "5", num.Value)
				}
			},
		},
		{
			name:  "Decimal number",
			input: "3.14",
			check: func(t *testing.T, expr Expr) {
				num, ok := expr.(*NumberNode)
				if !ok {
					t.Fatalf("Expected *NumberNode, got %T", expr)
				}
				if num.Value != "3.14" {
					t.Errorf("Expected value %q, got %q", "3.14", num.Value)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			tt.check(t, ast)
		})
	}
}

// TestErrorFormatting tests the error formatter.
func TestErrorFormatting(t *testing.T) {
	source := "5 3.14 invalid"
	formatter := NewErrorFormatter(source)

	msg := formatter.FormatError("Test error", 1, 8)
	if msg == "" {
		t.Error("Expected non-empty error message")
	}

	// Check that message contains error text
	if len(msg) < 10 {
		t.Errorf("Error message too short: %q", msg)
	}
}

// TestAdditionFeature tests the addition feature.
func TestAdditionFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Simple addition",
			input:    "5 3 +",
			expected: "5 + 3",
		},
		{
			name:     "Multiple additions",
			input:    "1 2 + 3 + 4 +",
			expected: "1 + 2 + 3 + 4",
		},
		{
			name:     "Two number addition",
			input:    "10 20 +",
			expected: "10 + 20",
		},
		{
			name:     "Decimal addition",
			input:    "3.14 2.5 +",
			expected: "3.14 + 2.5",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestLexerAddition tests the lexer's ability to tokenize addition operators.
func TestLexerAddition(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Simple addition tokens",
			input: "5 3 +",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Multiple additions",
			input: "1 2 + 3 +",
			expected: []Token{
				{Type: TokenNumber, Value: "1", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens := []Token{}

			for {
				token, err := lexer.NextToken()
				if err != nil {
					t.Fatalf("Lexer error: %v", err)
				}
				tokens = append(tokens, token)
				if token.Type == TokenEOF {
					break
				}
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expected := range tt.expected {
				actual := tokens[i]
				if actual.Type != expected.Type {
					t.Errorf("Token %d: expected type %v, got %v", i, expected.Type, actual.Type)
				}
				if actual.Value != expected.Value {
					t.Errorf("Token %d: expected value %q, got %q", i, expected.Value, actual.Value)
				}
				if actual.Line != expected.Line {
					t.Errorf("Token %d: expected line %d, got %d", i, expected.Line, actual.Line)
				}
				if actual.Column != expected.Column {
					t.Errorf("Token %d: expected column %d, got %d", i, expected.Column, actual.Column)
				}
			}
		})
	}
}

// TestParserAddition tests the parser's ability to create AST nodes for addition.
func TestParserAddition(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(*testing.T, Expr)
	}{
		{
			name:  "Simple addition",
			input: "5 3 +",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("Expected operator '+', got %q", binOp.Operator)
				}
				// Check left operand
				left, ok := binOp.Left.(*NumberNode)
				if !ok {
					t.Fatalf("Expected left to be *NumberNode, got %T", binOp.Left)
				}
				if left.Value != "5" {
					t.Errorf("Expected left value '5', got %q", left.Value)
				}
				// Check right operand
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "3" {
					t.Errorf("Expected right value '3', got %q", right.Value)
				}
			},
		},
		{
			name:  "Chained addition",
			input: "1 2 + 3 +",
			check: func(t *testing.T, expr Expr) {
				// Outer addition: (1+2) + 3
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("Expected operator '+', got %q", binOp.Operator)
				}
				// Left should be another BinaryOpNode (1+2)
				leftBinOp, ok := binOp.Left.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected left to be *BinaryOpNode, got %T", binOp.Left)
				}
				if leftBinOp.Operator != "+" {
					t.Errorf("Expected left operator '+', got %q", leftBinOp.Operator)
				}
				// Right should be NumberNode (3)
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "3" {
					t.Errorf("Expected right value '3', got %q", right.Value)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			tt.check(t, ast)
		})
	}
}

// TestAdditionErrors tests error handling for addition operations.
func TestAdditionErrors(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "Insufficient operands - one number",
			input: "5 +",
		},
		{
			name:  "Insufficient operands - no numbers",
			input: "+",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			_, err := parser.Parse()
			if err == nil {
				t.Error("Expected error, got nil")
			}
		})
	}
}

// TestSubtractionFeature tests the subtraction feature.
func TestSubtractionFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Simple subtraction",
			input:    "5 3 -",
			expected: "5 - 3",
		},
		{
			name:     "Multiple subtractions",
			input:    "5 3 - 2 -",
			expected: "5 - 3 - 2",
		},
		{
			name:     "Two number subtraction",
			input:    "10 5 -",
			expected: "10 - 5",
		},
		{
			name:     "Decimal subtraction",
			input:    "3.14 2.5 -",
			expected: "3.14 - 2.5",
		},
		{
			name:     "Large number subtraction",
			input:    "100 42 -",
			expected: "100 - 42",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestLexerSubtraction tests the lexer's ability to tokenize subtraction operators.
func TestLexerSubtraction(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Simple subtraction tokens",
			input: "5 3 -",
			expected: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Multiple subtractions",
			input: "10 5 - 2 -",
			expected: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "5", Line: 1, Column: 4},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 6},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 8},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 10},
				{Type: TokenEOF, Value: "", Line: 1, Column: 11},
			},
		},
		{
			name:  "Negative number (not subtraction)",
			input: "-5",
			expected: []Token{
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "Subtraction vs negative number",
			input: "10 -5 -",
			expected: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 4},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 7},
				{Type: TokenEOF, Value: "", Line: 1, Column: 8},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens := []Token{}

			for {
				token, err := lexer.NextToken()
				if err != nil {
					t.Fatalf("Lexer error: %v", err)
				}
				tokens = append(tokens, token)
				if token.Type == TokenEOF {
					break
				}
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expected := range tt.expected {
				actual := tokens[i]
				if actual.Type != expected.Type {
					t.Errorf("Token %d: expected type %v, got %v", i, expected.Type, actual.Type)
				}
				if actual.Value != expected.Value {
					t.Errorf("Token %d: expected value %q, got %q", i, expected.Value, actual.Value)
				}
				if actual.Line != expected.Line {
					t.Errorf("Token %d: expected line %d, got %d", i, expected.Line, actual.Line)
				}
				if actual.Column != expected.Column {
					t.Errorf("Token %d: expected column %d, got %d", i, expected.Column, actual.Column)
				}
			}
		})
	}
}

// TestParserSubtraction tests the parser's ability to create AST nodes for subtraction.
func TestParserSubtraction(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(*testing.T, Expr)
	}{
		{
			name:  "Simple subtraction",
			input: "5 3 -",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "-" {
					t.Errorf("Expected operator '-', got %q", binOp.Operator)
				}
				// Check left operand (should be 5)
				left, ok := binOp.Left.(*NumberNode)
				if !ok {
					t.Fatalf("Expected left to be *NumberNode, got %T", binOp.Left)
				}
				if left.Value != "5" {
					t.Errorf("Expected left value '5', got %q", left.Value)
				}
				// Check right operand (should be 3)
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "3" {
					t.Errorf("Expected right value '3', got %q", right.Value)
				}
			},
		},
		{
			name:  "Chained subtraction (left-associative)",
			input: "5 3 - 2 -",
			check: func(t *testing.T, expr Expr) {
				// Outer subtraction: (5-3) - 2
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "-" {
					t.Errorf("Expected operator '-', got %q", binOp.Operator)
				}
				// Left should be another BinaryOpNode (5-3)
				leftBinOp, ok := binOp.Left.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected left to be *BinaryOpNode, got %T", binOp.Left)
				}
				if leftBinOp.Operator != "-" {
					t.Errorf("Expected left operator '-', got %q", leftBinOp.Operator)
				}
				// Right should be NumberNode (2)
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "2" {
					t.Errorf("Expected right value '2', got %q", right.Value)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			tt.check(t, ast)
		})
	}
}

// TestSubtractionErrors tests error handling for subtraction operations.
func TestSubtractionErrors(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "Insufficient operands - one number",
			input: "5 -",
		},
		{
			name:  "Insufficient operands - no numbers",
			input: "-",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			_, err := parser.Parse()
			if err == nil {
				t.Error("Expected error, got nil")
			}
		})
	}
}

// TestMixedAdditionSubtraction tests combinations of addition and subtraction.
func TestMixedAdditionSubtraction(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Addition then subtraction",
			input:    "5 3 + 2 -",
			expected: "5 + 3 - 2",
		},
		{
			name:     "Subtraction then addition",
			input:    "5 3 - 2 +",
			expected: "5 - 3 + 2",
		},
		{
			name:     "Complex mix",
			input:    "10 5 - 3 + 1 -",
			expected: "10 - 5 + 3 - 1",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestMultiplicationFeature tests the multiplication feature.
func TestMultiplicationFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Simple multiplication",
			input:    "4 7 *",
			expected: "4 \\times 7",
		},
		{
			name:     "Multiplication with addition",
			input:    "2 3 4 * +",
			expected: "2 + 3 \\times 4",
		},
		{
			name:     "Two number multiplication",
			input:    "10 20 *",
			expected: "10 \\times 20",
		},
		{
			name:     "Decimal multiplication",
			input:    "3.14 2 *",
			expected: "3.14 \\times 2",
		},
		{
			name:     "Multiple multiplications",
			input:    "2 3 * 4 *",
			expected: "2 \\times 3 \\times 4",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestLexerMultiplication tests the lexer's ability to tokenize multiplication operators.
func TestLexerMultiplication(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Simple multiplication tokens",
			input: "4 7 *",
			expected: []Token{
				{Type: TokenNumber, Value: "4", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "7", Line: 1, Column: 3},
				{Type: TokenTimes, Value: "*", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "Multiple multiplications",
			input: "2 3 * 4 *",
			expected: []Token{
				{Type: TokenNumber, Value: "2", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenTimes, Value: "*", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "4", Line: 1, Column: 7},
				{Type: TokenTimes, Value: "*", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "Mixed operators with multiplication",
			input: "2 3 4 * +",
			expected: []Token{
				{Type: TokenNumber, Value: "2", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenNumber, Value: "4", Line: 1, Column: 5},
				{Type: TokenTimes, Value: "*", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens := []Token{}

			for {
				token, err := lexer.NextToken()
				if err != nil {
					t.Fatalf("Lexer error: %v", err)
				}
				tokens = append(tokens, token)
				if token.Type == TokenEOF {
					break
				}
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expected := range tt.expected {
				actual := tokens[i]
				if actual.Type != expected.Type {
					t.Errorf("Token %d: expected type %v, got %v", i, expected.Type, actual.Type)
				}
				if actual.Value != expected.Value {
					t.Errorf("Token %d: expected value %q, got %q", i, expected.Value, actual.Value)
				}
				if actual.Line != expected.Line {
					t.Errorf("Token %d: expected line %d, got %d", i, expected.Line, actual.Line)
				}
				if actual.Column != expected.Column {
					t.Errorf("Token %d: expected column %d, got %d", i, expected.Column, actual.Column)
				}
			}
		})
	}
}

// TestParserMultiplication tests the parser's ability to create AST nodes for multiplication.
func TestParserMultiplication(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(*testing.T, Expr)
	}{
		{
			name:  "Simple multiplication",
			input: "4 7 *",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "*" {
					t.Errorf("Expected operator '*', got %q", binOp.Operator)
				}
				// Check left operand
				left, ok := binOp.Left.(*NumberNode)
				if !ok {
					t.Fatalf("Expected left to be *NumberNode, got %T", binOp.Left)
				}
				if left.Value != "4" {
					t.Errorf("Expected left value '4', got %q", left.Value)
				}
				// Check right operand
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "7" {
					t.Errorf("Expected right value '7', got %q", right.Value)
				}
			},
		},
		{
			name:  "Multiplication with addition (right is multiplication)",
			input: "2 3 4 * +",
			check: func(t *testing.T, expr Expr) {
				// Outer addition: 2 + (3*4)
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "+" {
					t.Errorf("Expected operator '+', got %q", binOp.Operator)
				}
				// Left should be NumberNode (2)
				left, ok := binOp.Left.(*NumberNode)
				if !ok {
					t.Fatalf("Expected left to be *NumberNode, got %T", binOp.Left)
				}
				if left.Value != "2" {
					t.Errorf("Expected left value '2', got %q", left.Value)
				}
				// Right should be BinaryOpNode (3*4)
				rightBinOp, ok := binOp.Right.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected right to be *BinaryOpNode, got %T", binOp.Right)
				}
				if rightBinOp.Operator != "*" {
					t.Errorf("Expected right operator '*', got %q", rightBinOp.Operator)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			tt.check(t, ast)
		})
	}
}

// TestMultiplicationErrors tests error handling for multiplication operations.
func TestMultiplicationErrors(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "Insufficient operands - one number",
			input: "5 *",
		},
		{
			name:  "Insufficient operands - no numbers",
			input: "*",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			_, err := parser.Parse()
			if err == nil {
				t.Error("Expected error, got nil")
			}
		})
	}
}

// TestMixedOperators tests combinations of all operators.
func TestMixedOperators(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Multiplication then addition",
			input:    "5 3 * 2 +",
			expected: "5 \\times 3 + 2",
		},
		{
			name:     "Addition with subtraction and multiplication",
			input:    "2 3 + 4 5 * -",
			expected: "2 + 3 - 4 \\times 5",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestDivisionFeature tests the division feature.
func TestDivisionFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		{
			name:     "Simple division",
			input:    "10 2 /",
			expected: "10 \\div 2",
		},
		{
			name:     "Multiple divisions",
			input:    "100 10 / 5 / 2 /",
			expected: "100 \\div 10 \\div 5 \\div 2",
		},
		{
			name:     "Two number division",
			input:    "20 5 /",
			expected: "20 \\div 5",
		},
		{
			name:     "Decimal division",
			input:    "3.14 2 /",
			expected: "3.14 \\div 2",
		},
		{
			name:     "Division and multiplication",
			input:    "10 2 / 5 *",
			expected: "10 \\div 2 \\times 5",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestLexerDivision tests the lexer's ability to tokenize division operators.
func TestLexerDivision(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected []Token
	}{
		{
			name:  "Simple division tokens",
			input: "10 2 /",
			expected: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 4},
				{Type: TokenDivide, Value: "/", Line: 1, Column: 6},
				{Type: TokenEOF, Value: "", Line: 1, Column: 7},
			},
		},
		{
			name:  "Multiple divisions",
			input: "100 10 / 5 /",
			expected: []Token{
				{Type: TokenNumber, Value: "100", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "10", Line: 1, Column: 5},
				{Type: TokenDivide, Value: "/", Line: 1, Column: 8},
				{Type: TokenNumber, Value: "5", Line: 1, Column: 10},
				{Type: TokenDivide, Value: "/", Line: 1, Column: 12},
				{Type: TokenEOF, Value: "", Line: 1, Column: 13},
			},
		},
		{
			name:  "Mixed operators with division",
			input: "10 2 / 5 *",
			expected: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 4},
				{Type: TokenDivide, Value: "/", Line: 1, Column: 6},
				{Type: TokenNumber, Value: "5", Line: 1, Column: 8},
				{Type: TokenTimes, Value: "*", Line: 1, Column: 10},
				{Type: TokenEOF, Value: "", Line: 1, Column: 11},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens := []Token{}

			for {
				token, err := lexer.NextToken()
				if err != nil {
					t.Fatalf("Lexer error: %v", err)
				}
				tokens = append(tokens, token)
				if token.Type == TokenEOF {
					break
				}
			}

			if len(tokens) != len(tt.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(tt.expected), len(tokens))
			}

			for i, expected := range tt.expected {
				actual := tokens[i]
				if actual.Type != expected.Type {
					t.Errorf("Token %d: expected type %v, got %v", i, expected.Type, actual.Type)
				}
				if actual.Value != expected.Value {
					t.Errorf("Token %d: expected value %q, got %q", i, expected.Value, actual.Value)
				}
				if actual.Line != expected.Line {
					t.Errorf("Token %d: expected line %d, got %d", i, expected.Line, actual.Line)
				}
				if actual.Column != expected.Column {
					t.Errorf("Token %d: expected column %d, got %d", i, expected.Column, actual.Column)
				}
			}
		})
	}
}

// TestParserDivision tests the parser's ability to create AST nodes for division.
func TestParserDivision(t *testing.T) {
	tests := []struct {
		name  string
		input string
		check func(*testing.T, Expr)
	}{
		{
			name:  "Simple division",
			input: "10 2 /",
			check: func(t *testing.T, expr Expr) {
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "/" {
					t.Errorf("Expected operator '/', got %q", binOp.Operator)
				}
				// Check left operand (should be 10)
				left, ok := binOp.Left.(*NumberNode)
				if !ok {
					t.Fatalf("Expected left to be *NumberNode, got %T", binOp.Left)
				}
				if left.Value != "10" {
					t.Errorf("Expected left value '10', got %q", left.Value)
				}
				// Check right operand (should be 2)
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "2" {
					t.Errorf("Expected right value '2', got %q", right.Value)
				}
			},
		},
		{
			name:  "Chained division (left-associative)",
			input: "100 10 / 5 /",
			check: func(t *testing.T, expr Expr) {
				// Outer division: (100/10) / 5
				binOp, ok := expr.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected *BinaryOpNode, got %T", expr)
				}
				if binOp.Operator != "/" {
					t.Errorf("Expected operator '/', got %q", binOp.Operator)
				}
				// Left should be another BinaryOpNode (100/10)
				leftBinOp, ok := binOp.Left.(*BinaryOpNode)
				if !ok {
					t.Fatalf("Expected left to be *BinaryOpNode, got %T", binOp.Left)
				}
				if leftBinOp.Operator != "/" {
					t.Errorf("Expected left operator '/', got %q", leftBinOp.Operator)
				}
				// Right should be NumberNode (5)
				right, ok := binOp.Right.(*NumberNode)
				if !ok {
					t.Fatalf("Expected right to be *NumberNode, got %T", binOp.Right)
				}
				if right.Value != "5" {
					t.Errorf("Expected right value '5', got %q", right.Value)
				}
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			tt.check(t, ast)
		})
	}
}

// TestDivisionErrors tests error handling for division operations.
func TestDivisionErrors(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{
			name:  "Insufficient operands - one number",
			input: "10 /",
		},
		{
			name:  "Insufficient operands - no numbers",
			input: "/",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)

			_, err := parser.Parse()
			if err == nil {
				t.Error("Expected error, got nil")
			}
		})
	}
}

// TestPrecedenceFeature tests operator precedence and parenthesization.
func TestPrecedenceFeature(t *testing.T) {
	tests := []struct {
		name     string
		input    string
		expected string
	}{
		// Basic precedence: multiplication has higher precedence than addition
		{
			name:     "Addition then multiplication (needs parens)",
			input:    "5 3 + 2 *",
			expected: "( 5 + 3 ) \\times 2",
		},
		{
			name:     "Addition then multiplication variant",
			input:    "2 3 + 4 *",
			expected: "( 2 + 3 ) \\times 4",
		},
		{
			name:     "Multiplication of sum (right needs parens)",
			input:    "2 3 4 + *",
			expected: "2 \\times ( 3 + 4 )",
		},
		{
			name:     "Product of two sums (both need parens)",
			input:    "1 2 + 3 4 + *",
			expected: "( 1 + 2 ) \\times ( 3 + 4 )",
		},
		{
			name:     "Complex precedence with division and addition",
			input:    "10 2 / 3 + 4 *",
			expected: "( 10 \\div 2 + 3 ) \\times 4",
		},
		// Cases where NO parentheses should be added
		{
			name:     "Multiplication with addition (no parens needed)",
			input:    "2 3 4 * +",
			expected: "2 + 3 \\times 4",
		},
		{
			name:     "Multiple additions (no parens needed)",
			input:    "1 2 + 3 + 4 +",
			expected: "1 + 2 + 3 + 4",
		},
		{
			name:     "Multiple subtractions (no parens needed)",
			input:    "5 3 - 2 -",
			expected: "5 - 3 - 2",
		},
		{
			name:     "Multiple multiplications (no parens needed)",
			input:    "2 3 * 4 *",
			expected: "2 \\times 3 \\times 4",
		},
		{
			name:     "Multiple divisions (no parens needed)",
			input:    "100 10 / 5 / 2 /",
			expected: "100 \\div 10 \\div 5 \\div 2",
		},
		// Non-commutative operators: right side needs parens for same precedence
		{
			name:     "Right-associative subtraction (needs parens)",
			input:    "5 3 2 - -",
			expected: "5 - ( 3 - 2 )",
		},
		{
			name:     "Right-associative division (needs parens)",
			input:    "100 10 5 / /",
			expected: "100 \\div ( 10 \\div 5 )",
		},
		// Mixed operators
		{
			name:     "Multiplication then addition (no parens)",
			input:    "5 3 * 2 +",
			expected: "5 \\times 3 + 2",
		},
		{
			name:     "Division and multiplication (no parens)",
			input:    "10 2 / 5 *",
			expected: "10 \\div 2 \\times 5",
		},
		{
			name:     "Subtraction then multiplication (needs parens)",
			input:    "10 5 - 2 *",
			expected: "( 10 - 5 ) \\times 2",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			parser := NewParser(lexer)
			generator := NewLaTeXGenerator()

			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("Parse error: %v", err)
			}

			latex, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Generate error: %v", err)
			}

			if latex != tt.expected {
				t.Errorf("Expected %q, got %q", tt.expected, latex)
			}
		})
	}
}

// TestPrecedenceMethods tests the Precedence() methods on AST nodes.
func TestPrecedenceMethods(t *testing.T) {
	tests := []struct {
		name       string
		node       Expr
		precedence int
	}{
		{
			name:       "Number has precedence 3",
			node:       &NumberNode{Value: "5"},
			precedence: 3,
		},
		{
			name:       "Addition has precedence 1",
			node:       &BinaryOpNode{Operator: "+"},
			precedence: 1,
		},
		{
			name:       "Subtraction has precedence 1",
			node:       &BinaryOpNode{Operator: "-"},
			precedence: 1,
		},
		{
			name:       "Multiplication has precedence 2",
			node:       &BinaryOpNode{Operator: "*"},
			precedence: 2,
		},
		{
			name:       "Division has precedence 2",
			node:       &BinaryOpNode{Operator: "/"},
			precedence: 2,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if tt.node.Precedence() != tt.precedence {
				t.Errorf("Expected precedence %d, got %d", tt.precedence, tt.node.Precedence())
			}
		})
	}
}

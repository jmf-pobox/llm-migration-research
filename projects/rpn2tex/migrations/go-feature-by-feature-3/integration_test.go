package rpn2tex

import (
	"testing"
)

func TestIntegration_Numbers(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"integer", "5", "$5$"},
		{"float", "3.14", "$3.14$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

func TestIntegration_Addition(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"simple addition", "5 3 +", "$5 + 3$"},
		{"chained addition", "1 2 + 3 + 4 +", "$1 + 2 + 3 + 4$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

func TestIntegration_Subtraction(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"simple subtraction", "5 3 -", "$5 - 3$"},
		{"chained subtraction", "5 3 - 2 -", "$5 - 3 - 2$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

func TestIntegration_Multiplication(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"simple multiplication", "4 7 *", "$4 \\times 7$"},
		{"multiplication with addition", "2 3 4 * +", "$2 + 3 \\times 4$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

func TestIntegration_Division(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{"simple division", "10 2 /", "$10 \\div 2$"},
		{"chained division", "100 10 / 5 / 2 /", "$100 \\div 10 \\div 5 \\div 2$"},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

func TestIntegration_Precedence(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			name:  "addition on left of multiplication",
			input: "5 3 + 2 *",
			want:  "$( 5 + 3 ) \\times 2$",
		},
		{
			name:  "addition on both sides of multiplication",
			input: "2 3 + 4 *",
			want:  "$( 2 + 3 ) \\times 4$",
		},
		{
			name:  "addition on right of multiplication",
			input: "2 3 4 + *",
			want:  "$2 \\times ( 3 + 4 )$",
		},
		{
			name:  "both sides lower precedence",
			input: "1 2 + 3 4 + *",
			want:  "$( 1 + 2 ) \\times ( 3 + 4 )$",
		},
		{
			name:  "complex with division and addition",
			input: "10 2 / 3 + 4 *",
			want:  "$( 10 \\div 2 + 3 ) \\times 4$",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Tokenize
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				t.Fatalf("lexer error: %v", err)
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				t.Fatalf("parser error: %v", err)
			}

			// Generate
			generator := NewLaTeXGenerator()
			output := generator.Generate(ast)

			if output != tt.want {
				t.Errorf("expected '%s', got '%s'", tt.want, output)
			}
		})
	}
}

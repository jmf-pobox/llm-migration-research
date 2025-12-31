package rpn2tex

import "testing"

// TestIntegrationNumbers tests the complete pipeline for number parsing.
func TestIntegrationNumbers(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   string
		hasErr bool
	}{
		{
			name:   "single integer",
			input:  "5",
			want:   "$5$",
			hasErr: false,
		},
		{
			name:   "floating point",
			input:  "3.14",
			want:   "$3.14$",
			hasErr: false,
		},
		{
			name:   "integer with whitespace",
			input:  "  42  ",
			want:   "$42$",
			hasErr: false,
		},
		{
			name:   "zero",
			input:  "0",
			want:   "$0$",
			hasErr: false,
		},
		{
			name:   "decimal with leading zero",
			input:  "0.5",
			want:   "$0.5$",
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Tokenize() unexpected error: %v", err)
				}
				return
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Parse() unexpected error: %v", err)
				}
				return
			}

			// Generate
			generator := NewGenerator()
			got := generator.Generate(ast)

			if tt.hasErr {
				t.Errorf("Expected error but got output: %q", got)
				return
			}

			if got != tt.want {
				t.Errorf("Integration test: got %q, want %q", got, tt.want)
			}
		})
	}
}

// TestIntegrationAddition tests the complete pipeline for addition.
func TestIntegrationAddition(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   string
		hasErr bool
	}{
		{
			name:   "simple addition",
			input:  "5 3 +",
			want:   "$5 + 3$",
			hasErr: false,
		},
		{
			name:   "chained addition",
			input:  "1 2 + 3 + 4 +",
			want:   "$1 + 2 + 3 + 4$",
			hasErr: false,
		},
		{
			name:   "addition with floating point",
			input:  "1.5 0.5 +",
			want:   "$1.5 + 0.5$",
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Tokenize() unexpected error: %v", err)
				}
				return
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Parse() unexpected error: %v", err)
				}
				return
			}

			// Generate
			generator := NewGenerator()
			got := generator.Generate(ast)

			if tt.hasErr {
				t.Errorf("Expected error but got output: %q", got)
				return
			}

			if got != tt.want {
				t.Errorf("Integration test: got %q, want %q", got, tt.want)
			}
		})
	}
}

// TestIntegrationSubtraction tests the complete pipeline for subtraction.
func TestIntegrationSubtraction(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   string
		hasErr bool
	}{
		{
			name:   "simple subtraction",
			input:  "5 3 -",
			want:   "$5 - 3$",
			hasErr: false,
		},
		{
			name:   "chained subtraction",
			input:  "5 3 - 2 -",
			want:   "$5 - 3 - 2$",
			hasErr: false,
		},
		{
			name:   "subtraction with floating point",
			input:  "10.5 2.5 -",
			want:   "$10.5 - 2.5$",
			hasErr: false,
		},
		{
			name:   "negative number",
			input:  "-5",
			want:   "$-5$",
			hasErr: false,
		},
		{
			name:   "subtraction with negative number",
			input:  "10 -5 -",
			want:   "$10 - -5$",
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Tokenize() unexpected error: %v", err)
				}
				return
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Parse() unexpected error: %v", err)
				}
				return
			}

			// Generate
			generator := NewGenerator()
			got := generator.Generate(ast)

			if tt.hasErr {
				t.Errorf("Expected error but got output: %q", got)
				return
			}

			if got != tt.want {
				t.Errorf("Integration test: got %q, want %q", got, tt.want)
			}
		})
	}
}

// TestIntegrationMultiplication tests the complete pipeline for multiplication.
func TestIntegrationMultiplication(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   string
		hasErr bool
	}{
		{
			name:   "simple multiplication",
			input:  "4 7 *",
			want:   "$4 \\times 7$",
			hasErr: false,
		},
		{
			name:   "addition and multiplication (respects precedence)",
			input:  "2 3 4 * +",
			want:   "$2 + 3 \\times 4$",
			hasErr: false,
		},
		{
			name:   "multiplication with floating point",
			input:  "3.14 2 *",
			want:   "$3.14 \\times 2$",
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Tokenize() unexpected error: %v", err)
				}
				return
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Parse() unexpected error: %v", err)
				}
				return
			}

			// Generate
			generator := NewGenerator()
			got := generator.Generate(ast)

			if tt.hasErr {
				t.Errorf("Expected error but got output: %q", got)
				return
			}

			if got != tt.want {
				t.Errorf("Integration test: got %q, want %q", got, tt.want)
			}
		})
	}
}

// TestIntegrationDivision tests the complete pipeline for division.
func TestIntegrationDivision(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   string
		hasErr bool
	}{
		{
			name:   "simple division",
			input:  "10 2 /",
			want:   "$10 \\div 2$",
			hasErr: false,
		},
		{
			name:   "chained division",
			input:  "100 10 / 5 / 2 /",
			want:   "$100 \\div 10 \\div 5 \\div 2$",
			hasErr: false,
		},
		{
			name:   "division with floating point",
			input:  "10.5 2.5 /",
			want:   "$10.5 \\div 2.5$",
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Lex
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Tokenize() unexpected error: %v", err)
				}
				return
			}

			// Parse
			parser := NewParser(tokens)
			ast, err := parser.Parse()
			if err != nil {
				if !tt.hasErr {
					t.Errorf("Parse() unexpected error: %v", err)
				}
				return
			}

			// Generate
			generator := NewGenerator()
			got := generator.Generate(ast)

			if tt.hasErr {
				t.Errorf("Expected error but got output: %q", got)
				return
			}

			if got != tt.want {
				t.Errorf("Integration test: got %q, want %q", got, tt.want)
			}
		})
	}
}

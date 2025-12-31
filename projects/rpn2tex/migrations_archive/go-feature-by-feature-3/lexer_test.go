package rpn2tex

import (
	"testing"
)

func TestLexerTokenizeNumbers(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   []Token
		hasErr bool
	}{
		{
			name:  "single integer",
			input: "5",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 2},
			},
			hasErr: false,
		},
		{
			name:  "floating point",
			input: "3.14",
			want: []Token{
				{Type: TokenNumber, Value: "3.14", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 5},
			},
			hasErr: false,
		},
		{
			name:  "multiple numbers with whitespace",
			input: "5 3",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenEOF, Value: "", Line: 1, Column: 4},
			},
			hasErr: false,
		},
		{
			name:  "numbers with tabs and newlines",
			input: "5\t3\n42",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenNumber, Value: "42", Line: 2, Column: 1},
				{Type: TokenEOF, Value: "", Line: 2, Column: 3},
			},
			hasErr: false,
		},
		{
			name:   "unexpected character",
			input:  "5 @",
			want:   nil,
			hasErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Tokenize() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Tokenize() unexpected error: %v", err)
				return
			}

			if len(got) != len(tt.want) {
				t.Errorf("Tokenize() got %d tokens, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i].Type != tt.want[i].Type {
					t.Errorf("Token %d: Type = %v, want %v", i, got[i].Type, tt.want[i].Type)
				}
				if got[i].Value != tt.want[i].Value {
					t.Errorf("Token %d: Value = %q, want %q", i, got[i].Value, tt.want[i].Value)
				}
				if got[i].Line != tt.want[i].Line {
					t.Errorf("Token %d: Line = %d, want %d", i, got[i].Line, tt.want[i].Line)
				}
				if got[i].Column != tt.want[i].Column {
					t.Errorf("Token %d: Column = %d, want %d", i, got[i].Column, tt.want[i].Column)
				}
			}
		})
	}
}

func TestLexerTokenizeAddition(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   []Token
		hasErr bool
	}{
		{
			name:  "simple addition",
			input: "5 3 +",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name:  "chained addition",
			input: "1 2 + 3 + 4 +",
			want: []Token{
				{Type: TokenNumber, Value: "1", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 3},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenNumber, Value: "4", Line: 1, Column: 11},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 13},
				{Type: TokenEOF, Value: "", Line: 1, Column: 14},
			},
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Tokenize() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Tokenize() unexpected error: %v", err)
				return
			}

			if len(got) != len(tt.want) {
				t.Errorf("Tokenize() got %d tokens, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i].Type != tt.want[i].Type {
					t.Errorf("Token %d: Type = %v, want %v", i, got[i].Type, tt.want[i].Type)
				}
				if got[i].Value != tt.want[i].Value {
					t.Errorf("Token %d: Value = %q, want %q", i, got[i].Value, tt.want[i].Value)
				}
				if got[i].Line != tt.want[i].Line {
					t.Errorf("Token %d: Line = %d, want %d", i, got[i].Line, tt.want[i].Line)
				}
				if got[i].Column != tt.want[i].Column {
					t.Errorf("Token %d: Column = %d, want %d", i, got[i].Column, tt.want[i].Column)
				}
			}
		})
	}
}

func TestLexerTokenizeSubtraction(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   []Token
		hasErr bool
	}{
		{
			name:  "simple subtraction",
			input: "5 3 -",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name:  "negative number",
			input: "-5",
			want: []Token{
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 1},
				{Type: TokenEOF, Value: "", Line: 1, Column: 3},
			},
			hasErr: false,
		},
		{
			name:  "subtraction and negative number",
			input: "10 -5 -",
			want: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "-5", Line: 1, Column: 4},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 7},
				{Type: TokenEOF, Value: "", Line: 1, Column: 8},
			},
			hasErr: false,
		},
		{
			name:  "standalone minus",
			input: "5 - 3",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 3},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name:  "chained subtraction",
			input: "5 3 - 2 -",
			want: []Token{
				{Type: TokenNumber, Value: "5", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 5},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 7},
				{Type: TokenMinus, Value: "-", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Tokenize() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Tokenize() unexpected error: %v", err)
				return
			}

			if len(got) != len(tt.want) {
				t.Errorf("Tokenize() got %d tokens, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i].Type != tt.want[i].Type {
					t.Errorf("Token %d: Type = %v, want %v", i, got[i].Type, tt.want[i].Type)
				}
				if got[i].Value != tt.want[i].Value {
					t.Errorf("Token %d: Value = %q, want %q", i, got[i].Value, tt.want[i].Value)
				}
				if got[i].Line != tt.want[i].Line {
					t.Errorf("Token %d: Line = %d, want %d", i, got[i].Line, tt.want[i].Line)
				}
				if got[i].Column != tt.want[i].Column {
					t.Errorf("Token %d: Column = %d, want %d", i, got[i].Column, tt.want[i].Column)
				}
			}
		})
	}
}

func TestLexerTokenizeMultiplication(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   []Token
		hasErr bool
	}{
		{
			name:  "simple multiplication",
			input: "4 7 *",
			want: []Token{
				{Type: TokenNumber, Value: "4", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "7", Line: 1, Column: 3},
				{Type: TokenMul, Value: "*", Line: 1, Column: 5},
				{Type: TokenEOF, Value: "", Line: 1, Column: 6},
			},
			hasErr: false,
		},
		{
			name:  "multiplication with addition",
			input: "2 3 4 * +",
			want: []Token{
				{Type: TokenNumber, Value: "2", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "3", Line: 1, Column: 3},
				{Type: TokenNumber, Value: "4", Line: 1, Column: 5},
				{Type: TokenMul, Value: "*", Line: 1, Column: 7},
				{Type: TokenPlus, Value: "+", Line: 1, Column: 9},
				{Type: TokenEOF, Value: "", Line: 1, Column: 10},
			},
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Tokenize() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Tokenize() unexpected error: %v", err)
				return
			}

			if len(got) != len(tt.want) {
				t.Errorf("Tokenize() got %d tokens, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i].Type != tt.want[i].Type {
					t.Errorf("Token %d: Type = %v, want %v", i, got[i].Type, tt.want[i].Type)
				}
				if got[i].Value != tt.want[i].Value {
					t.Errorf("Token %d: Value = %q, want %q", i, got[i].Value, tt.want[i].Value)
				}
				if got[i].Line != tt.want[i].Line {
					t.Errorf("Token %d: Line = %d, want %d", i, got[i].Line, tt.want[i].Line)
				}
				if got[i].Column != tt.want[i].Column {
					t.Errorf("Token %d: Column = %d, want %d", i, got[i].Column, tt.want[i].Column)
				}
			}
		})
	}
}

func TestLexerTokenizeDivision(t *testing.T) {
	tests := []struct {
		name   string
		input  string
		want   []Token
		hasErr bool
	}{
		{
			name:  "simple division",
			input: "10 2 /",
			want: []Token{
				{Type: TokenNumber, Value: "10", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 4},
				{Type: TokenDiv, Value: "/", Line: 1, Column: 6},
				{Type: TokenEOF, Value: "", Line: 1, Column: 7},
			},
			hasErr: false,
		},
		{
			name:  "chained division",
			input: "100 10 / 5 / 2 /",
			want: []Token{
				{Type: TokenNumber, Value: "100", Line: 1, Column: 1},
				{Type: TokenNumber, Value: "10", Line: 1, Column: 5},
				{Type: TokenDiv, Value: "/", Line: 1, Column: 8},
				{Type: TokenNumber, Value: "5", Line: 1, Column: 10},
				{Type: TokenDiv, Value: "/", Line: 1, Column: 12},
				{Type: TokenNumber, Value: "2", Line: 1, Column: 14},
				{Type: TokenDiv, Value: "/", Line: 1, Column: 16},
				{Type: TokenEOF, Value: "", Line: 1, Column: 17},
			},
			hasErr: false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if tt.hasErr {
				if err == nil {
					t.Errorf("Tokenize() expected error but got none")
				}
				return
			}

			if err != nil {
				t.Errorf("Tokenize() unexpected error: %v", err)
				return
			}

			if len(got) != len(tt.want) {
				t.Errorf("Tokenize() got %d tokens, want %d", len(got), len(tt.want))
				return
			}

			for i := range got {
				if got[i].Type != tt.want[i].Type {
					t.Errorf("Token %d: Type = %v, want %v", i, got[i].Type, tt.want[i].Type)
				}
				if got[i].Value != tt.want[i].Value {
					t.Errorf("Token %d: Value = %q, want %q", i, got[i].Value, tt.want[i].Value)
				}
				if got[i].Line != tt.want[i].Line {
					t.Errorf("Token %d: Line = %d, want %d", i, got[i].Line, tt.want[i].Line)
				}
				if got[i].Column != tt.want[i].Column {
					t.Errorf("Token %d: Column = %d, want %d", i, got[i].Column, tt.want[i].Column)
				}
			}
		})
	}
}

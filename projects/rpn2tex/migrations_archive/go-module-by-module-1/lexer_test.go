package rpn2tex

import (
	"testing"
)

func TestLexer_BasicOperators(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "addition",
			input: "+",
			want: []Token{
				{Type: PLUS, Value: "+", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "subtraction",
			input: "-",
			want: []Token{
				{Type: MINUS, Value: "-", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "multiplication",
			input: "*",
			want: []Token{
				{Type: MULT, Value: "*", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "division",
			input: "/",
			want: []Token{
				{Type: DIV, Value: "/", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_Numbers(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "single digit",
			input: "5",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 2},
			},
		},
		{
			name:  "multi-digit integer",
			input: "123",
			want: []Token{
				{Type: NUMBER, Value: "123", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 4},
			},
		},
		{
			name:  "decimal number",
			input: "3.14",
			want: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 5},
			},
		},
		{
			name:  "decimal with single digit",
			input: "1.5",
			want: []Token{
				{Type: NUMBER, Value: "1.5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 4},
			},
		},
		{
			name:  "negative number",
			input: "-5",
			want: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "negative decimal",
			input: "-3.14",
			want: []Token{
				{Type: NUMBER, Value: "-3.14", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_SimpleExpressions(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "simple addition",
			input: "5 3 +",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple subtraction",
			input: "5 3 -",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple multiplication",
			input: "4 7 *",
			want: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "simple division",
			input: "10 2 /",
			want: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_ComplexExpressions(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "precedence test 1",
			input: "5 3 + 2 *",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "precedence test 2",
			input: "2 3 4 * +",
			want: []Token{
				{Type: NUMBER, Value: "2", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: NUMBER, Value: "4", Line: 1, Column: 5},
				{Type: MULT, Value: "*", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "decimal numbers",
			input: "3.14 2 *",
			want: []Token{
				{Type: NUMBER, Value: "3.14", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 6},
				{Type: MULT, Value: "*", Line: 1, Column: 8},
				{Type: EOF, Value: "", Line: 1, Column: 9},
			},
		},
		{
			name:  "chain of operations",
			input: "100 10 / 5 / 2 /",
			want: []Token{
				{Type: NUMBER, Value: "100", Line: 1, Column: 1},
				{Type: NUMBER, Value: "10", Line: 1, Column: 5},
				{Type: DIV, Value: "/", Line: 1, Column: 8},
				{Type: NUMBER, Value: "5", Line: 1, Column: 10},
				{Type: DIV, Value: "/", Line: 1, Column: 12},
				{Type: NUMBER, Value: "2", Line: 1, Column: 14},
				{Type: DIV, Value: "/", Line: 1, Column: 16},
				{Type: EOF, Value: "", Line: 1, Column: 17},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_WhitespaceHandling(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "multiple spaces",
			input: "5   3   +",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
		},
		{
			name:  "tabs",
			input: "5\t3\t+",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "leading whitespace",
			input: "  5 3 +",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 3},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 7},
				{Type: EOF, Value: "", Line: 1, Column: 8},
			},
		},
		{
			name:  "trailing whitespace",
			input: "5 3 +  ",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 8},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_PositionTracking(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "multiline input",
			input: "5 3 +\n2 *",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 2, Column: 1},
				{Type: MULT, Value: "*", Line: 2, Column: 3},
				{Type: EOF, Value: "", Line: 2, Column: 4},
			},
		},
		{
			name:  "carriage return",
			input: "5 3\r\n+",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 2, Column: 1},
				{Type: EOF, Value: "", Line: 2, Column: 2},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_ErrorCases(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		wantErr     bool
		wantErrLine int
		wantErrCol  int
		wantErrMsg  string
	}{
		{
			name:        "exponentiation operator",
			input:       "2 3 ^",
			wantErr:     true,
			wantErrLine: 1,
			wantErrCol:  5,
			wantErrMsg:  "Unexpected character '^'",
		},
		{
			name:        "exponentiation in middle",
			input:       "2 3 ^ 4 *",
			wantErr:     true,
			wantErrLine: 1,
			wantErrCol:  5,
			wantErrMsg:  "Unexpected character '^'",
		},
		{
			name:        "multiple exponentiation",
			input:       "2 3 4 ^ ^",
			wantErr:     true,
			wantErrLine: 1,
			wantErrCol:  7,
			wantErrMsg:  "Unexpected character '^'",
		},
		{
			name:        "invalid character",
			input:       "5 3 @",
			wantErr:     true,
			wantErrLine: 1,
			wantErrCol:  5,
			wantErrMsg:  "Unexpected character '@'",
		},
		{
			name:        "letter in input",
			input:       "5 a +",
			wantErr:     true,
			wantErrLine: 1,
			wantErrCol:  3,
			wantErrMsg:  "Unexpected character 'a'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()

			if !tt.wantErr {
				if err != nil {
					t.Errorf("Tokenize() unexpected error = %v", err)
				}
				return
			}

			if err == nil {
				t.Errorf("Tokenize() expected error but got none")
				return
			}

			lexErr, ok := err.(*LexerError)
			if !ok {
				t.Errorf("Tokenize() error is not LexerError: %v", err)
				return
			}

			if lexErr.Line != tt.wantErrLine {
				t.Errorf("LexerError line = %d, want %d", lexErr.Line, tt.wantErrLine)
			}
			if lexErr.Column != tt.wantErrCol {
				t.Errorf("LexerError column = %d, want %d", lexErr.Column, tt.wantErrCol)
			}
			if lexErr.Message != tt.wantErrMsg {
				t.Errorf("LexerError message = %q, want %q", lexErr.Message, tt.wantErrMsg)
			}
		})
	}
}

func TestLexer_NegativeNumberVsSubtraction(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  []Token
	}{
		{
			name:  "negative number",
			input: "-5",
			want: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
				{Type: EOF, Value: "", Line: 1, Column: 3},
			},
		},
		{
			name:  "subtraction operator",
			input: "5 3 -",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
		},
		{
			name:  "subtraction with spaces",
			input: "- 5",
			want: []Token{
				{Type: MINUS, Value: "-", Line: 1, Column: 1},
				{Type: NUMBER, Value: "5", Line: 1, Column: 3},
				{Type: EOF, Value: "", Line: 1, Column: 4},
			},
		},
		{
			name:  "negative after operator",
			input: "5 -3 +",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "-3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}
			if !tokensEqual(got, tt.want) {
				t.Errorf("Tokenize() = %v, want %v", got, tt.want)
			}
		})
	}
}

func TestLexer_EmptyInput(t *testing.T) {
	lexer := NewLexer("")
	got, err := lexer.Tokenize()
	if err != nil {
		t.Errorf("Tokenize() error = %v", err)
		return
	}

	want := []Token{
		{Type: EOF, Value: "", Line: 1, Column: 1},
	}

	if !tokensEqual(got, want) {
		t.Errorf("Tokenize() = %v, want %v", got, want)
	}
}

func TestLexer_WhitespaceOnly(t *testing.T) {
	tests := []struct {
		name  string
		input string
	}{
		{"spaces", "   "},
		{"tabs", "\t\t\t"},
		{"newlines", "\n\n"},
		{"mixed", " \t\n\r "},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()
			if err != nil {
				t.Errorf("Tokenize() error = %v", err)
				return
			}

			// Should only contain EOF token
			if len(got) != 1 || got[0].Type != EOF {
				t.Errorf("Tokenize() = %v, want single EOF token", got)
			}
		})
	}
}

// Helper function to compare token slices
func tokensEqual(a, b []Token) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i].Type != b[i].Type ||
			a[i].Value != b[i].Value ||
			a[i].Line != b[i].Line ||
			a[i].Column != b[i].Column {
			return false
		}
	}
	return true
}

package rpn2tex

import (
	"reflect"
	"testing"
)

func TestLexerTokenize(t *testing.T) {
	tests := []struct {
		name    string
		input   string
		want    []Token
		wantErr bool
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
			wantErr: false,
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
			wantErr: false,
		},
		{
			name:  "multiplication",
			input: "4 7 *",
			want: []Token{
				{Type: NUMBER, Value: "4", Line: 1, Column: 1},
				{Type: NUMBER, Value: "7", Line: 1, Column: 3},
				{Type: MULT, Value: "*", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			wantErr: false,
		},
		{
			name:  "division",
			input: "10 2 /",
			want: []Token{
				{Type: NUMBER, Value: "10", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 4},
				{Type: DIV, Value: "/", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
			wantErr: false,
		},
		{
			name:  "complex expression",
			input: "5 3 + 2 *",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: MULT, Value: "*", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
			wantErr: false,
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
			wantErr: false,
		},
		{
			name:  "multiple decimals",
			input: "1.5 0.5 +",
			want: []Token{
				{Type: NUMBER, Value: "1.5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "0.5", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: EOF, Value: "", Line: 1, Column: 10},
			},
			wantErr: false,
		},
		{
			name:  "multiple operations",
			input: "1 2 + 3 + 4 +",
			want: []Token{
				{Type: NUMBER, Value: "1", Line: 1, Column: 1},
				{Type: NUMBER, Value: "2", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: NUMBER, Value: "3", Line: 1, Column: 7},
				{Type: PLUS, Value: "+", Line: 1, Column: 9},
				{Type: NUMBER, Value: "4", Line: 1, Column: 11},
				{Type: PLUS, Value: "+", Line: 1, Column: 13},
				{Type: EOF, Value: "", Line: 1, Column: 14},
			},
			wantErr: false,
		},
		{
			name:  "multiple divisions",
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
			wantErr: false,
		},
		{
			name:  "whitespace handling - multiple spaces",
			input: "5   3  +",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 5},
				{Type: PLUS, Value: "+", Line: 1, Column: 8},
				{Type: EOF, Value: "", Line: 1, Column: 9},
			},
			wantErr: false,
		},
		{
			name:  "whitespace handling - tabs",
			input: "5\t3\t+",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: PLUS, Value: "+", Line: 1, Column: 5},
				{Type: EOF, Value: "", Line: 1, Column: 6},
			},
			wantErr: false,
		},
		{
			name:  "negative number",
			input: "-5 3 +",
			want: []Token{
				{Type: NUMBER, Value: "-5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 4},
				{Type: PLUS, Value: "+", Line: 1, Column: 6},
				{Type: EOF, Value: "", Line: 1, Column: 7},
			},
			wantErr: false,
		},
		{
			name:  "minus operator vs negative number",
			input: "5 3 - 2",
			want: []Token{
				{Type: NUMBER, Value: "5", Line: 1, Column: 1},
				{Type: NUMBER, Value: "3", Line: 1, Column: 3},
				{Type: MINUS, Value: "-", Line: 1, Column: 5},
				{Type: NUMBER, Value: "2", Line: 1, Column: 7},
				{Type: EOF, Value: "", Line: 1, Column: 8},
			},
			wantErr: false,
		},
		{
			name:  "empty expression",
			input: "",
			want: []Token{
				{Type: EOF, Value: "", Line: 1, Column: 1},
			},
			wantErr: false,
		},
		{
			name:  "whitespace only",
			input: "   ",
			want: []Token{
				{Type: EOF, Value: "", Line: 1, Column: 4},
			},
			wantErr: false,
		},
		{
			name:    "unsupported character - exponentiation",
			input:   "2 3 ^",
			want:    nil,
			wantErr: true,
		},
		{
			name:    "unsupported character in expression",
			input:   "2 3 ^ 4 *",
			want:    nil,
			wantErr: true,
		},
		{
			name:    "unsupported character - multiple",
			input:   "2 3 4 ^ ^",
			want:    nil,
			wantErr: true,
		},
		{
			name:    "unsupported character - at symbol",
			input:   "5 3 @",
			want:    nil,
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			got, err := lexer.Tokenize()

			if (err != nil) != tt.wantErr {
				t.Errorf("Tokenize() error = %v, wantErr %v", err, tt.wantErr)
				return
			}

			if !tt.wantErr && !reflect.DeepEqual(got, tt.want) {
				t.Errorf("Tokenize() mismatch:\nGot:  %v\nWant: %v", got, tt.want)
			}
		})
	}
}

func TestLexerErrorPosition(t *testing.T) {
	tests := []struct {
		name        string
		input       string
		wantLine    int
		wantColumn  int
		wantMessage string
	}{
		{
			name:        "error at position 5",
			input:       "2 3 ^",
			wantLine:    1,
			wantColumn:  5,
			wantMessage: "Unexpected character '^'",
		},
		{
			name:        "error at position 5 with more content",
			input:       "2 3 ^ 4 *",
			wantLine:    1,
			wantColumn:  5,
			wantMessage: "Unexpected character '^'",
		},
		{
			name:        "error at position 7",
			input:       "2 3 4 ^ ^",
			wantLine:    1,
			wantColumn:  7,
			wantMessage: "Unexpected character '^'",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			_, err := lexer.Tokenize()

			if err == nil {
				t.Fatalf("Expected error but got none")
			}

			compileErr, ok := err.(*CompileError)
			if !ok {
				t.Fatalf("Expected CompileError, got %T", err)
			}

			if compileErr.Line != tt.wantLine {
				t.Errorf("Error line = %d, want %d", compileErr.Line, tt.wantLine)
			}

			if compileErr.Column != tt.wantColumn {
				t.Errorf("Error column = %d, want %d", compileErr.Column, tt.wantColumn)
			}

			if compileErr.Message != tt.wantMessage {
				t.Errorf("Error message = %q, want %q", compileErr.Message, tt.wantMessage)
			}
		})
	}
}

func TestLexerPositionTracking(t *testing.T) {
	input := "5 3 +"
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()

	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	// Test that positions are correct
	expectedPositions := []struct {
		line   int
		column int
	}{
		{1, 1}, // "5"
		{1, 3}, // "3"
		{1, 5}, // "+"
		{1, 6}, // EOF
	}

	if len(tokens) != len(expectedPositions) {
		t.Fatalf("Expected %d tokens, got %d", len(expectedPositions), len(tokens))
	}

	for i, tok := range tokens {
		if tok.Line != expectedPositions[i].line {
			t.Errorf("Token %d: line = %d, want %d", i, tok.Line, expectedPositions[i].line)
		}
		if tok.Column != expectedPositions[i].column {
			t.Errorf("Token %d: column = %d, want %d", i, tok.Column, expectedPositions[i].column)
		}
	}
}

func TestLexerMultilineTracking(t *testing.T) {
	input := "5\n3\n+"
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()

	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	expectedPositions := []struct {
		line   int
		column int
	}{
		{1, 1}, // "5"
		{2, 1}, // "3"
		{3, 1}, // "+"
		{3, 2}, // EOF
	}

	if len(tokens) != len(expectedPositions) {
		t.Fatalf("Expected %d tokens, got %d", len(expectedPositions), len(tokens))
	}

	for i, tok := range tokens {
		if tok.Line != expectedPositions[i].line {
			t.Errorf("Token %d: line = %d, want %d", i, tok.Line, expectedPositions[i].line)
		}
		if tok.Column != expectedPositions[i].column {
			t.Errorf("Token %d: column = %d, want %d", i, tok.Column, expectedPositions[i].column)
		}
	}
}

func TestLexerDecimalNumbers(t *testing.T) {
	tests := []struct {
		name  string
		input string
		want  string
	}{
		{
			name:  "simple decimal",
			input: "3.14",
			want:  "3.14",
		},
		{
			name:  "leading zero decimal",
			input: "0.5",
			want:  "0.5",
		},
		{
			name:  "trailing digits",
			input: "1.5",
			want:  "1.5",
		},
		{
			name:  "multiple decimal places",
			input: "3.14159",
			want:  "3.14159",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			lexer := NewLexer(tt.input)
			tokens, err := lexer.Tokenize()

			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) < 1 {
				t.Fatalf("Expected at least 1 token, got %d", len(tokens))
			}

			if tokens[0].Value != tt.want {
				t.Errorf("Number value = %q, want %q", tokens[0].Value, tt.want)
			}
		})
	}
}

package main

import (
	"fmt"
	"rpn2tex"
)

func main() {
	tests := []struct {
		input    string
		expected string
	}{
		{"5 3 -", "$5 - 3$"},
		{"5 3 - 2 -", "$5 - 3 - 2$"},
	}

	for _, tt := range tests {
		lexer := rpn2tex.NewLexer(tt.input)
		tokens, err := lexer.Tokenize()
		if err != nil {
			fmt.Printf("FAIL: %s - Lexer error: %v\n", tt.input, err)
			continue
		}

		parser := rpn2tex.NewParser(tokens)
		ast, err := parser.Parse()
		if err != nil {
			fmt.Printf("FAIL: %s - Parser error: %v\n", tt.input, err)
			continue
		}

		generator := rpn2tex.NewLaTeXGenerator()
		output := generator.Generate(ast)

		if output == tt.expected {
			fmt.Printf("PASS: %s => %s\n", tt.input, output)
		} else {
			fmt.Printf("FAIL: %s => expected '%s', got '%s'\n", tt.input, tt.expected, output)
		}
	}
}

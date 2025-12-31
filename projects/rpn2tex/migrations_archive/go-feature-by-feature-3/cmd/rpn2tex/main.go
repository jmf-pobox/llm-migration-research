package main

import (
	"fmt"
	"io"
	"os"

	"rpn2tex"
)

func main() {
	os.Exit(run())
}

func run() int {
	// Read input from stdin or file
	var input string
	var err error

	if len(os.Args) > 1 {
		// Read from file
		data, err := os.ReadFile(os.Args[1])
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
			return 1
		}
		input = string(data)
	} else {
		// Read from stdin
		data, err := io.ReadAll(os.Stdin)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading stdin: %v\n", err)
			return 1
		}
		input = string(data)
	}

	// Lex
	lexer := rpn2tex.NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		return 1
	}

	// Parse
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		return 1
	}

	// Generate LaTeX
	generator := rpn2tex.NewGenerator()
	latex := generator.Generate(ast)

	// Output
	fmt.Println(latex)
	return 0
}

package main

import (
	"fmt"
	"os"
	"rpn2tex"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: rpn2tex <expression>\n")
		os.Exit(1)
	}

	input := os.Args[1]

	// Create pipeline components
	lexer := rpn2tex.NewLexer(input)
	parser := rpn2tex.NewParser(lexer)
	generator := rpn2tex.NewLaTeXGenerator()
	errorFormatter := rpn2tex.NewErrorFormatter(input)

	// Parse the input
	ast, err := parser.Parse()
	if err != nil {
		// Get position from error
		line, col := 1, 1
		fmt.Fprintf(os.Stderr, "%s", errorFormatter.FormatError(err.Error(), line, col))
		os.Exit(1)
	}

	// Generate LaTeX output
	latex, err := generator.Generate(ast)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		os.Exit(1)
	}

	// Wrap in LaTeX math delimiters and print
	fmt.Printf("$%s$\n", latex)
}

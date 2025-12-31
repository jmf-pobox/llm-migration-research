package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"rpn2tex"
)

func main() {
	var input string

	if len(os.Args) > 1 {
		// Read from command-line arguments
		input = strings.Join(os.Args[1:], " ")
	} else {
		// Read from stdin
		scanner := bufio.NewScanner(os.Stdin)
		if scanner.Scan() {
			input = scanner.Text()
		}
		if err := scanner.Err(); err != nil {
			fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
			os.Exit(1)
		}
	}

	// Tokenize
	lexer := rpn2tex.NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Parse
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}

	// Generate LaTeX
	generator := rpn2tex.NewLaTeXGenerator()
	output := generator.Generate(ast)

	// Print output without trailing newline
	fmt.Print(output)
}

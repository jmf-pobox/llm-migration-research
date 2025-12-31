package main

import (
	"fmt"
	"io"
	"os"
	"strings"

	"rpn2tex"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
}

func run() error {
	// Read input
	input, err := readInput()
	if err != nil {
		return fmt.Errorf("reading input: %w", err)
	}

	// Trim whitespace
	input = strings.TrimSpace(input)

	// Lexer
	lexer := rpn2tex.NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		return err
	}

	// Parser
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		return err
	}

	// Generator
	generator := rpn2tex.NewLaTeXGenerator()
	output := generator.Generate(ast)

	// Write output
	fmt.Println(output)

	return nil
}

func readInput() (string, error) {
	if len(os.Args) > 1 {
		// Read from file argument
		data, err := os.ReadFile(os.Args[1])
		if err != nil {
			return "", err
		}
		return string(data), nil
	}

	// Read from stdin
	data, err := io.ReadAll(os.Stdin)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

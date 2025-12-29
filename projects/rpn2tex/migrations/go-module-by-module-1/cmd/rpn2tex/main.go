// Package main provides the CLI entry point for the rpn2tex converter.
package main

import (
	"fmt"
	"io"
	"os"
	"rpn2tex"
)

func main() {
	os.Exit(run(os.Args[1:], os.Stdin, os.Stdout, os.Stderr))
}

// run executes the main CLI logic and returns an exit code.
// Returns 0 for success, 1 for errors.
// This function is separated from main() to allow testing without os.Exit().
func run(args []string, stdin io.Reader, stdout io.Writer, stderr io.Writer) int {
	// Parse command-line arguments
	// Usage: rpn2tex [input_file]
	// - If input_file is "-" or not provided, read from stdin
	// - Otherwise, read from the specified file

	var input string
	var err error

	if len(args) < 1 {
		// No arguments: read from stdin
		input, err = readInput(stdin)
		if err != nil {
			fmt.Fprintf(stderr, "Error reading stdin: %v\n", err)
			return 1
		}
	} else {
		inputArg := args[0]
		if inputArg == "-" {
			// Explicit stdin marker
			input, err = readInput(stdin)
			if err != nil {
				fmt.Fprintf(stderr, "Error reading stdin: %v\n", err)
				return 1
			}
		} else {
			// Read from file
			input, err = readFile(inputArg)
			if err != nil {
				fmt.Fprintf(stderr, "Error reading file: %v\n", err)
				return 1
			}
		}
	}

	// Run the conversion pipeline
	latex, err := convert(input)
	if err != nil {
		// Error formatting
		formatter := rpn2tex.NewErrorFormatter(input)

		// Check error type and format accordingly
		switch e := err.(type) {
		case *rpn2tex.LexerError:
			// Format lexer error with source context
			formatted := formatter.FormatError(
				fmt.Sprintf("Error: %s", e.Message),
				e.Line,
				e.Column,
				1,
			)
			fmt.Fprint(stderr, formatted)
		case *rpn2tex.ParserError:
			// Format parser error with source context
			formatted := formatter.FormatError(
				fmt.Sprintf("Error: %s", e.Message),
				e.Token.Line,
				e.Token.Column,
				1,
			)
			fmt.Fprint(stderr, formatted)
		default:
			// Generic error
			fmt.Fprintf(stderr, "Error: %v\n", err)
		}
		return 1
	}

	// Output successful result to stdout
	fmt.Fprintln(stdout, latex)
	return 0
}

// readInput reads all input from an io.Reader and returns it as a string.
func readInput(r io.Reader) (string, error) {
	data, err := io.ReadAll(r)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// readFile reads the contents of a file and returns it as a string.
func readFile(filename string) (string, error) {
	data, err := os.ReadFile(filename)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

// convert runs the full conversion pipeline: tokenize -> parse -> generate.
func convert(input string) (string, error) {
	// Tokenization phase
	lexer := rpn2tex.NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		return "", err
	}

	// Parsing phase
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		return "", err
	}

	// LaTeX generation phase
	generator := rpn2tex.NewLaTeXGenerator()
	latex := generator.Generate(ast)

	return latex, nil
}

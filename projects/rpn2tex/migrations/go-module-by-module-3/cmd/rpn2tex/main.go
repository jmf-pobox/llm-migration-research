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

// run is the main entry point that returns an exit code.
// It orchestrates reading input, processing through the pipeline,
// and writing output.
func run() int {
	// Parse command line arguments
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "Usage: rpn2tex <input-file|->")
		fmt.Fprintln(os.Stderr, "  Use '-' to read from stdin")
		return 1
	}

	inputPath := os.Args[1]

	// Read input
	input, err := readInput(inputPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading input: %s\n", err)
		return 1
	}

	// Process through pipeline: Lexer -> Parser -> LaTeX Generator
	output, err := process(input)
	if err != nil {
		// Format error with source context
		formatter := rpn2tex.NewErrorFormatter(input)

		// Check error type and format accordingly
		switch e := err.(type) {
		case *rpn2tex.LexerError:
			formatted := formatter.FormatError(e.Message, e.Line, e.Column, 1)
			fmt.Fprintln(os.Stderr, formatted)
		case *rpn2tex.ParserError:
			formatted := formatter.FormatError(e.Message, e.Token.Line, e.Token.Column, 1)
			fmt.Fprintln(os.Stderr, formatted)
		default:
			// Generic error
			fmt.Fprintf(os.Stderr, "Error: %s\n", err)
		}
		return 1
	}

	// Write output to stdout
	fmt.Println(output)

	return 0
}

// readInput reads from stdin if path is "-", otherwise from the specified file.
func readInput(path string) (string, error) {
	if path == "-" {
		// Read from stdin
		bytes, err := io.ReadAll(os.Stdin)
		if err != nil {
			return "", fmt.Errorf("reading stdin: %w", err)
		}
		return string(bytes), nil
	}

	// Read from file
	bytes, err := os.ReadFile(path)
	if err != nil {
		if os.IsNotExist(err) {
			return "", fmt.Errorf("file not found: %s", path)
		}
		if os.IsPermission(err) {
			return "", fmt.Errorf("permission denied: %s", path)
		}
		return "", fmt.Errorf("reading file: %w", err)
	}

	return string(bytes), nil
}

// process runs the input through the lexer, parser, and LaTeX generator pipeline.
func process(input string) (string, error) {
	// Step 1: Tokenize
	lexer := rpn2tex.NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		return "", err
	}

	// Step 2: Parse
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		return "", err
	}

	// Step 3: Generate LaTeX
	generator := rpn2tex.NewLaTeXGenerator()
	output := generator.Generate(ast)

	return output, nil
}

package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	// Check for command-line arguments
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <RPN expression>\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Example: %s \"5 3 +\"\n", os.Args[0])
		os.Exit(1)
	}

	// Join all arguments after the program name into a single expression
	input := strings.Join(os.Args[1:], " ")

	// Run the pipeline and exit with appropriate code
	if err := run(input); err != nil {
		fmt.Fprintf(os.Stderr, "Error: %v\n", err)
		os.Exit(1)
	}
	os.Exit(0)
}

// run executes the rpn2tex pipeline: lexer → parser → latex generator.
func run(input string) error {
	// Create error formatter for better error messages
	formatter := NewErrorFormatter(input)

	// Step 1: Tokenize the input
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		// Handle lexer errors with context
		if syntaxErr, ok := err.(*SyntaxError); ok {
			formatted := formatter.FormatError(syntaxErr.Message, syntaxErr.Line, syntaxErr.Column)
			return fmt.Errorf("%s", formatted)
		}
		return fmt.Errorf("lexer error: %w", err)
	}

	// Step 2: Parse tokens into AST
	parser := NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		// Handle parser errors with context
		if parserErr, ok := err.(*ParserError); ok {
			formatted := formatter.FormatError(parserErr.Message, parserErr.Token.Line, parserErr.Token.Column)
			return fmt.Errorf("%s", formatted)
		}
		return fmt.Errorf("parser error: %w", err)
	}

	// Step 3: Generate LaTeX from AST
	generator := NewLaTeXGenerator()
	latex := generator.Generate(ast)

	// Output the LaTeX result
	fmt.Println(latex)

	return nil
}

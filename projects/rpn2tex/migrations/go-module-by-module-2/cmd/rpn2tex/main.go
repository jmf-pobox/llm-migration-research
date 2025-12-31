package main

import (
	"flag"
	"fmt"
	"io"
	"os"
	"rpn2tex"
)

func main() {
	// Define command-line flags
	outputPath := flag.String("o", "", "Output LaTeX file (default: stdout)")
	flag.StringVar(outputPath, "output", "", "Output LaTeX file (long form)")

	// Customize usage message
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: rpn2tex [options] <input>\n\n")
		fmt.Fprintf(os.Stderr, "Convert RPN expressions to LaTeX notation.\n\n")
		fmt.Fprintf(os.Stderr, "Arguments:\n")
		fmt.Fprintf(os.Stderr, "  <input>    Input file containing RPN expression (use '-' for stdin)\n\n")
		fmt.Fprintf(os.Stderr, "Options:\n")
		flag.PrintDefaults()
	}

	flag.Parse()

	// Check for required positional argument
	if flag.NArg() != 1 {
		flag.Usage()
		os.Exit(1)
	}

	inputPath := flag.Arg(0)

	// Read input from file or stdin
	var text string
	if inputPath == "-" {
		// Read from stdin
		data, err := io.ReadAll(os.Stdin)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error reading stdin: %v\n", err)
			os.Exit(1)
		}
		text = string(data)
	} else {
		// Read from file
		data, err := os.ReadFile(inputPath)
		if err != nil {
			if os.IsNotExist(err) {
				fmt.Fprintf(os.Stderr, "Error: Input file not found: %s\n", inputPath)
			} else if os.IsPermission(err) {
				fmt.Fprintf(os.Stderr, "Error: Permission denied: %s\n", inputPath)
			} else {
				fmt.Fprintf(os.Stderr, "Error reading file: %v\n", err)
			}
			os.Exit(1)
		}
		text = string(data)
	}

	// Pipeline: Lex -> Parse -> Generate LaTeX

	// Step 1: Tokenize
	lexer := rpn2tex.NewLexer(text)
	tokens, err := lexer.Tokenize()
	if err != nil {
		// Check if it's a CompileError (with source context)
		if compileErr, ok := err.(*rpn2tex.CompileError); ok {
			fmt.Fprintln(os.Stderr, compileErr.Error())
		} else {
			fmt.Fprintf(os.Stderr, "Lexer error: %v\n", err)
		}
		os.Exit(1)
	}

	// Step 2: Parse tokens to AST
	parser := rpn2tex.NewParser(tokens)
	ast, err := parser.Parse()
	if err != nil {
		// Check if it's a CompileError (with source context)
		if compileErr, ok := err.(*rpn2tex.CompileError); ok {
			fmt.Fprintln(os.Stderr, compileErr.Error())
		} else {
			fmt.Fprintf(os.Stderr, "Parser error: %v\n", err)
		}
		os.Exit(1)
	}

	// Step 3: Generate LaTeX
	generator := rpn2tex.NewLaTeXGenerator()
	latex := generator.Generate(ast)

	// Write output to file or stdout
	if *outputPath != "" {
		// Write to file
		err := os.WriteFile(*outputPath, []byte(latex+"\n"), 0644)
		if err != nil {
			if os.IsPermission(err) {
				fmt.Fprintf(os.Stderr, "Error: Permission denied writing: %s\n", *outputPath)
			} else {
				fmt.Fprintf(os.Stderr, "Error writing file: %v\n", err)
			}
			os.Exit(1)
		}
		fmt.Fprintf(os.Stderr, "Generated: %s\n", *outputPath)
	} else {
		// Write to stdout
		fmt.Println(latex)
	}

	// Exit successfully
	os.Exit(0)
}

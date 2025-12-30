package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintln(os.Stderr, "Usage: rpn2tex <input_file_or_->")
		os.Exit(1)
	}

	var input string
	var err error

	// Read input from file or stdin
	if os.Args[1] == "-" {
		input, err = readStdin()
	} else {
		data, readErr := os.ReadFile(os.Args[1])
		if readErr != nil {
			fmt.Fprintf(os.Stderr, "Error reading file: %v\n", readErr)
			os.Exit(1)
		}
		input = string(data)
	}

	if err != nil {
		fmt.Fprintf(os.Stderr, "Error reading input: %v\n", err)
		os.Exit(1)
	}

	// Trim trailing newline if present
	input = strings.TrimRight(input, "\n\r")

	// Process the input
	output, err := processRPN(input)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v\n", err)
		os.Exit(1)
	}

	fmt.Println(output)
}

func readStdin() (string, error) {
	reader := bufio.NewReader(os.Stdin)
	var result strings.Builder

	for {
		line, err := reader.ReadString('\n')
		result.WriteString(line)
		if err != nil {
			if err == io.EOF {
				break
			}
			return "", err
		}
	}

	return result.String(), nil
}

func processRPN(input string) (string, error) {
	// Lexer
	lexer := NewLexer(input)
	tokens, err := lexer.Tokenize()
	if err != nil {
		return "", err
	}

	// Parser
	parser := NewParser(tokens)
	expr, err := parser.Parse()
	if err != nil {
		return "", err
	}

	// LaTeX Generator
	generator := NewLaTeXGenerator()
	output := generator.Generate(expr)

	return output, nil
}

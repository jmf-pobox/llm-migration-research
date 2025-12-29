package rpn2tex

import "fmt"

// Example demonstrates basic usage of the Lexer
func Example_lexer() {
	// Create a lexer with some RPN input
	lexer := NewLexer("5 3 +")

	// Tokenize the input
	tokens, err := lexer.Tokenize()
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	// Print each token
	for _, token := range tokens {
		fmt.Printf("%s at line %d, column %d\n", token.Type, token.Line, token.Column)
	}

	// Output:
	// NUMBER at line 1, column 1
	// NUMBER at line 1, column 3
	// PLUS at line 1, column 5
	// EOF at line 1, column 6
}

// Example demonstrates lexer error handling
func Example_lexerError() {
	// Create a lexer with invalid input (exponentiation not supported)
	lexer := NewLexer("2 3 ^")

	// Try to tokenize
	_, err := lexer.Tokenize()
	if err != nil {
		if lexErr, ok := err.(*LexerError); ok {
			fmt.Printf("Error at line %d, column %d: %s\n", lexErr.Line, lexErr.Column, lexErr.Message)
		}
	}

	// Output:
	// Error at line 1, column 5: Unexpected character '^'
}

// Example demonstrates decimal number preservation
func Example_lexerDecimalNumbers() {
	lexer := NewLexer("3.14 2 *")

	tokens, err := lexer.Tokenize()
	if err != nil {
		fmt.Printf("Error: %v\n", err)
		return
	}

	// Show that decimal numbers are preserved exactly
	fmt.Printf("First token value: %s\n", tokens[0].Value)
	fmt.Printf("Second token value: %s\n", tokens[1].Value)

	// Output:
	// First token value: 3.14
	// Second token value: 2
}

package main

import "fmt"

// ExampleTokenUsage demonstrates how to use the Token module
func ExampleTokenUsage() {
	// Create various token types
	numToken := Token{
		Type:   NUMBER,
		Value:  "42",
		Line:   1,
		Column: 1,
	}

	plusToken := Token{
		Type:   PLUS,
		Value:  "+",
		Line:   1,
		Column: 4,
	}

	// Print tokens using String() method
	fmt.Println(numToken)
	fmt.Println(plusToken)

	// Access token fields
	fmt.Printf("Token type: %s\n", numToken.Type)
	fmt.Printf("Token value: %s\n", numToken.Value)
	fmt.Printf("Position: %d:%d\n", numToken.Line, numToken.Column)

	// Create tokens for all operators
	tokens := []Token{
		{Type: NUMBER, Value: "5", Line: 1, Column: 1},
		{Type: NUMBER, Value: "3", Line: 1, Column: 3},
		{Type: PLUS, Value: "+", Line: 1, Column: 5},
		{Type: MINUS, Value: "-", Line: 1, Column: 7},
		{Type: MULT, Value: "*", Line: 1, Column: 9},
		{Type: DIV, Value: "/", Line: 1, Column: 11},
		{Type: EOF, Value: "", Line: 1, Column: 12},
	}

	fmt.Println("\nAll tokens:")
	for _, tok := range tokens {
		fmt.Printf("  %s\n", tok)
	}
}

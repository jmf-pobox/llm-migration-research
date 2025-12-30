package main

import (
	"fmt"
)

// ExampleNumber demonstrates creating a Number node.
func ExampleNumber() {
	num := &Number{
		Line:   1,
		Column: 1,
		Value:  "42",
	}

	fmt.Printf("Number value: %s at line %d, column %d\n",
		num.Value, num.GetLine(), num.GetColumn())
	// Output: Number value: 42 at line 1, column 1
}

// ExampleBinaryOp demonstrates creating a BinaryOp node for addition.
func ExampleBinaryOp() {
	// Create AST for: 5 + 3
	add := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 3, Value: "3"},
	}

	fmt.Printf("Binary operation: %s at line %d, column %d\n",
		add.Operator, add.GetLine(), add.GetColumn())
	// Output: Binary operation: + at line 1, column 5
}

// ExampleBinaryOp_nested demonstrates creating nested binary operations.
func ExampleBinaryOp_nested() {
	// Create AST for: (5 + 3) * 2
	// RPN notation: 5 3 + 2 *

	num5 := &Number{Line: 1, Column: 1, Value: "5"}
	num3 := &Number{Line: 1, Column: 3, Value: "3"}
	num2 := &Number{Line: 1, Column: 7, Value: "2"}

	// First operation: 5 + 3
	add := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     num5,
		Right:    num3,
	}

	// Second operation: (5 + 3) * 2
	mult := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left:     add,
		Right:    num2,
	}

	fmt.Printf("Root operation: %s at line %d, column %d\n",
		mult.Operator, mult.GetLine(), mult.GetColumn())

	// Access nested structure
	leftOp := mult.Left.(*BinaryOp)
	fmt.Printf("Left operation: %s\n", leftOp.Operator)

	// Output:
	// Root operation: * at line 1, column 9
	// Left operation: +
}

// ExampleExpr demonstrates using the Expr interface.
func ExampleExpr() {
	// Create different expression types
	exprs := []Expr{
		&Number{Line: 1, Column: 1, Value: "42"},
		&BinaryOp{
			Line:     2,
			Column:   5,
			Operator: "-",
			Left:     &Number{Line: 2, Column: 1, Value: "10"},
			Right:    &Number{Line: 2, Column: 3, Value: "3"},
		},
	}

	// Process through interface
	for i, expr := range exprs {
		fmt.Printf("Expression %d: line %d, column %d\n",
			i+1, expr.GetLine(), expr.GetColumn())
	}

	// Output:
	// Expression 1: line 1, column 1
	// Expression 2: line 2, column 5
}

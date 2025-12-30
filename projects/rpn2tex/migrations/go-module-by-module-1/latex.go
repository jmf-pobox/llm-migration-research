package main

import "fmt"

// LaTeXGenerator converts AST expressions to LaTeX notation.
type LaTeXGenerator struct{}

// Operator to LaTeX symbol mapping.
var binaryOps = map[string]string{
	"+": "+",
	"-": "-",
	"*": `\times`,
	"/": `\div`,
}

// Operator precedence levels.
var precedence = map[string]int{
	"+": 1,
	"-": 1,
	"*": 2,
	"/": 2,
}

// NewLaTeXGenerator creates a new LaTeX generator.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate converts an AST expression to LaTeX format.
// Returns the LaTeX string wrapped in inline math mode delimiters ($...$).
func (g *LaTeXGenerator) Generate(ast Expr) string {
	content := g.visit(ast)
	return "$" + content + "$"
}

// visit dispatches to the appropriate visitor method based on node type.
func (g *LaTeXGenerator) visit(node Expr) string {
	switch n := node.(type) {
	case *Number:
		return g.visitNumber(n)
	case *BinaryOp:
		return g.visitBinaryOp(n)
	default:
		panic(fmt.Sprintf("No visitor for %T", node))
	}
}

// visitNumber handles Number nodes.
func (g *LaTeXGenerator) visitNumber(node *Number) string {
	return node.Value
}

// visitBinaryOp handles BinaryOp nodes with precedence-aware parenthesization.
func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
	op := binaryOps[node.Operator]
	parentPrec := precedence[node.Operator]

	// Recursively visit children
	left := g.visit(node.Left)
	right := g.visit(node.Right)

	// Add parentheses to left child if needed
	if g.needsParens(node.Left, parentPrec, false) {
		left = "( " + left + " )"
	}

	// Add parentheses to right child if needed
	if g.needsParens(node.Right, parentPrec, true) {
		right = "( " + right + " )"
	}

	return left + " " + op + " " + right
}

// needsParens determines if a child expression needs parentheses.
// A child needs parentheses if:
// - It's a BinaryOp with lower precedence than the parent
// - It's a BinaryOp on the right side with equal precedence (for left-associative operators)
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	binOp, ok := child.(*BinaryOp)
	if !ok {
		// Numbers never need parentheses
		return false
	}

	childPrec := precedence[binOp.Operator]

	// Child has lower precedence - always needs parens
	if childPrec < parentPrecedence {
		return true
	}

	// Child has equal precedence on the right side
	// For left-associative operators (all of them in RPN), this needs parens
	if childPrec == parentPrecedence && isRight {
		return true
	}

	return false
}

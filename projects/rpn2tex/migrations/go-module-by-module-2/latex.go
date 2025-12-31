package rpn2tex

import (
	"fmt"
)

// LaTeXGenerator converts AST nodes to LaTeX mathematical notation.
type LaTeXGenerator struct {
	binaryOps  map[string]string
	precedence map[string]int
}

// NewLaTeXGenerator creates a new LaTeX generator with operator mappings.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{
		binaryOps: map[string]string{
			"+": "+",
			"-": "-",
			"*": `\times`,
			"/": `\div`,
		},
		precedence: map[string]int{
			"+": 1,
			"-": 1,
			"*": 2,
			"/": 2,
		},
	}
}

// Generate converts an AST expression to LaTeX notation wrapped in $...$.
func (g *LaTeXGenerator) Generate(ast Expr) string {
	content := g.visit(ast)
	return fmt.Sprintf("$%s$", content)
}

// visit dispatches to the appropriate visitor method based on node type.
func (g *LaTeXGenerator) visit(node Expr) string {
	switch n := node.(type) {
	case *Number:
		return g.visitNumber(n)
	case *BinaryOp:
		return g.visitBinaryOp(n)
	default:
		// This should never happen if AST is well-formed
		return ""
	}
}

// visitNumber returns the string value of a numeric literal.
func (g *LaTeXGenerator) visitNumber(node *Number) string {
	return node.Value
}

// visitBinaryOp converts a binary operation to LaTeX with proper parenthesization.
func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
	opLatex := g.binaryOps[node.Operator]
	myPrecedence := g.precedence[node.Operator]

	// Process left operand
	left := g.visit(node.Left)
	if g.needsParens(node.Left, myPrecedence, false) {
		left = fmt.Sprintf("( %s )", left)
	}

	// Process right operand
	right := g.visit(node.Right)
	if g.needsParens(node.Right, myPrecedence, true) {
		right = fmt.Sprintf("( %s )", right)
	}

	return fmt.Sprintf("%s %s %s", left, opLatex, right)
}

// needsParens determines if a child node needs parentheses based on precedence
// and associativity rules.
//
// A child needs parentheses if:
//  1. It has lower precedence than its parent
//  2. It has equal precedence, is on the right side, and the operators are
//     non-associative (- or /)
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	// Numbers never need parentheses
	childOp, ok := child.(*BinaryOp)
	if !ok {
		return false
	}

	childPrecedence := g.precedence[childOp.Operator]

	// Lower precedence always needs parens
	if childPrecedence < parentPrecedence {
		return true
	}

	// Equal precedence on right side needs parens for non-commutative operators
	// This handles left-associativity for - and /
	// Example: 5 - (3 - 2) needs parens, but (5 - 3) - 2 doesn't
	if childPrecedence == parentPrecedence && isRight {
		return childOp.Operator == "-" || childOp.Operator == "/"
	}

	return false
}

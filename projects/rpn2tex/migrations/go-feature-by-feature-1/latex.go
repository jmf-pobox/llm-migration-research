package main

// Precedence levels for operators
var precedence = map[string]int{
	"+": 1,
	"-": 1,
	"*": 2,
	"/": 2,
}

// Non-commutative operators that need special parenthesization on the right
var nonCommutative = map[string]bool{
	"-": true,
	"/": true,
}

// LaTeX operator mapping
var binaryOps = map[string]string{
	"+": "+",
	"-": "-",
	"*": `\times`,
	"/": `\div`,
}

// LaTeXGenerator generates LaTeX output from an AST
type LaTeXGenerator struct{}

// NewLaTeXGenerator creates a new LaTeX generator
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate generates LaTeX output for an expression
func (g *LaTeXGenerator) Generate(expr Expr) string {
	return "$" + g.visit(expr) + "$"
}

// visit visits an expression node and returns its LaTeX representation
func (g *LaTeXGenerator) visit(expr Expr) string {
	switch n := expr.(type) {
	case *Number:
		return g.visitNumber(n)
	case *BinaryOp:
		return g.visitBinaryOp(n)
	default:
		return ""
	}
}

// visitNumber generates LaTeX for a number node
func (g *LaTeXGenerator) visitNumber(n *Number) string {
	return n.Value
}

// visitBinaryOp generates LaTeX for a binary operation with precedence handling
func (g *LaTeXGenerator) visitBinaryOp(b *BinaryOp) string {
	// Get operator's LaTeX representation and precedence
	opLatex := binaryOps[b.Operator]
	myPrecedence := precedence[b.Operator]

	// Generate left operand, adding parens if needed
	left := g.visit(b.Left)
	if g.needsParens(b.Left, myPrecedence, false) {
		left = "( " + left + " )"
	}

	// Generate right operand, adding parens if needed
	right := g.visit(b.Right)
	if g.needsParens(b.Right, myPrecedence, true) {
		right = "( " + right + " )"
	}

	return left + " " + opLatex + " " + right
}

// needsParens determines if a child expression needs parentheses
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	// Only BinaryOp nodes need parentheses consideration
	binOp, ok := child.(*BinaryOp)
	if !ok {
		return false
	}

	childPrecedence := precedence[binOp.Operator]

	// Lower precedence always needs parens
	if childPrecedence < parentPrecedence {
		return true
	}

	// Equal precedence on right side needs parens for non-commutative operators
	if childPrecedence == parentPrecedence && isRight && nonCommutative[binOp.Operator] {
		return true
	}

	return false
}

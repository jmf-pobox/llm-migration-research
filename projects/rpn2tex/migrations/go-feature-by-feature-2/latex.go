package rpn2tex

import "fmt"

// binaryOps maps operator strings to LaTeX symbols.
var binaryOps = map[string]string{
	"+": "+",
	"-": "-",
	"*": `\times`,
	"/": `\div`,
}

// precedence defines operator precedence levels (higher = tighter binding).
var precedence = map[string]int{
	"+": 1,
	"-": 1,
	"*": 2,
	"/": 2,
}

// LaTeXGenerator converts an AST to LaTeX output.
type LaTeXGenerator struct{}

// NewLaTeXGenerator creates a new LaTeXGenerator.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate converts an AST to LaTeX string wrapped in $ delimiters.
func (g *LaTeXGenerator) Generate(expr Expr) string {
	return "$" + g.visit(expr) + "$"
}

// visit dispatches to the appropriate visitor method based on node type.
func (g *LaTeXGenerator) visit(expr Expr) string {
	switch e := expr.(type) {
	case *Number:
		return g.visitNumber(e)
	case *BinaryOp:
		return g.visitBinaryOp(e)
	default:
		panic(fmt.Sprintf("Unknown expression type: %T", expr))
	}
}

// visitNumber returns the numeric value as-is.
func (g *LaTeXGenerator) visitNumber(n *Number) string {
	return n.Value
}

// visitBinaryOp generates LaTeX for binary operations with precedence handling.
func (g *LaTeXGenerator) visitBinaryOp(b *BinaryOp) string {
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

// needsParens determines if a child expression needs parentheses.
//
// Parentheses are needed when:
//  1. Child has lower precedence than parent
//  2. Child has equal precedence and is on the right side
//     (for left-associative non-commutative operators like - and /)
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	// If child is not a binary operation, it never needs parens
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
	// (handles left-associativity of - and /)
	if childPrecedence == parentPrecedence && isRight {
		return binOp.Operator == "-" || binOp.Operator == "/"
	}

	return false
}

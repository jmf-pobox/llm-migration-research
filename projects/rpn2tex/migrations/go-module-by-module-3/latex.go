package rpn2tex

import "fmt"

// binaryOps maps operator strings to their LaTeX representation
var binaryOps = map[string]string{
	"+": "+",
	"-": "-",
	"*": `\times`,
	"/": `\div`,
}

// precedence maps operator strings to their precedence level (higher = tighter binding)
var precedence = map[string]int{
	"+": 1,
	"-": 1,
	"*": 2,
	"/": 2,
}

// LaTeXGenerator converts an AST to LaTeX representation
type LaTeXGenerator struct{}

// NewLaTeXGenerator creates a new LaTeX generator
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate converts an AST expression to LaTeX string wrapped in $...$
func (g *LaTeXGenerator) Generate(ast Expr) string {
	return "$" + g.visit(ast) + "$"
}

// visit dispatches to the appropriate visitor method based on node type
func (g *LaTeXGenerator) visit(node Expr) string {
	switch e := node.(type) {
	case *Number:
		return g.visitNumber(e)
	case *BinaryOp:
		return g.visitBinaryOp(e)
	default:
		panic(fmt.Sprintf("No visitor for %T", node))
	}
}

// visitNumber handles Number nodes by returning their string value
func (g *LaTeXGenerator) visitNumber(node *Number) string {
	return node.Value
}

// visitBinaryOp handles BinaryOp nodes by recursively visiting children
// and adding parentheses when necessary to preserve operator precedence
func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOp) string {
	// Get the LaTeX representation of the operator
	opLatex, ok := binaryOps[node.Operator]
	if !ok {
		panic(fmt.Sprintf("Unknown operator: %s", node.Operator))
	}

	// Get the precedence of this operator
	myPrecedence := precedence[node.Operator]

	// Visit left child
	leftStr := g.visit(node.Left)
	if g.needsParens(node.Left, myPrecedence, false) {
		leftStr = "( " + leftStr + " )"
	}

	// Visit right child
	rightStr := g.visit(node.Right)
	if g.needsParens(node.Right, myPrecedence, true) {
		rightStr = "( " + rightStr + " )"
	}

	// Combine with spaces around operator
	return leftStr + " " + opLatex + " " + rightStr
}

// needsParens determines if a child expression needs parentheses
// based on its precedence relative to the parent and its position (left or right)
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	// Numbers never need parentheses
	if _, ok := child.(*Number); ok {
		return false
	}

	// If child is a BinaryOp, check precedence
	if binOp, ok := child.(*BinaryOp); ok {
		childPrecedence := precedence[binOp.Operator]

		// Lower precedence child always needs parens
		if childPrecedence < parentPrecedence {
			return true
		}

		// Equal precedence on right side needs parens for non-commutative operators
		if childPrecedence == parentPrecedence && isRight {
			// Non-commutative operators: - and /
			if binOp.Operator == "-" || binOp.Operator == "/" {
				return true
			}
		}

		return false
	}

	return false
}

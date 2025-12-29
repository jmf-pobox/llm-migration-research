// Package rpn2tex provides LaTeX generation from AST nodes.
package rpn2tex

import "strings"

// Operator precedence levels.
const (
	PrecedenceLow  = 1 // Addition and subtraction
	PrecedenceHigh = 2 // Multiplication and division
)

// Precedence maps operators to their precedence levels.
var Precedence = map[string]int{
	"+": PrecedenceLow,
	"-": PrecedenceLow,
	"*": PrecedenceHigh,
	"/": PrecedenceHigh,
}

// BinaryOps maps RPN operators to their LaTeX representations.
var BinaryOps = map[string]string{
	"+": "+",
	"-": "-",
	"*": `\times`,
	"/": `\div`,
}

// LaTeXGenerator implements the Visitor interface to generate LaTeX from an AST.
type LaTeXGenerator struct{}

// NewLaTeXGenerator creates a new LaTeX generator.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate converts an AST expression to a LaTeX math mode string.
// The output is wrapped in $ delimiters: "$expression$"
func (g *LaTeXGenerator) Generate(ast Expr) string {
	content := ast.Accept(g)
	return "$" + content + "$"
}

// VisitNumber processes a Number node and returns its string value.
func (g *LaTeXGenerator) VisitNumber(n *Number) string {
	return n.Value
}

// VisitBinaryOp processes a BinaryOp node and returns the formatted LaTeX expression.
// This method handles parenthesization based on operator precedence.
func (g *LaTeXGenerator) VisitBinaryOp(b *BinaryOp) string {
	parentPrecedence := Precedence[b.Operator]
	latexOp := BinaryOps[b.Operator]

	// Visit left operand
	leftStr := b.Left.Accept(g)
	if g.needsParens(b.Left, parentPrecedence, false) {
		leftStr = "( " + leftStr + " )"
	}

	// Visit right operand
	rightStr := b.Right.Accept(g)
	if g.needsParens(b.Right, parentPrecedence, true) {
		rightStr = "( " + rightStr + " )"
	}

	// Build the expression: "left op right"
	var result strings.Builder
	result.WriteString(leftStr)
	result.WriteString(" ")
	result.WriteString(latexOp)
	result.WriteString(" ")
	result.WriteString(rightStr)

	return result.String()
}

// needsParens determines if a child expression needs parentheses
// based on operator precedence and associativity.
//
// Parentheses are needed when:
// 1. Child has lower precedence than parent (always)
// 2. Child has equal precedence, is on the right side, and uses a non-commutative operator (-, /)
//
// Parameters:
//   - child: The child expression to check
//   - parentPrecedence: The precedence level of the parent operator
//   - isRight: Whether the child is the right operand of the parent
//
// Returns true if parentheses are needed, false otherwise.
func (g *LaTeXGenerator) needsParens(child Expr, parentPrecedence int, isRight bool) bool {
	// Only BinaryOp nodes need parentheses; Number nodes never do
	binOp, ok := child.(*BinaryOp)
	if !ok {
		return false
	}

	childPrecedence := Precedence[binOp.Operator]

	// Lower precedence always needs parens
	if childPrecedence < parentPrecedence {
		return true
	}

	// Equal precedence on right side needs parens for non-commutative operators
	// This enforces left-associativity for subtraction and division
	if childPrecedence == parentPrecedence && isRight {
		return binOp.Operator == "-" || binOp.Operator == "/"
	}

	return false
}

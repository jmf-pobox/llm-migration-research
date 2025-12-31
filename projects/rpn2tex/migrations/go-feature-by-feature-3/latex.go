package rpn2tex

import "fmt"

// LaTeXGenerator generates LaTeX output from an AST.
type LaTeXGenerator struct {
	precedence map[string]int
	operators  map[string]string
}

// NewLaTeXGenerator creates a new LaTeX generator.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{
		precedence: map[string]int{
			"+": 1,
			"-": 1,
			"*": 2,
			"/": 2,
		},
		operators: map[string]string{
			"+": "+",
			"-": "-",
			"*": "\\times",
			"/": "\\div",
		},
	}
}

// Generate converts an AST to LaTeX output wrapped in $...$
func (g *LaTeXGenerator) Generate(ast Expr) string {
	content := g.visit(ast)
	return fmt.Sprintf("$%s$", content)
}

// precedenceOf returns the precedence level for an operator.
func (g *LaTeXGenerator) precedenceOf(op string) int {
	if prec, ok := g.precedence[op]; ok {
		return prec
	}
	return 0
}

// needsParens determines if a child expression needs parentheses.
// It checks:
//  1. If child has lower precedence than parent, add parens
//  2. If child has equal precedence as parent AND is on the right side
//     AND child operator is non-commutative (- or /), add parens
func (g *LaTeXGenerator) needsParens(child Expr, parentPrec int, isRight bool) bool {
	binOp, ok := child.(*BinaryOp)
	if !ok {
		return false
	}

	childPrec := g.precedenceOf(binOp.Operator)

	// Lower precedence always needs parens
	if childPrec < parentPrec {
		return true
	}

	// Equal precedence on right side needs parens for non-commutative operators
	if childPrec == parentPrec && isRight {
		return binOp.Operator == "-" || binOp.Operator == "/"
	}

	return false
}

func (g *LaTeXGenerator) visit(node Expr) string {
	switch n := node.(type) {
	case *Number:
		return n.Value
	case *BinaryOp:
		myPrec := g.precedenceOf(n.Operator)

		// Visit left child
		left := g.visit(n.Left)
		if g.needsParens(n.Left, myPrec, false) {
			left = fmt.Sprintf("( %s )", left)
		}

		// Visit right child
		right := g.visit(n.Right)
		if g.needsParens(n.Right, myPrec, true) {
			right = fmt.Sprintf("( %s )", right)
		}

		// Get LaTeX operator
		opLatex := g.operators[n.Operator]

		return fmt.Sprintf("%s %s %s", left, opLatex, right)
	default:
		return ""
	}
}

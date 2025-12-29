package rpn2tex

import "fmt"

// LaTeXGenerator generates LaTeX output from an AST.
type LaTeXGenerator struct{}

// NewLaTeXGenerator creates a new LaTeX generator.
func NewLaTeXGenerator() *LaTeXGenerator {
	return &LaTeXGenerator{}
}

// Generate generates LaTeX output from the given AST.
func (g *LaTeXGenerator) Generate(expr Expr) (string, error) {
	return g.visit(expr), nil
}

// visit dispatches to the appropriate visitor method based on node type.
func (g *LaTeXGenerator) visit(expr Expr) string {
	switch node := expr.(type) {
	case *NumberNode:
		return g.visitNumber(node)
	case *BinaryOpNode:
		return g.visitBinaryOp(node)
	default:
		panic(fmt.Sprintf("unknown node type: %T", expr))
	}
}

// visitNumber generates LaTeX for a number node.
func (g *LaTeXGenerator) visitNumber(node *NumberNode) string {
	return node.Value
}

// visitBinaryOp generates LaTeX for a binary operation node.
func (g *LaTeXGenerator) visitBinaryOp(node *BinaryOpNode) string {
	// Map operator to LaTeX representation
	var opLatex string
	switch node.Operator {
	case "+":
		opLatex = "+"
	case "-":
		opLatex = "-"
	case "*":
		opLatex = "\\times"
	case "/":
		opLatex = "\\div"
	default:
		opLatex = node.Operator
	}

	// Recursively generate left operand, adding parentheses if needed
	left := g.visit(node.Left)
	if needsParentheses(node.Left, node, true) {
		left = fmt.Sprintf("( %s )", left)
	}

	// Recursively generate right operand, adding parentheses if needed
	right := g.visit(node.Right)
	if needsParentheses(node.Right, node, false) {
		right = fmt.Sprintf("( %s )", right)
	}

	// Format as "left op right" with spaces
	return fmt.Sprintf("%s %s %s", left, opLatex, right)
}

// needsParentheses determines if a child expression needs parentheses
// when appearing as an operand of a parent binary operation.
func needsParentheses(child Expr, parent *BinaryOpNode, isLeft bool) bool {
	childPrec := child.Precedence()
	parentPrec := parent.Precedence()

	// Rule 1: Lower precedence always needs parentheses
	if childPrec < parentPrec {
		return true
	}

	// Rule 2: Equal precedence on right side needs parentheses
	// for non-commutative operators (- and /)
	if childPrec == parentPrec && !isLeft {
		if parent.Operator == "-" || parent.Operator == "/" {
			return true
		}
	}

	return false
}

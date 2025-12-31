package rpn2tex

// Generator generates LaTeX output from an AST.
type Generator struct{}

// NewGenerator creates a new LaTeX generator.
func NewGenerator() *Generator {
	return &Generator{}
}

// Generate generates LaTeX output from the root AST node.
func (g *Generator) Generate(expr Expr) string {
	return "$" + g.visit(expr) + "$"
}

// visit dispatches to the appropriate visitor method based on node type.
func (g *Generator) visit(expr Expr) string {
	switch n := expr.(type) {
	case *Number:
		return g.visitNumber(n)
	case *BinaryOp:
		return g.visitBinaryOp(n)
	default:
		panic("Unknown node type")
	}
}

// visitNumber generates LaTeX for a number literal.
func (g *Generator) visitNumber(n *Number) string {
	return n.Value
}

// visitBinaryOp generates LaTeX for a binary operation.
func (g *Generator) visitBinaryOp(node *BinaryOp) string {
	left := g.visit(node.Left)
	right := g.visit(node.Right)

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

	return left + " " + opLatex + " " + right
}

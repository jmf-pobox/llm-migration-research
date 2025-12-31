package rpn2tex

// Expr is the interface that all AST expression nodes implement.
// The unexported method ensures only types in this package can implement it.
type Expr interface {
	exprNode()
}

// Number represents a numeric literal in the AST.
type Number struct {
	Line   int    // 1-based line number in source
	Column int    // 1-based column number in source
	Value  string // String representation preserves precision (e.g., "3.14", "-5")
}

// exprNode marks Number as an Expr implementation.
func (*Number) exprNode() {}

// BinaryOp represents a binary operation in the AST.
type BinaryOp struct {
	Line     int    // 1-based line number in source
	Column   int    // 1-based column number in source
	Operator string // "+", "-", "*", or "/"
	Left     Expr   // Left operand
	Right    Expr   // Right operand
}

// exprNode marks BinaryOp as an Expr implementation.
func (*BinaryOp) exprNode() {}

// NewNumber creates a new Number node.
func NewNumber(line, column int, value string) *Number {
	return &Number{
		Line:   line,
		Column: column,
		Value:  value,
	}
}

// NewBinaryOp creates a new BinaryOp node.
func NewBinaryOp(line, column int, operator string, left, right Expr) *BinaryOp {
	return &BinaryOp{
		Line:     line,
		Column:   column,
		Operator: operator,
		Left:     left,
		Right:    right,
	}
}

package main

// Expr is the interface for all expression nodes
type Expr interface {
	exprNode()
}

// Number represents a numeric literal in the AST
type Number struct {
	Value  string
	Line   int
	Column int
}

// exprNode implements the Expr interface
func (n *Number) exprNode() {}

// BinaryOp represents a binary operation in the AST
type BinaryOp struct {
	Operator string
	Left     Expr
	Right    Expr
	Line     int
	Column   int
}

// exprNode implements the Expr interface
func (b *BinaryOp) exprNode() {}

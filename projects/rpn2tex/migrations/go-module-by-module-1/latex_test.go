package main

import "testing"

func TestLaTeXGenerator_Generate_SingleNumber(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &Number{Line: 1, Column: 1, Value: "42"}
	result := gen.Generate(ast)
	expected := "$42$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_FloatingPoint(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &Number{Line: 1, Column: 1, Value: "3.14"}
	result := gen.Generate(ast)
	expected := "$3.14$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_Addition(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 3, Value: "3"},
	}
	result := gen.Generate(ast)
	expected := "$5 + 3$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_Subtraction(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "-",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 3, Value: "3"},
	}
	result := gen.Generate(ast)
	expected := "$5 - 3$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_Multiplication(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "*",
		Left:     &Number{Line: 1, Column: 1, Value: "4"},
		Right:    &Number{Line: 1, Column: 3, Value: "7"},
	}
	result := gen.Generate(ast)
	expected := `$4 \times 7$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_Division(t *testing.T) {
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "/",
		Left:     &Number{Line: 1, Column: 1, Value: "10"},
		Right:    &Number{Line: 1, Column: 4, Value: "2"},
	}
	result := gen.Generate(ast)
	expected := `$10 \div 2$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_AdditionWithParens(t *testing.T) {
	// (5 + 3) * 2 - parentheses needed for addition with lower precedence
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 1, Value: "5"},
			Right:    &Number{Line: 1, Column: 5, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 7, Value: "2"},
	}
	result := gen.Generate(ast)
	expected := `$( 5 + 3 ) \times 2$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MultiplicationNoParens(t *testing.T) {
	// 5 * 3 + 2 - no parentheses needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "+",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "*",
			Left:     &Number{Line: 1, Column: 1, Value: "5"},
			Right:    &Number{Line: 1, Column: 5, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 7, Value: "2"},
	}
	result := gen.Generate(ast)
	expected := `$5 \times 3 + 2$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_LeftAssociativeSubtraction(t *testing.T) {
	// 5 - 3 - 2, parsed as (5 - 3) - 2, no parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "-",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "-",
			Left:     &Number{Line: 1, Column: 1, Value: "5"},
			Right:    &Number{Line: 1, Column: 5, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 7, Value: "2"},
	}
	result := gen.Generate(ast)
	expected := "$5 - 3 - 2$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_RightAssociativeSubtraction(t *testing.T) {
	// 5 - (3 - 2), parens needed on right side
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "-",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right: &BinaryOp{
			Line:     1,
			Column:   7,
			Operator: "-",
			Left:     &Number{Line: 1, Column: 5, Value: "3"},
			Right:    &Number{Line: 1, Column: 9, Value: "2"},
		},
	}
	result := gen.Generate(ast)
	expected := "$5 - ( 3 - 2 )$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_LeftAssociativeDivision(t *testing.T) {
	// 100 / 10 / 5 / 2, parsed as ((100 / 10) / 5) / 2, no parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   15,
		Operator: "/",
		Left: &BinaryOp{
			Line:     1,
			Column:   10,
			Operator: "/",
			Left: &BinaryOp{
				Line:     1,
				Column:   5,
				Operator: "/",
				Left:     &Number{Line: 1, Column: 1, Value: "100"},
				Right:    &Number{Line: 1, Column: 7, Value: "10"},
			},
			Right: &Number{Line: 1, Column: 12, Value: "5"},
		},
		Right: &Number{Line: 1, Column: 17, Value: "2"},
	}
	result := gen.Generate(ast)
	expected := `$100 \div 10 \div 5 \div 2$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_RightAssociativeDivision(t *testing.T) {
	// 10 / (2 / 5), parens needed on right side
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "/",
		Left:     &Number{Line: 1, Column: 1, Value: "10"},
		Right: &BinaryOp{
			Line:     1,
			Column:   7,
			Operator: "/",
			Left:     &Number{Line: 1, Column: 5, Value: "2"},
			Right:    &Number{Line: 1, Column: 9, Value: "5"},
		},
	}
	result := gen.Generate(ast)
	expected := `$10 \div ( 2 \div 5 )$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_ChainedAddition(t *testing.T) {
	// 1 + 2 + 3 + 4, parsed as (((1 + 2) + 3) + 4), no parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   13,
		Operator: "+",
		Left: &BinaryOp{
			Line:     1,
			Column:   9,
			Operator: "+",
			Left: &BinaryOp{
				Line:     1,
				Column:   5,
				Operator: "+",
				Left:     &Number{Line: 1, Column: 1, Value: "1"},
				Right:    &Number{Line: 1, Column: 3, Value: "2"},
			},
			Right: &Number{Line: 1, Column: 7, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 11, Value: "4"},
	}
	result := gen.Generate(ast)
	expected := "$1 + 2 + 3 + 4$"
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MixedPrecedence1(t *testing.T) {
	// 2 + 3 * 4, no parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "2"},
		Right: &BinaryOp{
			Line:     1,
			Column:   5,
			Operator: "*",
			Left:     &Number{Line: 1, Column: 3, Value: "3"},
			Right:    &Number{Line: 1, Column: 7, Value: "4"},
		},
	}
	result := gen.Generate(ast)
	expected := `$2 + 3 \times 4$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MixedPrecedence2(t *testing.T) {
	// (2 + 3) * 4, parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 1, Value: "2"},
			Right:    &Number{Line: 1, Column: 5, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 7, Value: "4"},
	}
	result := gen.Generate(ast)
	expected := `$( 2 + 3 ) \times 4$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MixedPrecedence3(t *testing.T) {
	// 2 * (3 + 4), parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "*",
		Left:     &Number{Line: 1, Column: 1, Value: "2"},
		Right: &BinaryOp{
			Line:     1,
			Column:   7,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 5, Value: "3"},
			Right:    &Number{Line: 1, Column: 9, Value: "4"},
		},
	}
	result := gen.Generate(ast)
	expected := `$2 \times ( 3 + 4 )$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MixedPrecedence4(t *testing.T) {
	// 2 * 3 + 4, no parens needed
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "+",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "*",
			Left:     &Number{Line: 1, Column: 1, Value: "2"},
			Right:    &Number{Line: 1, Column: 5, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 7, Value: "4"},
	}
	result := gen.Generate(ast)
	expected := `$2 \times 3 + 4$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_ComplexExpression1(t *testing.T) {
	// (1 + 2) * (3 + 4), both sides need parens
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left: &BinaryOp{
			Line:     1,
			Column:   3,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 1, Value: "1"},
			Right:    &Number{Line: 1, Column: 5, Value: "2"},
		},
		Right: &BinaryOp{
			Line:     1,
			Column:   13,
			Operator: "+",
			Left:     &Number{Line: 1, Column: 11, Value: "3"},
			Right:    &Number{Line: 1, Column: 15, Value: "4"},
		},
	}
	result := gen.Generate(ast)
	expected := `$( 1 + 2 ) \times ( 3 + 4 )$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_ComplexExpression2(t *testing.T) {
	// (10 / 2 + 3) * 4
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   13,
		Operator: "*",
		Left: &BinaryOp{
			Line:     1,
			Column:   9,
			Operator: "+",
			Left: &BinaryOp{
				Line:     1,
				Column:   4,
				Operator: "/",
				Left:     &Number{Line: 1, Column: 1, Value: "10"},
				Right:    &Number{Line: 1, Column: 6, Value: "2"},
			},
			Right: &Number{Line: 1, Column: 11, Value: "3"},
		},
		Right: &Number{Line: 1, Column: 15, Value: "4"},
	}
	result := gen.Generate(ast)
	expected := `$( 10 \div 2 + 3 ) \times 4$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_Generate_MixedDivisionMultiplication(t *testing.T) {
	// 10 / 2 * 5, no parens needed (same precedence, left associative)
	gen := NewLaTeXGenerator()
	ast := &BinaryOp{
		Line:     1,
		Column:   9,
		Operator: "*",
		Left: &BinaryOp{
			Line:     1,
			Column:   4,
			Operator: "/",
			Left:     &Number{Line: 1, Column: 1, Value: "10"},
			Right:    &Number{Line: 1, Column: 6, Value: "2"},
		},
		Right: &Number{Line: 1, Column: 11, Value: "5"},
	}
	result := gen.Generate(ast)
	expected := `$10 \div 2 \times 5$`
	if result != expected {
		t.Errorf("Generate() = %q, want %q", result, expected)
	}
}

func TestLaTeXGenerator_NeedsParens_Number(t *testing.T) {
	gen := NewLaTeXGenerator()
	num := &Number{Line: 1, Column: 1, Value: "5"}

	if gen.needsParens(num, 2, false) {
		t.Error("Number should never need parens")
	}
	if gen.needsParens(num, 2, true) {
		t.Error("Number should never need parens on right")
	}
}

func TestLaTeXGenerator_NeedsParens_LowerPrecedence(t *testing.T) {
	gen := NewLaTeXGenerator()
	// Addition (precedence 1) as child of multiplication (precedence 2)
	child := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "+",
		Left:     &Number{Line: 1, Column: 1, Value: "2"},
		Right:    &Number{Line: 1, Column: 5, Value: "3"},
	}

	if !gen.needsParens(child, 2, false) {
		t.Error("Lower precedence child should need parens on left")
	}
	if !gen.needsParens(child, 2, true) {
		t.Error("Lower precedence child should need parens on right")
	}
}

func TestLaTeXGenerator_NeedsParens_EqualPrecedenceLeft(t *testing.T) {
	gen := NewLaTeXGenerator()
	// Subtraction (precedence 1) as left child of subtraction (precedence 1)
	child := &BinaryOp{
		Line:     1,
		Column:   3,
		Operator: "-",
		Left:     &Number{Line: 1, Column: 1, Value: "5"},
		Right:    &Number{Line: 1, Column: 5, Value: "3"},
	}

	if gen.needsParens(child, 1, false) {
		t.Error("Equal precedence on left should not need parens")
	}
}

func TestLaTeXGenerator_NeedsParens_EqualPrecedenceRight(t *testing.T) {
	gen := NewLaTeXGenerator()
	// Subtraction (precedence 1) as right child of subtraction (precedence 1)
	child := &BinaryOp{
		Line:     1,
		Column:   7,
		Operator: "-",
		Left:     &Number{Line: 1, Column: 5, Value: "3"},
		Right:    &Number{Line: 1, Column: 9, Value: "2"},
	}

	if !gen.needsParens(child, 1, true) {
		t.Error("Equal precedence on right should need parens for left-associative operators")
	}
}

func TestLaTeXGenerator_NeedsParens_HigherPrecedence(t *testing.T) {
	gen := NewLaTeXGenerator()
	// Multiplication (precedence 2) as child of addition (precedence 1)
	child := &BinaryOp{
		Line:     1,
		Column:   5,
		Operator: "*",
		Left:     &Number{Line: 1, Column: 3, Value: "3"},
		Right:    &Number{Line: 1, Column: 7, Value: "4"},
	}

	if gen.needsParens(child, 1, false) {
		t.Error("Higher precedence child should not need parens on left")
	}
	if gen.needsParens(child, 1, true) {
		t.Error("Higher precedence child should not need parens on right")
	}
}

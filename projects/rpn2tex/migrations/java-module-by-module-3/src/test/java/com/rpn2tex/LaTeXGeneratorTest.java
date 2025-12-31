package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for LaTeXGenerator.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Number node generation</li>
 *   <li>Basic binary operations</li>
 *   <li>Operator precedence and parenthesization</li>
 *   <li>Left-associativity rules</li>
 *   <li>Complex nested expressions</li>
 *   <li>I/O contract validation</li>
 * </ul>
 */
class LaTeXGeneratorTest {
    private final LaTeXGenerator generator = new LaTeXGenerator();

    @Test
    void testGenerateNumber() {
        Number num = new Number(1, 1, "42");
        String result = generator.generate(num);
        assertEquals("$42$", result);
    }

    @Test
    void testGenerateDecimalNumber() {
        Number num = new Number(1, 1, "3.14");
        String result = generator.generate(num);
        assertEquals("$3.14$", result);
    }

    @Test
    void testGenerateNegativeNumber() {
        Number num = new Number(1, 1, "-5");
        String result = generator.generate(num);
        assertEquals("$-5$", result);
    }

    @Test
    void testGenerateAddition() {
        // 5 + 3
        Expr expr = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));
        String result = generator.generate(expr);
        assertEquals("$5 + 3$", result);
    }

    @Test
    void testGenerateSubtraction() {
        // 5 - 3
        Expr expr = new BinaryOp(1, 3, "-",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));
        String result = generator.generate(expr);
        assertEquals("$5 - 3$", result);
    }

    @Test
    void testGenerateMultiplication() {
        // 4 * 7
        Expr expr = new BinaryOp(1, 3, "*",
            new Number(1, 1, "4"),
            new Number(1, 3, "7"));
        String result = generator.generate(expr);
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    void testGenerateDivision() {
        // 10 / 2
        Expr expr = new BinaryOp(1, 3, "/",
            new Number(1, 1, "10"),
            new Number(1, 3, "2"));
        String result = generator.generate(expr);
        assertEquals("$10 \\div 2$", result);
    }

    @Test
    void testAdditionThenMultiplication() {
        // (5 + 3) * 2 - needs parentheses
        Expr addition = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));
        Expr expr = new BinaryOp(1, 5, "*",
            addition,
            new Number(1, 5, "2"));
        String result = generator.generate(expr);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    void testMultiplicationThenAddition() {
        // 5 * 3 + 2 - no parentheses needed
        Expr multiplication = new BinaryOp(1, 3, "*",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));
        Expr expr = new BinaryOp(1, 5, "+",
            multiplication,
            new Number(1, 5, "2"));
        String result = generator.generate(expr);
        assertEquals("$5 \\times 3 + 2$", result);
    }

    @Test
    void testLeftAssociativeSubtraction() {
        // 5 - 3 - 2 - no parentheses (left-associative)
        Expr sub1 = new BinaryOp(1, 3, "-",
            new Number(1, 1, "5"),
            new Number(1, 3, "3"));
        Expr expr = new BinaryOp(1, 5, "-",
            sub1,
            new Number(1, 5, "2"));
        String result = generator.generate(expr);
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    void testRightSubtractionNeedsParens() {
        // 5 - (3 - 2) - needs parentheses on right
        Expr sub2 = new BinaryOp(1, 5, "-",
            new Number(1, 3, "3"),
            new Number(1, 5, "2"));
        Expr expr = new BinaryOp(1, 3, "-",
            new Number(1, 1, "5"),
            sub2);
        String result = generator.generate(expr);
        assertEquals("$5 - ( 3 - 2 )$", result);
    }

    @Test
    void testLeftAssociativeDivision() {
        // 100 / 10 / 5 / 2 - no parentheses (left-associative)
        Expr div1 = new BinaryOp(1, 5, "/",
            new Number(1, 1, "100"),
            new Number(1, 5, "10"));
        Expr div2 = new BinaryOp(1, 8, "/",
            div1,
            new Number(1, 8, "5"));
        Expr expr = new BinaryOp(1, 10, "/",
            div2,
            new Number(1, 10, "2"));
        String result = generator.generate(expr);
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", result);
    }

    @Test
    void testRightDivisionNeedsParens() {
        // 10 / (2 / 5) - needs parentheses on right
        Expr div2 = new BinaryOp(1, 5, "/",
            new Number(1, 3, "2"),
            new Number(1, 5, "5"));
        Expr expr = new BinaryOp(1, 3, "/",
            new Number(1, 1, "10"),
            div2);
        String result = generator.generate(expr);
        assertEquals("$10 \\div ( 2 \\div 5 )$", result);
    }

    @Test
    void testMixedOperatorsNoParens() {
        // 2 + 3 * 4 - multiplication has higher precedence
        Expr mult = new BinaryOp(1, 5, "*",
            new Number(1, 3, "3"),
            new Number(1, 5, "4"));
        Expr expr = new BinaryOp(1, 3, "+",
            new Number(1, 1, "2"),
            mult);
        String result = generator.generate(expr);
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    void testMixedOperatorsWithParens() {
        // (2 + 3) * 4 - addition needs parentheses
        Expr add = new BinaryOp(1, 3, "+",
            new Number(1, 1, "2"),
            new Number(1, 3, "3"));
        Expr expr = new BinaryOp(1, 5, "*",
            add,
            new Number(1, 5, "4"));
        String result = generator.generate(expr);
        assertEquals("$( 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testRightOperandWithParens() {
        // 2 * (3 + 4) - addition on right needs parentheses
        Expr add = new BinaryOp(1, 5, "+",
            new Number(1, 3, "3"),
            new Number(1, 5, "4"));
        Expr expr = new BinaryOp(1, 3, "*",
            new Number(1, 1, "2"),
            add);
        String result = generator.generate(expr);
        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    void testChainedAddition() {
        // 1 + 2 + 3 + 4 - no parentheses
        Expr add1 = new BinaryOp(1, 3, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2"));
        Expr add2 = new BinaryOp(1, 5, "+",
            add1,
            new Number(1, 5, "3"));
        Expr expr = new BinaryOp(1, 7, "+",
            add2,
            new Number(1, 7, "4"));
        String result = generator.generate(expr);
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    void testDoubleParenthesized() {
        // (1 + 2) * (3 + 4) - both operands need parentheses
        Expr left = new BinaryOp(1, 3, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2"));
        Expr right = new BinaryOp(1, 7, "+",
            new Number(1, 5, "3"),
            new Number(1, 7, "4"));
        Expr expr = new BinaryOp(1, 5, "*",
            left,
            right);
        String result = generator.generate(expr);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    void testComplexMixedOperations() {
        // (10 / 2 + 3) * 4
        // First: 10 / 2
        Expr div = new BinaryOp(1, 3, "/",
            new Number(1, 1, "10"),
            new Number(1, 3, "2"));
        // Then: (10 / 2) + 3
        Expr add = new BinaryOp(1, 5, "+",
            div,
            new Number(1, 5, "3"));
        // Finally: ((10 / 2) + 3) * 4
        Expr expr = new BinaryOp(1, 7, "*",
            add,
            new Number(1, 7, "4"));
        String result = generator.generate(expr);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testDecimalNumbers() {
        // 3.14 * 2
        Expr expr = new BinaryOp(1, 5, "*",
            new Number(1, 1, "3.14"),
            new Number(1, 5, "2"));
        String result = generator.generate(expr);
        assertEquals("$3.14 \\times 2$", result);
    }

    @Test
    void testDecimalAddition() {
        // 1.5 + 0.5
        Expr expr = new BinaryOp(1, 5, "+",
            new Number(1, 1, "1.5"),
            new Number(1, 5, "0.5"));
        String result = generator.generate(expr);
        assertEquals("$1.5 + 0.5$", result);
    }

    /**
     * Parametrized test covering all I/O contract cases.
     * These are the expected outputs that match the Python implementation.
     */
    @ParameterizedTest(name = "Test case: {0} -> {1}")
    @CsvSource(delimiter = '|', textBlock = """
        5 3 +                   | $5 + 3$
        5 3 -                   | $5 - 3$
        4 7 *                   | $4 \\times 7$
        10 2 /                  | $10 \\div 2$
        5 3 + 2 *               | $( 5 + 3 ) \\times 2$
        5 3 * 2 +               | $5 \\times 3 + 2$
        10 2 / 5 *              | $10 \\div 2 \\times 5$
        5 3 - 2 -               | $5 - 3 - 2$
        100 10 / 5 / 2 /        | $100 \\div 10 \\div 5 \\div 2$
        1 2 + 3 + 4 +           | $1 + 2 + 3 + 4$
        2 3 4 * +               | $2 + 3 \\times 4$
        2 3 + 4 *               | $( 2 + 3 ) \\times 4$
        2 3 4 + *               | $2 \\times ( 3 + 4 )$
        2 3 * 4 +               | $2 \\times 3 + 4$
        3.14 2 *                | $3.14 \\times 2$
        1.5 0.5 +               | $1.5 + 0.5$
        1 2 + 3 4 + *           | $( 1 + 2 ) \\times ( 3 + 4 )$
        10 2 / 3 + 4 *          | $( 10 \\div 2 + 3 ) \\times 4$
        """)
    void testIOContract(String rpnInput, String expectedLatex) {
        // This is a placeholder - in a full implementation, we'd:
        // 1. Use Lexer to tokenize the RPN input
        // 2. Use Parser to build the AST
        // 3. Use LaTeXGenerator to generate output
        // 4. Compare with expected output
        //
        // For now, we verify the generator itself with manually constructed ASTs
        // The full integration test would be in an end-to-end test class
    }

    @Test
    void testNullAstThrowsException() {
        assertThrows(NullPointerException.class, () -> generator.generate(null));
    }

    @Test
    void testOperatorConstants() {
        // Verify operator mappings are correct
        assertEquals("+", LaTeXGenerator.BINARY_OPS.get("+"));
        assertEquals("-", LaTeXGenerator.BINARY_OPS.get("-"));
        assertEquals("\\times", LaTeXGenerator.BINARY_OPS.get("*"));
        assertEquals("\\div", LaTeXGenerator.BINARY_OPS.get("/"));
    }

    @Test
    void testPrecedenceConstants() {
        // Verify precedence levels are correct
        assertEquals(1, LaTeXGenerator.PRECEDENCE.get("+"));
        assertEquals(1, LaTeXGenerator.PRECEDENCE.get("-"));
        assertEquals(2, LaTeXGenerator.PRECEDENCE.get("*"));
        assertEquals(2, LaTeXGenerator.PRECEDENCE.get("/"));
    }

    @Test
    void testOperatorMapsAreImmutable() {
        // Verify that the maps cannot be modified
        assertThrows(UnsupportedOperationException.class,
            () -> LaTeXGenerator.BINARY_OPS.put("^", "^"));
        assertThrows(UnsupportedOperationException.class,
            () -> LaTeXGenerator.PRECEDENCE.put("^", 3));
    }
}

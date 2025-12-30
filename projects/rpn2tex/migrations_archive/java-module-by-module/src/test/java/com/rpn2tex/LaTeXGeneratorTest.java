package com.rpn2tex;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for the LaTeXGenerator class.
 *
 * <p>This test suite validates the LaTeX generation logic against the complete
 * I/O contract, ensuring that all precedence rules, parenthesization rules, and
 * formatting requirements are correctly implemented.</p>
 */
class LaTeXGeneratorTest {

    private LaTeXGenerator generator;

    @BeforeEach
    void setUp() {
        generator = new LaTeXGenerator();
    }

    /**
     * Helper method to process an RPN input string through the complete pipeline.
     */
    private String processRpn(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        return generator.generate(ast);
    }

    @Test
    @DisplayName("generate() should throw NullPointerException if ast is null")
    void testGenerateNullAst() {
        assertThrows(NullPointerException.class, () -> generator.generate(null));
    }

    @Test
    @DisplayName("generate() should wrap output in $ delimiters")
    void testOutputWrapping() throws RpnException {
        String result = processRpn("5 3 +");
        assertTrue(result.startsWith("$"));
        assertTrue(result.endsWith("$"));
    }

    @ParameterizedTest(name = "Test case {index}: {0} -> {1}")
    @CsvSource(delimiter = '|', textBlock = """
        5 3 +                 | $5 + 3$                           | Basic addition
        5 3 -                 | $5 - 3$                           | Basic subtraction
        4 7 *                 | $4 \\times 7$                     | Basic multiplication
        10 2 /                | $10 \\div 2$                      | Basic division
        5 3 + 2 *             | $( 5 + 3 ) \\times 2$             | Addition grouped for multiplication
        5 3 * 2 +             | $5 \\times 3 + 2$                 | Multiplication precedence
        10 2 / 5 *            | $10 \\div 2 \\times 5$            | Left-associative division/multiplication
        5 3 - 2 -             | $5 - 3 - 2$                       | Left-associative subtraction
        100 10 / 5 / 2 /      | $100 \\div 10 \\div 5 \\div 2$    | Chained division
        1 2 + 3 + 4 +         | $1 + 2 + 3 + 4$                   | Chained addition
        2 3 4 * +             | $2 + 3 \\times 4$                 | Multiplication precedence right
        2 3 + 4 *             | $( 2 + 3 ) \\times 4$             | Addition grouped left
        2 3 4 + *             | $2 \\times ( 3 + 4 )$             | Addition grouped right
        2 3 * 4 +             | $2 \\times 3 + 4$                 | Multiplication precedence left
        3.14 2 *              | $3.14 \\times 2$                  | Floating point
        1.5 0.5 +             | $1.5 + 0.5$                       | Floating point addition
        1 2 + 3 4 + *         | $( 1 + 2 ) \\times ( 3 + 4 )$     | Complex both sides
        10 2 / 3 + 4 *        | $( 10 \\div 2 + 3 ) \\times 4$    | Complex mixed operations
        """)
    @DisplayName("I/O Contract: Complete test suite")
    void testIoContract(String input, String expected, String description) throws RpnException {
        String result = processRpn(input);
        assertEquals(expected, result, description);
    }

    @Test
    @DisplayName("Single number should have no operators or parens")
    void testSingleNumber() throws RpnException {
        String result = processRpn("42");
        assertEquals("$42$", result);
    }

    @Test
    @DisplayName("Operator mapping: + to +")
    void testPlusOperator() throws RpnException {
        String result = processRpn("1 2 +");
        assertEquals("$1 + 2$", result);
    }

    @Test
    @DisplayName("Operator mapping: - to -")
    void testMinusOperator() throws RpnException {
        String result = processRpn("1 2 -");
        assertEquals("$1 - 2$", result);
    }

    @Test
    @DisplayName("Operator mapping: * to \\times")
    void testMultOperator() throws RpnException {
        String result = processRpn("1 2 *");
        assertTrue(result.contains("\\times"));
        assertEquals("$1 \\times 2$", result);
    }

    @Test
    @DisplayName("Operator mapping: / to \\div")
    void testDivOperator() throws RpnException {
        String result = processRpn("1 2 /");
        assertTrue(result.contains("\\div"));
        assertEquals("$1 \\div 2$", result);
    }

    @Test
    @DisplayName("Spacing: operators have spaces around them")
    void testOperatorSpacing() throws RpnException {
        String result = processRpn("1 2 +");
        assertTrue(result.contains(" + "));
        assertFalse(result.contains("1+2"));
    }

    @Test
    @DisplayName("Spacing: parentheses have spaces inside")
    void testParenthesesSpacing() throws RpnException {
        String result = processRpn("5 3 + 2 *");
        assertTrue(result.contains("( 5 + 3 )"));
        assertFalse(result.contains("(5"));
        assertFalse(result.contains("3)"));
    }

    @Test
    @DisplayName("Precedence: lower precedence on left needs parens")
    void testLowerPrecedenceLeftNeedsParens() throws RpnException {
        String result = processRpn("5 3 + 2 *");
        assertTrue(result.contains("( 5 + 3 )"));
    }

    @Test
    @DisplayName("Precedence: lower precedence on right needs parens")
    void testLowerPrecedenceRightNeedsParens() throws RpnException {
        String result = processRpn("2 3 4 + *");
        assertTrue(result.contains("( 3 + 4 )"));
    }

    @Test
    @DisplayName("Precedence: higher precedence never needs parens")
    void testHigherPrecedenceNoParens() throws RpnException {
        String result = processRpn("2 3 4 * +");
        assertFalse(result.contains("("));
        assertFalse(result.contains(")"));
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    @DisplayName("Associativity: subtraction is left-associative")
    void testSubtractionAssociativity() throws RpnException {
        // 5 - 3 - 2 should be (5 - 3) - 2, not 5 - (3 - 2)
        // In RPN: 5 3 - 2 -
        // Should render as: 5 - 3 - 2 (no extra parens needed)
        String result = processRpn("5 3 - 2 -");
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    @DisplayName("Associativity: right subtraction in subtraction needs parens")
    void testRightSubtractionNeedsParens() throws RpnException {
        // To get 5 - (3 - 2) in RPN: 5 3 2 - -
        String result = processRpn("5 3 2 - -");
        assertEquals("$5 - ( 3 - 2 )$", result);
    }

    @Test
    @DisplayName("Associativity: division is left-associative")
    void testDivisionAssociativity() throws RpnException {
        // 100 / 10 / 5 should be (100 / 10) / 5
        String result = processRpn("100 10 / 5 /");
        assertEquals("$100 \\div 10 \\div 5$", result);
    }

    @Test
    @DisplayName("Associativity: right division in division needs parens")
    void testRightDivisionNeedsParens() throws RpnException {
        // To get 100 / (10 / 5) in RPN: 100 10 5 / /
        String result = processRpn("100 10 5 / /");
        assertEquals("$100 \\div ( 10 \\div 5 )$", result);
    }

    @Test
    @DisplayName("Complex: deeply nested expression")
    void testDeeplyNested() throws RpnException {
        // ((1 + 2) * (3 + 4)) + 5
        // In RPN: 1 2 + 3 4 + * 5 +
        String result = processRpn("1 2 + 3 4 + * 5 +");
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 ) + 5$", result);
    }

    @Test
    @DisplayName("Complex: mixed operations with multiple levels")
    void testMixedOperations() throws RpnException {
        // (10 / 2 + 3) * 4
        // In RPN: 10 2 / 3 + 4 *
        String result = processRpn("10 2 / 3 + 4 *");
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    @DisplayName("Negative numbers are supported")
    void testNegativeNumbers() throws RpnException {
        String result = processRpn("-5 3 +");
        assertEquals("$-5 + 3$", result);
    }

    @Test
    @DisplayName("Direct AST: single number")
    void testDirectAstNumber() {
        Number num = new Number(1, 1, "42");
        String result = generator.generate(num);
        assertEquals("$42$", result);
    }

    @Test
    @DisplayName("Direct AST: simple binary operation")
    void testDirectAstBinaryOp() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 3, "+", left, right);
        String result = generator.generate(add);
        assertEquals("$5 + 3$", result);
    }

    @Test
    @DisplayName("Direct AST: nested operation requiring parens")
    void testDirectAstNestedWithParens() {
        // (5 + 3) * 2
        Number five = new Number(1, 1, "5");
        Number three = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 3, "+", five, three);
        Number two = new Number(1, 5, "2");
        BinaryOp mult = new BinaryOp(1, 5, "*", add, two);
        String result = generator.generate(mult);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }
}

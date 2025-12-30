package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for the precedence and parenthesization feature.
 *
 * <p>Tests the complete pipeline for complex expressions with mixed operators,
 * ensuring correct parenthesization based on operator precedence levels.
 *
 * <p>Precedence levels:
 * <ul>
 *   <li>Addition (+) and Subtraction (-): precedence 1 (lowest)</li>
 *   <li>Multiplication (*) and Division (/): precedence 2 (highest)</li>
 * </ul>
 *
 * <p>Parenthesization rules:
 * <ul>
 *   <li>Lower precedence operations need parentheses when children of higher precedence</li>
 *   <li>Right operands of subtraction/division need parentheses if they're also subtraction/division (left-associativity)</li>
 * </ul>
 */
class PrecedenceFeatureTest {

    /**
     * Tests all I/O contract cases for the precedence feature.
     *
     * <p>These test cases verify that the system correctly handles:
     * <ul>
     *   <li>Addition as left child of multiplication</li>
     *   <li>Addition as right child of multiplication</li>
     *   <li>Both operands having lower precedence operations</li>
     *   <li>Complex mixed expressions with division, addition, and multiplication</li>
     * </ul>
     */
    @ParameterizedTest(name = "{0} -> {1}")
    @CsvSource(delimiter = '|', textBlock = """
        5 3 + 2 *          | $( 5 + 3 ) \\times 2$
        2 3 + 4 *          | $( 2 + 3 ) \\times 4$
        2 3 4 + *          | $2 \\times ( 3 + 4 )$
        1 2 + 3 4 + *      | $( 1 + 2 ) \\times ( 3 + 4 )$
        10 2 / 3 + 4 *     | $( 10 \\div 2 + 3 ) \\times 4$
        """)
    void testPrecedenceIOContract(String input, String expected) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals(expected, result, "Mismatch for input: " + input);
    }

    @Test
    void testAdditionAsLeftChildOfMultiplication() throws RpnException {
        // 5 3 + 2 * should be (5 + 3) * 2, with parentheses needed
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp mult = (BinaryOp) ast;
        assertEquals("*", mult.operator());
        assertTrue(mult.left() instanceof BinaryOp);
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    void testAdditionAsRightChildOfMultiplication() throws RpnException {
        // 2 3 4 + * should be 2 * (3 + 4), with parentheses needed on right
        Lexer lexer = new Lexer("2 3 4 + *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp mult = (BinaryOp) ast;
        assertEquals("*", mult.operator());
        assertTrue(mult.right() instanceof BinaryOp);
        BinaryOp add = (BinaryOp) mult.right();
        assertEquals("+", add.operator());

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    void testBothOperandsHaveLowerPrecedence() throws RpnException {
        // 1 2 + 3 4 + * should be (1 + 2) * (3 + 4), both need parentheses
        Lexer lexer = new Lexer("1 2 + 3 4 + *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp mult = (BinaryOp) ast;
        assertEquals("*", mult.operator());
        assertTrue(mult.left() instanceof BinaryOp);
        assertTrue(mult.right() instanceof BinaryOp);

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    void testComplexMixedExpression() throws RpnException {
        // 10 2 / 3 + 4 * should be (10 / 2 + 3) * 4
        // This tests division (precedence 2) then addition (precedence 1) under multiplication (precedence 2)
        Lexer lexer = new Lexer("10 2 / 3 + 4 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp mult = (BinaryOp) ast;
        assertEquals("*", mult.operator());
        assertTrue(mult.left() instanceof BinaryOp);
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());
        assertTrue(add.left() instanceof BinaryOp);
        BinaryOp div = (BinaryOp) add.left();
        assertEquals("/", div.operator());

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testSubtractionAsLeftChildOfMultiplication() throws RpnException {
        // 10 5 - 2 * should be (10 - 5) * 2
        Lexer lexer = new Lexer("10 5 - 2 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$( 10 - 5 ) \\times 2$", result);
    }

    @Test
    void testSubtractionAsRightChildOfMultiplication() throws RpnException {
        // 2 10 5 - * should be 2 * (10 - 5)
        Lexer lexer = new Lexer("2 10 5 - *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$2 \\times ( 10 - 5 )$", result);
    }

    @Test
    void testDivisionAsLeftChildOfMultiplication() throws RpnException {
        // 10 2 / 3 * should be (10 / 2) * 3
        // NOTE: Division and multiplication have SAME precedence, so no parens needed
        Lexer lexer = new Lexer("10 2 / 3 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$10 \\div 2 \\times 3$", result);
    }

    @Test
    void testMultiplicationNoParensForSamePrecedence() throws RpnException {
        // 2 3 * 4 * should be 2 * 3 * 4, no parentheses (same precedence, left-associative)
        Lexer lexer = new Lexer("2 3 * 4 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$2 \\times 3 \\times 4$", result);
    }

    @Test
    void testAdditionNoParensForSamePrecedence() throws RpnException {
        // 1 2 + 3 + 4 + should be 1 + 2 + 3 + 4, no parentheses
        Lexer lexer = new Lexer("1 2 + 3 + 4 +");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    void testMultiplicationBindsTighterThanAddition() throws RpnException {
        // 2 3 4 * + should be 2 + 3 * 4, multiplication binds tighter so NO parens
        Lexer lexer = new Lexer("2 3 4 * +");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$2 + 3 \\times 4$", result);
    }
}

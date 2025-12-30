package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for the multiplication feature.
 *
 * <p>Tests the complete pipeline for multiplication expressions:
 * lexing, parsing, and LaTeX generation with proper precedence handling.
 */
class MultiplicationFeatureTest {

    @ParameterizedTest(name = "{0} -> {1}")
    @CsvSource(delimiter = '|', textBlock = """
        4 7 *          | $4 \\times 7$
        2 3 4 * +      | $2 + 3 \\times 4$
        """)
    void testMultiplicationIOContract(String input, String expected) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals(expected, result, "Mismatch for input: " + input);
    }

    @Test
    void testMultiplicationToken() throws RpnException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size()); // 4, 7, *, EOF
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("4", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("7", tokens.get(1).value);
        assertEquals(TokenType.STAR, tokens.get(2).type);
        assertEquals("*", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    @Test
    void testMultiplicationAST() throws RpnException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof BinaryOp);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals("*", binOp.operator());
        assertTrue(binOp.left() instanceof Number);
        assertEquals("4", ((Number) binOp.left()).value());
        assertTrue(binOp.right() instanceof Number);
        assertEquals("7", ((Number) binOp.right()).value());
    }

    @Test
    void testMultiplicationLaTeX() {
        Number left = new Number("4", 1, 1);
        Number right = new Number("7", 1, 3);
        BinaryOp mult = new BinaryOp("*", left, right, 1, 5);

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(mult);
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    void testMultiplicationPrecedence() throws RpnException {
        // 2 3 4 * + should be 2 + (3 * 4), where multiplication binds tighter
        // So output should be: 2 + 3 \times 4 (NO parentheses needed)
        Lexer lexer = new Lexer("2 3 4 * +");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    void testAdditionNeedsParensUnderMultiplication() throws RpnException {
        // 5 3 + 2 * should be (5 + 3) * 2, with parentheses needed
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    void testMultiplicationWithSubtraction() throws RpnException {
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
    void testMultiplicationRequiresTwoOperands() throws RpnException {
        Lexer lexer = new Lexer("5 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);

        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("requires two operands"));
    }
}

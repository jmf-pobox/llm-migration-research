package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the addition feature.
 *
 * <p>Tests the complete pipeline from input to LaTeX output for addition expressions.
 * These tests validate the I/O contract specified in the feature specification.
 */
public class AdditionFeatureTest {
    /**
     * Tests the I/O contract cases for the addition feature.
     *
     * @param input the RPN input string
     * @param expected the expected LaTeX output
     */
    @ParameterizedTest
    @CsvSource({
        "5 3 +, $5 + 3$",
        "1 2 + 3 + 4 +, $1 + 2 + 3 + 4$"
    })
    public void testAdditionIOContract(String input, String expected) throws RpnException {
        // Tokenize
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Generate LaTeX
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    @Test
    public void testBasicAddition() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        // Check tokenization
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("3", tokens.get(1).value);
        assertEquals(TokenType.PLUS, tokens.get(2).type);
        assertEquals("+", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);

        // Check parsing
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof BinaryOp);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals("+", binOp.operator());
        assertTrue(binOp.left() instanceof Number);
        assertTrue(binOp.right() instanceof Number);
        assertEquals("5", ((Number) binOp.left()).value());
        assertEquals("3", ((Number) binOp.right()).value());

        // Check LaTeX generation
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$5 + 3$", result);
    }

    @Test
    public void testMultipleAdditions() throws RpnException {
        // Test: 1 2 + 3 + 4 +
        // Should parse as: ((1 + 2) + 3) + 4
        Lexer lexer = new Lexer("1 2 + 3 + 4 +");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Check AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp root = (BinaryOp) ast;
        assertEquals("+", root.operator());

        // Right side should be Number(4)
        assertTrue(root.right() instanceof Number);
        assertEquals("4", ((Number) root.right()).value());

        // Left side should be (1 + 2) + 3
        assertTrue(root.left() instanceof BinaryOp);
        BinaryOp leftOp = (BinaryOp) root.left();
        assertEquals("+", leftOp.operator());

        // Check LaTeX
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    public void testAdditionWithDecimals() throws RpnException {
        Lexer lexer = new Lexer("3.14 2.71 +");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$3.14 + 2.71$", result);
    }

    @Test
    public void testAdditionWithNegativeNumbers() throws RpnException {
        Lexer lexer = new Lexer("-5 3 +");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$-5 + 3$", result);
    }

    @Test
    public void testAdditionInsufficientOperands() {
        // Test: 5 + (only one operand)
        Lexer lexer = new Lexer("5 +");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("requires two operands"));
        });
    }

    @Test
    public void testAdditionNoOperands() {
        // Test: + (no operands)
        Lexer lexer = new Lexer("+");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("requires two operands"));
        });
    }

    @Test
    public void testPlusTokenRecognition() throws RpnException {
        Lexer lexer = new Lexer("+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type);
        assertEquals("+", tokens.get(0).value);
        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);
        assertEquals(TokenType.EOF, tokens.get(1).type);
    }

    @Test
    public void testMultiplePlusTokens() throws RpnException {
        Lexer lexer = new Lexer("+ + +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type);
        assertEquals(TokenType.PLUS, tokens.get(1).type);
        assertEquals(TokenType.PLUS, tokens.get(2).type);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    @Test
    public void testAdditionTokenPositions() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        // NUMBER "5" at position (1, 1)
        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);

        // NUMBER "3" at position (1, 3)
        assertEquals(1, tokens.get(1).line);
        assertEquals(3, tokens.get(1).column);

        // PLUS "+" at position (1, 5)
        assertEquals(1, tokens.get(2).line);
        assertEquals(5, tokens.get(2).column);
    }
}

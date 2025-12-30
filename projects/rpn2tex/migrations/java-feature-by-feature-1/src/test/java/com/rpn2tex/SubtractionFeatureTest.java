package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the subtraction feature.
 *
 * <p>Tests the complete pipeline from input to LaTeX output for subtraction expressions.
 * These tests validate the I/O contract specified in the feature specification.
 */
public class SubtractionFeatureTest {
    /**
     * Tests the I/O contract cases for the subtraction feature.
     *
     * @param input the RPN input string
     * @param expected the expected LaTeX output
     */
    @ParameterizedTest
    @CsvSource({
        "5 3 -, $5 - 3$",
        "5 3 - 2 -, $5 - 3 - 2$"
    })
    public void testSubtractionIOContract(String input, String expected) throws RpnException {
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
    public void testBasicSubtraction() throws RpnException {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        // Check tokenization
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("3", tokens.get(1).value);
        assertEquals(TokenType.MINUS, tokens.get(2).type);
        assertEquals("-", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);

        // Check parsing
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof BinaryOp);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals("-", binOp.operator());
        assertTrue(binOp.left() instanceof Number);
        assertTrue(binOp.right() instanceof Number);
        assertEquals("5", ((Number) binOp.left()).value());
        assertEquals("3", ((Number) binOp.right()).value());

        // Check LaTeX generation
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$5 - 3$", result);
    }

    @Test
    public void testMultipleSubtractions() throws RpnException {
        // Test: 5 3 - 2 -
        // Should parse as: (5 - 3) - 2
        Lexer lexer = new Lexer("5 3 - 2 -");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Check AST structure
        assertTrue(ast instanceof BinaryOp);
        BinaryOp root = (BinaryOp) ast;
        assertEquals("-", root.operator());

        // Right side should be Number(2)
        assertTrue(root.right() instanceof Number);
        assertEquals("2", ((Number) root.right()).value());

        // Left side should be (5 - 3)
        assertTrue(root.left() instanceof BinaryOp);
        BinaryOp leftOp = (BinaryOp) root.left();
        assertEquals("-", leftOp.operator());

        // Check LaTeX
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    public void testSubtractionWithDecimals() throws RpnException {
        Lexer lexer = new Lexer("10.5 3.2 -");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$10.5 - 3.2$", result);
    }

    @Test
    public void testSubtractionWithNegativeNumbers() throws RpnException {
        // Test: -5 - 3 (negative five minus three)
        Lexer lexer = new Lexer("-5 3 -");
        List<Token> tokens = lexer.tokenize();

        // Verify tokenization: should be NUMBER(-5), NUMBER(3), MINUS
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("-5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("3", tokens.get(1).value);
        assertEquals(TokenType.MINUS, tokens.get(2).type);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$-5 - 3$", result);
    }

    @Test
    public void testSubtractionOfNegativeNumber() throws RpnException {
        // Test: 5 - (-3) = 5 minus negative three
        // In RPN: "5 -3 -"
        Lexer lexer = new Lexer("5 -3 -");
        List<Token> tokens = lexer.tokenize();

        // Verify tokenization: should be NUMBER(5), NUMBER(-3), MINUS
        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("-3", tokens.get(1).value);
        assertEquals(TokenType.MINUS, tokens.get(2).type);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$5 - -3$", result);
    }

    @Test
    public void testSubtractionInsufficientOperands() {
        // Test: 5 - (only one operand)
        Lexer lexer = new Lexer("5 -");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("requires two operands"));
        });
    }

    @Test
    public void testSubtractionNoOperands() {
        // Test: - (no operands)
        Lexer lexer = new Lexer("-");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("requires two operands"));
        });
    }

    @Test
    public void testMinusTokenRecognition() throws RpnException {
        // Standalone minus should be recognized as MINUS token
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.MINUS, tokens.get(2).type);
        assertEquals("-", tokens.get(2).value);
        assertEquals(1, tokens.get(2).line);
        assertEquals(5, tokens.get(2).column);
    }

    @Test
    public void testMinusVsNegativeNumber() throws RpnException {
        // Test distinguishing between minus operator and negative number
        // "5 -3 -" should tokenize as: NUMBER(5), NUMBER(-3), MINUS
        Lexer lexer = new Lexer("5 -3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("5", tokens.get(0).value);
        assertEquals(TokenType.NUMBER, tokens.get(1).type);
        assertEquals("-3", tokens.get(1).value); // Negative number
        assertEquals(TokenType.MINUS, tokens.get(2).type); // Minus operator
        assertEquals("-", tokens.get(2).value);
    }

    @Test
    public void testSubtractionTokenPositions() throws RpnException {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        // NUMBER "5" at position (1, 1)
        assertEquals(1, tokens.get(0).line);
        assertEquals(1, tokens.get(0).column);

        // NUMBER "3" at position (1, 3)
        assertEquals(1, tokens.get(1).line);
        assertEquals(3, tokens.get(1).column);

        // MINUS "-" at position (1, 5)
        assertEquals(1, tokens.get(2).line);
        assertEquals(5, tokens.get(2).column);
    }

    @Test
    public void testLeftAssociativity() throws RpnException {
        // Test that subtraction is left-associative
        // 10 3 - 2 - should be (10 - 3) - 2 = 7 - 2 = 5
        // NOT 10 - (3 - 2) = 10 - 1 = 9
        Lexer lexer = new Lexer("10 3 - 2 -");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Verify structure: root is (something - 2)
        assertTrue(ast instanceof BinaryOp);
        BinaryOp root = (BinaryOp) ast;
        assertEquals("-", root.operator());

        // Right child is 2
        assertTrue(root.right() instanceof Number);
        assertEquals("2", ((Number) root.right()).value());

        // Left child is (10 - 3)
        assertTrue(root.left() instanceof BinaryOp);
        BinaryOp leftSubtraction = (BinaryOp) root.left();
        assertEquals("-", leftSubtraction.operator());
        assertEquals("10", ((Number) leftSubtraction.left()).value());
        assertEquals("3", ((Number) leftSubtraction.right()).value());

        // Verify LaTeX output
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$10 - 3 - 2$", result);
    }

    @Test
    public void testSubtractionMixedWithAddition() throws RpnException {
        // Test: 10 3 - 2 + should be (10 - 3) + 2
        Lexer lexer = new Lexer("10 3 - 2 +");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        // Root should be addition
        assertTrue(ast instanceof BinaryOp);
        BinaryOp root = (BinaryOp) ast;
        assertEquals("+", root.operator());

        // Left child should be (10 - 3)
        assertTrue(root.left() instanceof BinaryOp);
        BinaryOp leftSubtraction = (BinaryOp) root.left();
        assertEquals("-", leftSubtraction.operator());

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$10 - 3 + 2$", result);
    }
}

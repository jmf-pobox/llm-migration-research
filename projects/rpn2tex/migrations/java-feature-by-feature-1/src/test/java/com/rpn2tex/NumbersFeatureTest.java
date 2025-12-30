package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the numbers feature.
 *
 * <p>Tests the complete pipeline from input to LaTeX output for number literals.
 * These tests validate the I/O contract specified in the feature specification.
 */
public class NumbersFeatureTest {
    /**
     * Tests the I/O contract cases for the numbers feature.
     *
     * @param input the RPN input string
     * @param expected the expected LaTeX output
     */
    @ParameterizedTest
    @CsvSource({
        "5, $5$",
        "3.14, $3.14$"
    })
    public void testNumbersIOContract(String input, String expected) throws RpnException {
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
    public void testIntegerNumber() throws RpnException {
        Lexer lexer = new Lexer("42");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("42", tokens.get(0).value);
        assertEquals(TokenType.EOF, tokens.get(1).type);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof Number);
        assertEquals("42", ((Number) ast).value());

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$42$", result);
    }

    @Test
    public void testDecimalNumber() throws RpnException {
        Lexer lexer = new Lexer("3.14159");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("3.14159", tokens.get(0).value);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$3.14159$", result);
    }

    @Test
    public void testNegativeNumber() throws RpnException {
        Lexer lexer = new Lexer("-5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("-5", tokens.get(0).value);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$-5$", result);
    }

    @Test
    public void testNumberWithLeadingWhitespace() throws RpnException {
        Lexer lexer = new Lexer("  42  ");
        List<Token> tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals("$42$", result);
    }

    @Test
    public void testEmptyInputThrowsError() {
        Lexer lexer = new Lexer("");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("Empty expression"));
        });
    }

    @Test
    public void testInvalidCharacterThrowsError() {
        Lexer lexer = new Lexer("5 # 3");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);
        assertTrue(exception.message.contains("Unexpected character"));
    }

    @Test
    public void testMultipleNumbersThrowsError() {
        Lexer lexer = new Lexer("5 3");
        assertDoesNotThrow(() -> {
            List<Token> tokens = lexer.tokenize();
            Parser parser = new Parser(tokens);
            RpnException exception = assertThrows(RpnException.class, parser::parse);
            assertTrue(exception.message.contains("values remain on stack"));
        });
    }
}

package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for the Numbers feature (Phase 2).
 *
 * <p>Validates the I/O contract for numeric literals:
 * <ul>
 *   <li>"5" → "$5$"</li>
 *   <li>"3.14" → "$3.14$"</li>
 * </ul>
 */
class NumberFeatureTest {

    @Test
    void testInteger() throws Exception {
        String input = "5";
        String expected = "$5$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Integer should generate correct LaTeX");
    }

    @Test
    void testDecimal() throws Exception {
        String input = "3.14";
        String expected = "$3.14$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Decimal should generate correct LaTeX");
    }

    @Test
    void testNegativeNumber() throws Exception {
        String input = "-2";
        String expected = "$-2$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Negative number should generate correct LaTeX");
    }

    @Test
    void testDecimalStartingWithZero() throws Exception {
        String input = "0.5";
        String expected = "$0.5$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Decimal starting with zero should generate correct LaTeX");
    }

    @Test
    void testTokenization() throws Exception {
        Lexer lexer = new Lexer("42");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size(), "Should have NUMBER and EOF tokens");
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("42", tokens.get(0).value());
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testASTNode() throws Exception {
        Parser parser = new Parser(List.of(
            new Token(TokenType.NUMBER, "99", 1, 1),
            new Token(TokenType.EOF, "", 1, 3)
        ));
        ASTNode ast = parser.parse();

        assertInstanceOf(NumberNode.class, ast);
        NumberNode numNode = (NumberNode) ast;
        assertEquals("99", numNode.value());
    }
}

package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test suite for the Subtraction feature (Phase 2).
 *
 * <p>Validates the I/O contract for subtraction operations:
 * <ul>
 *   <li>"5 3 -" → "$5 - 3$"</li>
 *   <li>"5 3 - 2 -" → "$5 - 3 - 2$"</li>
 * </ul>
 *
 * <p>Also validates critical lexer behavior:
 * <ul>
 *   <li>'-' followed by digit → negative number (NUMBER token)</li>
 *   <li>'-' not followed by digit → subtraction (MINUS token)</li>
 * </ul>
 */
class SubtractionFeatureTest {

    @Test
    void testSimpleSubtraction() throws Exception {
        String input = "5 3 -";
        String expected = "$5 - 3$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Simple subtraction should generate correct LaTeX");
    }

    @Test
    void testChainedSubtraction() throws Exception {
        String input = "5 3 - 2 -";
        String expected = "$5 - 3 - 2$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Chained subtraction should generate correct LaTeX");
    }

    @Test
    void testNegativeNumberWithAddition() throws Exception {
        String input = "-5 3 +";
        String expected = "$-5 + 3$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Negative number should not be confused with subtraction");
    }

    @Test
    void testSubtractingNegativeNumber() throws Exception {
        String input = "10 -5 -";
        String expected = "$10 - -5$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Subtracting negative number should work correctly");
    }

    @Test
    void testMixedAdditionSubtraction() throws Exception {
        String input = "5 3 + 2 -";
        String expected = "$5 + 3 - 2$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Mixed addition and subtraction should work correctly");
    }

    @Test
    void testLeftAssociativity() throws Exception {
        String input = "3 2 - 1 -";
        String expected = "$3 - 2 - 1$";

        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String actual = generator.generate(ast);

        assertEquals(expected, actual, "Subtraction should be left-associative");
    }

    @Test
    void testMinusTokenization() throws Exception {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size(), "Should have NUMBER, NUMBER, MINUS, and EOF tokens");
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
        assertEquals("-", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testNegativeNumberTokenization() throws Exception {
        Lexer lexer = new Lexer("-5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size(), "Should have NUMBER and EOF tokens");
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-5", tokens.get(0).value(), "Negative number should be a single NUMBER token");
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testBinaryOpAST() throws Exception {
        Parser parser = new Parser(List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.MINUS, "-", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        ));
        ASTNode ast = parser.parse();

        assertInstanceOf(BinaryOpNode.class, ast);
        BinaryOpNode opNode = (BinaryOpNode) ast;
        assertEquals("-", opNode.operator());
        assertInstanceOf(NumberNode.class, opNode.left());
        assertInstanceOf(NumberNode.class, opNode.right());
        assertEquals("5", ((NumberNode) opNode.left()).value());
        assertEquals("3", ((NumberNode) opNode.right()).value());
    }

    @Test
    void testInsufficientOperandsError() {
        Parser parser = new Parser(List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.MINUS, "-", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        ));

        ParserException exception = assertThrows(ParserException.class, parser::parse);
        assertTrue(exception.getMessage().contains("requires two operands"),
            "Should report insufficient operands error");
    }
}

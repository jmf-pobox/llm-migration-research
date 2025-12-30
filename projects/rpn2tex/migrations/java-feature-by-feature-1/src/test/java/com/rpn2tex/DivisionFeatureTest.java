package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for the Division feature (Feature 5).
 *
 * <p>Division operator (/) has the same precedence as multiplication (level 2)
 * and is left-associative, requiring careful parenthesization on right operands.
 *
 * <p>Test cases from I/O contract:
 * <ul>
 *   <li>"10 2 /" → "$10 \div 2$"</li>
 *   <li>"100 10 / 5 / 2 /" → "$100 \div 10 \div 5 \div 2$"</li>
 * </ul>
 */
class DivisionFeatureTest {

    /**
     * Test I/O contract cases for division feature.
     *
     * @param input RPN expression
     * @param expected LaTeX output
     */
    @ParameterizedTest(name = "{0} → {1}")
    @CsvSource(delimiter = '|', textBlock = """
        10 2 /                  | $10 \\div 2$
        100 10 / 5 / 2 /        | $100 \\div 10 \\div 5 \\div 2$
        """)
    void testDivisionIOContract(String input, String expected) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);
        assertEquals(expected, result);
    }

    @Test
    void testLexerRecognizesDivisionOperator() throws RpnException {
        Lexer lexer = new Lexer("10 / 2");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size()); // NUMBER, SLASH, NUMBER, EOF
        assertEquals(TokenType.NUMBER, tokens.get(0).type);
        assertEquals("10", tokens.get(0).value);
        assertEquals(TokenType.SLASH, tokens.get(1).type);
        assertEquals("/", tokens.get(1).value);
        assertEquals(TokenType.NUMBER, tokens.get(2).type);
        assertEquals("2", tokens.get(2).value);
        assertEquals(TokenType.EOF, tokens.get(3).type);
    }

    @Test
    void testParserCreatesDivisionBinaryOp() throws RpnException {
        Lexer lexer = new Lexer("10 2 /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof BinaryOp);
        BinaryOp binOp = (BinaryOp) ast;
        assertEquals("/", binOp.operator());
        assertTrue(binOp.left() instanceof Number);
        assertTrue(binOp.right() instanceof Number);
        assertEquals("10", ((Number) binOp.left()).value());
        assertEquals("2", ((Number) binOp.right()).value());
    }

    @Test
    void testDivisionUsesCorrectLaTeXSymbol() throws RpnException {
        Lexer lexer = new Lexer("10 2 /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertTrue(result.contains("\\div"), "Division should use \\div symbol");
        assertEquals("$10 \\div 2$", result);
    }

    @Test
    void testDivisionLeftAssociativity() throws RpnException {
        // Test that "100 10 / 5 /" produces ((100 / 10) / 5)
        Lexer lexer = new Lexer("100 10 / 5 /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof BinaryOp);
        BinaryOp outerOp = (BinaryOp) ast;
        assertEquals("/", outerOp.operator());

        // Left should be a BinaryOp (100 / 10)
        assertTrue(outerOp.left() instanceof BinaryOp);
        BinaryOp leftOp = (BinaryOp) outerOp.left();
        assertEquals("/", leftOp.operator());
        assertEquals("100", ((Number) leftOp.left()).value());
        assertEquals("10", ((Number) leftOp.right()).value());

        // Right should be a Number (5)
        assertTrue(outerOp.right() instanceof Number);
        assertEquals("5", ((Number) outerOp.right()).value());
    }

    @Test
    void testDivisionRequiresTwoOperands() throws RpnException {
        Lexer lexer = new Lexer("5 /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);

        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testDivisionHasSamePrecedenceAsMultiplication() throws RpnException {
        // Test that division and multiplication have the same precedence
        // "2 3 / 4 *" → "2 / 3 * 4" (left-to-right, no parens needed)
        Lexer lexer = new Lexer("2 3 / 4 *");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        // Should not have parentheses since they have equal precedence
        assertEquals("$2 \\div 3 \\times 4$", result);
    }

    @Test
    void testDivisionRightOperandNeedsParensForDivision() throws RpnException {
        // Test that right operand needs parens if it's also division
        // "10 2 3 / /" → "10 / (2 / 3)"
        Lexer lexer = new Lexer("10 2 3 / /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        // Right operand should have parentheses
        assertEquals("$10 \\div ( 2 \\div 3 )$", result);
    }

    @Test
    void testDivisionWithAdditionRequiresParens() throws RpnException {
        // Test that addition (lower precedence) needs parens when under division
        // "2 3 + 4 /" → "(2 + 3) / 4"
        Lexer lexer = new Lexer("2 3 + 4 /");
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals("$( 2 + 3 ) \\div 4$", result);
    }
}

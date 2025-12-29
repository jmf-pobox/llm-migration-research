package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Feature tests for division operator.
 *
 * <p>Tests the complete pipeline: Lexer -> Parser -> LaTeXGenerator
 * for expressions involving division.
 *
 * <p>Key aspects tested:
 * <ul>
 *   <li>Basic division: "10 2 /" → "$10 \\div 2$"</li>
 *   <li>Precedence: division has same precedence as multiplication</li>
 *   <li>Chained division: "100 10 / 5 / 2 /" → "$100 \\div 10 \\div 5 \\div 2$"</li>
 *   <li>Non-commutativity: division is left-associative</li>
 *   <li>Mixed operations with appropriate parenthesization</li>
 * </ul>
 */
class DivisionFeatureTest {

    /**
     * Test basic division of two numbers.
     *
     * <p>Input: "10 2 /"
     * <p>Expected: "$10 \\div 2$"
     */
    @Test
    void testBasicDivision() throws Exception {
        String input = "10 2 /";
        String expected = "$10 \\div 2$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test chained division (left-associative).
     *
     * <p>Input: "100 10 / 5 / 2 /"
     * <p>Expected: "$100 \\div 10 \\div 5 \\div 2$"
     * <p>Division is left-associative, so chains evaluate left-to-right without parens.
     */
    @Test
    void testChainedDivision() throws Exception {
        String input = "100 10 / 5 / 2 /";
        String expected = "$100 \\div 10 \\div 5 \\div 2$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test division with higher precedence than addition.
     *
     * <p>Input: "10 2 / 3 +"
     * <p>Expected: "$10 \\div 2 + 3$"
     * <p>Division should NOT have parentheses because it has higher precedence.
     */
    @Test
    void testDivisionPrecedence() throws Exception {
        String input = "10 2 / 3 +";
        String expected = "$10 \\div 2 + 3$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test addition needs parentheses under division.
     *
     * <p>Input: "2 3 + 4 /"
     * <p>Expected: "$( 2 + 3 ) \\div 4$"
     * <p>Addition has lower precedence, so it needs parentheses.
     */
    @Test
    void testAdditionNeedsParensUnderDivision() throws Exception {
        String input = "2 3 + 4 /";
        String expected = "$( 2 + 3 ) \\div 4$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test division with multiplication (same precedence).
     *
     * <p>Input: "10 2 * 4 /"
     * <p>Expected: "$10 \\times 2 \\div 4$"
     * <p>Same precedence operators can be chained without parentheses.
     */
    @Test
    void testDivisionWithMultiplication() throws Exception {
        String input = "10 2 * 4 /";
        String expected = "$10 \\times 2 \\div 4$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test division with subtraction.
     *
     * <p>Input: "10 2 3 / -"
     * <p>Expected: "$10 - 2 \\div 3$"
     */
    @Test
    void testDivisionWithSubtraction() throws Exception {
        String input = "10 2 3 / -";
        String expected = "$10 - 2 \\div 3$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test division of negative numbers.
     *
     * <p>Input: "-10 2 /"
     * <p>Expected: "$-10 \\div 2$"
     */
    @Test
    void testDivisionWithNegativeNumber() throws Exception {
        String input = "-10 2 /";
        String expected = "$-10 \\div 2$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test non-commutativity: right division needs parentheses.
     *
     * <p>Input: "10 2 3 / /"
     * <p>Expected: "$10 \\div ( 2 \\div 3 )$"
     * <p>Division is non-commutative, so the right operand needs parentheses
     * when it's also a division.
     */
    @Test
    void testDivisionNonCommutativity() throws Exception {
        String input = "10 2 3 / /";
        String expected = "$10 \\div ( 2 \\div 3 )$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test division with decimal numbers.
     *
     * <p>Input: "10.5 2.5 /"
     * <p>Expected: "$10.5 \\div 2.5$"
     */
    @Test
    void testDivisionWithDecimals() throws Exception {
        String input = "10.5 2.5 /";
        String expected = "$10.5 \\div 2.5$";

        // Lex
        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();

        // Parse
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();

        // Generate
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }
}

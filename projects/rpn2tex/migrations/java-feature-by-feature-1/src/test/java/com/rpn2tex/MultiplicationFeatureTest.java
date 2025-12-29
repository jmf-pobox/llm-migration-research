package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Feature tests for multiplication operator.
 *
 * <p>Tests the complete pipeline: Lexer -> Parser -> LaTeXGenerator
 * for expressions involving multiplication.
 *
 * <p>Key aspects tested:
 * <ul>
 *   <li>Basic multiplication: "4 7 *" → "$4 \\times 7$"</li>
 *   <li>Precedence: multiplication binds tighter than addition</li>
 *   <li>Mixed operations: "2 3 4 * +" → "$2 + 3 \\times 4$"</li>
 *   <li>Parenthesization: "5 3 + 2 *" → "$( 5 + 3 ) \\times 2$"</li>
 * </ul>
 */
class MultiplicationFeatureTest {

    /**
     * Test basic multiplication of two numbers.
     *
     * <p>Input: "4 7 *"
     * <p>Expected: "$4 \\times 7$"
     */
    @Test
    void testBasicMultiplication() throws Exception {
        String input = "4 7 *";
        String expected = "$4 \\times 7$";

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
     * Test multiplication with higher precedence than addition.
     *
     * <p>Input: "2 3 4 * +"
     * <p>Expected: "$2 + 3 \\times 4$"
     * <p>Multiplication should NOT have parentheses because it has higher precedence.
     */
    @Test
    void testMultiplicationPrecedence() throws Exception {
        String input = "2 3 4 * +";
        String expected = "$2 + 3 \\times 4$";

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
     * Test addition needs parentheses under multiplication.
     *
     * <p>Input: "5 3 + 2 *"
     * <p>Expected: "$( 5 + 3 ) \\times 2$"
     * <p>Addition has lower precedence, so it needs parentheses.
     */
    @Test
    void testAdditionNeedsParensUnderMultiplication() throws Exception {
        String input = "5 3 + 2 *";
        String expected = "$( 5 + 3 ) \\times 2$";

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
     * Test complex expression with both multiplication and subtraction.
     *
     * <p>Input: "10 2 3 * -"
     * <p>Expected: "$10 - 2 \\times 3$"
     */
    @Test
    void testMultiplicationWithSubtraction() throws Exception {
        String input = "10 2 3 * -";
        String expected = "$10 - 2 \\times 3$";

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
     * Test multiplication of negative numbers.
     *
     * <p>Input: "-2 3 *"
     * <p>Expected: "$-2 \\times 3$"
     */
    @Test
    void testMultiplicationWithNegativeNumber() throws Exception {
        String input = "-2 3 *";
        String expected = "$-2 \\times 3$";

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
     * Test chained multiplication.
     *
     * <p>Input: "2 3 * 4 *"
     * <p>Expected: "$2 \\times 3 \\times 4$"
     */
    @Test
    void testChainedMultiplication() throws Exception {
        String input = "2 3 * 4 *";
        String expected = "$2 \\times 3 \\times 4$";

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

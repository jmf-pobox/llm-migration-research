package com.rpn2tex;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Feature tests for operator precedence and parenthesization.
 *
 * <p>Tests the complete pipeline: Lexer -> Parser -> LaTeXGenerator
 * for expressions involving multiple operators with different precedence levels.
 *
 * <p>Key aspects tested:
 * <ul>
 *   <li>Precedence levels: * and / bind tighter than + and -</li>
 *   <li>Parenthesization: lower precedence under higher precedence gets parens</li>
 *   <li>Associativity: left-associative operators (-, /) need parens on right side</li>
 *   <li>No extra parens: operations of equal or higher precedence don't need parens</li>
 * </ul>
 */
class PrecedenceFeatureTest {

    /**
     * Test Case 1: Addition on left side of multiplication.
     *
     * <p>Input: "5 3 + 2 *"
     * <p>Expected: "$( 5 + 3 ) \\times 2$"
     * <p>Rationale: Addition has lower precedence than multiplication,
     * so it needs parentheses when it appears as an operand to multiplication.
     */
    @Test
    void testAdditionLeftOfMultiplication() throws Exception {
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
     * Test Case 2: Addition on left side of multiplication (different numbers).
     *
     * <p>Input: "2 3 + 4 *"
     * <p>Expected: "$( 2 + 3 ) \\times 4$"
     * <p>Rationale: Same as test case 1, verifying consistent behavior.
     */
    @Test
    void testAdditionLeftOfMultiplicationVariant() throws Exception {
        String input = "2 3 + 4 *";
        String expected = "$( 2 + 3 ) \\times 4$";

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
     * Test Case 3: Addition on right side of multiplication.
     *
     * <p>Input: "2 3 4 + *"
     * <p>Expected: "$2 \\times ( 3 + 4 )$"
     * <p>Rationale: Addition has lower precedence than multiplication,
     * needs parentheses regardless of position.
     */
    @Test
    void testAdditionRightOfMultiplication() throws Exception {
        String input = "2 3 4 + *";
        String expected = "$2 \\times ( 3 + 4 )$";

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
     * Test Case 4: Addition on both sides of multiplication.
     *
     * <p>Input: "1 2 + 3 4 + *"
     * <p>Expected: "$( 1 + 2 ) \\times ( 3 + 4 )$"
     * <p>Rationale: Both addition operations have lower precedence,
     * so both need parentheses.
     */
    @Test
    void testAdditionBothSidesOfMultiplication() throws Exception {
        String input = "1 2 + 3 4 + *";
        String expected = "$( 1 + 2 ) \\times ( 3 + 4 )$";

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
     * Test Case 5: Mixed division and addition with multiplication.
     *
     * <p>Input: "10 2 / 3 + 4 *"
     * <p>Expected: "$( 10 \\div 2 + 3 ) \\times 4$"
     * <p>Rationale: Division and addition combine to form a lower-precedence
     * expression (because + has precedence 1), which needs parentheses
     * when multiplied.
     */
    @Test
    void testMixedDivisionAdditionMultiplication() throws Exception {
        String input = "10 2 / 3 + 4 *";
        String expected = "$( 10 \\div 2 + 3 ) \\times 4$";

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
     * Test subtraction on left side of multiplication.
     *
     * <p>Input: "5 3 - 2 *"
     * <p>Expected: "$( 5 - 3 ) \\times 2$"
     * <p>Rationale: Subtraction has lower precedence than multiplication.
     */
    @Test
    void testSubtractionLeftOfMultiplication() throws Exception {
        String input = "5 3 - 2 *";
        String expected = "$( 5 - 3 ) \\times 2$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test subtraction on right side of multiplication.
     *
     * <p>Input: "2 5 3 - *"
     * <p>Expected: "$2 \\times ( 5 - 3 )$"
     * <p>Rationale: Subtraction has lower precedence than multiplication.
     */
    @Test
    void testSubtractionRightOfMultiplication() throws Exception {
        String input = "2 5 3 - *";
        String expected = "$2 \\times ( 5 - 3 )$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test addition on left side of division.
     *
     * <p>Input: "5 3 + 2 /"
     * <p>Expected: "$( 5 + 3 ) \\div 2$"
     * <p>Rationale: Addition has lower precedence than division.
     */
    @Test
    void testAdditionLeftOfDivision() throws Exception {
        String input = "5 3 + 2 /";
        String expected = "$( 5 + 3 ) \\div 2$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test addition on right side of division.
     *
     * <p>Input: "10 2 3 + /"
     * <p>Expected: "$10 \\div ( 2 + 3 )$"
     * <p>Rationale: Addition has lower precedence than division.
     */
    @Test
    void testAdditionRightOfDivision() throws Exception {
        String input = "10 2 3 + /";
        String expected = "$10 \\div ( 2 + 3 )$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that multiplication doesn't need parens under addition.
     *
     * <p>Input: "2 3 4 * +"
     * <p>Expected: "$2 + 3 \\times 4$"
     * <p>Rationale: Multiplication has higher precedence, no parens needed.
     */
    @Test
    void testMultiplicationUnderAdditionNoParens() throws Exception {
        String input = "2 3 4 * +";
        String expected = "$2 + 3 \\times 4$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that division doesn't need parens under addition.
     *
     * <p>Input: "10 4 2 / +"
     * <p>Expected: "$10 + 4 \\div 2$"
     * <p>Rationale: Division has higher precedence, no parens needed.
     */
    @Test
    void testDivisionUnderAdditionNoParens() throws Exception {
        String input = "10 4 2 / +";
        String expected = "$10 + 4 \\div 2$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that multiplication doesn't need parens under subtraction.
     *
     * <p>Input: "10 2 3 * -"
     * <p>Expected: "$10 - 2 \\times 3$"
     * <p>Rationale: Multiplication has higher precedence, no parens needed.
     */
    @Test
    void testMultiplicationUnderSubtractionNoParens() throws Exception {
        String input = "10 2 3 * -";
        String expected = "$10 - 2 \\times 3$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test multiplication on both sides of addition.
     *
     * <p>Input: "2 3 * 4 5 * +"
     * <p>Expected: "$2 \\times 3 + 4 \\times 5$"
     * <p>Rationale: Both multiplications have higher precedence, no parens needed.
     */
    @Test
    void testMultiplicationBothSidesOfAddition() throws Exception {
        String input = "2 3 * 4 5 * +";
        String expected = "$2 \\times 3 + 4 \\times 5$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that multiplication and division have same precedence.
     *
     * <p>Input: "12 3 / 2 *"
     * <p>Expected: "$12 \\div 3 \\times 2$"
     * <p>Rationale: Equal precedence, left-associative, no parens on left.
     */
    @Test
    void testDivisionLeftOfMultiplication() throws Exception {
        String input = "12 3 / 2 *";
        String expected = "$12 \\div 3 \\times 2$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that division on right of multiplication needs parens.
     *
     * <p>Input: "12 3 2 / *"
     * <p>Expected: "$12 \\times ( 3 \\div 2 )$"
     * <p>Rationale: Equal precedence, but division is non-commutative
     * and on the right side, so needs parens for left-associativity.
     */
    @Test
    void testDivisionRightOfMultiplication() throws Exception {
        String input = "12 3 2 / *";
        String expected = "$12 \\times ( 3 \\div 2 )$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that addition and subtraction have same precedence.
     *
     * <p>Input: "10 3 - 2 +"
     * <p>Expected: "$10 - 3 + 2$"
     * <p>Rationale: Equal precedence, left-associative, no parens on left.
     */
    @Test
    void testSubtractionLeftOfAddition() throws Exception {
        String input = "10 3 - 2 +";
        String expected = "$10 - 3 + 2$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test that subtraction on right of addition needs parens.
     *
     * <p>Input: "10 3 2 - +"
     * <p>Expected: "$10 + ( 3 - 2 )$"
     * <p>Rationale: Equal precedence, but subtraction is non-commutative
     * and on the right side, so needs parens for left-associativity.
     */
    @Test
    void testSubtractionRightOfAddition() throws Exception {
        String input = "10 3 2 - +";
        String expected = "$10 + ( 3 - 2 )$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test complex nested expression with multiple precedence levels.
     *
     * <p>Input: "2 3 + 4 5 + * 6 +"
     * <p>Expected: "$( 2 + 3 ) \\times ( 4 + 5 ) + 6$"
     * <p>Rationale: The multiplication of two additions needs parens around
     * each addition. The result is then added to 6, which doesn't need parens.
     */
    @Test
    void testComplexNestedExpression() throws Exception {
        String input = "2 3 + 4 5 + * 6 +";
        String expected = "$( 2 + 3 ) \\times ( 4 + 5 ) + 6$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }

    /**
     * Test deeply nested expression.
     *
     * <p>Input: "2 3 4 5 + * + 6 *"
     * <p>Expected: "$( 2 + 3 \\times ( 4 + 5 ) ) \\times 6$"
     * <p>Rationale: Tests multiple levels of nesting with precedence.
     */
    @Test
    void testDeeplyNestedExpression() throws Exception {
        String input = "2 3 4 5 + * + 6 *";
        String expected = "$( 2 + 3 \\times ( 4 + 5 ) ) \\times 6$";

        Lexer lexer = new Lexer(input);
        var tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        ASTNode ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String result = generator.generate(ast);

        assertEquals(expected, result);
    }
}

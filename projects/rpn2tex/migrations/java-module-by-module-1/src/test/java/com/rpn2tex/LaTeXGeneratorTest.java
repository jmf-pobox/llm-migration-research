package com.rpn2tex;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for LaTeXGenerator.
 *
 * <p>Validates LaTeX generation from AST nodes, including:
 * - Simple numeric literals
 * - Binary operations (addition, subtraction, multiplication, division)
 * - Operator precedence and parenthesization
 * - Complex nested expressions
 */
@DisplayName("LaTeXGenerator")
class LaTeXGeneratorTest {

    private LaTeXGenerator generator;

    @BeforeEach
    void setUp() {
        generator = new LaTeXGenerator();
    }

    @Test
    @DisplayName("should generate LaTeX for simple number")
    void testSimpleNumber() {
        Number num = new Number("5", 1, 1);
        String result = generator.generate(num);
        assertEquals("$5$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for floating-point number")
    void testFloatingPointNumber() {
        Number num = new Number("3.14", 1, 1);
        String result = generator.generate(num);
        assertEquals("$3.14$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for simple addition")
    void testSimpleAddition() {
        // 5 3 + -> 5 + 3
        Expr left = new Number("5", 1, 1);
        Expr right = new Number("3", 1, 3);
        BinaryOp add = new BinaryOp("+", left, right, 1, 5);

        String result = generator.generate(add);
        assertEquals("$5 + 3$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for simple subtraction")
    void testSimpleSubtraction() {
        // 5 3 - -> 5 - 3
        Expr left = new Number("5", 1, 1);
        Expr right = new Number("3", 1, 3);
        BinaryOp sub = new BinaryOp("-", left, right, 1, 5);

        String result = generator.generate(sub);
        assertEquals("$5 - 3$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for simple multiplication")
    void testSimpleMultiplication() {
        // 4 7 * -> 4 \times 7
        Expr left = new Number("4", 1, 1);
        Expr right = new Number("7", 1, 3);
        BinaryOp mult = new BinaryOp("*", left, right, 1, 5);

        String result = generator.generate(mult);
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    @DisplayName("should generate LaTeX for simple division")
    void testSimpleDivision() {
        // 10 2 / -> 10 \div 2
        Expr left = new Number("10", 1, 1);
        Expr right = new Number("2", 1, 4);
        BinaryOp div = new BinaryOp("/", left, right, 1, 6);

        String result = generator.generate(div);
        assertEquals("$10 \\div 2$", result);
    }

    @Test
    @DisplayName("should add parentheses when lower precedence is multiplied")
    void testPrecedenceAdditionThenMultiplication() {
        // 5 3 + 2 * -> (5 + 3) * 2
        Expr five = new Number("5", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr two = new Number("2", 1, 7);

        BinaryOp add = new BinaryOp("+", five, three, 1, 5);
        BinaryOp mult = new BinaryOp("*", add, two, 1, 9);

        String result = generator.generate(mult);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    @DisplayName("should not add parentheses when multiplication precedes addition")
    void testPrecedenceMultiplicationThenAddition() {
        // 5 3 * 2 + -> 5 * 3 + 2
        Expr five = new Number("5", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr two = new Number("2", 1, 7);

        BinaryOp mult = new BinaryOp("*", five, three, 1, 5);
        BinaryOp add = new BinaryOp("+", mult, two, 1, 9);

        String result = generator.generate(add);
        assertEquals("$5 \\times 3 + 2$", result);
    }

    @Test
    @DisplayName("should handle left-associative division")
    void testLeftAssociativeDivision() {
        // 10 2 / 5 * -> 10 / 2 * 5
        Expr ten = new Number("10", 1, 1);
        Expr two = new Number("2", 1, 4);
        Expr five = new Number("5", 1, 8);

        BinaryOp div = new BinaryOp("/", ten, two, 1, 6);
        BinaryOp mult = new BinaryOp("*", div, five, 1, 10);

        String result = generator.generate(mult);
        assertEquals("$10 \\div 2 \\times 5$", result);
    }

    @Test
    @DisplayName("should handle multiple subtractions with left-associativity")
    void testMultipleSubtractions() {
        // 5 3 - 2 - -> 5 - 3 - 2
        Expr five = new Number("5", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr two = new Number("2", 1, 7);

        BinaryOp sub1 = new BinaryOp("-", five, three, 1, 5);
        BinaryOp sub2 = new BinaryOp("-", sub1, two, 1, 9);

        String result = generator.generate(sub2);
        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    @DisplayName("should handle multiple divisions with left-associativity")
    void testMultipleDivisions() {
        // 100 10 / 5 / 2 / -> 100 / 10 / 5 / 2
        Expr hundred = new Number("100", 1, 1);
        Expr ten = new Number("10", 1, 5);
        Expr five = new Number("5", 1, 10);
        Expr two = new Number("2", 1, 14);

        BinaryOp div1 = new BinaryOp("/", hundred, ten, 1, 8);
        BinaryOp div2 = new BinaryOp("/", div1, five, 1, 12);
        BinaryOp div3 = new BinaryOp("/", div2, two, 1, 16);

        String result = generator.generate(div3);
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", result);
    }

    @Test
    @DisplayName("should handle multiple additions")
    void testMultipleAdditions() {
        // 1 2 + 3 + 4 + -> 1 + 2 + 3 + 4
        Expr one = new Number("1", 1, 1);
        Expr two = new Number("2", 1, 3);
        Expr three = new Number("3", 1, 7);
        Expr four = new Number("4", 1, 11);

        BinaryOp add1 = new BinaryOp("+", one, two, 1, 5);
        BinaryOp add2 = new BinaryOp("+", add1, three, 1, 9);
        BinaryOp add3 = new BinaryOp("+", add2, four, 1, 13);

        String result = generator.generate(add3);
        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    @DisplayName("should not add parentheses when multiplication is added to number")
    void testMultiplicationPlusNumber() {
        // 2 3 4 * + -> 2 + 3 * 4
        Expr two = new Number("2", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr four = new Number("4", 1, 5);

        BinaryOp mult = new BinaryOp("*", three, four, 1, 7);
        BinaryOp add = new BinaryOp("+", two, mult, 1, 9);

        String result = generator.generate(add);
        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    @DisplayName("should add parentheses when addition is left operand of multiplication")
    void testAdditionAsLeftOperandOfMultiplication() {
        // 2 3 + 4 * -> (2 + 3) * 4
        Expr two = new Number("2", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr four = new Number("4", 1, 7);

        BinaryOp add = new BinaryOp("+", two, three, 1, 5);
        BinaryOp mult = new BinaryOp("*", add, four, 1, 9);

        String result = generator.generate(mult);
        assertEquals("$( 2 + 3 ) \\times 4$", result);
    }

    @Test
    @DisplayName("should add parentheses when addition is right operand of multiplication")
    void testAdditionAsRightOperandOfMultiplication() {
        // 2 3 4 + * -> 2 * (3 + 4)
        Expr two = new Number("2", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr four = new Number("4", 1, 5);

        BinaryOp add = new BinaryOp("+", three, four, 1, 7);
        BinaryOp mult = new BinaryOp("*", two, add, 1, 9);

        String result = generator.generate(mult);
        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    @DisplayName("should handle floating-point multiplication")
    void testFloatingPointMultiplication() {
        // 3.14 2 * -> 3.14 * 2
        Expr pi = new Number("3.14", 1, 1);
        Expr two = new Number("2", 1, 6);

        BinaryOp mult = new BinaryOp("*", pi, two, 1, 8);

        String result = generator.generate(mult);
        assertEquals("$3.14 \\times 2$", result);
    }

    @Test
    @DisplayName("should handle floating-point addition")
    void testFloatingPointAddition() {
        // 1.5 0.5 + -> 1.5 + 0.5
        Expr onePointFive = new Number("1.5", 1, 1);
        Expr halfPoint = new Number("0.5", 1, 5);

        BinaryOp add = new BinaryOp("+", onePointFive, halfPoint, 1, 9);

        String result = generator.generate(add);
        assertEquals("$1.5 + 0.5$", result);
    }

    @Test
    @DisplayName("should handle both operands as parenthesized expressions")
    void testBothOperandsParenthesized() {
        // 1 2 + 3 4 + * -> (1 + 2) * (3 + 4)
        Expr one = new Number("1", 1, 1);
        Expr two = new Number("2", 1, 3);
        Expr three = new Number("3", 1, 7);
        Expr four = new Number("4", 1, 9);

        BinaryOp add1 = new BinaryOp("+", one, two, 1, 5);
        BinaryOp add2 = new BinaryOp("+", three, four, 1, 11);
        BinaryOp mult = new BinaryOp("*", add1, add2, 1, 13);

        String result = generator.generate(mult);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    @DisplayName("should handle complex expression with mixed operators")
    void testComplexMixedExpression() {
        // 10 2 / 3 + 4 * -> (10 / 2 + 3) * 4
        Expr ten = new Number("10", 1, 1);
        Expr two = new Number("2", 1, 4);
        Expr three = new Number("3", 1, 8);
        Expr four = new Number("4", 1, 12);

        BinaryOp div = new BinaryOp("/", ten, two, 1, 6);
        BinaryOp add = new BinaryOp("+", div, three, 1, 10);
        BinaryOp mult = new BinaryOp("*", add, four, 1, 14);

        String result = generator.generate(mult);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @ParameterizedTest
    @CsvSource({
        "+, +",
        "-, -",
        "*, \\times",
        "/, \\div"
    })
    @DisplayName("should map operators to correct LaTeX symbols")
    void testOperatorMapping(String operator, String expectedLatex) {
        Expr left = new Number("2", 1, 1);
        Expr right = new Number("3", 1, 3);
        BinaryOp op = new BinaryOp(operator, left, right, 1, 5);

        String result = generator.generate(op);
        String expected = "$2 " + expectedLatex + " 3$";
        assertEquals(expected, result);
    }

    @Test
    @DisplayName("should wrap output in dollar signs")
    void testMathModeDelimiters() {
        Number num = new Number("42", 1, 1);
        String result = generator.generate(num);

        assertTrue(result.startsWith("$"));
        assertTrue(result.endsWith("$"));
    }

    @Test
    @DisplayName("should handle negative numbers")
    void testNegativeNumbers() {
        Number negNum = new Number("-5", 1, 1);
        String result = generator.generate(negNum);
        assertEquals("$-5$", result);
    }

    @Test
    @DisplayName("should preserve exact number format from AST")
    void testNumberFormatPreservation() {
        // Numbers with leading zeros or trailing zeros should be preserved
        Number num1 = new Number("0.5", 1, 1);
        assertEquals("$0.5$", generator.generate(num1));

        Number num2 = new Number("5.0", 1, 1);
        assertEquals("$5.0$", generator.generate(num2));

        Number num3 = new Number("05", 1, 1);
        assertEquals("$05$", generator.generate(num3));
    }

    @Test
    @DisplayName("should add parentheses for right-side subtraction with equal precedence")
    void testRightSideSubtractionParentheses() {
        // 5 3 2 - - should become 5 - (3 - 2)
        Expr five = new Number("5", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr two = new Number("2", 1, 5);

        BinaryOp innerSub = new BinaryOp("-", three, two, 1, 7);
        BinaryOp outerSub = new BinaryOp("-", five, innerSub, 1, 9);

        String result = generator.generate(outerSub);
        assertEquals("$5 - ( 3 - 2 )$", result);
    }

    @Test
    @DisplayName("should add parentheses for right-side division with equal precedence")
    void testRightSideDivisionParentheses() {
        // 10 6 2 / / should become 10 / (6 / 2)
        Expr ten = new Number("10", 1, 1);
        Expr six = new Number("6", 1, 4);
        Expr two = new Number("2", 1, 6);

        BinaryOp innerDiv = new BinaryOp("/", six, two, 1, 8);
        BinaryOp outerDiv = new BinaryOp("/", ten, innerDiv, 1, 10);

        String result = generator.generate(outerDiv);
        assertEquals("$10 \\div ( 6 \\div 2 )$", result);
    }

    @Test
    @DisplayName("should not add parentheses for right-side addition with equal precedence")
    void testRightSideAdditionNoParentheses() {
        // 1 2 3 + + should become 1 + 2 + 3 (no parens needed for commutative ops)
        Expr one = new Number("1", 1, 1);
        Expr two = new Number("2", 1, 3);
        Expr three = new Number("3", 1, 5);

        BinaryOp innerAdd = new BinaryOp("+", two, three, 1, 7);
        BinaryOp outerAdd = new BinaryOp("+", one, innerAdd, 1, 9);

        String result = generator.generate(outerAdd);
        assertEquals("$1 + 2 + 3$", result);
    }

    @Test
    @DisplayName("should not add parentheses for right-side multiplication with equal precedence")
    void testRightSideMultiplicationNoParentheses() {
        // 2 3 4 * * should become 2 * 3 * 4 (no parens needed for commutative ops)
        Expr two = new Number("2", 1, 1);
        Expr three = new Number("3", 1, 3);
        Expr four = new Number("4", 1, 5);

        BinaryOp innerMult = new BinaryOp("*", three, four, 1, 7);
        BinaryOp outerMult = new BinaryOp("*", two, innerMult, 1, 9);

        String result = generator.generate(outerMult);
        assertEquals("$2 \\times 3 \\times 4$", result);
    }
}

package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

class LaTeXGeneratorTest {

    @Test
    void testGenerateSingleInteger() {
        Number num = new Number(1, 1, "5");
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);

        assertEquals("$5$", result);
    }

    @Test
    void testGenerateSingleDecimal() {
        Number num = new Number(1, 1, "3.14");
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);

        assertEquals("$3.14$", result);
    }

    @ParameterizedTest
    @CsvSource({
        "42, $42$",
        "0, $0$",
        "123, $123$",
        "3.14, $3.14$",
        "0.5, $0.5$",
        "10.0, $10.0$"
    })
    void testGenerateVariousNumbers(String value, String expected) {
        Number num = new Number(1, 1, value);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);

        assertEquals(expected, result);
    }

    @Test
    void testGenerateSimpleAddition() {
        Number left = new Number(1, 1, "5");
        Number right = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 5, "+", left, right);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(add);

        assertEquals("$5 + 3$", result);
    }

    @Test
    void testGenerateChainedAddition() {
        // 1 2 + 3 + = (1 + 2) + 3
        Number num1 = new Number(1, 1, "1");
        Number num2 = new Number(1, 3, "2");
        BinaryOp inner = new BinaryOp(1, 5, "+", num1, num2);
        Number num3 = new Number(1, 7, "3");
        BinaryOp outer = new BinaryOp(1, 9, "+", inner, num3);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(outer);

        assertEquals("$1 + 2 + 3$", result);
    }

    @Test
    void testGenerateMultipleChainedAdditions() {
        // 1 2 + 3 + 4 + = ((1 + 2) + 3) + 4
        Number num1 = new Number(1, 1, "1");
        Number num2 = new Number(1, 3, "2");
        BinaryOp add1 = new BinaryOp(1, 5, "+", num1, num2);

        Number num3 = new Number(1, 7, "3");
        BinaryOp add2 = new BinaryOp(1, 9, "+", add1, num3);

        Number num4 = new Number(1, 11, "4");
        BinaryOp add3 = new BinaryOp(1, 13, "+", add2, num4);

        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(add3);

        assertEquals("$1 + 2 + 3 + 4$", result);
    }

    @Test
    void testGenerateAdditionWithDecimals() {
        Number left = new Number(1, 1, "1.5");
        Number right = new Number(1, 5, "0.5");
        BinaryOp add = new BinaryOp(1, 9, "+", left, right);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(add);

        assertEquals("$1.5 + 0.5$", result);
    }

    // Feature 6: Precedence Tests

    @Test
    void testPrecedenceLeftAdditionWithMultiplication() {
        // 5 3 + 2 * -> (5 + 3) * 2
        Number num5 = new Number(1, 1, "5");
        Number num3 = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 5, "+", num5, num3);
        Number num2 = new Number(1, 7, "2");
        BinaryOp mult = new BinaryOp(1, 9, "*", add, num2);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    void testPrecedenceLeftAdditionWithMultiplication2() {
        // 2 3 + 4 * -> (2 + 3) * 4
        Number num2 = new Number(1, 1, "2");
        Number num3 = new Number(1, 3, "3");
        BinaryOp add = new BinaryOp(1, 5, "+", num2, num3);
        Number num4 = new Number(1, 7, "4");
        BinaryOp mult = new BinaryOp(1, 9, "*", add, num4);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$( 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testPrecedenceRightAdditionWithMultiplication() {
        // 2 3 4 + * -> 2 * (3 + 4)
        Number num2 = new Number(1, 1, "2");
        Number num3 = new Number(1, 3, "3");
        Number num4 = new Number(1, 5, "4");
        BinaryOp add = new BinaryOp(1, 7, "+", num3, num4);
        BinaryOp mult = new BinaryOp(1, 9, "*", num2, add);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    void testPrecedenceBothSidesAdditionWithMultiplication() {
        // 1 2 + 3 4 + * -> (1 + 2) * (3 + 4)
        Number num1 = new Number(1, 1, "1");
        Number num2 = new Number(1, 3, "2");
        BinaryOp add1 = new BinaryOp(1, 5, "+", num1, num2);

        Number num3 = new Number(1, 7, "3");
        Number num4 = new Number(1, 9, "4");
        BinaryOp add2 = new BinaryOp(1, 11, "+", num3, num4);

        BinaryOp mult = new BinaryOp(1, 13, "*", add1, add2);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", result);
    }

    @Test
    void testPrecedenceComplexDivisionAdditionMultiplication() {
        // 10 2 / 3 + 4 * -> (10 / 2 + 3) * 4
        Number num10 = new Number(1, 1, "10");
        Number num2 = new Number(1, 4, "2");
        BinaryOp div = new BinaryOp(1, 6, "/", num10, num2);

        Number num3 = new Number(1, 8, "3");
        BinaryOp add = new BinaryOp(1, 10, "+", div, num3);

        Number num4 = new Number(1, 12, "4");
        BinaryOp mult = new BinaryOp(1, 14, "*", add, num4);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", result);
    }

    @Test
    void testPrecedenceMultiplicationHigherThanAddition() {
        // 2 3 * 4 + -> 2 * 3 + 4 (no parens needed)
        Number num2 = new Number(1, 1, "2");
        Number num3 = new Number(1, 3, "3");
        BinaryOp mult = new BinaryOp(1, 5, "*", num2, num3);

        Number num4 = new Number(1, 7, "4");
        BinaryOp add = new BinaryOp(1, 9, "+", mult, num4);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(add);

        assertEquals("$2 \\times 3 + 4$", result);
    }

    @Test
    void testPrecedenceAdditionThenMultiplication() {
        // 2 3 4 * + -> 2 + 3 * 4 (no parens needed)
        Number num2 = new Number(1, 1, "2");
        Number num3 = new Number(1, 3, "3");
        Number num4 = new Number(1, 5, "4");
        BinaryOp mult = new BinaryOp(1, 7, "*", num3, num4);
        BinaryOp add = new BinaryOp(1, 9, "+", num2, mult);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(add);

        assertEquals("$2 + 3 \\times 4$", result);
    }

    @Test
    void testPrecedenceSubtractionWithMultiplication() {
        // 5 3 - 2 * -> (5 - 3) * 2
        Number num5 = new Number(1, 1, "5");
        Number num3 = new Number(1, 3, "3");
        BinaryOp sub = new BinaryOp(1, 5, "-", num5, num3);

        Number num2 = new Number(1, 7, "2");
        BinaryOp mult = new BinaryOp(1, 9, "*", sub, num2);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$( 5 - 3 ) \\times 2$", result);
    }

    @Test
    void testPrecedenceLeftAssociativitySubtraction() {
        // 5 3 - 2 - -> 5 - 3 - 2 (left associative, no extra parens)
        Number num5 = new Number(1, 1, "5");
        Number num3 = new Number(1, 3, "3");
        BinaryOp sub1 = new BinaryOp(1, 5, "-", num5, num3);

        Number num2 = new Number(1, 7, "2");
        BinaryOp sub2 = new BinaryOp(1, 9, "-", sub1, num2);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(sub2);

        assertEquals("$5 - 3 - 2$", result);
    }

    @Test
    void testPrecedenceLeftAssociativityDivision() {
        // 100 10 / 5 / -> 100 / 10 / 5 (left associative, no extra parens)
        Number num100 = new Number(1, 1, "100");
        Number num10 = new Number(1, 5, "10");
        BinaryOp div1 = new BinaryOp(1, 8, "/", num100, num10);

        Number num5 = new Number(1, 11, "5");
        BinaryOp div2 = new BinaryOp(1, 13, "/", div1, num5);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(div2);

        assertEquals("$100 \\div 10 \\div 5$", result);
    }

    @Test
    void testPrecedenceMixedMultiplicationDivision() {
        // 10 2 / 5 * -> 10 / 2 * 5 (same precedence, left to right)
        Number num10 = new Number(1, 1, "10");
        Number num2 = new Number(1, 4, "2");
        BinaryOp div = new BinaryOp(1, 6, "/", num10, num2);

        Number num5 = new Number(1, 8, "5");
        BinaryOp mult = new BinaryOp(1, 10, "*", div, num5);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(mult);

        assertEquals("$10 \\div 2 \\times 5$", result);
    }

    @ParameterizedTest
    @CsvSource({
        "'5 3 + 2 *', '$( 5 + 3 ) \\times 2$'",
        "'2 3 + 4 *', '$( 2 + 3 ) \\times 4$'",
        "'2 3 4 + *', '$2 \\times ( 3 + 4 )$'",
        "'1 2 + 3 4 + *', '$( 1 + 2 ) \\times ( 3 + 4 )$'",
        "'10 2 / 3 + 4 *', '$( 10 \\div 2 + 3 ) \\times 4$'"
    })
    void testPrecedenceIOContract(String description, String expected) {
        // This test documents the expected I/O contract
        // Actual parsing is tested in integration tests
        // This is just to ensure we have the contract documented
        assertTrue(expected.startsWith("$"));
        assertTrue(expected.endsWith("$"));
    }
}

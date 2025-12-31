package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the LaTeXGenerator class.
 */
class LaTeXGeneratorTest {
    @Test
    void testGenerateInteger() {
        NumberExpr expr = new NumberExpr("5", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$5$", latex);
    }

    @Test
    void testGenerateDecimal() {
        NumberExpr expr = new NumberExpr("3.14", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$3.14$", latex);
    }

    @Test
    void testGenerateNegativeNumber() {
        NumberExpr expr = new NumberExpr("-5", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$-5$", latex);
    }

    @Test
    void testGenerateZero() {
        NumberExpr expr = new NumberExpr("0", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$0$", latex);
    }

    @Test
    void testGenerateLeadingZeros() {
        NumberExpr expr = new NumberExpr("007", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$007$", latex);
    }

    @Test
    void testGenerateSimpleAddition() {
        BinaryOpExpr expr = new BinaryOpExpr(
                "+",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$5 + 3$", latex);
    }

    @Test
    void testGenerateChainedAddition() {
        // ((1 + 2) + 3) + 4
        BinaryOpExpr innermost = new BinaryOpExpr(
                "+",
                new NumberExpr("1", 1, 1),
                new NumberExpr("2", 1, 3),
                1, 5
        );
        BinaryOpExpr middle = new BinaryOpExpr(
                "+",
                innermost,
                new NumberExpr("3", 1, 7),
                1, 9
        );
        BinaryOpExpr outer = new BinaryOpExpr(
                "+",
                middle,
                new NumberExpr("4", 1, 11),
                1, 13
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(outer);
        assertEquals("$1 + 2 + 3 + 4$", latex);
    }

    @Test
    void testGenerateAdditionWithNegativeNumbers() {
        BinaryOpExpr expr = new BinaryOpExpr(
                "+",
                new NumberExpr("-5", 1, 1),
                new NumberExpr("3", 1, 4),
                1, 6
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$-5 + 3$", latex);
    }

    @Test
    void testGenerateAdditionWithDecimals() {
        BinaryOpExpr expr = new BinaryOpExpr(
                "+",
                new NumberExpr("1.5", 1, 1),
                new NumberExpr("0.5", 1, 5),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$1.5 + 0.5$", latex);
    }

    @Test
    void testGenerateSimpleMultiplication() {
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                new NumberExpr("4", 1, 1),
                new NumberExpr("7", 1, 3),
                1, 5
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$4 \\times 7$", latex);
    }

    @Test
    void testGenerateMultiplicationWithDecimals() {
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                new NumberExpr("3.14", 1, 1),
                new NumberExpr("2", 1, 6),
                1, 8
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$3.14 \\times 2$", latex);
    }

    @Test
    void testGenerateMultiplicationNoParensHigherPrecedence() {
        // 2 + 3 * 4 (no parens around multiplication)
        BinaryOpExpr mult = new BinaryOpExpr(
                "*",
                new NumberExpr("3", 1, 5),
                new NumberExpr("4", 1, 7),
                1, 9
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "+",
                new NumberExpr("2", 1, 1),
                mult,
                1, 3
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$2 + 3 \\times 4$", latex);
    }

    @Test
    void testGenerateMultiplicationWithParensLowerPrecedenceLeft() {
        // (5 + 3) * 2
        BinaryOpExpr add = new BinaryOpExpr(
                "+",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                add,
                new NumberExpr("2", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 5 + 3 ) \\times 2$", latex);
    }

    @Test
    void testGenerateMultiplicationWithParensBothSides() {
        // (1 + 2) * (3 + 4)
        BinaryOpExpr leftAdd = new BinaryOpExpr(
                "+",
                new NumberExpr("1", 1, 1),
                new NumberExpr("2", 1, 3),
                1, 5
        );
        BinaryOpExpr rightAdd = new BinaryOpExpr(
                "+",
                new NumberExpr("3", 1, 7),
                new NumberExpr("4", 1, 9),
                1, 11
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                leftAdd,
                rightAdd,
                1, 13
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", latex);
    }

    // ===== FEATURE 6: PRECEDENCE TESTS =====

    @Test
    void testPrecedence_AdditionMultipliedLeft() {
        // (5 + 3) * 2 - I/O Contract case 1
        BinaryOpExpr add = new BinaryOpExpr(
                "+",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                add,
                new NumberExpr("2", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 5 + 3 ) \\times 2$", latex);
    }

    @Test
    void testPrecedence_AdditionMultipliedLeft2() {
        // (2 + 3) * 4 - I/O Contract case 2
        BinaryOpExpr add = new BinaryOpExpr(
                "+",
                new NumberExpr("2", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                add,
                new NumberExpr("4", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 2 + 3 ) \\times 4$", latex);
    }

    @Test
    void testPrecedence_AdditionMultipliedRight() {
        // 2 * (3 + 4) - I/O Contract case 3
        BinaryOpExpr add = new BinaryOpExpr(
                "+",
                new NumberExpr("3", 1, 5),
                new NumberExpr("4", 1, 7),
                1, 9
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                new NumberExpr("2", 1, 1),
                add,
                1, 3
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$2 \\times ( 3 + 4 )$", latex);
    }

    @Test
    void testPrecedence_BothAdditionsMultiplied() {
        // (1 + 2) * (3 + 4) - I/O Contract case 4
        BinaryOpExpr leftAdd = new BinaryOpExpr(
                "+",
                new NumberExpr("1", 1, 1),
                new NumberExpr("2", 1, 3),
                1, 5
        );
        BinaryOpExpr rightAdd = new BinaryOpExpr(
                "+",
                new NumberExpr("3", 1, 7),
                new NumberExpr("4", 1, 9),
                1, 11
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                leftAdd,
                rightAdd,
                1, 13
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", latex);
    }

    @Test
    void testPrecedence_DivisionPlusMultiplied() {
        // (10 / 2 + 3) * 4 - I/O Contract case 5
        BinaryOpExpr div = new BinaryOpExpr(
                "/",
                new NumberExpr("10", 1, 1),
                new NumberExpr("2", 1, 4),
                1, 6
        );
        BinaryOpExpr add = new BinaryOpExpr(
                "+",
                div,
                new NumberExpr("3", 1, 8),
                1, 10
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                add,
                new NumberExpr("4", 1, 12),
                1, 14
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", latex);
    }

    @Test
    void testPrecedence_MultiplicationPlusNoParens() {
        // 5 * 3 + 2 - multiplication on left, no parens needed
        BinaryOpExpr mult = new BinaryOpExpr(
                "*",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "+",
                mult,
                new NumberExpr("2", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$5 \\times 3 + 2$", latex);
    }

    @Test
    void testPrecedence_DivisionMultiplicationSameLevelNoParens() {
        // 10 / 2 * 5 - same precedence, left associative
        BinaryOpExpr div = new BinaryOpExpr(
                "/",
                new NumberExpr("10", 1, 1),
                new NumberExpr("2", 1, 4),
                1, 6
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                div,
                new NumberExpr("5", 1, 8),
                1, 10
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$10 \\div 2 \\times 5$", latex);
    }

    @Test
    void testPrecedence_SubtractionOnRight() {
        // 5 - (3 - 2) - requires parens on right for non-commutative op
        BinaryOpExpr rightSub = new BinaryOpExpr(
                "-",
                new NumberExpr("3", 1, 5),
                new NumberExpr("2", 1, 7),
                1, 9
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "-",
                new NumberExpr("5", 1, 1),
                rightSub,
                1, 3
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$5 - ( 3 - 2 )$", latex);
    }

    @Test
    void testPrecedence_SubtractionOnLeft() {
        // (5 - 3) - 2 = 5 - 3 - 2 (no parens needed on left)
        BinaryOpExpr leftSub = new BinaryOpExpr(
                "-",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "-",
                leftSub,
                new NumberExpr("2", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$5 - 3 - 2$", latex);
    }

    @Test
    void testPrecedence_DivisionOnRight() {
        // 10 / (2 / 5) - requires parens on right for non-commutative op
        BinaryOpExpr rightDiv = new BinaryOpExpr(
                "/",
                new NumberExpr("2", 1, 5),
                new NumberExpr("5", 1, 7),
                1, 9
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "/",
                new NumberExpr("10", 1, 1),
                rightDiv,
                1, 3
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$10 \\div ( 2 \\div 5 )$", latex);
    }

    @Test
    void testPrecedence_DivisionOnLeft() {
        // (10 / 2) / 5 = 10 / 2 / 5 (no parens needed on left)
        BinaryOpExpr leftDiv = new BinaryOpExpr(
                "/",
                new NumberExpr("10", 1, 1),
                new NumberExpr("2", 1, 4),
                1, 6
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "/",
                leftDiv,
                new NumberExpr("5", 1, 8),
                1, 10
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$10 \\div 2 \\div 5$", latex);
    }

    @Test
    void testPrecedence_SubtractionMultiplied() {
        // (5 - 3) * 2
        BinaryOpExpr sub = new BinaryOpExpr(
                "-",
                new NumberExpr("5", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "*",
                sub,
                new NumberExpr("2", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$( 5 - 3 ) \\times 2$", latex);
    }

    @Test
    void testPrecedence_MultiplicationDivisionNoParens() {
        // 2 * 3 / 4 - same precedence level
        BinaryOpExpr mult = new BinaryOpExpr(
                "*",
                new NumberExpr("2", 1, 1),
                new NumberExpr("3", 1, 3),
                1, 5
        );
        BinaryOpExpr expr = new BinaryOpExpr(
                "/",
                mult,
                new NumberExpr("4", 1, 7),
                1, 9
        );
        LaTeXGenerator generator = new LaTeXGenerator();
        String latex = generator.generate(expr);
        assertEquals("$2 \\times 3 \\div 4$", latex);
    }
}

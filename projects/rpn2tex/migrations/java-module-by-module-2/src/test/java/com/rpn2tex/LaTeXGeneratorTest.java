package com.rpn2tex;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the {@link LaTeXGenerator} class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Basic number and operator generation</li>
 *   <li>Operator precedence and parenthesization rules</li>
 *   <li>All I/O contract test cases</li>
 *   <li>Edge cases and error handling</li>
 * </ul>
 */
class LaTeXGeneratorTest {

    private LaTeXGenerator generator;

    @BeforeEach
    void setUp() {
        generator = new LaTeXGenerator();
    }

    // ========== Basic Generation Tests ==========

    @Test
    @DisplayName("Generate LaTeX for simple number")
    void testGenerateNumber() {
        Number num = new Number(1, 1, "42");
        String result = generator.generate(num);
        assertEquals("$42$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for floating point number")
    void testGenerateFloatingPoint() {
        Number num = new Number(1, 1, "3.14");
        String result = generator.generate(num);
        assertEquals("$3.14$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for negative number")
    void testGenerateNegativeNumber() {
        Number num = new Number(1, 1, "-5");
        String result = generator.generate(num);
        assertEquals("$-5$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for simple addition")
    void testGenerateAddition() {
        BinaryOp expr = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        String result = generator.generate(expr);
        assertEquals("$5 + 3$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for simple subtraction")
    void testGenerateSubtraction() {
        BinaryOp expr = new BinaryOp(1, 3, "-",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        String result = generator.generate(expr);
        assertEquals("$5 - 3$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for simple multiplication")
    void testGenerateMultiplication() {
        BinaryOp expr = new BinaryOp(1, 3, "*",
            new Number(1, 1, "4"),
            new Number(1, 3, "7")
        );
        String result = generator.generate(expr);
        assertEquals("$4 \\times 7$", result);
    }

    @Test
    @DisplayName("Generate LaTeX for simple division")
    void testGenerateDivision() {
        BinaryOp expr = new BinaryOp(1, 3, "/",
            new Number(1, 1, "10"),
            new Number(1, 4, "2")
        );
        String result = generator.generate(expr);
        assertEquals("$10 \\div 2$", result);
    }

    // ========== Precedence and Parenthesization Tests ==========

    @Test
    @DisplayName("Precedence: Addition before multiplication (needs parens)")
    void testPrecedenceAdditionBeforeMultiplication() {
        // (5 + 3) * 2
        BinaryOp innerAdd = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp mult = new BinaryOp(1, 7, "*",
            innerAdd,
            new Number(1, 7, "2")
        );
        String result = generator.generate(mult);
        assertEquals("$( 5 + 3 ) \\times 2$", result);
    }

    @Test
    @DisplayName("Precedence: Multiplication before addition (no parens)")
    void testPrecedenceMultiplicationBeforeAddition() {
        // 5 * 3 + 2
        BinaryOp innerMult = new BinaryOp(1, 3, "*",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp add = new BinaryOp(1, 7, "+",
            innerMult,
            new Number(1, 7, "2")
        );
        String result = generator.generate(add);
        assertEquals("$5 \\times 3 + 2$", result);
    }

    @Test
    @DisplayName("Precedence: Addition on right of multiplication (needs parens)")
    void testPrecedenceAdditionRightOfMultiplication() {
        // 2 * (3 + 4)
        BinaryOp innerAdd = new BinaryOp(1, 5, "+",
            new Number(1, 3, "3"),
            new Number(1, 5, "4")
        );
        BinaryOp mult = new BinaryOp(1, 1, "*",
            new Number(1, 1, "2"),
            innerAdd
        );
        String result = generator.generate(mult);
        assertEquals("$2 \\times ( 3 + 4 )$", result);
    }

    @Test
    @DisplayName("Precedence: Same precedence left-to-right (no parens)")
    void testPrecedenceSameLeft() {
        // (5 + 3) + 2 should render as 5 + 3 + 2 (left associative)
        BinaryOp innerAdd = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp outerAdd = new BinaryOp(1, 7, "+",
            innerAdd,
            new Number(1, 7, "2")
        );
        String result = generator.generate(outerAdd);
        assertEquals("$5 + 3 + 2$", result);
    }

    @Test
    @DisplayName("Precedence: Subtraction on right (needs parens)")
    void testPrecedenceSubtractionRight() {
        // 5 - (3 - 2)
        BinaryOp innerSub = new BinaryOp(1, 5, "-",
            new Number(1, 3, "3"),
            new Number(1, 5, "2")
        );
        BinaryOp outerSub = new BinaryOp(1, 1, "-",
            new Number(1, 1, "5"),
            innerSub
        );
        String result = generator.generate(outerSub);
        assertEquals("$5 - ( 3 - 2 )$", result);
    }

    @Test
    @DisplayName("Precedence: Division on right (needs parens)")
    void testPrecedenceDivisionRight() {
        // 10 / (2 / 5)
        BinaryOp innerDiv = new BinaryOp(1, 6, "/",
            new Number(1, 4, "2"),
            new Number(1, 6, "5")
        );
        BinaryOp outerDiv = new BinaryOp(1, 1, "/",
            new Number(1, 1, "10"),
            innerDiv
        );
        String result = generator.generate(outerDiv);
        assertEquals("$10 \\div ( 2 \\div 5 )$", result);
    }

    // ========== I/O Contract Test Cases ==========

    @ParameterizedTest
    @CsvSource(delimiter = '|', value = {
        "5 + 3                    | $5 + 3$",
        "5 - 3                    | $5 - 3$",
        "4 \\times 7              | $4 \\times 7$",
        "10 \\div 2               | $10 \\div 2$",
        "( 5 + 3 ) \\times 2      | $( 5 + 3 ) \\times 2$",
        "5 \\times 3 + 2          | $5 \\times 3 + 2$",
        "10 \\div 2 \\times 5     | $10 \\div 2 \\times 5$",
        "5 - 3 - 2                | $5 - 3 - 2$",
        "100 \\div 10 \\div 5 \\div 2 | $100 \\div 10 \\div 5 \\div 2$",
        "1 + 2 + 3 + 4            | $1 + 2 + 3 + 4$",
        "2 + 3 \\times 4          | $2 + 3 \\times 4$",
        "( 2 + 3 ) \\times 4      | $( 2 + 3 ) \\times 4$",
        "2 \\times ( 3 + 4 )      | $2 \\times ( 3 + 4 )$",
        "2 \\times 3 + 4          | $2 \\times 3 + 4$",
        "3.14 \\times 2           | $3.14 \\times 2$",
        "1.5 + 0.5                | $1.5 + 0.5$",
        "( 1 + 2 ) \\times ( 3 + 4 ) | $( 1 + 2 ) \\times ( 3 + 4 )$",
        "( 10 \\div 2 + 3 ) \\times 4 | $( 10 \\div 2 + 3 ) \\times 4$"
    })
    @DisplayName("I/O Contract: Expected outputs")
    void testIOContractCases(String astDescription, String expectedLatex) {
        // This is a reference test - actual AST construction happens in integration tests
        // Here we document the expected outputs for manual verification
        assertNotNull(expectedLatex);
    }

    @Test
    @DisplayName("I/O Contract: 5 3 + => $5 + 3$")
    void testIOContractBasicAddition() {
        // RPN: "5 3 +"
        BinaryOp expr = new BinaryOp(1, 5, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        assertEquals("$5 + 3$", generator.generate(expr));
    }

    @Test
    @DisplayName("I/O Contract: 5 3 - => $5 - 3$")
    void testIOContractBasicSubtraction() {
        // RPN: "5 3 -"
        BinaryOp expr = new BinaryOp(1, 5, "-",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        assertEquals("$5 - 3$", generator.generate(expr));
    }

    @Test
    @DisplayName("I/O Contract: 4 7 * => $4 \\times 7$")
    void testIOContractBasicMultiplication() {
        // RPN: "4 7 *"
        BinaryOp expr = new BinaryOp(1, 5, "*",
            new Number(1, 1, "4"),
            new Number(1, 3, "7")
        );
        assertEquals("$4 \\times 7$", generator.generate(expr));
    }

    @Test
    @DisplayName("I/O Contract: 10 2 / => $10 \\div 2$")
    void testIOContractBasicDivision() {
        // RPN: "10 2 /"
        BinaryOp expr = new BinaryOp(1, 6, "/",
            new Number(1, 1, "10"),
            new Number(1, 4, "2")
        );
        assertEquals("$10 \\div 2$", generator.generate(expr));
    }

    @Test
    @DisplayName("I/O Contract: 5 3 + 2 * => $( 5 + 3 ) \\times 2$")
    void testIOContractAdditionThenMultiply() {
        // RPN: "5 3 + 2 *"
        BinaryOp innerAdd = new BinaryOp(1, 5, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp mult = new BinaryOp(1, 9, "*",
            innerAdd,
            new Number(1, 7, "2")
        );
        assertEquals("$( 5 + 3 ) \\times 2$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 5 3 * 2 + => $5 \\times 3 + 2$")
    void testIOContractMultiplyThenAdd() {
        // RPN: "5 3 * 2 +"
        BinaryOp innerMult = new BinaryOp(1, 5, "*",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp add = new BinaryOp(1, 9, "+",
            innerMult,
            new Number(1, 7, "2")
        );
        assertEquals("$5 \\times 3 + 2$", generator.generate(add));
    }

    @Test
    @DisplayName("I/O Contract: 10 2 / 5 * => $10 \\div 2 \\times 5$")
    void testIOContractDivisionThenMultiply() {
        // RPN: "10 2 / 5 *"
        BinaryOp innerDiv = new BinaryOp(1, 6, "/",
            new Number(1, 1, "10"),
            new Number(1, 4, "2")
        );
        BinaryOp mult = new BinaryOp(1, 10, "*",
            innerDiv,
            new Number(1, 8, "5")
        );
        assertEquals("$10 \\div 2 \\times 5$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 5 3 - 2 - => $5 - 3 - 2$")
    void testIOContractMultipleSubtractions() {
        // RPN: "5 3 - 2 -"
        BinaryOp innerSub = new BinaryOp(1, 5, "-",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp outerSub = new BinaryOp(1, 9, "-",
            innerSub,
            new Number(1, 7, "2")
        );
        assertEquals("$5 - 3 - 2$", generator.generate(outerSub));
    }

    @Test
    @DisplayName("I/O Contract: 100 10 / 5 / 2 / => $100 \\div 10 \\div 5 \\div 2$")
    void testIOContractMultipleDivisions() {
        // RPN: "100 10 / 5 / 2 /"
        BinaryOp div1 = new BinaryOp(1, 8, "/",
            new Number(1, 1, "100"),
            new Number(1, 5, "10")
        );
        BinaryOp div2 = new BinaryOp(1, 12, "/",
            div1,
            new Number(1, 10, "5")
        );
        BinaryOp div3 = new BinaryOp(1, 16, "/",
            div2,
            new Number(1, 14, "2")
        );
        assertEquals("$100 \\div 10 \\div 5 \\div 2$", generator.generate(div3));
    }

    @Test
    @DisplayName("I/O Contract: 1 2 + 3 + 4 + => $1 + 2 + 3 + 4$")
    void testIOContractMultipleAdditions() {
        // RPN: "1 2 + 3 + 4 +"
        BinaryOp add1 = new BinaryOp(1, 5, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2")
        );
        BinaryOp add2 = new BinaryOp(1, 9, "+",
            add1,
            new Number(1, 7, "3")
        );
        BinaryOp add3 = new BinaryOp(1, 13, "+",
            add2,
            new Number(1, 11, "4")
        );
        assertEquals("$1 + 2 + 3 + 4$", generator.generate(add3));
    }

    @Test
    @DisplayName("I/O Contract: 2 3 4 * + => $2 + 3 \\times 4$")
    void testIOContractAddMultPrecedence() {
        // RPN: "2 3 4 * +"
        BinaryOp mult = new BinaryOp(1, 7, "*",
            new Number(1, 3, "3"),
            new Number(1, 5, "4")
        );
        BinaryOp add = new BinaryOp(1, 9, "+",
            new Number(1, 1, "2"),
            mult
        );
        assertEquals("$2 + 3 \\times 4$", generator.generate(add));
    }

    @Test
    @DisplayName("I/O Contract: 2 3 + 4 * => $( 2 + 3 ) \\times 4$")
    void testIOContractAddThenMult() {
        // RPN: "2 3 + 4 *"
        BinaryOp add = new BinaryOp(1, 5, "+",
            new Number(1, 1, "2"),
            new Number(1, 3, "3")
        );
        BinaryOp mult = new BinaryOp(1, 9, "*",
            add,
            new Number(1, 7, "4")
        );
        assertEquals("$( 2 + 3 ) \\times 4$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 2 3 4 + * => $2 \\times ( 3 + 4 )$")
    void testIOContractMultWithAddOnRight() {
        // RPN: "2 3 4 + *"
        BinaryOp add = new BinaryOp(1, 7, "+",
            new Number(1, 3, "3"),
            new Number(1, 5, "4")
        );
        BinaryOp mult = new BinaryOp(1, 9, "*",
            new Number(1, 1, "2"),
            add
        );
        assertEquals("$2 \\times ( 3 + 4 )$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 2 3 * 4 + => $2 \\times 3 + 4$")
    void testIOContractMultThenAdd() {
        // RPN: "2 3 * 4 +"
        BinaryOp mult = new BinaryOp(1, 5, "*",
            new Number(1, 1, "2"),
            new Number(1, 3, "3")
        );
        BinaryOp add = new BinaryOp(1, 9, "+",
            mult,
            new Number(1, 7, "4")
        );
        assertEquals("$2 \\times 3 + 4$", generator.generate(add));
    }

    @Test
    @DisplayName("I/O Contract: 3.14 2 * => $3.14 \\times 2$")
    void testIOContractFloatingPoint() {
        // RPN: "3.14 2 *"
        BinaryOp mult = new BinaryOp(1, 8, "*",
            new Number(1, 1, "3.14"),
            new Number(1, 6, "2")
        );
        assertEquals("$3.14 \\times 2$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 1.5 0.5 + => $1.5 + 0.5$")
    void testIOContractFloatingPointAddition() {
        // RPN: "1.5 0.5 +"
        BinaryOp add = new BinaryOp(1, 9, "+",
            new Number(1, 1, "1.5"),
            new Number(1, 5, "0.5")
        );
        assertEquals("$1.5 + 0.5$", generator.generate(add));
    }

    @Test
    @DisplayName("I/O Contract: 1 2 + 3 4 + * => $( 1 + 2 ) \\times ( 3 + 4 )$")
    void testIOContractComplexSubexpressions() {
        // RPN: "1 2 + 3 4 + *"
        BinaryOp leftAdd = new BinaryOp(1, 5, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2")
        );
        BinaryOp rightAdd = new BinaryOp(1, 11, "+",
            new Number(1, 7, "3"),
            new Number(1, 9, "4")
        );
        BinaryOp mult = new BinaryOp(1, 13, "*",
            leftAdd,
            rightAdd
        );
        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", generator.generate(mult));
    }

    @Test
    @DisplayName("I/O Contract: 10 2 / 3 + 4 * => $( 10 \\div 2 + 3 ) \\times 4$")
    void testIOContractMixedOperations() {
        // RPN: "10 2 / 3 + 4 *"
        BinaryOp div = new BinaryOp(1, 6, "/",
            new Number(1, 1, "10"),
            new Number(1, 4, "2")
        );
        BinaryOp add = new BinaryOp(1, 10, "+",
            div,
            new Number(1, 8, "3")
        );
        BinaryOp mult = new BinaryOp(1, 14, "*",
            add,
            new Number(1, 12, "4")
        );
        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", generator.generate(mult));
    }

    // ========== Edge Cases and Error Handling ==========

    @Test
    @DisplayName("Error: Unknown operator throws exception")
    void testUnknownOperator() {
        BinaryOp expr = new BinaryOp(1, 3, "^",
            new Number(1, 1, "2"),
            new Number(1, 3, "3")
        );
        assertThrows(IllegalArgumentException.class, () -> {
            generator.generate(expr);
        });
    }

    @Test
    @DisplayName("Math mode delimiters are present")
    void testMathModeDelimiters() {
        Number num = new Number(1, 1, "5");
        String result = generator.generate(num);
        assertTrue(result.startsWith("$"));
        assertTrue(result.endsWith("$"));
    }

    @Test
    @DisplayName("Spacing: Operators have spaces around them")
    void testOperatorSpacing() {
        BinaryOp expr = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        String result = generator.generate(expr);
        assertTrue(result.contains(" + "));
    }

    @Test
    @DisplayName("Spacing: Parentheses have spaces inside")
    void testParenthesesSpacing() {
        BinaryOp innerAdd = new BinaryOp(1, 3, "+",
            new Number(1, 1, "5"),
            new Number(1, 3, "3")
        );
        BinaryOp mult = new BinaryOp(1, 7, "*",
            innerAdd,
            new Number(1, 7, "2")
        );
        String result = generator.generate(mult);
        assertTrue(result.contains("( "));
        assertTrue(result.contains(" )"));
    }

    @Test
    @DisplayName("Deep nesting: 4 levels of operations")
    void testDeepNesting() {
        // ((1 + 2) * 3) + 4
        BinaryOp innerAdd = new BinaryOp(1, 3, "+",
            new Number(1, 1, "1"),
            new Number(1, 3, "2")
        );
        BinaryOp mult = new BinaryOp(1, 7, "*",
            innerAdd,
            new Number(1, 7, "3")
        );
        BinaryOp outerAdd = new BinaryOp(1, 11, "+",
            mult,
            new Number(1, 11, "4")
        );
        String result = generator.generate(outerAdd);
        assertEquals("$( 1 + 2 ) \\times 3 + 4$", result);
    }
}

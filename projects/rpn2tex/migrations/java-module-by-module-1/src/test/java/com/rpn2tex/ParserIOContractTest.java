package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.junit.jupiter.api.Assertions.*;

/**
 * I/O Contract validation tests for the Parser.
 *
 * <p>These tests validate that the parser correctly handles all the test cases
 * from the migration specification's I/O contract. Each test case represents
 * an RPN expression that should be successfully parsed into a valid AST.
 */
@DisplayName("Parser I/O Contract Tests")
class ParserIOContractTest {

    /**
     * Provides test cases from the I/O contract for successful parses.
     * Each case includes: RPN input, description, expected AST type.
     */
    static Stream<Arguments> ioContractTestCases() {
        return Stream.of(
            // Basic operations
            Arguments.of("5 3 +", "Basic addition", "BinaryOp(+)"),
            Arguments.of("5 3 -", "Basic subtraction", "BinaryOp(-)"),
            Arguments.of("4 7 *", "Basic multiplication", "BinaryOp(*)"),
            Arguments.of("10 2 /", "Basic division", "BinaryOp(/)"),

            // Nested expressions
            Arguments.of("5 3 + 2 *", "Nested: (5+3)*2", "BinaryOp(*, BinaryOp(+), Num)"),
            Arguments.of("5 3 * 2 +", "Precedence: 5*3+2", "BinaryOp(+, BinaryOp(*), Num)"),
            Arguments.of("2 3 4 * +", "Nested: 2+3*4", "BinaryOp(+, Num, BinaryOp(*))"),
            Arguments.of("2 3 + 4 *", "Nested: (2+3)*4", "BinaryOp(*, BinaryOp(+), Num)"),
            Arguments.of("2 3 4 + *", "Nested: 2*(3+4)", "BinaryOp(*, Num, BinaryOp(+))"),
            Arguments.of("2 3 * 4 +", "Precedence: 2*3+4", "BinaryOp(+, BinaryOp(*), Num)"),

            // Multiple operations
            Arguments.of("10 2 / 5 *", "Left-associative: 10/2*5", "BinaryOp(*, BinaryOp(/), Num)"),
            Arguments.of("5 3 - 2 -", "Left-associative: 5-3-2", "BinaryOp(-, BinaryOp(-), Num)"),
            Arguments.of("100 10 / 5 / 2 /", "Multiple divisions", "BinaryOp(/, BinaryOp(/, BinaryOp(/), Num), Num)"),
            Arguments.of("1 2 + 3 + 4 +", "Multiple additions", "BinaryOp(+, BinaryOp(+, BinaryOp(+), Num), Num)"),

            // Floating-point
            Arguments.of("3.14 2 *", "Floating-point multiplication", "BinaryOp(*)"),
            Arguments.of("1.5 0.5 +", "Floating-point addition", "BinaryOp(+)"),

            // Complex expressions
            Arguments.of("1 2 + 3 4 + *", "Complex: (1+2)*(3+4)", "BinaryOp(*, BinaryOp(+), BinaryOp(+))"),
            Arguments.of("10 2 / 3 + 4 *", "Complex: (10/2+3)*4", "BinaryOp(*, BinaryOp(+, BinaryOp(/), Num), Num)")
        );
    }

    @ParameterizedTest(name = "{1}")
    @MethodSource("ioContractTestCases")
    @DisplayName("I/O Contract: Valid RPN expressions")
    void testIOContractValidExpressions(String input, String description, String expectedStructure) {
        try {
            // Tokenize the input
            Lexer lexer = new Lexer(input);
            var tokens = lexer.tokenize();

            // Parse into AST
            Parser parser = new Parser(tokens);
            Expr result = parser.parse();

            // Verify we got a valid AST
            assertNotNull(result, "Parser should return non-null AST for: " + description);
            assertNotNull(result.getClass(), "AST should have a concrete type");

            // Verify structure matches expected type
            if (expectedStructure.startsWith("BinaryOp")) {
                assertInstanceOf(BinaryOp.class, result,
                    "Expected BinaryOp for: " + description);

                // Extract the root operator (first operator in the structure)
                BinaryOp op = (BinaryOp) result;
                if (expectedStructure.startsWith("BinaryOp(+")) {
                    assertEquals("+", op.operator(), "Expected + operator for: " + description);
                } else if (expectedStructure.startsWith("BinaryOp(-")) {
                    assertEquals("-", op.operator(), "Expected - operator for: " + description);
                } else if (expectedStructure.startsWith("BinaryOp(*")) {
                    assertEquals("*", op.operator(), "Expected * operator for: " + description);
                } else if (expectedStructure.startsWith("BinaryOp(/")) {
                    assertEquals("/", op.operator(), "Expected / operator for: " + description);
                }
            }

            // Verify position tracking
            assertTrue(result.line() >= 1, "Line should be 1-based");
            assertTrue(result.column() >= 1, "Column should be 1-based");

        } catch (RpnException e) {
            fail("Parser failed on valid input '" + input + "': " + e.getMessage());
        }
    }

    @Test
    @DisplayName("I/O Contract: Single number")
    void testSingleNumber() throws RpnException {
        Lexer lexer = new Lexer("42");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        assertEquals("42", ((Number) result).value());
    }

    @Test
    @DisplayName("I/O Contract: Verify AST structure for 5 3 + 2 *")
    void testDetailedASTStructure() throws RpnException {
        // This is a critical test case from the I/O contract
        // Input: "5 3 + 2 *"
        // Expected: ((5 + 3) * 2)
        // AST: BinaryOp(*, BinaryOp(+, Number(5), Number(3)), Number(2))

        Lexer lexer = new Lexer("5 3 + 2 *");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Root should be multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left child should be addition
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());

        // Addition's children
        assertInstanceOf(Number.class, add.left());
        assertInstanceOf(Number.class, add.right());
        assertEquals("5", ((Number) add.left()).value());
        assertEquals("3", ((Number) add.right()).value());

        // Right child of multiplication
        assertInstanceOf(Number.class, mult.right());
        assertEquals("2", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Verify AST structure for 2 3 4 + *")
    void testRightNestedStructure() throws RpnException {
        // Input: "2 3 4 + *"
        // Expected: (2 * (3 + 4))
        // AST: BinaryOp(*, Number(2), BinaryOp(+, Number(3), Number(4)))

        Lexer lexer = new Lexer("2 3 4 + *");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Root should be multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left child should be number 2
        assertInstanceOf(Number.class, mult.left());
        assertEquals("2", ((Number) mult.left()).value());

        // Right child should be addition
        assertInstanceOf(BinaryOp.class, mult.right());
        BinaryOp add = (BinaryOp) mult.right();
        assertEquals("+", add.operator());
        assertEquals("3", ((Number) add.left()).value());
        assertEquals("4", ((Number) add.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Verify left-associativity for subtraction")
    void testLeftAssociativitySubtraction() throws RpnException {
        // Input: "5 3 - 2 -"
        // Expected: ((5 - 3) - 2)
        // NOT: (5 - (3 - 2))

        Lexer lexer = new Lexer("5 3 - 2 -");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp outer = (BinaryOp) result;
        assertEquals("-", outer.operator());

        // Left child should be subtraction (5 - 3)
        assertInstanceOf(BinaryOp.class, outer.left());
        BinaryOp inner = (BinaryOp) outer.left();
        assertEquals("-", inner.operator());
        assertEquals("5", ((Number) inner.left()).value());
        assertEquals("3", ((Number) inner.right()).value());

        // Right child should be 2
        assertEquals("2", ((Number) outer.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Verify left-associativity for division")
    void testLeftAssociativityDivision() throws RpnException {
        // Input: "10 2 / 5 *"
        // Expected: ((10 / 2) * 5)

        Lexer lexer = new Lexer("10 2 / 5 *");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left child should be division
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp div = (BinaryOp) mult.left();
        assertEquals("/", div.operator());
        assertEquals("10", ((Number) div.left()).value());
        assertEquals("2", ((Number) div.right()).value());

        // Right child should be 5
        assertEquals("5", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Complex expression with multiple parenthesized subexpressions")
    void testComplexMultipleSubexpressions() throws RpnException {
        // Input: "1 2 + 3 4 + *"
        // Expected: ((1 + 2) * (3 + 4))

        Lexer lexer = new Lexer("1 2 + 3 4 + *");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Both children should be additions
        assertInstanceOf(BinaryOp.class, mult.left());
        assertInstanceOf(BinaryOp.class, mult.right());

        BinaryOp leftAdd = (BinaryOp) mult.left();
        BinaryOp rightAdd = (BinaryOp) mult.right();

        assertEquals("+", leftAdd.operator());
        assertEquals("+", rightAdd.operator());

        assertEquals("1", ((Number) leftAdd.left()).value());
        assertEquals("2", ((Number) leftAdd.right()).value());
        assertEquals("3", ((Number) rightAdd.left()).value());
        assertEquals("4", ((Number) rightAdd.right()).value());
    }

    @Test
    @DisplayName("I/O Contract: Floating-point number preservation")
    void testFloatingPointPreservation() throws RpnException {
        Lexer lexer = new Lexer("3.14 2 *");
        var tokens = lexer.tokenize();

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;

        // Verify the floating-point value is preserved exactly
        assertInstanceOf(Number.class, mult.left());
        assertEquals("3.14", ((Number) mult.left()).value());
        assertEquals("2", ((Number) mult.right()).value());
    }
}

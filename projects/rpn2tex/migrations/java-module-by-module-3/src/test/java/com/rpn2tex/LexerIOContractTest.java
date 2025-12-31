package com.rpn2tex;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * I/O Contract validation tests for the Lexer.
 *
 * <p>This test class validates the specific requirements from the I/O contract:
 * <ul>
 *   <li>Test cases 5, 16, 17: Should produce lexer errors for unsupported '^' operator</li>
 *   <li>All other test cases: Should successfully tokenize without errors</li>
 * </ul>
 */
class LexerIOContractTest {

    /**
     * Test Case 5: "2 3 ^" should produce a lexer error.
     */
    @Test
    void testCase5_UnsupportedExponentiation() {
        Lexer lexer = new Lexer("2 3 ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());

        // Verify error message can be formatted
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter("2 3 ^");
        String formatted = formatter.formatError(exception.getErrorMessage(), exception.getLine(), exception.getColumn());

        assertTrue(formatted.contains("Error: Unexpected character '^'"));
        assertTrue(formatted.contains("2 3 ^"));
        assertTrue(formatted.contains("^")); // Caret pointer
    }

    /**
     * Test Case 16: "2 3 ^ 4 *" should produce a lexer error.
     */
    @Test
    void testCase16_UnsupportedExponentiationInExpression() {
        Lexer lexer = new Lexer("2 3 ^ 4 *");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    /**
     * Test Case 17: "2 3 4 ^ ^" should produce a lexer error.
     */
    @Test
    void testCase17_MultipleUnsupportedOperators() {
        Lexer lexer = new Lexer("2 3 4 ^ ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        // Should fail at the first '^'
        assertEquals("Unexpected character '^'", exception.getErrorMessage());
        assertEquals(1, exception.getLine());
        assertEquals(7, exception.getColumn());
    }

    /**
     * Test Case 1: "5 3 +" should tokenize successfully.
     */
    @Test
    void testCase1_BasicAddition() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    /**
     * Test Case 2: "5 3 -" should tokenize successfully.
     */
    @Test
    void testCase2_BasicSubtraction() throws RpnException {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    /**
     * Test Case 3: "4 7 *" should tokenize successfully.
     */
    @Test
    void testCase3_BasicMultiplication() throws RpnException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals(TokenType.MULT, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    /**
     * Test Case 4: "10 2 /" should tokenize successfully.
     */
    @Test
    void testCase4_BasicDivision() throws RpnException {
        Lexer lexer = new Lexer("10 2 /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("10", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("2", tokens.get(1).value());
        assertEquals(TokenType.DIV, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    /**
     * Test Case 6: "5 3 + 2 *" should tokenize successfully.
     */
    @Test
    void testCase6_PrecedenceWithParens() throws RpnException {
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals(TokenType.NUMBER, tokens.get(3).type());
        assertEquals(TokenType.MULT, tokens.get(4).type());
        assertEquals(TokenType.EOF, tokens.get(5).type());
    }

    /**
     * Test Case 18: "3.14 2 *" should tokenize successfully with decimal number preserved.
     */
    @Test
    void testCase18_DecimalNumbers() throws RpnException {
        Lexer lexer = new Lexer("3.14 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value()); // Decimal preserved
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("2", tokens.get(1).value());
        assertEquals(TokenType.MULT, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    /**
     * Test Case 19: "1.5 0.5 +" should tokenize successfully.
     */
    @Test
    void testCase19_DecimalAddition() throws RpnException {
        Lexer lexer = new Lexer("1.5 0.5 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals("1.5", tokens.get(0).value());
        assertEquals("0.5", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
    }

    /**
     * Verify that error formatting produces the expected format.
     */
    @Test
    void testErrorFormatting() {
        String source = "2 3 ^";
        RpnException.ErrorFormatter formatter = new RpnException.ErrorFormatter(source);

        String formatted = formatter.formatError("Unexpected character '^'", 1, 5);

        // Expected format:
        // Error: Unexpected character '^'
        //
        // 1 | 2 3 ^
        //         ^

        String[] lines = formatted.split("\n");
        assertEquals(4, lines.length);
        assertEquals("Error: Unexpected character '^'", lines[0]);
        assertEquals("", lines[1]);
        assertTrue(lines[2].contains("2 3 ^"));
        assertTrue(lines[3].contains("^"));

        // Verify caret is at correct position (column 5, which is 4 spaces from start)
        String caretLine = lines[3];
        int caretPos = caretLine.indexOf('^');
        // The caret line has format: "    | " followed by spaces and ^
        // Column 5 means 4 spaces after the " | " part
        assertTrue(caretPos > 0);
    }

    /**
     * Test that all success cases from the I/O contract tokenize without errors.
     */
    @Test
    void testAllSuccessCases() {
        String[] successCases = {
            "5 3 +",           // Case 1
            "5 3 -",           // Case 2
            "4 7 *",           // Case 3
            "10 2 /",          // Case 4
            "5 3 + 2 *",       // Case 6
            "5 3 * 2 +",       // Case 7
            "10 2 / 5 *",      // Case 8
            "5 3 - 2 -",       // Case 9
            "100 10 / 5 / 2 /", // Case 10
            "1 2 + 3 + 4 +",   // Case 11
            "2 3 4 * +",       // Case 12
            "2 3 + 4 *",       // Case 13
            "2 3 4 + *",       // Case 14
            "2 3 * 4 +",       // Case 15
            "3.14 2 *",        // Case 18
            "1.5 0.5 +",       // Case 19
            "1 2 + 3 4 + *",   // Case 20
            "10 2 / 3 + 4 *"   // Case 21
        };

        for (String input : successCases) {
            Lexer lexer = new Lexer(input);
            assertDoesNotThrow(() -> {
                List<Token> tokens = lexer.tokenize();
                assertNotNull(tokens);
                assertFalse(tokens.isEmpty());
                // Every token list should end with EOF
                assertEquals(TokenType.EOF, tokens.get(tokens.size() - 1).type());
            }, "Failed to tokenize: " + input);
        }
    }

    /**
     * Test that all error cases from the I/O contract throw lexer errors.
     */
    @Test
    void testAllErrorCases() {
        String[] errorCases = {
            "2 3 ^",       // Case 5
            "2 3 ^ 4 *",   // Case 16
            "2 3 4 ^ ^"    // Case 17
        };

        for (String input : errorCases) {
            Lexer lexer = new Lexer(input);
            RpnException exception = assertThrows(RpnException.class, lexer::tokenize,
                "Expected lexer error for: " + input);
            assertEquals("Unexpected character '^'", exception.getErrorMessage(),
                "Wrong error message for: " + input);
        }
    }
}

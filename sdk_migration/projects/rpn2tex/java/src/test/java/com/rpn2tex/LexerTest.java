package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;
import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for Lexer class.
 *
 * Tests cover:
 * - Basic operators (+, -, *, /)
 * - Numbers (integers and decimals)
 * - Negative numbers
 * - Whitespace handling
 * - Position tracking
 * - Error cases (unsupported characters)
 * - I/O contract validation
 */
class LexerTest {

    @Test
    void testConstructorWithValidText() {
        Lexer lexer = new Lexer("5 3 +");
        assertNotNull(lexer);
    }

    @Test
    void testConstructorWithNullText() {
        assertThrows(NullPointerException.class, () -> {
            new Lexer(null);
        });
    }

    @Test
    void testEmptyInput() throws RpnException {
        Lexer lexer = new Lexer("");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testWhitespaceOnly() throws RpnException {
        Lexer lexer = new Lexer("   \t\n  ");
        List<Token> tokens = lexer.tokenize();

        assertEquals(1, tokens.size());
        assertEquals(TokenType.EOF, tokens.get(0).type());
    }

    @Test
    void testSingleNumber() throws RpnException {
        Lexer lexer = new Lexer("42");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("42", tokens.get(0).value());
        assertEquals(TokenType.EOF, tokens.get(1).type());
    }

    @Test
    void testDecimalNumber() throws RpnException {
        Lexer lexer = new Lexer("3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value());
    }

    @Test
    void testNegativeNumber() throws RpnException {
        Lexer lexer = new Lexer("-5");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-5", tokens.get(0).value());
    }

    @Test
    void testNegativeDecimal() throws RpnException {
        Lexer lexer = new Lexer("-3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-3.14", tokens.get(0).value());
    }

    @Test
    void testPlusOperator() throws RpnException {
        Lexer lexer = new Lexer("+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type());
        assertEquals("+", tokens.get(0).value());
    }

    @Test
    void testMinusOperator() throws RpnException {
        Lexer lexer = new Lexer("5 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(3, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals(TokenType.MINUS, tokens.get(1).type());
        assertEquals("-", tokens.get(1).value());
    }

    @Test
    void testMultOperator() throws RpnException {
        Lexer lexer = new Lexer("*");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.MULT, tokens.get(0).type());
        assertEquals("*", tokens.get(0).value());
    }

    @Test
    void testDivOperator() throws RpnException {
        Lexer lexer = new Lexer("/");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.DIV, tokens.get(0).type());
        assertEquals("/", tokens.get(0).value());
    }

    @Test
    void testBasicAddition() throws RpnException {
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

    @Test
    void testBasicSubtraction() throws RpnException {
        Lexer lexer = new Lexer("5 3 -");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
    }

    @Test
    void testBasicMultiplication() throws RpnException {
        Lexer lexer = new Lexer("4 7 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("4", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("7", tokens.get(1).value());
        assertEquals(TokenType.MULT, tokens.get(2).type());
    }

    @Test
    void testBasicDivision() throws RpnException {
        Lexer lexer = new Lexer("10 2 /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("10", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("2", tokens.get(1).value());
        assertEquals(TokenType.DIV, tokens.get(2).type());
    }

    @Test
    void testComplexExpression() throws RpnException {
        Lexer lexer = new Lexer("5 3 + 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(6, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals(TokenType.NUMBER, tokens.get(3).type());
        assertEquals("2", tokens.get(3).value());
        assertEquals(TokenType.MULT, tokens.get(4).type());
        assertEquals(TokenType.EOF, tokens.get(5).type());
    }

    @Test
    void testFloatingPointOperands() throws RpnException {
        Lexer lexer = new Lexer("3.14 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("2", tokens.get(1).value());
        assertEquals(TokenType.MULT, tokens.get(2).type());
    }

    @Test
    void testMultipleFloatingPoints() throws RpnException {
        Lexer lexer = new Lexer("1.5 0.5 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("1.5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("0.5", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
    }

    @Test
    void testPositionTracking() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        // 5 at column 1
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());

        // 3 at column 3
        assertEquals(1, tokens.get(1).line());
        assertEquals(3, tokens.get(1).column());

        // + at column 5
        assertEquals(1, tokens.get(2).line());
        assertEquals(5, tokens.get(2).column());
    }

    @Test
    void testPositionTrackingWithNewlines() throws RpnException {
        Lexer lexer = new Lexer("5\n3\n+");
        List<Token> tokens = lexer.tokenize();

        // 5 at line 1, column 1
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());

        // 3 at line 2, column 1
        assertEquals(2, tokens.get(1).line());
        assertEquals(1, tokens.get(1).column());

        // + at line 3, column 1
        assertEquals(3, tokens.get(2).line());
        assertEquals(1, tokens.get(2).column());
    }

    @Test
    void testVariousWhitespace() throws RpnException {
        Lexer lexer = new Lexer("5  \t\t  3\n\n+");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
    }

    @Test
    void testUnexpectedCharacterCaret() {
        Lexer lexer = new Lexer("2 3 ^");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testUnexpectedCharacterInComplexExpression() {
        Lexer lexer = new Lexer("2 3 ^ 4 *");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testUnexpectedCharacterAtDifferentPosition() {
        Lexer lexer = new Lexer("2 3 4 ^ ^");

        RpnException exception = assertThrows(RpnException.class, () -> {
            lexer.tokenize();
        });

        // Should fail at first caret
        assertTrue(exception.getMessage().contains("Unexpected character"));
        assertTrue(exception.getMessage().contains("^"));
        assertEquals(1, exception.getLine());
        assertEquals(7, exception.getColumn());
    }

    @Test
    void testUnexpectedCharacterOtherSymbols() {
        // Test various unsupported characters
        String[] invalidInputs = {"@", "#", "$", "%", "&", "!", "?", "~"};

        for (String input : invalidInputs) {
            Lexer lexer = new Lexer(input);
            assertThrows(RpnException.class, () -> lexer.tokenize(),
                "Should throw exception for: " + input);
        }
    }

    @Test
    void testMinusVsNegativeNumber() throws RpnException {
        // Test "5 - 3" (subtraction)
        Lexer lexer1 = new Lexer("5 - 3");
        List<Token> tokens1 = lexer1.tokenize();

        assertEquals(4, tokens1.size());
        assertEquals(TokenType.NUMBER, tokens1.get(0).type());
        assertEquals("5", tokens1.get(0).value());
        assertEquals(TokenType.MINUS, tokens1.get(1).type());
        assertEquals("-", tokens1.get(1).value());
        assertEquals(TokenType.NUMBER, tokens1.get(2).type());
        assertEquals("3", tokens1.get(2).value());

        // Test "5 -3" (5 and negative 3)
        Lexer lexer2 = new Lexer("5 -3");
        List<Token> tokens2 = lexer2.tokenize();

        assertEquals(3, tokens2.size());
        assertEquals(TokenType.NUMBER, tokens2.get(0).type());
        assertEquals("5", tokens2.get(0).value());
        assertEquals(TokenType.NUMBER, tokens2.get(1).type());
        assertEquals("-3", tokens2.get(1).value());
    }

    @Test
    void testEOFPosition() throws RpnException {
        Lexer lexer = new Lexer("42");
        List<Token> tokens = lexer.tokenize();

        Token eofToken = tokens.get(tokens.size() - 1);
        assertEquals(TokenType.EOF, eofToken.type());
        assertEquals("", eofToken.value());
        // EOF should be at position after last character
        assertEquals(1, eofToken.line());
        assertEquals(3, eofToken.column()); // After "42"
    }

    @Test
    void testChainedOperations() throws RpnException {
        Lexer lexer = new Lexer("100 10 / 5 / 2 /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(8, tokens.size());
        assertEquals("100", tokens.get(0).value());
        assertEquals("10", tokens.get(1).value());
        assertEquals(TokenType.DIV, tokens.get(2).type());
        assertEquals("5", tokens.get(3).value());
        assertEquals(TokenType.DIV, tokens.get(4).type());
        assertEquals("2", tokens.get(5).value());
        assertEquals(TokenType.DIV, tokens.get(6).type());
        assertEquals(TokenType.EOF, tokens.get(7).type());
    }
}

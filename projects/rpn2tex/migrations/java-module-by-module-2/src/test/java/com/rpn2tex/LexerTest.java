package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the {@link Lexer} class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Basic tokenization of numbers and operators</li>
 *   <li>Whitespace handling</li>
 *   <li>Decimal numbers</li>
 *   <li>Negative numbers</li>
 *   <li>Position tracking</li>
 *   <li>Error cases (unsupported characters)</li>
 * </ul>
 */
class LexerTest {

    @Test
    void testEmptyInput() throws RpnException {
        Lexer lexer = new Lexer("");
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
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());
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
        Lexer lexer = new Lexer("-42");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-42", tokens.get(0).value());
    }

    @Test
    void testNegativeDecimalNumber() throws RpnException {
        Lexer lexer = new Lexer("-3.14");
        List<Token> tokens = lexer.tokenize();

        assertEquals(2, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("-3.14", tokens.get(0).value());
    }

    @Test
    void testAllOperators() throws RpnException {
        Lexer lexer = new Lexer("+ - * /");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(TokenType.PLUS, tokens.get(0).type());
        assertEquals("+", tokens.get(0).value());
        assertEquals(TokenType.MINUS, tokens.get(1).type());
        assertEquals("-", tokens.get(1).value());
        assertEquals(TokenType.MULT, tokens.get(2).type());
        assertEquals("*", tokens.get(2).value());
        assertEquals(TokenType.DIV, tokens.get(3).type());
        assertEquals("/", tokens.get(3).value());
        assertEquals(TokenType.EOF, tokens.get(4).type());
    }

    @Test
    void testSimpleExpression() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.PLUS, tokens.get(2).type());
        assertEquals("+", tokens.get(2).value());
        assertEquals(TokenType.EOF, tokens.get(3).type());
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
    void testMultipleWhitespace() throws RpnException {
        Lexer lexer = new Lexer("5   3\t+\n");
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
    void testPositionTracking() throws RpnException {
        Lexer lexer = new Lexer("5 3 +");
        List<Token> tokens = lexer.tokenize();

        // "5 3 +"
        //  ^     position 1
        Token first = tokens.get(0);
        assertEquals(1, first.line());
        assertEquals(1, first.column());

        //  5 3 +
        //    ^   position 3
        Token second = tokens.get(1);
        assertEquals(1, second.line());
        assertEquals(3, second.column());

        //  5 3 +
        //      ^ position 5
        Token third = tokens.get(2);
        assertEquals(1, third.line());
        assertEquals(5, third.column());
    }

    @Test
    void testMultilinePositionTracking() throws RpnException {
        Lexer lexer = new Lexer("5 3\n+ 2");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        // Line 1
        assertEquals(1, tokens.get(0).line());
        assertEquals(1, tokens.get(0).column());
        assertEquals(1, tokens.get(1).line());
        assertEquals(3, tokens.get(1).column());

        // Line 2
        assertEquals(2, tokens.get(2).line());
        assertEquals(1, tokens.get(2).column());
        assertEquals(2, tokens.get(3).line());
        assertEquals(3, tokens.get(3).column());
    }

    @Test
    void testUnexpectedCharacterCaret() {
        Lexer lexer = new Lexer("2 3 ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
    }

    @Test
    void testUnexpectedCharacterAt() {
        Lexer lexer = new Lexer("5 @ 3");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Line 1, column 3: Unexpected character '@'", exception.getMessage());
        assertEquals(1, exception.getLine());
        assertEquals(3, exception.getColumn());
    }

    @Test
    void testUnexpectedCharacterInMiddle() {
        Lexer lexer = new Lexer("5 3 + 2 & 1");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertTrue(exception.getMessage().contains("Unexpected character '&'"));
    }

    @ParameterizedTest
    @ValueSource(strings = {"^", "%", "&", "#", "!", "~", "`", "?", ":", ";", ",", ".", "(", ")", "[", "]", "{", "}"})
    void testUnsupportedCharacters(String character) {
        Lexer lexer = new Lexer(character);
        assertThrows(RpnException.class, lexer::tokenize);
    }

    @Test
    void testFloatingPointFromContract() throws RpnException {
        Lexer lexer = new Lexer("3.14 2 *");
        List<Token> tokens = lexer.tokenize();

        assertEquals(4, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("3.14", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("2", tokens.get(1).value());
        assertEquals(TokenType.MULT, tokens.get(2).type());
        assertEquals(TokenType.EOF, tokens.get(3).type());
    }

    @Test
    void testContractCase_2_3_caret() {
        // Test case from I/O contract: "2 3 ^" should throw at column 5
        Lexer lexer = new Lexer("2 3 ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Line 1, column 5: Unexpected character '^'", exception.getMessage());
        assertEquals(1, exception.getLine());
        assertEquals(5, exception.getColumn());
        assertEquals("Unexpected character '^'", exception.getErrorMessage());
    }

    @Test
    void testContractCase_2_3_4_caret_caret() {
        // Test case: "2 3 4 ^ ^" should throw at column 7
        Lexer lexer = new Lexer("2 3 4 ^ ^");
        RpnException exception = assertThrows(RpnException.class, lexer::tokenize);

        assertEquals("Line 1, column 7: Unexpected character '^'", exception.getMessage());
        assertEquals(1, exception.getLine());
        assertEquals(7, exception.getColumn());
    }

    @Test
    void testMinusAsOperatorNotNegativeNumber() throws RpnException {
        // "5 3 - 2" should have minus as operator, not negative number
        Lexer lexer = new Lexer("5 3 - 2");
        List<Token> tokens = lexer.tokenize();

        assertEquals(5, tokens.size());
        assertEquals(TokenType.NUMBER, tokens.get(0).type());
        assertEquals("5", tokens.get(0).value());
        assertEquals(TokenType.NUMBER, tokens.get(1).type());
        assertEquals("3", tokens.get(1).value());
        assertEquals(TokenType.MINUS, tokens.get(2).type());
        assertEquals("-", tokens.get(2).value());
        assertEquals(TokenType.NUMBER, tokens.get(3).type());
        assertEquals("2", tokens.get(3).value());
    }

    @Test
    void testNullInputThrowsException() {
        assertThrows(NullPointerException.class, () -> new Lexer(null));
    }
}

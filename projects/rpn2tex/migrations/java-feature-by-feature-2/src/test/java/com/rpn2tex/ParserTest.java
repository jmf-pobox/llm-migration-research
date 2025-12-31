package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class ParserTest {

    @Test
    void testParseSingleNumber() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.EOF, "", 1, 2)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals("5", num.value());
        assertEquals(1, num.line());
        assertEquals(1, num.column());
    }

    @ParameterizedTest
    @ValueSource(strings = {"42", "3.14", "0", "0.5", "123.456"})
    void testParseVariousNumbers(String value) throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, value, 1, 1),
            new Token(TokenType.EOF, "", 1, value.length() + 1)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals(value, num.value());
    }

    @Test
    void testParseEmptyExpressionThrowsException() {
        List<Token> tokens = List.of(
            new Token(TokenType.EOF, "", 1, 1)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("Empty expression"));
    }

    @Test
    void testParseTooManyOperandsThrowsException() {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("Too many operands"));
    }

    @Test
    void testParseSimpleAddition() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("+", binOp.operator());

        assertInstanceOf(Number.class, binOp.left());
        assertEquals("5", ((Number) binOp.left()).value());

        assertInstanceOf(Number.class, binOp.right());
        assertEquals("3", ((Number) binOp.right()).value());
    }

    @Test
    void testParseChainedAddition() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "1", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "3", 1, 7),
            new Token(TokenType.PLUS, "+", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );

        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp outer = (BinaryOp) result;
        assertEquals("+", outer.operator());

        assertInstanceOf(BinaryOp.class, outer.left());
        BinaryOp inner = (BinaryOp) outer.left();
        assertEquals("+", inner.operator());
        assertEquals("1", ((Number) inner.left()).value());
        assertEquals("2", ((Number) inner.right()).value());

        assertInstanceOf(Number.class, outer.right());
        assertEquals("3", ((Number) outer.right()).value());
    }

    @Test
    void testParseInsufficientOperandsForAddition() {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.PLUS, "+", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testParseNoOperandsForAddition() {
        List<Token> tokens = List.of(
            new Token(TokenType.PLUS, "+", 1, 1),
            new Token(TokenType.EOF, "", 1, 2)
        );

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("requires two operands"));
    }
}

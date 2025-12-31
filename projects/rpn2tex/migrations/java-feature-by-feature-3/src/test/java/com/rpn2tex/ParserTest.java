package com.rpn2tex;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Parser class.
 */
class ParserTest {
    @Test
    void testParseSingleNumber() throws RpnException {
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "5", 1, 1),
                new Token(TokenType.EOF, "", 1, 2)
        );
        Parser parser = new Parser(tokens);
        Expr expr = parser.parse();

        assertInstanceOf(NumberExpr.class, expr);
        NumberExpr numberExpr = (NumberExpr) expr;
        assertEquals("5", numberExpr.value());
    }

    @Test
    void testParseDecimalNumber() throws RpnException {
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "3.14", 1, 1),
                new Token(TokenType.EOF, "", 1, 5)
        );
        Parser parser = new Parser(tokens);
        Expr expr = parser.parse();

        assertInstanceOf(NumberExpr.class, expr);
        NumberExpr numberExpr = (NumberExpr) expr;
        assertEquals("3.14", numberExpr.value());
    }

    @Test
    void testParseNegativeNumber() throws RpnException {
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "-5", 1, 1),
                new Token(TokenType.EOF, "", 1, 3)
        );
        Parser parser = new Parser(tokens);
        Expr expr = parser.parse();

        assertInstanceOf(NumberExpr.class, expr);
        NumberExpr numberExpr = (NumberExpr) expr;
        assertEquals("-5", numberExpr.value());
    }

    @Test
    void testParseEmptyExpression() {
        List<Token> tokens = List.of(
                new Token(TokenType.EOF, "", 1, 1)
        );
        Parser parser = new Parser(tokens);
        assertThrows(ParserException.class, () -> parser.parse());
    }

    @Test
    void testParseTooManyOperands() {
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "5", 1, 1),
                new Token(TokenType.NUMBER, "3", 1, 3),
                new Token(TokenType.EOF, "", 1, 4)
        );
        Parser parser = new Parser(tokens);
        ParserException exception = assertThrows(ParserException.class, () -> parser.parse());
        assertTrue(exception.getMessage().contains("too many operands"));
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
        Expr expr = parser.parse();

        assertInstanceOf(BinaryOpExpr.class, expr);
        BinaryOpExpr binaryOpExpr = (BinaryOpExpr) expr;
        assertEquals("+", binaryOpExpr.operator());
        assertInstanceOf(NumberExpr.class, binaryOpExpr.left());
        assertInstanceOf(NumberExpr.class, binaryOpExpr.right());
        assertEquals("5", ((NumberExpr) binaryOpExpr.left()).value());
        assertEquals("3", ((NumberExpr) binaryOpExpr.right()).value());
    }

    @Test
    void testParseChainedAddition() throws RpnException {
        // "1 2 + 3 +" parses as (1 + 2) + 3
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "1", 1, 1),
                new Token(TokenType.NUMBER, "2", 1, 3),
                new Token(TokenType.PLUS, "+", 1, 5),
                new Token(TokenType.NUMBER, "3", 1, 7),
                new Token(TokenType.PLUS, "+", 1, 9),
                new Token(TokenType.EOF, "", 1, 10)
        );
        Parser parser = new Parser(tokens);
        Expr expr = parser.parse();

        assertInstanceOf(BinaryOpExpr.class, expr);
        BinaryOpExpr outerAdd = (BinaryOpExpr) expr;
        assertEquals("+", outerAdd.operator());
        assertInstanceOf(BinaryOpExpr.class, outerAdd.left());
        assertInstanceOf(NumberExpr.class, outerAdd.right());

        BinaryOpExpr innerAdd = (BinaryOpExpr) outerAdd.left();
        assertEquals("+", innerAdd.operator());
        assertEquals("1", ((NumberExpr) innerAdd.left()).value());
        assertEquals("2", ((NumberExpr) innerAdd.right()).value());
        assertEquals("3", ((NumberExpr) outerAdd.right()).value());
    }

    @Test
    void testParseAdditionInsufficientOperands() {
        List<Token> tokens = List.of(
                new Token(TokenType.NUMBER, "5", 1, 1),
                new Token(TokenType.PLUS, "+", 1, 3),
                new Token(TokenType.EOF, "", 1, 4)
        );
        Parser parser = new Parser(tokens);
        ParserException exception = assertThrows(ParserException.class, () -> parser.parse());
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testParseAdditionNoOperands() {
        List<Token> tokens = List.of(
                new Token(TokenType.PLUS, "+", 1, 1),
                new Token(TokenType.EOF, "", 1, 2)
        );
        Parser parser = new Parser(tokens);
        assertThrows(ParserException.class, () -> parser.parse());
    }
}

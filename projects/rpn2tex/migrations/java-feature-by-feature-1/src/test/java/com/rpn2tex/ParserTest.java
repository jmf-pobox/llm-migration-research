package com.rpn2tex;

import org.junit.jupiter.api.Test;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Parser class.
 */
public class ParserTest {
    @Test
    public void testParseSingleNumber() throws RpnException {
        Token numToken = new Token(TokenType.NUMBER, "5", 1, 1);
        Token eofToken = new Token(TokenType.EOF, "", 1, 2);
        List<Token> tokens = List.of(numToken, eofToken);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof Number);
        Number num = (Number) ast;
        assertEquals("5", num.value());
        assertEquals(1, num.line());
        assertEquals(1, num.column());
    }

    @Test
    public void testParseDecimalNumber() throws RpnException {
        Token numToken = new Token(TokenType.NUMBER, "3.14", 1, 1);
        Token eofToken = new Token(TokenType.EOF, "", 1, 5);
        List<Token> tokens = List.of(numToken, eofToken);

        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();

        assertTrue(ast instanceof Number);
        assertEquals("3.14", ((Number) ast).value());
    }

    @Test
    public void testEmptyExpression() {
        Token eofToken = new Token(TokenType.EOF, "", 1, 1);
        List<Token> tokens = List.of(eofToken);

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.message.contains("Empty expression"));
    }

    @Test
    public void testMultipleValuesRemaining() {
        Token num1 = new Token(TokenType.NUMBER, "5", 1, 1);
        Token num2 = new Token(TokenType.NUMBER, "3", 1, 3);
        Token eofToken = new Token(TokenType.EOF, "", 1, 4);
        List<Token> tokens = List.of(num1, num2, eofToken);

        Parser parser = new Parser(tokens);
        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.message.contains("values remain on stack"));
    }
}

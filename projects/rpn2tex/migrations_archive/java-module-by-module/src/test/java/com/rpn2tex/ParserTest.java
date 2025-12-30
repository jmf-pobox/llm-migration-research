package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the Parser class.
 *
 * <p>Tests cover:
 * <ul>
 *   <li>Basic parsing of numbers and binary operations</li>
 *   <li>Complex expressions with multiple operations</li>
 *   <li>Error cases (empty expression, insufficient operands, too many operands)</li>
 *   <li>Integration with Lexer and LaTeXGenerator for I/O contract validation</li>
 * </ul>
 */
class ParserTest {

    @Test
    @DisplayName("Parse single number")
    void parseSingleNumber() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "42", 1, 1),
            new Token(TokenType.EOF, "", 1, 3)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(Number.class, result);
        Number num = (Number) result;
        assertEquals("42", num.value());
    }

    @Test
    @DisplayName("Parse simple addition: 5 + 3")
    void parseSimpleAddition() throws RpnException {
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
    @DisplayName("Parse subtraction: 10 - 3")
    void parseSubtraction() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 4),
            new Token(TokenType.MINUS, "-", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("-", binOp.operator());
    }

    @Test
    @DisplayName("Parse multiplication: 4 * 7")
    void parseMultiplication() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "4", 1, 1),
            new Token(TokenType.NUMBER, "7", 1, 3),
            new Token(TokenType.MULT, "*", 1, 5),
            new Token(TokenType.EOF, "", 1, 6)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("*", binOp.operator());
    }

    @Test
    @DisplayName("Parse division: 10 / 2")
    void parseDivision() throws RpnException {
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "10", 1, 1),
            new Token(TokenType.NUMBER, "2", 1, 4),
            new Token(TokenType.DIV, "/", 1, 6),
            new Token(TokenType.EOF, "", 1, 7)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        assertInstanceOf(BinaryOp.class, result);
        BinaryOp binOp = (BinaryOp) result;
        assertEquals("/", binOp.operator());
    }

    @Test
    @DisplayName("Parse complex expression: (5 + 3) * 2")
    void parseComplexExpression() throws RpnException {
        // RPN: 5 3 + 2 *
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.PLUS, "+", 1, 5),
            new Token(TokenType.NUMBER, "2", 1, 7),
            new Token(TokenType.MULT, "*", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Root should be multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left child should be addition (5 + 3)
        assertInstanceOf(BinaryOp.class, mult.left());
        BinaryOp add = (BinaryOp) mult.left();
        assertEquals("+", add.operator());
        assertEquals("5", ((Number) add.left()).value());
        assertEquals("3", ((Number) add.right()).value());

        // Right child should be number 2
        assertInstanceOf(Number.class, mult.right());
        assertEquals("2", ((Number) mult.right()).value());
    }

    @Test
    @DisplayName("Parse deeply nested: 2 * (3 + 4)")
    void parseDeeplyNested() throws RpnException {
        // RPN: 2 3 4 + *
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "2", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.NUMBER, "4", 1, 5),
            new Token(TokenType.PLUS, "+", 1, 7),
            new Token(TokenType.MULT, "*", 1, 9),
            new Token(TokenType.EOF, "", 1, 10)
        );
        Parser parser = new Parser(tokens);
        Expr result = parser.parse();

        // Root should be multiplication
        assertInstanceOf(BinaryOp.class, result);
        BinaryOp mult = (BinaryOp) result;
        assertEquals("*", mult.operator());

        // Left child should be number 2
        assertInstanceOf(Number.class, mult.left());
        assertEquals("2", ((Number) mult.left()).value());

        // Right child should be addition (3 + 4)
        assertInstanceOf(BinaryOp.class, mult.right());
        BinaryOp add = (BinaryOp) mult.right();
        assertEquals("+", add.operator());
        assertEquals("3", ((Number) add.left()).value());
        assertEquals("4", ((Number) add.right()).value());
    }

    @Test
    @DisplayName("Error: Empty expression")
    void errorEmptyExpression() {
        List<Token> tokens = List.of(
            new Token(TokenType.EOF, "", 1, 1)
        );
        Parser parser = new Parser(tokens);

        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertEquals("Empty expression", exception.getMessage());
    }

    @Test
    @DisplayName("Error: Insufficient operands for operator")
    void errorInsufficientOperands() {
        // RPN: 5 +  (missing second operand)
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.PLUS, "+", 1, 3),
            new Token(TokenType.EOF, "", 1, 4)
        );
        Parser parser = new Parser(tokens);

        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("Not enough operands"));
    }

    @Test
    @DisplayName("Error: Too many operands")
    void errorTooManyOperands() {
        // RPN: 5 3 2 +  (leaves two values on stack: 5 and result of 3+2)
        List<Token> tokens = List.of(
            new Token(TokenType.NUMBER, "5", 1, 1),
            new Token(TokenType.NUMBER, "3", 1, 3),
            new Token(TokenType.NUMBER, "2", 1, 5),
            new Token(TokenType.PLUS, "+", 1, 7),
            new Token(TokenType.EOF, "", 1, 8)
        );
        Parser parser = new Parser(tokens);

        RpnException exception = assertThrows(RpnException.class, parser::parse);
        assertTrue(exception.getMessage().contains("values remain on stack"));
    }

    @Test
    @DisplayName("Integration: Basic addition through full pipeline")
    void integrationBasicAddition() throws RpnException {
        String input = "5 3 +";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$5 + 3$", output);
    }

    @Test
    @DisplayName("Integration: Basic subtraction through full pipeline")
    void integrationBasicSubtraction() throws RpnException {
        String input = "5 3 -";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$5 - 3$", output);
    }

    @Test
    @DisplayName("Integration: Basic multiplication through full pipeline")
    void integrationBasicMultiplication() throws RpnException {
        String input = "4 7 *";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$4 \\times 7$", output);
    }

    @Test
    @DisplayName("Integration: Basic division through full pipeline")
    void integrationBasicDivision() throws RpnException {
        String input = "10 2 /";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$10 \\div 2$", output);
    }

    @Test
    @DisplayName("Integration: Complex with parentheses: (5 + 3) * 2")
    void integrationComplexParentheses() throws RpnException {
        String input = "5 3 + 2 *";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$( 5 + 3 ) \\times 2$", output);
    }

    @Test
    @DisplayName("Integration: Precedence: 5 * 3 + 2")
    void integrationPrecedence() throws RpnException {
        String input = "5 3 * 2 +";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$5 \\times 3 + 2$", output);
    }

    @Test
    @DisplayName("Integration: Floating point numbers")
    void integrationFloatingPoint() throws RpnException {
        String input = "3.14 2 *";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$3.14 \\times 2$", output);
    }

    @Test
    @DisplayName("Integration: Chained addition: 1 + 2 + 3 + 4")
    void integrationChainedAddition() throws RpnException {
        String input = "1 2 + 3 + 4 +";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$1 + 2 + 3 + 4$", output);
    }

    @Test
    @DisplayName("Integration: Complex nested: (1 + 2) * (3 + 4)")
    void integrationComplexNested() throws RpnException {
        String input = "1 2 + 3 4 + *";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$( 1 + 2 ) \\times ( 3 + 4 )$", output);
    }

    @Test
    @DisplayName("Integration: Very complex: (10 / 2 + 3) * 4")
    void integrationVeryComplex() throws RpnException {
        String input = "10 2 / 3 + 4 *";
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        String output = generator.generate(ast);

        assertEquals("$( 10 \\div 2 + 3 ) \\times 4$", output);
    }
}

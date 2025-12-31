package com.rpn2tex;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the complete RPN to LaTeX conversion pipeline.
 * Tests the I/O contract specified in the migration specification.
 */
class IntegrationTest {

    @Test
    void testFeature1NumbersSimpleInteger() throws RpnException {
        String input = "5";
        String expected = "$5$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature1NumbersDecimal() throws RpnException {
        String input = "3.14";
        String expected = "$3.14$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "5, $5$",
        "3.14, $3.14$",
        "0, $0$",
        "42, $42$",
        "0.5, $0.5$",
        "123.456, $123.456$"
    })
    void testVariousNumbers(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testNumberWithWhitespace() throws RpnException {
        String input = "  5  \n";
        String expected = "$5$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testEmptyInput() {
        String input = "";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("Empty expression"));
    }

    @Test
    void testMultipleNumbersThrowsError() {
        String input = "5 3";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("Too many operands"));
    }

    @Test
    void testFeature2AdditionSimple() throws RpnException {
        String input = "5 3 +";
        String expected = "$5 + 3$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature2AdditionChained() throws RpnException {
        String input = "1 2 + 3 + 4 +";
        String expected = "$1 + 2 + 3 + 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "'5 3 +', '$5 + 3$'",
        "'1 2 + 3 + 4 +', '$1 + 2 + 3 + 4$'",
        "'0 0 +', '$0 + 0$'",
        "'1.5 0.5 +', '$1.5 + 0.5$'"
    })
    void testVariousAdditions(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testInsufficientOperandsForAddition() {
        String input = "5 +";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testSingleOperandForAddition() {
        String input = "+";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testFeature3SubtractionSimple() throws RpnException {
        String input = "5 3 -";
        String expected = "$5 - 3$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature3SubtractionChained() throws RpnException {
        String input = "5 3 - 2 -";
        String expected = "$5 - 3 - 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "'5 3 -', '$5 - 3$'",
        "'5 3 - 2 -', '$5 - 3 - 2$'",
        "'10 5 -', '$10 - 5$'",
        "'0 0 -', '$0 - 0$'",
        "'1.5 0.5 -', '$1.5 - 0.5$'"
    })
    void testVariousSubtractions(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testNegativeNumberSupport() throws RpnException {
        String input = "-5 3 +";
        String expected = "$-5 + 3$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testInsufficientOperandsForSubtraction() {
        String input = "5 -";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testSingleOperandForSubtraction() {
        String input = "-";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testFeature4MultiplicationSimple() throws RpnException {
        String input = "4 7 *";
        String expected = "$4 \\times 7$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature4MultiplicationWithAddition() throws RpnException {
        String input = "2 3 4 * +";
        String expected = "$2 + 3 \\times 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature4MultiplicationPrecedenceLeft() throws RpnException {
        String input = "5 3 * 2 +";
        String expected = "$5 \\times 3 + 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature4AdditionThenMultiply() throws RpnException {
        String input = "5 3 + 2 *";
        String expected = "$( 5 + 3 ) \\times 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "'4 7 *', '$4 \\times 7$'",
        "'2 3 4 * +', '$2 + 3 \\times 4$'",
        "'5 3 * 2 +', '$5 \\times 3 + 2$'",
        "'5 3 + 2 *', '$( 5 + 3 ) \\times 2$'",
        "'2 3 * 4 +', '$2 \\times 3 + 4$'",
        "'3.14 2 *', '$3.14 \\times 2$'"
    })
    void testVariousMultiplications(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testInsufficientOperandsForMultiplication() {
        String input = "5 *";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testSingleOperandForMultiplication() {
        String input = "*";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testFeature5DivisionSimple() throws RpnException {
        String input = "10 2 /";
        String expected = "$10 \\div 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature5DivisionChained() throws RpnException {
        String input = "100 10 / 5 / 2 /";
        String expected = "$100 \\div 10 \\div 5 \\div 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature5DivisionWithMultiplication() throws RpnException {
        String input = "10 2 / 5 *";
        String expected = "$10 \\div 2 \\times 5$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature5DivisionWithAddition() throws RpnException {
        String input = "10 2 / 3 + 4 *";
        String expected = "$( 10 \\div 2 + 3 ) \\times 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "'10 2 /', '$10 \\div 2$'",
        "'100 10 / 5 / 2 /', '$100 \\div 10 \\div 5 \\div 2$'",
        "'10 2 / 5 *', '$10 \\div 2 \\times 5$'",
        "'20 4 /', '$20 \\div 4$'",
        "'1.5 0.5 /', '$1.5 \\div 0.5$'"
    })
    void testVariousDivisions(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testInsufficientOperandsForDivision() {
        String input = "5 /";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    @Test
    void testSingleOperandForDivision() {
        String input = "/";
        RpnException exception = assertThrows(RpnException.class, () -> Main.convert(input));
        assertTrue(exception.getMessage().contains("requires two operands"));
    }

    // Feature 6: Precedence - Comprehensive tests for all edge cases

    @Test
    void testFeature6PrecedenceLeftAdditionWithMultiplication() throws RpnException {
        String input = "5 3 + 2 *";
        String expected = "$( 5 + 3 ) \\times 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceLeftAdditionWithMultiplication2() throws RpnException {
        String input = "2 3 + 4 *";
        String expected = "$( 2 + 3 ) \\times 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceRightAdditionWithMultiplication() throws RpnException {
        String input = "2 3 4 + *";
        String expected = "$2 \\times ( 3 + 4 )$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceBothSidesAdditionWithMultiplication() throws RpnException {
        String input = "1 2 + 3 4 + *";
        String expected = "$( 1 + 2 ) \\times ( 3 + 4 )$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceComplexDivisionAdditionMultiplication() throws RpnException {
        String input = "10 2 / 3 + 4 *";
        String expected = "$( 10 \\div 2 + 3 ) \\times 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceMultiplicationHigherThanAddition() throws RpnException {
        String input = "2 3 * 4 +";
        String expected = "$2 \\times 3 + 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceAdditionThenMultiplication() throws RpnException {
        String input = "2 3 4 * +";
        String expected = "$2 + 3 \\times 4$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceSubtractionWithMultiplication() throws RpnException {
        String input = "5 3 - 2 *";
        String expected = "$( 5 - 3 ) \\times 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceLeftAssociativitySubtraction() throws RpnException {
        String input = "5 3 - 2 -";
        String expected = "$5 - 3 - 2$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceLeftAssociativityDivision() throws RpnException {
        String input = "100 10 / 5 /";
        String expected = "$100 \\div 10 \\div 5$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @Test
    void testFeature6PrecedenceMixedMultiplicationDivision() throws RpnException {
        String input = "10 2 / 5 *";
        String expected = "$10 \\div 2 \\times 5$";
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }

    @ParameterizedTest
    @CsvSource({
        "'5 3 + 2 *', '$( 5 + 3 ) \\times 2$'",
        "'2 3 + 4 *', '$( 2 + 3 ) \\times 4$'",
        "'2 3 4 + *', '$2 \\times ( 3 + 4 )$'",
        "'1 2 + 3 4 + *', '$( 1 + 2 ) \\times ( 3 + 4 )$'",
        "'10 2 / 3 + 4 *', '$( 10 \\div 2 + 3 ) \\times 4$'"
    })
    void testFeature6PrecedenceIOContract(String input, String expected) throws RpnException {
        String actual = Main.convert(input);
        assertEquals(expected, actual);
    }
}

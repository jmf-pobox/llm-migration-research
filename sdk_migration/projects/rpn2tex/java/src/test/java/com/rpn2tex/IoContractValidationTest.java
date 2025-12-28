package com.rpn2tex;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.CsvSource;

import java.util.List;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * Validates the complete pipeline against the I/O contract.
 *
 * <p>This test suite ensures that all 18 successful test cases from the
 * I/O contract produce identical outputs to the Python implementation.</p>
 *
 * <p>Reference: /Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/io_contract.md</p>
 */
@DisplayName("I/O Contract Validation: Complete Pipeline")
class IoContractValidationTest {

    /**
     * Processes an RPN expression through the complete pipeline:
     * Input -> Lexer -> Parser -> LaTeXGenerator -> Output
     */
    private String process(String input) throws RpnException {
        Lexer lexer = new Lexer(input);
        List<Token> tokens = lexer.tokenize();
        Parser parser = new Parser(tokens);
        Expr ast = parser.parse();
        LaTeXGenerator generator = new LaTeXGenerator();
        return generator.generate(ast);
    }

    @ParameterizedTest(name = "Case {index}: {0}")
    @CsvSource(delimiter = '|', textBlock = """
        1  | 5 3 +                 | $5 + 3$                           | Basic addition
        2  | 5 3 -                 | $5 - 3$                           | Basic subtraction
        3  | 4 7 *                 | $4 \\times 7$                     | Basic multiplication with \\times symbol
        4  | 10 2 /                | $10 \\div 2$                      | Basic division with \\div symbol
        6  | 5 3 + 2 *             | $( 5 + 3 ) \\times 2$             | Addition result grouped in parentheses for multiplication
        7  | 5 3 * 2 +             | $5 \\times 3 + 2$                 | Multiplication has higher precedence than addition
        8  | 10 2 / 5 *            | $10 \\div 2 \\times 5$            | Division and multiplication are left-associative
        9  | 5 3 - 2 -             | $5 - 3 - 2$                       | Subtraction is left-associative
        10 | 100 10 / 5 / 2 /      | $100 \\div 10 \\div 5 \\div 2$    | Chained division is left-associative
        11 | 1 2 + 3 + 4 +         | $1 + 2 + 3 + 4$                   | Chained addition
        12 | 2 3 4 * +             | $2 + 3 \\times 4$                 | Multiplication has higher precedence than addition
        13 | 2 3 + 4 *             | $( 2 + 3 ) \\times 4$             | Addition grouped when multiplied
        14 | 2 3 4 + *             | $2 \\times ( 3 + 4 )$             | Addition grouped on right side
        15 | 2 3 * 4 +             | $2 \\times 3 + 4$                 | Multiplication has higher precedence than addition
        18 | 3.14 2 *              | $3.14 \\times 2$                  | Floating point operands supported
        19 | 1.5 0.5 +             | $1.5 + 0.5$                       | Floating point addition
        20 | 1 2 + 3 4 + *         | $( 1 + 2 ) \\times ( 3 + 4 )$     | Complex expression with multiple sub-expressions
        21 | 10 2 / 3 + 4 *        | $( 10 \\div 2 + 3 ) \\times 4$    | Complex expression with division and addition grouped
        """)
    @DisplayName("All 18 successful I/O contract test cases")
    void validateIoContract(int caseNumber, String input, String expected, String description) throws RpnException {
        String result = process(input);
        assertEquals(expected, result,
            String.format("Case %d failed: %s\nInput: %s\nExpected: %s\nActual: %s",
                caseNumber, description, input, expected, result));
    }
}

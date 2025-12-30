package com.rpn2tex;

/**
 * Demo class to visually verify error formatter output.
 * This is not a unit test, just for manual verification.
 */
public class ErrorFormatterDemo {

    public static void main(String[] args) {
        System.out.println("=== Error Formatter Output Demo ===\n");

        // I/O Contract Test Case 5: "2 3 ^"
        System.out.println("Test Case 5: Input '2 3 ^'");
        System.out.println("Expected error at line 1, column 5");
        System.out.println();
        ErrorFormatter formatter1 = new ErrorFormatter("2 3 ^");
        String output1 = formatter1.formatError("Unexpected character '^'", 1, 5);
        System.out.println(output1);
        System.out.println();

        // I/O Contract Test Case 17: "2 3 4 ^ ^"
        System.out.println("Test Case 17: Input '2 3 4 ^ ^'");
        System.out.println("Expected error at line 1, column 7");
        System.out.println();
        ErrorFormatter formatter2 = new ErrorFormatter("2 3 4 ^ ^");
        String output2 = formatter2.formatError("Unexpected character '^'", 1, 7);
        System.out.println(output2);
        System.out.println();

        // Multiline example
        System.out.println("Multiline Example:");
        System.out.println();
        String multilineSource = "5 3 +\n2 4 ^\n1 2 *";
        ErrorFormatter formatter3 = new ErrorFormatter(multilineSource);
        String output3 = formatter3.formatError("Unexpected character '^'", 2, 5, 1);
        System.out.println(output3);
        System.out.println();

        // Test with RpnException
        System.out.println("Using RpnException:");
        System.out.println();
        try {
            throw new RpnException("Not enough operands for operator", 1, 7);
        } catch (RpnException e) {
            ErrorFormatter formatter4 = new ErrorFormatter("5 +");
            String output4 = formatter4.formatError(e.getMessage(), e.getLine(), e.getColumn());
            System.out.println(output4);
        }
    }
}

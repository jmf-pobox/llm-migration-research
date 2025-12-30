package com.rpn2tex;

public class ErrorFormatterManualTest {
    public static void main(String[] args) {
        // Test case 1: Simple error
        ErrorFormatter formatter1 = new ErrorFormatter("5 3 ^");
        String result1 = formatter1.formatError("Unexpected character '^'", 1, 5);
        System.out.println("=== Test 1: Simple error ===");
        System.out.println(repr(result1));
        System.out.println(result1);

        // Test case 2: Multi-line with context
        String source2 = "line 1\nline 2 error\nline 3";
        ErrorFormatter formatter2 = new ErrorFormatter(source2);
        String result2 = formatter2.formatError("Error message", 2, 6, 1);
        System.out.println("\n=== Test 2: Multi-line context ===");
        System.out.println(repr(result2));
        System.out.println(result2);

        // Test case 3: No context
        ErrorFormatter formatter3 = new ErrorFormatter(source2);
        String result3 = formatter3.formatError("Error message", 2, 6, 0);
        System.out.println("\n=== Test 3: No context ===");
        System.out.println(repr(result3));
        System.out.println(result3);
    }

    static String repr(String s) {
        StringBuilder sb = new StringBuilder("\"");
        for (char c : s.toCharArray()) {
            if (c == '\n') sb.append("\\n");
            else if (c == '\r') sb.append("\\r");
            else if (c == '\t') sb.append("\\t");
            else sb.append(c);
        }
        sb.append("\"");
        return sb.toString();
    }
}

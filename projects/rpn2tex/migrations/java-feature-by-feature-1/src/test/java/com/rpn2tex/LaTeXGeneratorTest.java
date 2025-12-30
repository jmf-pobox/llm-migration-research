package com.rpn2tex;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for the LaTeXGenerator class.
 */
public class LaTeXGeneratorTest {
    @Test
    public void testGenerateNumber() {
        Number num = new Number("5", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);
        assertEquals("$5$", result);
    }

    @Test
    public void testGenerateDecimal() {
        Number num = new Number("3.14", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);
        assertEquals("$3.14$", result);
    }

    @Test
    public void testGenerateNegativeNumber() {
        Number num = new Number("-42", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);
        assertEquals("$-42$", result);
    }

    @Test
    public void testGenerateLargeNumber() {
        Number num = new Number("1234567890", 1, 1);
        LaTeXGenerator generator = new LaTeXGenerator();

        String result = generator.generate(num);
        assertEquals("$1234567890$", result);
    }
}

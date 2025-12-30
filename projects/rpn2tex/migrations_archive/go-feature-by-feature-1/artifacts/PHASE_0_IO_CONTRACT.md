# Phase 0: I/O Contract for rpn2tex Migration

## Overview

This document contains the ground truth outputs from the Python implementation
of rpn2tex. These outputs serve as the validation baseline for the Go migration.

**Implementation Location:** `source/` directory
**CLI Entry Point:** `source/cli.py`
**Generation Date:** 2025-12-29

## Numbers

### Test: Single integer
**Input:** `5`
**Expected:** `$5$`
**Actual:** `$5$`
**Status:** PASS

### Test: Decimal number
**Input:** `3.14`
**Expected:** `$3.14$`
**Actual:** `$3.14$`
**Status:** PASS

## Addition

### Test: Simple addition
**Input:** `5 3 +`
**Expected:** `$5 + 3$`
**Actual:** `$5 + 3$`
**Status:** PASS

### Test: Multiple additions
**Input:** `1 2 + 3 + 4 +`
**Expected:** `$1 + 2 + 3 + 4$`
**Actual:** `$1 + 2 + 3 + 4$`
**Status:** PASS

## Subtraction

### Test: Simple subtraction
**Input:** `5 3 -`
**Expected:** `$5 - 3$`
**Actual:** `$5 - 3$`
**Status:** PASS

### Test: Multiple subtractions
**Input:** `5 3 - 2 -`
**Expected:** `$5 - 3 - 2$`
**Actual:** `$5 - 3 - 2$`
**Status:** PASS

## Multiplication

### Test: Simple multiplication
**Input:** `4 7 *`
**Expected:** `$4 \times 7$`
**Actual:** `$4 \times 7$`
**Status:** PASS

### Test: Multiplication with addition
**Input:** `2 3 4 * +`
**Expected:** `$2 + 3 \times 4$`
**Actual:** `$2 + 3 \times 4$`
**Status:** PASS

## Division

### Test: Simple division
**Input:** `10 2 /`
**Expected:** `$10 \div 2$`
**Actual:** `$10 \div 2$`
**Status:** PASS

### Test: Multiple divisions
**Input:** `100 10 / 5 / 2 /`
**Expected:** `$100 \div 10 \div 5 \div 2$`
**Actual:** `$100 \div 10 \div 5 \div 2$`
**Status:** PASS

## Operator Precedence

### Test: Addition then multiplication
**Input:** `5 3 + 2 *`
**Expected:** `$( 5 + 3 ) \times 2$`
**Actual:** `$( 5 + 3 ) \times 2$`
**Status:** PASS

### Test: Addition then multiplication
**Input:** `2 3 + 4 *`
**Expected:** `$( 2 + 3 ) \times 4$`
**Actual:** `$( 2 + 3 ) \times 4$`
**Status:** PASS

### Test: Multiplication of sum
**Input:** `2 3 4 + *`
**Expected:** `$2 \times ( 3 + 4 )$`
**Actual:** `$2 \times ( 3 + 4 )$`
**Status:** PASS

### Test: Product of two sums
**Input:** `1 2 + 3 4 + *`
**Expected:** `$( 1 + 2 ) \times ( 3 + 4 )$`
**Actual:** `$( 1 + 2 ) \times ( 3 + 4 )$`
**Status:** PASS

### Test: Complex precedence
**Input:** `10 2 / 3 + 4 *`
**Expected:** `$( 10 \div 2 + 3 ) \times 4$`
**Actual:** `$( 10 \div 2 + 3 ) \times 4$`
**Status:** PASS

## Mixed Operations

### Test: Multiplication then addition
**Input:** `5 3 * 2 +`
**Expected:** `$5 \times 3 + 2$`
**Actual:** `$5 \times 3 + 2$`
**Status:** PASS

### Test: Division and multiplication
**Input:** `10 2 / 5 *`
**Expected:** `$10 \div 2 \times 5$`
**Actual:** `$10 \div 2 \times 5$`
**Status:** PASS

### Test: Multiplication then addition
**Input:** `2 3 * 4 +`
**Expected:** `$2 \times 3 + 4$`
**Actual:** `$2 \times 3 + 4$`
**Status:** PASS

## Floating Point

### Test: Float multiplication
**Input:** `3.14 2 *`
**Expected:** `$3.14 \times 2$`
**Actual:** `$3.14 \times 2$`
**Status:** PASS

### Test: Float addition
**Input:** `1.5 0.5 +`
**Expected:** `$1.5 + 0.5$`
**Actual:** `$1.5 + 0.5$`
**Status:** PASS

## Unsupported (Error Cases)

### Test: Exponentiation operator not supported
**Input:** `2 3 ^`
**Expected:** `ERROR`
**Actual:** `ERROR`
**Error:** `Error: Unexpected character '^'`
**Status:** PASS (Error as expected)

### Test: Exponentiation with other operators
**Input:** `2 3 ^ 4 *`
**Expected:** `ERROR`
**Actual:** `ERROR`
**Error:** `Error: Unexpected character '^'`
**Status:** PASS (Error as expected)

### Test: Chained exponentiation
**Input:** `2 3 4 ^ ^`
**Expected:** `ERROR`
**Actual:** `ERROR`
**Error:** `Error: Unexpected character '^'`
**Status:** PASS (Error as expected)

## Test Results Summary

- **Total Tests:** 23
- **Passed:** 23
- **Failed:** 0
- **Pass Rate:** 100.0%

## Error Cases Detail

### Input: `2 3 ^`
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

### Input: `2 3 ^ 4 *`
```
Error: Unexpected character '^'

1 | 2 3 ^ 4 *
  |     ^
```

### Input: `2 3 4 ^ ^`
```
Error: Unexpected character '^'

1 | 2 3 4 ^ ^
  |       ^
```

## Validation Notes

1. All outputs include dollar sign delimiters (`$...$`) for LaTeX math mode
2. Spaces around operators follow LaTeX conventions:
   - Addition: ` + `
   - Subtraction: ` - `
   - Multiplication: ` \times `
   - Division: ` \div `
3. Operator precedence is handled via parentheses when needed
4. Caret operator (`^`) is not supported (reserved for future use)


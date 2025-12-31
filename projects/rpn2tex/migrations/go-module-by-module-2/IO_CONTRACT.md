# I/O Contract for rpn2tex Migration

## Implementation Details

- **Source:** `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- **Language:** Python 3.10+
- **Entry Point:** `rpn2tex.cli.main()`
- **Input Method:** stdin via `-` argument
- **Output Format:** LaTeX math mode expressions (e.g., `$...$`)

## Test Results Summary

- **Total Tests:** 21
- **Passed:** 18
- **Failed:** 3

## Test Cases

| # | Input | Expected Output | Status |
|---|-------|-----------------|--------|
| 1 | `5 3 +` | `$5 + 3$` | PASS |
| 2 | `5 3 -` | `$5 - 3$` | PASS |
| 3 | `4 7 *` | `$4 \times 7$` | PASS |
| 4 | `10 2 /` | `$10 \div 2$` | PASS |
| 5 | `2 3 ^` | `ERROR: Error: Unexpected character '^'` | FAIL |
| 6 | `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | PASS |
| 7 | `5 3 * 2 +` | `$5 \times 3 + 2$` | PASS |
| 8 | `10 2 / 5 *` | `$10 \div 2 \times 5$` | PASS |
| 9 | `5 3 - 2 -` | `$5 - 3 - 2$` | PASS |
| 10 | `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | PASS |
| 11 | `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | PASS |
| 12 | `2 3 4 * +` | `$2 + 3 \times 4$` | PASS |
| 13 | `2 3 + 4 *` | `$( 2 + 3 ) \times 4$` | PASS |
| 14 | `2 3 4 + *` | `$2 \times ( 3 + 4 )$` | PASS |
| 15 | `2 3 * 4 +` | `$2 \times 3 + 4$` | PASS |
| 16 | `2 3 ^ 4 *` | `ERROR: Error: Unexpected character '^'` | FAIL |
| 17 | `2 3 4 ^ ^` | `ERROR: Error: Unexpected character '^'` | FAIL |
| 18 | `3.14 2 *` | `$3.14 \times 2$` | PASS |
| 19 | `1.5 0.5 +` | `$1.5 + 0.5$` | PASS |
| 20 | `1 2 + 3 4 + *` | `$( 1 + 2 ) \times ( 3 + 4 )$` | PASS |
| 21 | `10 2 / 3 + 4 *` | `$( 10 \div 2 + 3 ) \times 4$` | PASS |

## Success Cases (18)

### Details

**Input:** `5 3 +`  
**Output:** `$5 + 3$`

**Input:** `5 3 -`  
**Output:** `$5 - 3$`

**Input:** `4 7 *`  
**Output:** `$4 \times 7$`

**Input:** `10 2 /`  
**Output:** `$10 \div 2$`

**Input:** `5 3 + 2 *`  
**Output:** `$( 5 + 3 ) \times 2$`

**Input:** `5 3 * 2 +`  
**Output:** `$5 \times 3 + 2$`

**Input:** `10 2 / 5 *`  
**Output:** `$10 \div 2 \times 5$`

**Input:** `5 3 - 2 -`  
**Output:** `$5 - 3 - 2$`

**Input:** `100 10 / 5 / 2 /`  
**Output:** `$100 \div 10 \div 5 \div 2$`

**Input:** `1 2 + 3 + 4 +`  
**Output:** `$1 + 2 + 3 + 4$`

**Input:** `2 3 4 * +`  
**Output:** `$2 + 3 \times 4$`

**Input:** `2 3 + 4 *`  
**Output:** `$( 2 + 3 ) \times 4$`

**Input:** `2 3 4 + *`  
**Output:** `$2 \times ( 3 + 4 )$`

**Input:** `2 3 * 4 +`  
**Output:** `$2 \times 3 + 4$`

**Input:** `3.14 2 *`  
**Output:** `$3.14 \times 2$`

**Input:** `1.5 0.5 +`  
**Output:** `$1.5 + 0.5$`

**Input:** `1 2 + 3 4 + *`  
**Output:** `$( 1 + 2 ) \times ( 3 + 4 )$`

**Input:** `10 2 / 3 + 4 *`  
**Output:** `$( 10 \div 2 + 3 ) \times 4$`


## Error Cases (3)

These inputs are not supported by the current Python implementation.

### Input: `2 3 ^`
**Status:** FAIL (Exit Code: 1)

**Error Message:**
```
Error: Unexpected character '^'

1 | 2 3 ^
  |     ^
```

### Input: `2 3 ^ 4 *`
**Status:** FAIL (Exit Code: 1)

**Error Message:**
```
Error: Unexpected character '^'

1 | 2 3 ^ 4 *
  |     ^
```

### Input: `2 3 4 ^ ^`
**Status:** FAIL (Exit Code: 1)

**Error Message:**
```
Error: Unexpected character '^'

1 | 2 3 4 ^ ^
  |       ^
```


## Notes

- The `^` (exponentiation) operator is not supported in the current Python implementation
- All supported operators are correctly converted to LaTeX:
  - `+` → `+` (addition)
  - `-` → `-` (subtraction)
  - `*` → `\times` (multiplication)
  - `/` → `\div` (division)
- Parentheses are added for proper operator precedence in the LaTeX output
- Decimal numbers are supported (e.g., 3.14, 1.5)

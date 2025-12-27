# Test Execution Report - rpn2tex Python Implementation

**Date:** 2025-12-27  
**Implementation:** Python rpn2tex at `/Users/jfreeman/Coding/rpn2tex/src/rpn2tex/`  
**Test Method:** Direct module invocation via stdin  
**Total Tests:** 21  
**Passed:** 18  
**Errors (Expected):** 3  

---

## Execution Summary

All 21 test cases were executed against the Python implementation. The results below document the exact output behavior needed for the Rust port.

### Success Rate by Category
- Basic Operations (1-4): 4/4 passed
- Precedence/Parentheses (6-14): 8/8 passed
- Associativity (9-10): 2/2 passed
- Floating-Point (18-19): 2/2 passed
- Complex Expressions (20-21): 2/2 passed
- **Exponentiation Errors (5, 16-17): 3/3 correct errors**

---

## Detailed Test Results

### Test 1: Basic Addition
**Input:** `5 3 +`  
**Output:** `$5 + 3$`  
**Status:** PASS  
**Exit Code:** 0  

### Test 2: Basic Subtraction
**Input:** `5 3 -`  
**Output:** `$5 - 3$`  
**Status:** PASS  
**Exit Code:** 0  

### Test 3: Basic Multiplication
**Input:** `4 7 *`  
**Output:** `$4 \times 7$`  
**Status:** PASS  
**Notes:** Operator `*` correctly mapped to LaTeX `\times`  
**Exit Code:** 0  

### Test 4: Basic Division
**Input:** `10 2 /`  
**Output:** `$10 \div 2$`  
**Status:** PASS  
**Notes:** Operator `/` correctly mapped to LaTeX `\div`  
**Exit Code:** 0  

### Test 5: Exponentiation (Not Supported)
**Input:** `2 3 ^`  
**Output:** (empty, error on stderr)  
**Stderr:** `Error: Unexpected character '^'`  
         ```
         1 | 2 3 ^
           |     ^
         ```  
**Status:** ERROR (expected)  
**Exit Code:** 0  
**Notes:** Caret operator is not implemented in the Python version. Error position correctly identifies column 5 (the `^` character).

### Test 6: Addition with Multiplication (Precedence)
**Input:** `5 3 + 2 *`  
**Output:** `$( 5 + 3 ) \times 2$`  
**Status:** PASS  
**Notes:** Addition (lower precedence) wrapped in parentheses inside multiplication (higher precedence)  
**Exit Code:** 0  

### Test 7: Multiplication then Addition
**Input:** `5 3 * 2 +`  
**Output:** `$5 \times 3 + 2$`  
**Status:** PASS  
**Notes:** No parentheses needed; multiplication has higher precedence  
**Exit Code:** 0  

### Test 8: Division and Multiplication Chain
**Input:** `10 2 / 5 *`  
**Output:** `$10 \div 2 \times 5$`  
**Status:** PASS  
**Notes:** Equal precedence, left-associative; no parentheses  
**Exit Code:** 0  

### Test 9: Subtraction Chain (Left-Associative)
**Input:** `5 3 - 2 -`  
**Output:** `$5 - 3 - 2$`  
**Status:** PASS  
**Notes:** Left side doesn't need parentheses; right side would in other contexts  
**Exit Code:** 0  

### Test 10: Division Chain (Left-Associative)
**Input:** `100 10 / 5 / 2 /`  
**Output:** `$100 \div 10 \div 5 \div 2$`  
**Status:** PASS  
**Notes:** Three-level chain maintains left-associativity without parentheses  
**Exit Code:** 0  

### Test 11: Addition Chain
**Input:** `1 2 + 3 + 4 +`  
**Output:** `$1 + 2 + 3 + 4$`  
**Status:** PASS  
**Notes:** Addition is commutative; no parentheses needed for left-associative chain  
**Exit Code:** 0  

### Test 12: Multiplication inside Addition
**Input:** `2 3 4 * +`  
**Output:** `$2 + 3 \times 4$`  
**Status:** PASS  
**Notes:** Multiplication (higher precedence) doesn't need parentheses inside addition  
**Exit Code:** 0  

### Test 13: Addition inside Multiplication (Left)
**Input:** `2 3 + 4 *`  
**Output:** `$( 2 + 3 ) \times 4$`  
**Status:** PASS  
**Notes:** Addition on LEFT of multiplication needs parentheses  
**Exit Code:** 0  

### Test 14: Addition inside Multiplication (Right)
**Input:** `2 3 4 + *`  
**Output:** `$2 \times ( 3 + 4 )$`  
**Status:** PASS  
**Notes:** Addition on RIGHT of multiplication needs parentheses  
**Exit Code:** 0  

### Test 15: Multiplication then Addition
**Input:** `2 3 * 4 +`  
**Output:** `$2 \times 3 + 4$`  
**Status:** PASS  
**Notes:** Higher-precedence multiplication doesn't need parentheses  
**Exit Code:** 0  

### Test 16: Exponentiation with Multiplication (Not Supported)
**Input:** `2 3 ^ 4 *`  
**Output:** (empty, error on stderr)  
**Stderr:** `Error: Unexpected character '^'`  
         ```
         1 | 2 3 ^ 4 *
           |     ^
         ```  
**Status:** ERROR (expected)  
**Exit Code:** 0  
**Notes:** Parser stops at first `^` character; column 5 is the error position  

### Test 17: Double Exponentiation (Not Supported)
**Input:** `2 3 4 ^ ^`  
**Output:** (empty, error on stderr)  
**Stderr:** `Error: Unexpected character '^'`  
         ```
         1 | 2 3 4 ^ ^
           |       ^
         ```  
**Status:** ERROR (expected)  
**Exit Code:** 0  
**Notes:** First `^` at column 7 triggers error; parser never reaches second `^`  

### Test 18: Floating-Point Multiplication
**Input:** `3.14 2 *`  
**Output:** `$3.14 \times 2$`  
**Status:** PASS  
**Notes:** Decimal numbers preserved exactly as input  
**Exit Code:** 0  

### Test 19: Floating-Point Addition
**Input:** `1.5 0.5 +`  
**Output:** `$1.5 + 0.5$`  
**Status:** PASS  
**Notes:** Multiple decimal operands work correctly  
**Exit Code:** 0  

### Test 20: Two Sub-Expressions with Multiplication
**Input:** `1 2 + 3 4 + *`  
**Output:** `$( 1 + 2 ) \times ( 3 + 4 )$`  
**Status:** PASS  
**Notes:** Both addition sub-expressions need parentheses inside multiplication  
**Exit Code:** 0  

### Test 21: Complex Mixed Operations
**Input:** `10 2 / 3 + 4 *`  
**Output:** `$( 10 \div 2 + 3 ) \times 4$`  
**Status:** PASS  
**Notes:** Division and addition on left side need parentheses inside multiplication. Demonstrates operator precedence with three operations and proper associativity.  
**Exit Code:** 0  

---

## Key Findings

### Spacing Convention
All outputs use consistent spacing:
- Space before and after all binary operators: ` + `, ` - `, ` \times `, ` \div `
- Space inside parentheses: `( expr )`

### Parenthesization Algorithm
The generator implements:
1. **Precedence Rule:** Child with lower precedence than parent always gets parentheses
2. **Associativity Rule:** Equal precedence on right side of `-` or `/` gets parentheses
3. **No Other Parentheses:** Addition chains, multiplication chains don't add unnecessary parentheses

### LaTeX Command Formatting
- Multiplication: `\times` (with backslash)
- Division: `\div` (with backslash)
- Both commands properly escaped in output

### Number Preservation
Numbers are NOT computed or transformed:
- `3.14` stays `3.14`
- `1.5` stays `1.5`
- No arithmetic evaluation happens

### Error Handling
- Unsupported operators produce LexerError
- Error message format: `Error: <message>`
- Error location format: Line number and column number (both 1-based)
- Visual error indicator shows the offending character
- Parse errors return exit code 0 (not 1)

---

## Implementation Quality Notes

### AST Structure
The Python implementation uses:
- `Number` nodes for numeric literals
- `BinaryOp` nodes for operations with left and right operands
- Proper position tracking (line, column) for error reporting

### Operator Mapping
| Python Token | LaTeX Output | Python Representation |
|--------------|--------------|----------------------|
| `+` | `+` | TokenType.PLUS |
| `-` | `-` | TokenType.MINUS |
| `*` | `\times` | TokenType.MULT |
| `/` | `\div` | TokenType.DIV |

### Parser Semantics
- RPN: operands before operators
- Stack-based approach
- Requires exactly one value after EOF
- Reports "too many operands" if stack has >1 value
- Reports "not enough operands" if operator lacks two operands

---

## Validation Checklist for Rust Port

- [ ] All 18 successful tests produce EXACT same output
- [ ] All 3 error tests produce EXACT same error message
- [ ] Parenthesization matches precedence/associativity rules
- [ ] LaTeX operators use `\times` and `\div`
- [ ] Spacing matches (spaces around operators and inside parentheses)
- [ ] Numbers preserved exactly as input
- [ ] Floating-point decimals work
- [ ] Error positions (line, column) correct
- [ ] Exit codes match (0 for all cases in tests)
- [ ] Output wrapped in `$...$`
- [ ] No trailing newline in output

---

## Files Generated

1. **IO_CONTRACT.md** - Complete specification document
2. **TEST_EXECUTION_REPORT.md** - This detailed report
3. **TEST_VALIDATION_QUICK_REF.md** - Quick reference table for validation


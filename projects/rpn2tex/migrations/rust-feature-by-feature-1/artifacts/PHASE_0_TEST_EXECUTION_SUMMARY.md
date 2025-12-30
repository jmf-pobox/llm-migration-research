# Test Execution Summary

**Execution Date**: 2025-12-29T14:04:21.866295
**Python Implementation**: /Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source

## Overall Results

- Total Tests: 23
- Passed: 20
- Failed: 3
- Success Rate: 87.0%

## Feature Breakdown

| Feature | Passed | Total | Success Rate |
|---------|--------|-------|--------------|
| numbers | 2 | 2 | 100% |
| addition | 2 | 2 | 100% |
| subtraction | 2 | 2 | 100% |
| multiplication | 2 | 2 | 100% |
| division | 2 | 2 | 100% |
| precedence | 5 | 5 | 100% |
| mixed_operators | 3 | 3 | 100% |
| floating_point | 2 | 2 | 100% |
| exponentiation | 0 | 3 | 0% |

## Numbers Tests

### [PASS] 5

**Expected**: `$5$`
**Actual**: `$5$`

### [PASS] 3.14

**Expected**: `$3.14$`
**Actual**: `$3.14$`

## Addition Tests

### [PASS] 5 3 +

**Expected**: `$5 + 3$`
**Actual**: `$5 + 3$`

### [PASS] 1 2 + 3 + 4 +

**Expected**: `$1 + 2 + 3 + 4$`
**Actual**: `$1 + 2 + 3 + 4$`

## Subtraction Tests

### [PASS] 5 3 -

**Expected**: `$5 - 3$`
**Actual**: `$5 - 3$`

### [PASS] 5 3 - 2 -

**Expected**: `$5 - 3 - 2$`
**Actual**: `$5 - 3 - 2$`

## Multiplication Tests

### [PASS] 4 7 *

**Expected**: `$4 \times 7$`
**Actual**: `$4 \times 7$`

### [PASS] 2 3 4 * +

**Expected**: `$2 + 3 \times 4$`
**Actual**: `$2 + 3 \times 4$`

## Division Tests

### [PASS] 10 2 /

**Expected**: `$10 \div 2$`
**Actual**: `$10 \div 2$`

### [PASS] 100 10 / 5 / 2 /

**Expected**: `$100 \div 10 \div 5 \div 2$`
**Actual**: `$100 \div 10 \div 5 \div 2$`

## Precedence Tests

### [PASS] 5 3 + 2 *

**Expected**: `$( 5 + 3 ) \times 2$`
**Actual**: `$( 5 + 3 ) \times 2$`

### [PASS] 2 3 + 4 *

**Expected**: `$( 2 + 3 ) \times 4$`
**Actual**: `$( 2 + 3 ) \times 4$`

### [PASS] 2 3 4 + *

**Expected**: `$2 \times ( 3 + 4 )$`
**Actual**: `$2 \times ( 3 + 4 )$`

### [PASS] 1 2 + 3 4 + *

**Expected**: `$( 1 + 2 ) \times ( 3 + 4 )$`
**Actual**: `$( 1 + 2 ) \times ( 3 + 4 )$`

### [PASS] 10 2 / 3 + 4 *

**Expected**: `$( 10 \div 2 + 3 ) \times 4$`
**Actual**: `$( 10 \div 2 + 3 ) \times 4$`

## Mixed_Operators Tests

### [PASS] 5 3 * 2 +

**Expected**: `$5 \times 3 + 2$`
**Actual**: `$5 \times 3 + 2$`

### [PASS] 10 2 / 5 *

**Expected**: `$10 \div 2 \times 5$`
**Actual**: `$10 \div 2 \times 5$`

### [PASS] 2 3 * 4 +

**Expected**: `$2 \times 3 + 4$`
**Actual**: `$2 \times 3 + 4$`

## Floating_Point Tests

### [PASS] 3.14 2 *

**Expected**: `$3.14 \times 2$`
**Actual**: `$3.14 \times 2$`

### [PASS] 1.5 0.5 +

**Expected**: `$1.5 + 0.5$`
**Actual**: `$1.5 + 0.5$`

## Exponentiation Tests

### [FAIL] 2 3 ^

**Expected**: `$2^{3}$`
**Actual**: ``
**Error**: Error: Unexpected character '^'

1 | 2 3 ^
  |     ^

### [FAIL] 2 3 ^ 4 *

**Expected**: `$2^{3} \times 4$`
**Actual**: ``
**Error**: Error: Unexpected character '^'

1 | 2 3 ^ 4 *
  |     ^

### [FAIL] 2 3 4 ^ ^

**Expected**: `$2^{3^{4}}$`
**Actual**: ``
**Error**: Error: Unexpected character '^'

1 | 2 3 4 ^ ^
  |       ^

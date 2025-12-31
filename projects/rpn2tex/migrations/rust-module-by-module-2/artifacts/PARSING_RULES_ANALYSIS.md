# Parsing Rules Analysis - rpn2tex

This document analyzes the parsing and precedence rules observed from the test execution.

## RPN to AST Conversion Rules

### RPN Stack-Based Evaluation
The implementation correctly handles Reverse Polish Notation:
1. Numbers are pushed onto the stack
2. Binary operators pop two operands and create operator nodes
3. The final stack should contain exactly one value (the root AST node)

### Test Case Analysis: RPN Conversion

```
Input: "5 3 +"
Stack: [5] -> [5, 3] -> (BinOp(5, +, 3))
Output: 5 + 3

Input: "5 3 + 2 *"
Stack: [5] -> [5, 3] -> (BinOp(5, +, 3)) -> (BinOp(5, +, 3), 2) -> (BinOp(BinOp(5, +, 3), *, 2))
Output: (5 + 3) × 2
Note: Parentheses added because + has lower precedence than *

Input: "5 3 * 2 +"
Stack: [5] -> [5, 3] -> (BinOp(5, *, 3)) -> (BinOp(5, *, 3), 2) -> (BinOp(BinOp(5, *, 3), +, 2))
Output: 5 × 3 + 2
Note: No parentheses because * has higher precedence than +
```

## Operator Precedence Rules

From the test cases, we can infer the following precedence hierarchy:

### Precedence Levels (highest to lowest)
1. **Level 2 (Highest):** Multiplication (*), Division (/)
2. **Level 1 (Lowest):** Addition (+), Subtraction (-)

### Precedence Examples

```
Case 1: Higher precedence on left
Input: "5 3 * 2 +"
Output: 5 × 3 + 2
Reason: * has higher precedence than +, so no parentheses needed

Case 2: Lower precedence on left
Input: "5 3 + 2 *"
Output: (5 + 3) × 2
Reason: + has lower precedence than *, so parentheses needed around left operand

Case 3: Same precedence
Input: "5 3 - 2 -"
Output: 5 - 3 - 2
Reason: Subtraction is left-associative, no parentheses needed
```

## Parenthesization Rules

The implementation adds parentheses based on the following logic:

### Rule 1: Left Operand Parenthesization
Parentheses are added around the left operand if:
- The left operand is a BinOp
- AND the left operand's operator has **lower precedence** than the current operator

```
Examples:
"5 3 + 2 *" -> (5 + 3) × 2  (+ < *, add parens)
"5 3 * 2 +" -> 5 × 3 + 2    (* > +, no parens)
"2 3 4 * +" -> 2 + 3 × 4    (left is number, no parens)
```

### Rule 2: Right Operand Parenthesization
Parentheses are added around the right operand if:
- The right operand is a BinOp
- AND the right operand's operator has **lower OR EQUAL precedence** than the current operator

```
Examples:
"2 3 4 + *" -> 2 × (3 + 4)    (+ < *, add parens)
"2 3 * 4 +" -> 2 × 3 + 4      (3 + 4 is evaluated as addition of constants, no parens)
"5 3 - 2 -" -> 5 - 3 - 2      (all subtraction, left-associative, no parens)
"10 2 / 5 *" -> 10 ÷ 2 × 5    (same precedence, left-associative)
```

### Rule 2 Detailed: Right Operand Lower Precedence
When the right operand's operator has lower precedence:
```
"2 3 4 + *"
AST: BinOp(2, *, BinOp(3, +, 4))
     ^Right operand has lower precedence (+)
Output: 2 × (3 + 4)
```

### Right Operand Same Precedence (Important!)
When operators have the same precedence, associativity matters:
```
"5 3 - 2 -"
AST: BinOp(BinOp(5, -, 3), -, 2)
     ^Right operand is just a number (2), not a BinOp
Output: 5 - 3 - 2

"10 2 / 5 *"
AST: BinOp(BinOp(10, /, 2), *, 5)
     ^Right operand is just a number (5), not a BinOp
Output: 10 ÷ 2 × 5
```

## Special Case: Right Operand with Equal Precedence

When the right operand is a BinOp with equal precedence, NO parentheses are added:
```
"2 3 * 4 +"
AST: BinOp(BinOp(2, *, 3), +, 4)
     The right operand is just 4 (number)
Output: 2 × 3 + 4

"1 2 + 3 + 4 +"
AST: BinOp(BinOp(BinOp(1, +, 2), +, 3), +, 4)
     Each right operand is a number
Output: 1 + 2 + 3 + 4
```

But when right operand IS a BinOp:
```
"1 2 + 3 4 + *"
AST: BinOp(BinOp(1, +, 2), *, BinOp(3, +, 4))
     Right operand is BinOp(3, +, 4) with lower precedence
Output: (1 + 2) × (3 + 4)
```

## Parenthesization Algorithm (Inferred)

```
function render_binop(node):
    left_latex = render(node.left)
    right_latex = render(node.right)
    op_symbol = get_operator_symbol(node.op)

    # Parenthesize left if needed
    if node.left is BinOp and precedence(node.left.op) < precedence(node.op):
        left_latex = "( " + left_latex + " )"

    # Parenthesize right if needed
    if node.right is BinOp and precedence(node.right.op) <= precedence(node.op):
        right_latex = "( " + right_latex + " )"

    return left_latex + " " + op_symbol + " " + right_latex

precedence table:
    + : 1
    - : 1
    * : 2
    / : 2
```

## Test Case Verification

Let's verify the algorithm against test cases:

### Test: "2 3 + 4 *"
AST: BinOp(BinOp(2, +, 3), *, 4)
1. Current: BinOp(?, *, 4), left=BinOp(2, +, 3)
2. Check left: precedence(+) = 1 < precedence(*) = 2 ? YES -> add parens
3. Check right: 4 is number, no parens
4. Result: "( 2 + 3 ) \times 4" ✓

### Test: "2 3 4 + *"
AST: BinOp(2, *, BinOp(3, +, 4))
1. Current: BinOp(2, *, ?), right=BinOp(3, +, 4)
2. Check left: 2 is number, no parens
3. Check right: precedence(+) = 1 <= precedence(*) = 2 ? YES -> add parens
4. Result: "2 \times ( 3 + 4 )" ✓

### Test: "5 3 * 2 +"
AST: BinOp(BinOp(5, *, 3), +, 2)
1. Current: BinOp(?, +, 2), left=BinOp(5, *, 3)
2. Check left: precedence(*) = 2 < precedence(+) = 1 ? NO -> no parens
3. Check right: 2 is number, no parens
4. Result: "5 \times 3 + 2" ✓

### Test: "100 10 / 5 / 2 /"
AST: BinOp(BinOp(BinOp(100, /, 10), /, 5), /, 2)
1. Leftmost: BinOp(100, /, 10) -> "100 \div 10"
2. Middle: BinOp(?, /, 5), left=result_of_previous
   - Check left: precedence(/) = 2 < precedence(/) = 2 ? NO -> no parens
3. Rightmost: similar
4. Result: "100 \div 10 \div 5 \div 2" ✓

## Floating-Point Handling

The implementation preserves decimal representation:
```
Input: "3.14 2 *"
AST: BinOp(3.14, *, 2)
Output: $3.14 \times 2$
Note: The number "3.14" is preserved exactly as input
```

## LaTeX Symbol Mapping

| Operator | LaTeX Output |
|----------|--------------|
| + | + |
| - | - |
| * | \times |
| / | \div |

## Space Formatting

All outputs follow this pattern:
- Single space before operator
- Single space after operator
- Parentheses with surrounding spaces: `( ... )`

Example: `$5 + 3$` (space on both sides of +)
Example: `$( 5 + 3 ) \times 2$` (space inside and outside parentheses)

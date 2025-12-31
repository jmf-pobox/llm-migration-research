# Review: Subtraction Feature

## Feature Overview
The subtraction feature adds support for the `-` operator in RPN expressions with left-associativity and proper handling of negative number literals.

---

## API Completeness

### Token Definitions (token.go)
- [x] TokenMinus constant defined (line 9)
- [x] Token struct with Type, Value, Line, Column fields (lines 14-19)

### Lexer (lexer.go)
- [x] NewLexer constructor (line 14)
- [x] Tokenize method returns ([]Token, error) (line 24)
- [x] scanToken handles minus operator (lines 102-114)
- [x] Negative number detection with lookahead (line 105)
- [x] Error handling via LexerError (line 116)

### Parser (parser.go)
- [x] NewParser constructor (line 12)
- [x] Parse method returns (Expr, error) (line 20)
- [x] TokenMinus handling in parse loop (line 34)
- [x] BinaryOp creation with minus operator (lines 59-65)
- [x] Stack-based RPN evaluation (lines 46-50)

### AST Nodes (ast.go)
- [x] BinaryOp struct with Operator field (lines 18-23)
- [x] Operator field stores "-" string (verified in tests)

### LaTeX Generator (latex.go)
- [x] Generate method wraps output in $...$ (line 15)
- [x] visit method handles BinaryOp nodes (line 24)
- [x] Operator rendered as-is from AST node (line 27)

---

## Behavioral Correctness

### Lexer Behavior
The lexer correctly:
- Tokenizes "-" as TokenMinus when followed by whitespace or end-of-input
- Tokenizes "-5" as a single NUMBER token when minus is immediately followed by a digit
- Uses lookahead to distinguish operator from negative prefix
- Preserves position information (line, column)

**Evidence:** Test `TestLexer_MinusOperator` (lexer_test.go:120-143) and `TestLexer_NegativeNumber` (lexer_test.go:145-162) both pass.

### Parser Behavior
The parser correctly:
- Pops two operands from the stack when encountering MINUS token
- Creates BinaryOp with operator="-"
- Preserves left-associativity through stack order
- For "5 3 - 2 -": builds BinaryOp("-", BinaryOp("-", 5, 3), 2)

**Evidence:** Test `TestParser_Subtraction` (parser_test.go:122-153) verifies correct AST structure.

### LaTeX Generator Behavior
The generator correctly:
- Renders BinaryOp with "-" operator as "{left} - {right}"
- For chained subtraction, renders naturally left-to-right without incorrect parentheses
- Output format: space-padded operators " - "

**Evidence:** Test `TestLaTeXGenerator_Subtraction` (latex_test.go:70-85) and `TestLaTeXGenerator_ChainedSubtraction` (latex_test.go:87-109) both pass.

---

## Test Coverage

### Unit Tests Exist
- [x] Lexer tests (lexer_test.go)
  - TestLexer_MinusOperator: Tokenizes "5 3 -" correctly
  - TestLexer_NegativeNumber: Handles "-5" as single token
  - TestLexer_MinusVsNegative: Distinguishes "-" operator from negative prefix with whitespace

- [x] Parser tests (parser_test.go)
  - TestParser_Subtraction: Builds correct AST with "-" operator
  - TestParser_SubtractionUnderflow: Errors when insufficient operands

- [x] LaTeX tests (latex_test.go)
  - TestLaTeXGenerator_Subtraction: Single subtraction "5 - 3"
  - TestLaTeXGenerator_ChainedSubtraction: Chained "5 - 3 - 2"

### Public API Coverage
- [x] NewLexer tested
- [x] Tokenize tested
- [x] NewParser tested
- [x] Parse tested
- [x] NewLaTeXGenerator tested
- [x] Generate tested

### Integration Tests
- [x] TestIntegration_Subtraction covers full pipeline
  - Input: "5 3 -" → Output: "$5 - 3$"
  - Input: "5 3 - 2 -" → Output: "$5 - 3 - 2$"

---

## I/O Contract Compliance

### Contract Validation
All test inputs from the analysis specification pass:

| Input | Expected Output | Go Output | Status |
|-------|-----------------|-----------|--------|
| `5 3 -` | `$5 - 3$` | `$5 - 3$` | PASS |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$5 - 3 - 2$` | PASS |

**Validation Method:** Ran custom I/O contract test (cmd/test/main.go) which confirms exact output matching.

### Output Format Details
- Operators rendered with spaces: " - " (not "-" or " -" or "- ")
- LaTeX wrapping: "$...$" with no trailing newline
- Left-associativity preserved in output: "5 - 3 - 2" (not "5 - (3 - 2)")
- Number formats preserved exactly as input

---

## Go Idioms

### Exported Identifiers Documentation
- [x] TokenType documented (token.go:3)
- [x] Token documented (token.go:13)
- [x] Lexer documented (lexer.go:5)
- [x] NewLexer documented (lexer.go:13)
- [x] Tokenize documented (lexer.go:23)
- [x] Parser documented (parser.go:5)
- [x] NewParser documented (parser.go:11)
- [x] Parse documented (parser.go:19)
- [x] BinaryOp documented (ast.go:17)
- [x] LaTeXGenerator documented (latex.go:5)
- [x] NewLaTeXGenerator documented (latex.go:9)
- [x] Generate documented (latex.go:14)

### Error Handling
- [x] All errors from Tokenize() checked (lexer.go:34-36)
- [x] All errors from Parse() checked (parser.go validation)
- [x] Errors wrapped with context where appropriate
- [x] No ignored error returns (blank identifier _)

### Code Quality
- [x] go fmt: No formatting issues (verified)
- [x] go vet: No static analysis issues (verified)
- [x] No unused variables
- [x] No unused imports
- [x] Proper use of slices for dynamic collections
- [x] Proper use of interfaces (Expr interface at line ast.go:4)
- [x] Type assertions with ok idiom in tests
- [x] No naked returns (all returns explicit)
- [x] No data races (simple stack-based algorithm, no concurrency)

### Package Organization
- [x] All files in rpn2tex package
- [x] Main program in cmd/rpn2tex/main.go (separate from library)
- [x] Test files follow naming convention (*_test.go)
- [x] Clear separation of concerns

---

## Backward Compatibility

### Previous Features Still Work
- [x] Addition feature still works (TestIntegration_Addition passes)
- [x] Numbers feature still works (TestIntegration_Numbers passes)
- [x] No breaking changes to public API

**Evidence:** All 31 test cases pass, including:
- Numbers: 2 tests
- Addition: 2 tests
- Subtraction: 2 tests
- Parser tests: 11 tests
- LaTeX tests: 6 tests
- Lexer tests: 8 tests

---

## Edge Cases Handled

### Negative Number vs Minus Operator
- [x] "-5" parsed as negative number (single NUMBER token)
- [x] "5 -3" with whitespace parsed as operator followed by negative number
- [x] "5 3 -" with whitespace parsed as operator token
- **Test:** TestLexer_MinusVsNegative (lexer_test.go:164-186)

### Insufficient Operands
- [x] "5 -" correctly errors (insufficient operands)
- **Test:** TestParser_SubtractionUnderflow (parser_test.go:155-167)

### Chained Operations (Associativity)
- [x] "5 3 - 2 -" renders as "5 - 3 - 2" (left-associative)
- [x] No incorrect parenthesization for equal-precedence operators
- **Test:** TestLaTeXGenerator_ChainedSubtraction (latex_test.go:87-109)

---

## Quality Gates Status

### All Checks Pass
- [x] Code formatting: PASS (go fmt)
- [x] Static analysis: PASS (go vet)
- [x] Unit tests: PASS (31/31 tests)
- [x] Integration tests: PASS (2/2 subtraction tests)
- [x] I/O contract: PASS (2/2 contract cases)
- [x] Error handling: PASS (all errors checked)
- [x] Documentation: PASS (all exported identifiers documented)
- [x] Go idioms: PASS (proper error handling, interfaces, type assertions)

---

## Verdict

### PASS

The Go implementation of the subtraction feature successfully migrates all Python behavior:

1. **API Completeness**: All required components present
   - TokenMinus token type
   - Minus operator tokenization with negative number detection
   - Parser stack operations for binary minus
   - LaTeX rendering with space-padded operator

2. **Behavioral Correctness**: Implementation matches Python specification
   - Lexer distinguishes "-" operator from "-" in negative numbers via lookahead
   - Parser creates correct BinaryOp AST nodes with left-associativity
   - LaTeX generator renders operators with proper spacing

3. **Test Coverage**: Comprehensive testing across all layers
   - Unit tests for each component
   - Integration tests for full pipeline
   - Edge cases handled (negative numbers, underflow, chaining)

4. **I/O Contract Compliance**: All test cases match expected outputs exactly
   - "5 3 -" → "$5 - 3$" ✓
   - "5 3 - 2 -" → "$5 - 3 - 2$" ✓

5. **Code Quality**: Follows Go idioms and best practices
   - Proper error handling and checking
   - All exported identifiers documented
   - No static analysis warnings
   - No data races or unsafe patterns

6. **Backward Compatibility**: Previous features remain functional
   - Numbers and addition features still work
   - No API changes
   - All 31 tests pass

**Summary**: The subtraction feature is production-ready. The implementation correctly handles operator precedence preparation (via proper AST structure), left-associativity through stack order, and the critical distinction between minus operator and negative number literals.

# I/O Contract Index

**Last Updated**: 2025-12-30
**Status**: Complete and Verified

## Document Index

### Primary Specification
- **PHASE_0_IO_CONTRACT.md** (141 lines)
  - Authoritative specification document
  - Complete test case definitions
  - Operator precedence and mapping
  - Output format requirements
  - **START HERE for implementation**

### Verification Evidence
- **VERIFICATION_REPORT.md** (256 lines)
  - Proof of successful testing
  - Individual test analysis
  - Error handling verification
  - Migration recommendations
  - **REFERENCE for implementation validation**

### Machine-Readable Data
- **TEST_DATA.json** (234 lines)
  - Structured test data (21 tests)
  - Operator definitions
  - Output format rules
  - **USE for automated testing**

### Navigation Guide
- **README.md** (172 lines)
  - Quick reference tables
  - File usage guide
  - Implementation checklist
  - **QUICK START for new implementers**

## Test Coverage Summary

### Categories Covered
1. **Basic Operations** (6 tests)
   - Single operators: +, -, *, /
   - Floating-point numbers
   - All tests: PASS

2. **Chained Operations** (3 tests)
   - Multiple sequential operators
   - Left-associativity verification
   - All tests: PASS

3. **Operator Precedence** (7 tests)
   - Parenthesization logic
   - Mixed precedence levels
   - Right-operand handling
   - All tests: PASS

4. **Complex Expressions** (2 tests)
   - Nested operations
   - Multiple operator types
   - All tests: PASS

5. **Unsupported Features** (3 tests)
   - Exponent operator (^)
   - Status: UNSUPPORTED (Phase 1)

## Key Specifications

### Operator Table
```
Operator  Input  Output    Precedence  Associativity
--------  -----  --------  -----------  -----------
Plus      +      +         1 (low)      Left
Minus     -      -         1 (low)      Left
Times     *      \times    2 (high)     Left
Divide    /      \div      2 (high)     Left
```

### Output Pattern
```
Format:    $<expression>$
Spacing:   single space around operators
Parens:    ( expr ) with spaces
Numbers:   preserved as input
```

### Precedence Rules
1. Lower precedence always needs parentheses
2. Equal precedence on right needs parens for non-commutative ops (-, /)
3. Higher precedence needs no parentheses

## Implementation Path

### For Language Migrations

**Phase 1: Specification Review**
- Read PHASE_0_IO_CONTRACT.md
- Understand operator definitions
- Review output format rules
- Study parenthesization logic

**Phase 2: Implementation**
- Implement lexer (tokenizer)
- Implement parser (RPN stack-based)
- Implement AST nodes
- Implement LaTeX generator
- Implement CLI

**Phase 3: Testing**
- Run all 18 supported tests
- Compare outputs exactly
- Fix any discrepancies
- Document results

**Phase 4: Validation**
- Generate verification report
- Confirm 100% test pass rate
- Document any deviations
- Mark implementation complete

### For Verification

1. Run implementation on each test input
2. Capture exact output
3. Compare with expected_output in TEST_DATA.json
4. Report pass/fail for each test
5. Calculate success rate

## Test Execution Results

### Summary Statistics
- Total Tests: 21
- Passed: 18 (100% of supported)
- Failed: 0
- Unsupported: 3 (exponent operator)
- **Success Rate: 100% for Phase 0 scope**

### Test Coverage
- Basic Operators: 6/6 (100%)
- Chaining: 3/3 (100%)
- Precedence: 7/7 (100%)
- Complex: 2/2 (100%)
- Unsupported: 3/3 (expected)

## Document Usage Guide

### PHASE_0_IO_CONTRACT.md
**Purpose**: Authoritative specification
**Read when**: You need to know exact expected behavior
**Contains**:
- Test case table (all 21 tests)
- Operator mappings
- Precedence levels
- Parenthesization rules
- Output format specifications
- Migration notes

**Best for**: Implementers, code review, specification disputes

### VERIFICATION_REPORT.md
**Purpose**: Proof of correct implementation
**Read when**: You need evidence of testing
**Contains**:
- Test execution results
- Individual test analysis
- Error handling verification
- Key findings summary
- Edge cases verified
- Migration guidance

**Best for**: QA, validation, progress verification

### TEST_DATA.json
**Purpose**: Machine-readable test data
**Read when**: You need to automate testing
**Contains**:
- 21 test cases in JSON format
- Metadata (counts, dates, status)
- Operator definitions
- Output format rules
- Individual test IDs and notes

**Best for**: Test automation, CI/CD pipelines, programmatic validation

### README.md
**Purpose**: Quick reference guide
**Read when**: You need a quick overview
**Contains**:
- Quick reference tables
- File navigation guide
- Test category descriptions
- Implementation checklist
- Key operators at a glance

**Best for**: New implementers, quick lookup, onboarding

## File Relationships

```
PHASE_0_IO_CONTRACT.md (Specification)
    |
    +-- Defines test cases, operators, format rules
    |
VERIFICATION_REPORT.md (Evidence)
    |
    +-- Shows tests pass the specification
    |
TEST_DATA.json (Data)
    |
    +-- Provides structured test data
    |
README.md (Navigation)
    |
    +-- Helps navigate all documents
```

## Implementation Checklist

### Before Implementation
- [ ] Read PHASE_0_IO_CONTRACT.md completely
- [ ] Understand operator precedence (1 for +/-, 2 for */)
- [ ] Review parenthesization rules
- [ ] Note output format requirements
- [ ] Understand left-associativity

### During Implementation
- [ ] Implement lexer for +, -, *, / only
- [ ] Implement RPN stack-based parser
- [ ] Implement LaTeX symbol mapping
- [ ] Implement precedence-based parenthesization
- [ ] Implement output formatting

### After Implementation
- [ ] Run all 18 supported tests
- [ ] Verify exact output match
- [ ] Run category by category:
  - [ ] Basic operations (tests 1-6)
  - [ ] Chaining (tests 7-9)
  - [ ] Precedence (tests 10-16)
  - [ ] Complex (tests 17-18)
- [ ] Document any deviations
- [ ] Sign off on verification

## Versions and Changes

| Version | Date | Changes | Status |
|---------|------|---------|--------|
| 1.0 | 2025-12-30 | Initial contract with 18 supported tests | Complete |

## Related Documents

- Python implementation source code: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source/`
- Verification script: `/Users/jfreeman/Coding/rpn2tex-rust-migration/verify_io_contract_extended.py`
- Migration status tracking: [See parent directory logs/]

## Quality Metrics

- Test coverage: 100% of Phase 0 operators
- Documentation: 803 total lines across 4 documents
- Machine readability: JSON format provided
- Specification clarity: 3-level hierarchy (overview, detail, data)
- Verification completeness: All 18 tests verified with exact output

## Contact

For questions about this contract:
1. Check README.md for quick answers
2. Check PHASE_0_IO_CONTRACT.md for detailed specs
3. Check VERIFICATION_REPORT.md for test details
4. Check TEST_DATA.json for structured data

## License and Attribution

These I/O contract artifacts document the Python rpn2tex implementation behavior as of 2025-12-30. They are provided as the specification for language migration projects.

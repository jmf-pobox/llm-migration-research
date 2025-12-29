# I/O Contract Package Index

## Package Overview

This directory contains the complete I/O contract for rpn2tex, generated from the Python reference implementation. All outputs have been captured by running the actual Python CLI on 21 test inputs.

**Generation Date:** 2025-12-28
**Source Implementation:** Python (rpn2tex)
**Target Implementation:** Java (rpn2tex-java)
**Status:** Complete and Verified

## Document Guide

### 1. README.md (START HERE)
Quick introduction and usage guide for the I/O contract package.
- Overview of all files in the package
- Quick start guide for Java developers
- Key test categories and statistics
- Critical implementation details

**Read this first for orientation.**

### 2. io_contract.md (AUTHORITATIVE SPECIFICATION)
The main I/O contract containing the complete specification.
- Executive summary of Python implementation behavior
- Full test case matrix (21 tests with inputs and outputs)
- Detailed observations on operator behavior
- Supported and unsupported features
- Error handling specifications
- Notes for Java migration

**Use this as the primary reference for implementation validation.**

### 3. DETAILED_TEST_RESULTS.md (VALIDATION CHECKLIST)
Comprehensive test-by-test breakdown with detailed analysis.
- Individual test cases with full context
- Output patterns and rules
- Parenthesization logic explained
- Precedence hierarchy documented
- Error handling specifications
- 21-item validation checklist for Java implementation

**Use this to validate your Java implementation against each test case.**

### 4. EXECUTION_SUMMARY.md (PROCESS DOCUMENTATION)
Executive summary of the I/O contract generation process.
- Execution methodology
- Test results breakdown by category
- Overall statistics
- Key findings and patterns
- Validation protocol
- Conclusion

**Read this to understand the scope and rigor of the contract.**

### 5. test_data.csv (MACHINE-READABLE DATA)
CSV file with all test data in structured format.
- Test number
- Input string
- Expected output
- Status (SUCCESS/ERROR)
- Test category
- Notes

**Use this for automated testing and CI/CD integration.**

## How to Use This Package

### For Implementation
1. Read README.md for orientation
2. Study io_contract.md for behavioral specification
3. Implement your Java version following the specification
4. Use test_data.csv to generate test cases

### For Testing & Validation
1. Run your Java implementation on each test case
2. Compare outputs with test_data.csv
3. Use DETAILED_TEST_RESULTS.md as checklist
4. Verify error handling matches Python implementation
5. Check exit codes (0 for success, 1 for errors)

### For Debugging
1. Refer to DETAILED_TEST_RESULTS.md for test-by-test analysis
2. Check io_contract.md for operator behavior rules
3. Review EXECUTION_SUMMARY.md for pattern explanations
4. Validate against test_data.csv for exact output matching

## Quick Statistics

| Metric | Value |
|--------|-------|
| Total Tests | 21 |
| Successful | 18 (85.7%) |
| Expected Errors | 3 (14.3%) |
| Test Categories | 6 |
| Operators Tested | 4 (supported) + 1 (unsupported) |
| Document Files | 4 |
| Test Data Lines | 21 |

## Key Specification Points

### Supported Operators
- Addition (+) - renders as `+`
- Subtraction (-) - renders as `-`
- Multiplication (*) - renders as `\times`
- Division (/)) - renders as `\div`

### Operator Precedence
1. Multiplication (*) and Division (/) - Higher precedence
2. Addition (+) and Subtraction (-) - Lower precedence

### Output Format
- LaTeX inline math mode: `$ ... $`
- Spaces around operators
- Parentheses for precedence: `( ... )`

### Error Handling
- Unsupported operator ^ produces: "Error: Unexpected character '^'"
- Errors sent to stderr
- Exit code 1 on error

## File Relationships

```
INDEX.md (this file)
├── README.md (orientation)
├── io_contract.md (specification)
├── DETAILED_TEST_RESULTS.md (validation)
├── EXECUTION_SUMMARY.md (process doc)
└── test_data.csv (test data)
```

## Next Steps

1. **For Implementation:**
   - Read README.md
   - Study io_contract.md
   - Code your Java implementation

2. **For Testing:**
   - Use test_data.csv to generate tests
   - Run against your implementation
   - Check DETAILED_TEST_RESULTS.md for validation

3. **For Quality Assurance:**
   - Verify all 21 tests pass
   - Confirm exit codes
   - Check error message format

## Questions or Issues?

Refer to the appropriate document:
- **"How do I get started?"** → README.md
- **"What exactly should my code do?"** → io_contract.md
- **"Does my code match Python?"** → DETAILED_TEST_RESULTS.md
- **"How was this generated?"** → EXECUTION_SUMMARY.md
- **"I need machine-readable test data"** → test_data.csv

## Verification Checklist

Before considering your Java implementation complete:

- [ ] Read all 4 markdown documents
- [ ] Understand operator precedence rules
- [ ] Know LaTeX output format
- [ ] Can explain error handling
- [ ] All 18 success tests pass exactly
- [ ] All 3 error cases handled correctly
- [ ] Exit codes are correct (0/1)
- [ ] Error messages match Python

## Implementation Status Tracking

Use this document to track implementation progress:

```
Status: [NOT STARTED / IN PROGRESS / TESTING / COMPLETE]

Successful Tests Passing: ___ / 18
Error Cases Handled: ___ / 3
Exit Codes Correct: [ ]
Error Messages Match: [ ]

Notes:
[Use this space to track issues or notes]
```

---

Generated: 2025-12-28
Contract Version: 1.0
Python Implementation: rpn2tex
Target Implementation: Java (rpn2tex-java)
Phase: 0 - Reference Implementation Analysis


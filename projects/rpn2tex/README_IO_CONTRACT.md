# rpn2tex I/O Contract Documentation

## Quick Reference

This directory contains the complete I/O contract for the rpn2tex project, verified against the Python implementation.

### Key Files

| File | Purpose | Audience |
|------|---------|----------|
| **I_O_CONTRACT_SUMMARY.md** | Executive summary and overview | Everyone |
| **IO_CONTRACT.md** | Detailed contract with all test cases | Developers, QA |
| **VERIFIED_TEST_CASES.md** | Per-test documentation | Implementers |
| **io_contract.json** | Machine-readable contract | Automated tools, Test frameworks |

---

## Starting Points

### I Want to Understand the Contract
Start here: **I_O_CONTRACT_SUMMARY.md**

This gives you:
- Executive summary of what's verified
- Test results organized by feature
- Output format specification
- Implementation guidance for migration

### I'm Implementing the Rust Version
Start here: **io_contract.json**

Use this to:
- Get exact expected outputs for each test case
- Automate test comparison
- Validate your implementation
- Report conformance metrics

### I Need Detailed Test Documentation
Start here: **VERIFIED_TEST_CASES.md**

You'll find:
- Detailed breakdown of each test
- Input/output pairs
- Success/failure status
- Explanations of error cases

### I Want the Complete Reference
Start here: **IO_CONTRACT.md**

This contains:
- All test results organized by feature
- Error case analysis
- Output format specification
- Implementation file locations
- How to run the CLI

---

## Quick Facts

- **Total Test Cases**: 20
- **Passing**: 17 (85%)
- **Not Supported**: 3 (exponentiation operator)
- **Unexpected Errors**: 0

### Supported Features
✓ Numbers (integers and decimals)
✓ Addition, subtraction, multiplication, division
✓ Operator precedence with automatic parenthesization
✓ Multi-operand expressions
✓ Error reporting with line/column info

### Not Supported
✗ Exponentiation operator (^)
✗ Square root function
✗ Nth root function

---

## Output Format at a Glance

All outputs are wrapped in LaTeX inline math mode: `$...$`

Examples:
```
"5" → "$5$"
"5 3 +" → "$5 + 3$"
"5 3 + 2 *" → "$( 5 + 3 ) \times 2$"
"10 2 /" → "$10 \div 2$"
```

Operators use:
- ` + ` for addition
- ` - ` for subtraction  
- ` \times ` for multiplication
- ` \div ` for division

---

## How to Verify Your Implementation

### Using the JSON Contract

```bash
# For each test case in io_contract.json:
# 1. Run your Rust implementation with the input
# 2. Compare output with expected_output
# 3. Verify all 17 passing cases match exactly
```

### Using Python Reference

```bash
# Run the original Python implementation:
echo "5 3 +" | python /projects/rpn2tex/source/cli.py -
# Output: $5 + 3$
```

### Automated Testing

You can parse `io_contract.json` to:
- Extract test cases programmatically
- Create regression test suites
- Report coverage metrics
- Validate output byte-for-byte

---

## File Structure

```
/projects/rpn2tex/
├── source/                    # Python implementation
│   ├── cli.py                # Entry point
│   ├── lexer.py              # Tokenization
│   ├── parser.py             # RPN parsing
│   ├── latex_gen.py          # Output generation
│   └── ...
│
├── I_O_CONTRACT_SUMMARY.md    # This phase's overview
├── IO_CONTRACT.md             # Complete detailed contract
├── VERIFIED_TEST_CASES.md     # Individual test documentation
└── io_contract.json           # Machine-readable format
```

---

## How We Verified This Contract

1. Located the Python source implementation
2. Identified the CLI entry point (`cli.py`)
3. Created a test harness to run all 20 test cases
4. Captured exact stdout output from each test
5. Documented all results with expected vs actual comparison
6. Analyzed implementation files to understand design decisions
7. Created comprehensive documentation in three formats

**Verification Date**: 2025-12-28  
**Implementation Version**: Python (as-is from source/)

---

## Important Notes

### About the Exponentiation Operator

Three test cases fail because the caret (^) operator is not implemented in the Python source. This is **intentional** - it's explicitly marked as an exercise in the code:

```python
# Exercise tokens (not implemented):
#     CARET: Exponentiation operator (^)
#     SQRT: Square root function (sqrt)
#     ROOT: Nth root function (root)
```

These failures are NOT bugs. The contract documents the actual behavior of the implementation.

### Output Consistency

All outputs are deterministic and consistent:
- No randomness
- No platform-specific behavior
- No rounding variations
- Exact spacing and symbols

You can rely on byte-for-byte matching.

---

## Next Steps

1. **For Rust Implementation**: Start with `io_contract.json` as your test oracle
2. **For Documentation**: Refer to `I_O_CONTRACT_SUMMARY.md`
3. **For Deep Understanding**: Read `IO_CONTRACT.md` in full
4. **For Each Test**: Check `VERIFIED_TEST_CASES.md` for detailed explanation

---

## Questions?

- **What's the exact output format?** See "Output Format Specification" in `I_O_CONTRACT_SUMMARY.md`
- **How do I run the reference implementation?** See "How to Run" section in `IO_CONTRACT.md`
- **What's not supported and why?** See "Not Supported Features" in each document
- **Can I use the JSON in my tests?** Yes! It's designed for that purpose
- **How do I know if my Rust version is correct?** Compare against `expected_output` in `io_contract.json`

---

## Documents Generated

This contract was generated by running all test cases through the Python implementation and capturing exact outputs. No manual editing or approximation was used.

**Status**: VERIFIED AND READY FOR MIGRATION

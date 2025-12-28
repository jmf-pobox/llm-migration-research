# rpn2tex I/O Contract Index

**Project**: rpn2tex (Python to Rust Migration)  
**Phase**: PHASE 0 - I/O Contract Verification  
**Status**: COMPLETE  
**Date**: 2025-12-28

---

## Document Guide

### For Quick Understanding (5-10 minutes)
**Start here**: `README_IO_CONTRACT.md`
- Navigation guide for all audiences
- Quick reference card
- Key facts at a glance
- FAQ section

### For Executive Summary (15 minutes)
**Read**: `I_O_CONTRACT_SUMMARY.md`
- Executive overview
- Test results by feature
- Output format specification
- Migration guidance

### For Detailed Reference (30 minutes)
**Study**: `IO_CONTRACT.md`
- Complete contract with all test cases
- Error analysis
- Implementation file references
- How to run the CLI

### For Per-Test Details (30 minutes)
**Review**: `VERIFIED_TEST_CASES.md`
- Individual test documentation
- Input/output pairs with descriptions
- Summary statistics by feature
- Implementation notes

### For Automation (varies)
**Use**: `io_contract.json`
- Machine-readable test cases
- Programmatic test oracle
- Feature mapping
- Metadata

### For Quick Lookup (1 minute)
**Reference**: `QUICK_REFERENCE.txt`
- Test cases and outputs
- Output format rules
- Supported vs unsupported features
- Quick start checklist

### For Technical Deep Dive (45 minutes)
**Read**: `PHASE_0_COMPLETION_REPORT.md`
- Complete verification methodology
- Source code analysis
- Implementation architecture
- Success criteria

---

## Quick Stats

| Metric | Value |
|--------|-------|
| Test Cases | 20 |
| Passing | 17 (85%) |
| Unsupported | 3 (exponentiation) |
| Documentation Files | 7 |
| Total Documentation | 42.8 KB |
| Python Source Location | projects/rpn2tex/source/ |

---

## Key Takeaways

### What Works (17 passing cases)
- Numbers (integers and decimals)
- Basic math operators (+, -, *, /)
- Operator precedence with parentheses
- Multi-operand expressions
- Error reporting with position info

### What Doesn't Work (3 cases)
- Exponentiation operator (^) - intentionally not implemented
- Marked as exercise in source code
- Proper error messages generated

### Output Format
```
$<LaTeX expression>$
```
Where LaTeX expression uses:
- ` + ` for addition
- ` - ` for subtraction
- ` \times ` for multiplication
- ` \div ` for division

---

## Finding Information

### Question: "What's the exact output for input X?"
Answer: See `io_contract.json` (test_cases array) or `QUICK_REFERENCE.txt`

### Question: "What features are supported?"
Answer: `README_IO_CONTRACT.md` (quick facts) or `I_O_CONTRACT_SUMMARY.md` (detailed)

### Question: "How do I implement the Rust version?"
Answer: `I_O_CONTRACT_SUMMARY.md` (migration section) and `io_contract.json` (test oracle)

### Question: "What's the output format exactly?"
Answer: `QUICK_REFERENCE.txt` (output format section) or `IO_CONTRACT.md` (detailed)

### Question: "Why do some tests fail?"
Answer: `PHASE_0_COMPLETION_REPORT.md` (error analysis) or `VERIFIED_TEST_CASES.md` (per-test)

### Question: "How was this verified?"
Answer: `PHASE_0_COMPLETION_REPORT.md` (verification methodology)

### Question: "Can I use this JSON in automated tests?"
Answer: Yes! See `README_IO_CONTRACT.md` (automated testing section)

---

## File Sizes

```
README_IO_CONTRACT.md (5.6K)       - Navigation and quick ref
I_O_CONTRACT_SUMMARY.md (7.4K)     - Executive summary
IO_CONTRACT.md (5.2K)              - Detailed contract
VERIFIED_TEST_CASES.md (4.2K)      - Per-test documentation
io_contract.json (6.4K)            - Machine-readable format
QUICK_REFERENCE.txt (4.3K)         - Quick lookup
PHASE_0_COMPLETION_REPORT.md (9.7K) - Technical analysis
INDEX.md (this file)               - Navigation
```

Total: approximately 42.8 KB of comprehensive documentation

---

## How to Use This Contract

### For Development

1. **Understanding Phase** (15 min)
   - Read `README_IO_CONTRACT.md`
   - Skim `I_O_CONTRACT_SUMMARY.md`

2. **Reference Phase** (ongoing)
   - Keep `QUICK_REFERENCE.txt` nearby
   - Use `io_contract.json` as test oracle

3. **Implementation Phase** (several days)
   - Build Rust version matching expected outputs
   - Compare against `io_contract.json`
   - Verify all 17 cases pass

4. **Testing Phase** (ongoing)
   - Parse `io_contract.json` for test cases
   - Create regression tests
   - Automate output validation

### For QA/Testing

1. Obtain `io_contract.json`
2. Parse test_cases array
3. For each test case:
   - Run input through implementation
   - Compare actual vs expected_output
   - Report any mismatches

### For Documentation

1. Reference `I_O_CONTRACT_SUMMARY.md` for readers
2. Link to `README_IO_CONTRACT.md` for navigation
3. Point developers to `io_contract.json` for details
4. Use `QUICK_REFERENCE.txt` in README or wiki

---

## Source Implementation

### Location
`/Users/jfreeman/Coding/rpn2tex-rust-migration/sdk_migration/projects/rpn2tex/source/`

### Key Files
- `cli.py` - Entry point and command orchestration
- `lexer.py` - Tokenization (handles +, -, *, /)
- `parser.py` - RPN parsing to AST
- `latex_gen.py` - LaTeX generation with precedence
- `tokens.py` - Token type definitions
- `errors.py` - Error formatting
- `ast_nodes.py` - AST node classes

### How to Run
```bash
# From stdin
echo "5 3 +" | python projects/rpn2tex/source/cli.py -

# From file
python projects/rpn2tex/source/cli.py input.rpn

# With output file
python projects/rpn2tex/source/cli.py input.rpn -o output.tex
```

---

## Next Steps

1. Review the appropriate documentation for your role
2. If implementing in Rust:
   - Start with `I_O_CONTRACT_SUMMARY.md`
   - Use `io_contract.json` as your test oracle
3. If testing/QA:
   - Use `io_contract.json` for test cases
   - Compare outputs with expected values
4. If documenting:
   - Reference `README_IO_CONTRACT.md`
   - Link to specific documents for details

---

## Questions?

- **How do I navigate this documentation?** Start with `README_IO_CONTRACT.md`
- **Where's the test oracle?** `io_contract.json`
- **What features are supported?** `I_O_CONTRACT_SUMMARY.md`
- **How do I verify my implementation?** Compare against `io_contract.json`
- **What's not supported and why?** `PHASE_0_COMPLETION_REPORT.md`

---

## Contract Status

- Verification: COMPLETE
- Documentation: COMPLETE
- All 20 tests executed: YES
- All outputs captured: YES
- Ready for implementation: YES

Generated: 2025-12-28

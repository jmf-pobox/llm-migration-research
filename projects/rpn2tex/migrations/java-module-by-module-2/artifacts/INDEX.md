# I/O Contract Files Index

## Quick Navigation

### Start Here
- **[README.md](./README.md)** - Master guide to all contract documentation

### For Implementation
- **[PHASE_0_IO_CONTRACT.md](./PHASE_0_IO_CONTRACT.md)** - Complete specification with all test cases

### For Quick Reference
- **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** - One-page guide with all essential info

### For Testing/Automation
- **[../io_contract.txt](../io_contract.txt)** - Machine-readable test case data

### For Deep Understanding
- **[EXECUTION_SUMMARY.txt](./EXECUTION_SUMMARY.txt)** - Technical methodology and architecture

## File Organization

```
migrations/java-module-by-module-2/
├── artifacts/
│   ├── INDEX.md                      (THIS FILE)
│   ├── README.md                     (Master guide)
│   ├── PHASE_0_IO_CONTRACT.md        (Main specification)
│   ├── QUICK_REFERENCE.md            (Quick lookup)
│   ├── EXECUTION_SUMMARY.txt         (Technical details)
│   └── EXECUTION_SUMMARY.txt         (Execution details)
└── io_contract.txt                   (Structured data format)
```

## Which File Should I Use?

### I'm Starting Implementation
→ Read **[README.md](./README.md)** then **[PHASE_0_IO_CONTRACT.md](./PHASE_0_IO_CONTRACT.md)**

### I Need Quick Implementation Reference
→ Use **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)**

### I Need to Understand the Design
→ Read **[EXECUTION_SUMMARY.txt](./EXECUTION_SUMMARY.txt)**

### I'm Setting Up Test Automation
→ Use **[../io_contract.txt](../io_contract.txt)** for parsing

### I Need Everything in One Place
→ Check **[PHASE_0_IO_CONTRACT.md](./PHASE_0_IO_CONTRACT.md)** for full details

## Content Summary

| File | Format | Lines | Purpose |
|------|--------|-------|---------|
| README.md | Markdown | 350+ | Master guide and usage instructions |
| PHASE_0_IO_CONTRACT.md | Markdown | 90+ | Complete test specification |
| QUICK_REFERENCE.md | Markdown | 81+ | One-page summary of key info |
| EXECUTION_SUMMARY.txt | Text | 242+ | Technical methodology and criteria |
| io_contract.txt | Text | 178+ | Machine-readable test data |
| INDEX.md | Markdown | This file | Navigation guide |

## Test Case Breakdown

- **Total Tests**: 21
- **Passing Tests**: 18 (85.7%)
- **Error Tests**: 3 (14.3%)

All test cases and their expected outputs are documented in:
- PHASE_0_IO_CONTRACT.md (human-readable table)
- QUICK_REFERENCE.md (condensed table)
- io_contract.txt (structured format)

## Key Specifications at a Glance

**Supported Operators**:
- `+` → ` + ` (addition)
- `-` → ` - ` (subtraction)
- `*` → ` \times ` (multiplication)
- `/` → ` \div ` (division)

**Not Supported**:
- `^` (exponentiation) - Must produce LexerError

**Output Format**:
- Wrapped in `$ ... $` (LaTeX math mode)
- Spaces around all operators
- Parentheses as `( ` and ` )`

**Precedence**: Multiplication/Division > Addition/Subtraction

## Next Steps

1. Choose your target language (Java, Go, or Rust)
2. Read [README.md](./README.md) for overview
3. Refer to [PHASE_0_IO_CONTRACT.md](./PHASE_0_IO_CONTRACT.md) while implementing
4. Use [QUICK_REFERENCE.md](./QUICK_REFERENCE.md) for specific rules
5. Run your implementation against all 21 test cases
6. Verify outputs match expected values exactly

## Additional Resources

- **Python Reference Implementation**: `/Users/jfreeman/Coding/rpn2tex-rust-migration/projects/rpn2tex/source`
- **Test Methodology**: See [EXECUTION_SUMMARY.txt](./EXECUTION_SUMMARY.txt)
- **Validation Criteria**: See [README.md](./README.md)

---

**Contract Version**: 1.0
**Generated**: 2025-12-29
**Status**: Ready for use in migrations

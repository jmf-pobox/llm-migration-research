# Code Migration with Claude Agent SDK: Experimental Notes

**Author:** James Freeman (Pembroke College, University of Oxford)
**Date:** December 2025

---

## What This Document Is

Notes from experiments using the Claude Agent SDK to migrate a trivial Python codebase to Rust and Java. This is not a framework or methodology - it's observations from prompt engineering experiments.

---

## Summary

We wrote prompts describing a migration task and passed them to the Claude Agent SDK. The SDK and model did the actual work. Both migrations produced working code with 100% behavioral equivalence on our test set.

| Target | Duration | Cost | I/O Match |
|--------|----------|------|-----------|
| Rust | ~25 min | $3.74 | 100% (21/21) |
| Java | ~25 min | $7.24 | 100% (21/21) |

### What We Built

~500 lines of Python:
- Prompt templates describing migration tasks
- YAML config parsing
- CLI wrapper around SDK call

### What Anthropic Built

Everything else:
- Claude Agent SDK (orchestration, tool execution, state management)
- Claude model (code understanding, generation, reasoning)

---

## Source Codebase

`rpn2tex` - an RPN to LaTeX converter. Deliberately trivial:

| Metric | Value |
|--------|-------|
| Production LOC | 352 |
| Modules | 7 |
| Avg Cyclomatic Complexity | 2.8 |
| External Dependencies | None |

---

## Experiments

We ran 5 experiments with Rust, then 1 with Java.

### Experiment Summary

| Run | Description | Duration | Cost | Result |
|-----|-------------|----------|------|--------|
| 1 | Baseline | 24 min | $4.47 | Works |
| 2 | Better paths | 17 min | $3.64 | Works |
| 3 | Embed source in prompt | 68 min | $5.04 | Works but slow |
| 4 | Describe phases in prompt | 25 min | $3.74 | 81% I/O match |
| 5 | Add I/O contract to prompt | ~45 min | ~$4.00 | 100% I/O match |
| 6 | Java target | 25 min | $7.24 | 100% I/O match |

### Key Observation: Embedding Source Made Things Worse

Run 3 embedded all Python source in the prompt. Result: 4x slower, 38% more expensive.

Why: Every sub-agent inherited the full context. Large contexts = slow responses.

### Key Observation: I/O Contracts Helped

Without pre-capturing expected outputs (Runs 1-4), the migrations produced code that worked but had different output formatting.

After adding "run the Python first and capture outputs" to the prompt (Run 5+), outputs matched exactly.

---

## What the Prompts Look Like

We send one prompt describing what we want:

```
Migrate this Python codebase to Rust.

First, run the Python on these test inputs and capture the outputs.
Then read all the source files and write an analysis.
Then migrate each module and verify outputs match.
Then review each module.

Use these sub-agents:
- io_contract: run Python, capture outputs
- analyst: read source, write analysis
- migrator: write Rust code
- reviewer: check the code
```

The model figures out how to execute this. We don't implement any orchestration logic.

---

## Code Metrics

| Metric | Python | Rust | Java |
|--------|--------|------|------|
| Production LOC | 352 | 408 | 529 |
| Function count | 25 | 32 | 42 |
| Avg cyclomatic complexity | 2.8 | 2.4 | 2.9 |

---

## Test Coverage

| Language | Line Coverage | Tests |
|----------|--------------|-------|
| Rust | 97.66% | 93 |
| Java | 95.87% | 226 |

---

## I/O Test Cases

21 input/output pairs we used to verify behavioral equivalence:

| Input | Expected Output |
|-------|-----------------|
| `5 3 +` | `$5 + 3$` |
| `4 7 *` | `$4 \times 7$` |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` |
| `5 3 - 2 -` | `$5 - 3 - 2$` |
| ... | ... |

Full list: 18 valid expressions, 3 error cases.

---

## Observations

### What Seemed to Help

1. **Pre-capturing expected outputs** - The model matched them exactly when given explicit targets
2. **Describing phases in the prompt** - More structured prompts gave more predictable results
3. **Including build commands** - The model ran them when we listed them

### What We Don't Know

- Whether this works for larger codebases
- Whether this works with external dependencies
- How much is prompt engineering vs. SDK behavior vs. model capability

### Limitations

- Only tested on trivial codebase (352 LOC, no dependencies)
- No control group (didn't test other approaches)
- "100% match" is on 21 test cases, not exhaustive
- We don't understand why some prompts work better than others

---

## Migration Strategy Comparison

### Experiment: Module-by-Module vs Feature-by-Feature

After validating module-by-module migration, we tested an alternative strategy: feature-by-feature migration. Both strategies migrated rpn2tex from Python to Rust.

### Strategy Definitions

**Module-by-Module**: Vertical slices. Complete one module (e.g., `lexer.py` → `lexer.rs`) before starting the next. Dependencies dictate order.

**Feature-by-Feature**: Horizontal slices. Complete one feature (e.g., "addition operator") across all modules (lexer → parser → generator) before starting the next feature.

### Results Comparison

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration | ~25 min | ~43 min |
| Total Source Lines | 2,530 | 1,057 |
| Production Code | 1,184 | 931 |
| Inline Test Code | 1,346 | 126 |
| Integration Tests | 0 | 24 |
| Total Tests | 93 | 51 |
| External Dependencies | 2 (clap, thiserror) | 0 |
| I/O Contract Match | 100% | 100% |

### Key Finding: Duration Difference

Feature-by-feature took 72% longer (43 vs 25 minutes) due to incremental I/O validation after each of the 6 feature slices. This overhead is acceptable for larger codebases where catching errors earlier saves rework.

### Key Finding: Production Code Difference

The 58% total LOC difference (2,530 vs 1,057) was misleading. Actual breakdown:

- **Test location**: Module-by-module has 74 unit tests inline in source files; feature-by-feature has tests in separate `tests/` directory
- **Production code**: Only 21% difference (1,184 vs 931 lines)
- **Coding style**: Module-by-module used external crates and more elaborate error formatting

### Output Verification

Both strategies produce **identical output** for all test cases:

```
Input: 5 3 + 2 *
Module-by-module: $( 5 + 3 ) \times 2$
Feature-by-feature: $( 5 + 3 ) \times 2$

Input: 100 10 / 5 / 2 /
Module-by-module: $100 \div 10 \div 5 \div 2$
Feature-by-feature: $100 \div 10 \div 5 \div 2$
```

### When to Use Each Strategy

**Module-by-Module** is better when:
- Clear module boundaries exist
- Modules have minimal cross-cutting concerns
- You want comprehensive unit test coverage per module

**Feature-by-Feature** is better when:
- Features span multiple modules
- Incremental I/O validation is important
- Codebase has high complexity functions that benefit from feature isolation

### Recommendation for Larger Codebases

For txt2tex (~10k LOC, avg CC 6.7, max CC 40), feature-by-feature is the recommended strategy because:
1. High-complexity functions (e.g., parser with CC=40) can be tackled incrementally
2. Each feature has its own I/O contract for validation
3. Dependency ordering is handled per-feature, not per-module

---

## Next Steps

1. Apply feature-by-feature strategy to txt2tex (~10k LOC)
2. Validate with external dependencies
3. Compare cost/time scaling between strategies

---

*December 2025*

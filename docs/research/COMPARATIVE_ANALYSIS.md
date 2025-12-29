# Code Migration with Claude Agent SDK: Experimental Notes

**Author:** James Freeman (Pembroke College, University of Oxford)
**Date:** December 2025

---

## What This Document Is

Notes from experiments using the Claude Agent SDK to migrate a trivial Python codebase to Rust, Java, and Go. This is not a framework or methodology - it's observations from prompt engineering experiments.

---

## Summary

We wrote prompts describing a migration task and passed them to the Claude Agent SDK. The SDK and model did the actual work. All migrations produced working code with 100% behavioral equivalence on our test set.

| Target | Strategy | Duration | Cost | I/O Match |
|--------|----------|----------|------|-----------|
| Rust | Module-by-module | ~32 min | $6.53 | 100% (21/21) |
| Rust | Feature-by-feature | ~32 min | $4.63 | 100% (21/21) |
| Java | Module-by-module | ~26 min | $4.92 | 100% (21/21) |
| Java | Feature-by-feature | ~51 min | $6.27 | 100% (21/21) |
| Go | Module-by-module | ~29 min | $6.85 | 100% (21/21) |
| Go | Feature-by-feature | ~32 min | $5.60 | 100% (21/21) |

**Key finding:** Strategy efficiency varies by target language. For Rust and Go, feature-by-feature saves ~18-30%. For Java, module-by-module saves ~22%.

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

| Metric | Python | Rust | Java | Go |
|--------|--------|------|------|-----|
| Production LOC | 352 | 579 | 368 | 781 |
| Function count | 25 | 38 | 31 | 42 |
| Avg cyclomatic complexity | 2.8 | 2.4 | 2.9 | 2.6 |

---

## Test Coverage

| Language | Strategy | Line Coverage | Tests |
|----------|----------|--------------|-------|
| Rust | Mod-by-mod | 96.9% | 24 |
| Rust | Feat-by-feat | 84.9% | 19 |
| Go | Mod-by-mod | 83.8% | 230 |
| Go | Feat-by-feat | 74.1% | 125 |
| Java | Mod-by-mod | --- | 0 |
| Java | Feat-by-feat | 72.0% | 50 |

**Notes:**
- Rust: Measured via cargo-llvm-cov; test counts are doctests
- Go: Built-in coverage reporting via `go test -cover`
- Java MbM: Standalone demo programs, not JUnit tests (no coverage possible)
- Java FbF: Measured via JaCoCo

---

## Tool Invocations

| Target | Strategy | Bash | Read | Write | Edit |
|--------|----------|------|------|-------|------|
| Rust | Mod-by-mod | 301 | 134 | 18 | 46 |
| Rust | Feat-by-feat | 191 | 100 | 10 | 33 |
| Java | Mod-by-mod | 268 | 99 | 21 | 5 |
| Java | Feat-by-feat | 235 | 153 | 23 | 25 |
| Go | Mod-by-mod | 255 | 114 | 34 | 9 |
| Go | Feat-by-feat | 110 | 80 | 16 | 41 |

**Observations:**
- Module-by-module uses more Bash invocations (avg 275 vs 179)
- Feature-by-feature shows higher Edit-to-Write ratios in some cases
- Go feature-by-feature notably efficient (110 Bash calls)

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

We tested two migration strategies on rpn2tex (Python → Rust). Both produced working code with 100% I/O contract match.

### Strategy Definitions

**Module-by-Module**: Vertical slices. Complete one module (e.g., `lexer.py` → `lexer.rs`) before starting the next. Dependencies dictate order.

**Feature-by-Feature**: Horizontal slices. Complete one feature (e.g., "addition operator") across all modules (lexer → parser → generator) before starting the next feature.

### Results (Rust Target, December 2025)

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration (wall clock) | ~32 min | ~32 min |
| Cost | $6.53 | $4.63 |
| Messages | 937 | 742 |
| API time | 55 min | 33 min |
| Subagent calls | 16 | 9 |
| I/O Contract Match | 100% | 100% |

### Key Finding: Cost Efficiency

Feature-by-feature costs **~30% less** than module-by-module with equivalent wall-clock time and identical output quality. The savings come from:

1. **Fewer API round-trips** (33 min vs 55 min API time)
2. **More focused subagent tasks** (9 vs 16 Task invocations)
3. **Better cache efficiency** on repeated patterns within features

### Key Finding: Both Strategies Succeed

Both strategies reliably produce:
- Working code that compiles
- 100% I/O contract match
- Passing test suites
- Clean linting (clippy, fmt)

The choice between strategies is about cost optimization, not correctness.

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

**Feature-by-Feature** (recommended default):
- Lower cost (~30% savings)
- Incremental I/O validation catches errors early
- Works well when features span multiple modules

**Module-by-Module**:
- Easier to reason about for simple codebases
- Better when modules are highly independent
- Produces more comprehensive per-module test suites

---

## Cross-Language Comparison

### Java Results (December 2025)

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration (wall clock) | ~26 min | ~51 min |
| Cost | $4.92 | $6.27 |
| Messages | 871 | 1026 |
| API time | 45 min | 54 min |
| Subagent calls | 16 | 14 |
| I/O Contract Match | 100% | 100% |

### Go Results (December 2025)

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration (wall clock) | ~29 min | ~32 min |
| Cost | $6.85 | $5.60 |
| Messages | 918 | 581 |
| API time | 57 min | 34 min |
| Subagent calls | 16 | 9 |
| I/O Contract Match | 100% | 100% |
| Production LOC | 781 | 613 |
| Test Coverage | 83.8% | 74.1% |
| Tests | 230 | 125 |

### Key Finding: Strategy Efficiency Varies by Target

| Target | Better Strategy | Cost Savings |
|--------|-----------------|--------------|
| Rust | Feature-by-feature | ~30% |
| Java | Module-by-module | ~22% |
| Go | Feature-by-feature | ~18% |

The pattern suggests:
- **Rust**: Feature-by-feature benefits from Rust's module system and cargo's incremental compilation
- **Go**: Feature-by-feature benefits from Go's flat package structure and fast compilation
- **Java**: Module-by-module benefits from Java's class-based structure where complete classes are easier to verify

### All Strategies Work

Regardless of which is more efficient, both strategies reliably produce:
- Working code that compiles
- 100% I/O contract match
- Passing test suites

The cost difference is optimization, not correctness.

---

## Next Steps

1. ~~Run both strategies on Java target for comparison~~ ✓ Done
2. ~~Run both strategies on Go target for comparison~~ ✓ Done
3. Apply strategies to txt2tex (~10k LOC) to test scaling
4. Validate with external dependencies
5. Investigate why strategy efficiency varies by target language

---

*December 2025*

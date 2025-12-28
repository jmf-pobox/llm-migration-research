# Migration Experiment Report

## Status

**Phase 1 complete:** Tested with trivial codebase, two target languages
**Next:** Test with larger codebase and external dependencies

## Results

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 | $7.24 |
| I/O Match | 21/21 | 21/21 |
| Test Coverage | 97.66% | 95.87% |

## Source Codebase

`rpn2tex` - Python RPN-to-LaTeX converter

| Metric | Value |
|--------|-------|
| Production LOC | 352 |
| Avg Cyclomatic Complexity | 2.8 |
| External Dependencies | None |

## Implementation

~500 lines of Python:
- Prompt templates (~250 LOC)
- SDK call + logging (~260 LOC)
- CLI and config parsing

The prompt describes:
- File paths and module order
- Build commands
- Test inputs and expected outputs
- Sub-agent definitions

The Claude Agent SDK handles execution.

## Complexity Metrics

| Metric | Python | Rust | Java |
|--------|--------|------|------|
| Production LOC | 352 | 408 | 529 |
| Function count | 25 | 32 | 42 |
| Avg cyclomatic complexity | 2.8 | 2.4 | 2.9 |

## Observations

1. Pre-capturing expected outputs (I/O contract) helped ensure behavioral equivalence
2. Structured prompts gave more predictable results than unstructured
3. Embedding source in prompts increased latency 4x

## Limitations

- Trivial codebase (352 LOC, no dependencies)
- 21 test cases
- Single source language (Python)

## Next Steps

Test with larger codebases that have external dependencies.

---

*December 2025*
*James Freeman, Pembroke College, University of Oxford*

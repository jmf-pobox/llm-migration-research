# Migration Experiment Report

## Status

**Phase 1 complete:** Tested with trivial codebase, two target languages, two migration strategies
**Next:** Test with larger codebase (txt2tex, ~10k LOC) and external dependencies

## Results

### By Target Language

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 | $7.24 |
| I/O Match | 21/21 | 21/21 |
| Test Coverage | 97.66% | 95.87% |

### By Migration Strategy (Rust)

| Metric | Module-by-Module | Feature-by-Feature |
|--------|------------------|-------------------|
| Duration | ~25 min | ~43 min |
| Production LOC | 1,184 | 931 |
| Test LOC | 1,346 | 126 (inline) + tests/ |
| Total Tests | 93 | 51 |
| I/O Match | 21/21 | 21/21 |

Both strategies produce **identical output** for all test cases.

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

## Migration Strategies

### Module-by-Module
Migrates each Python module completely before moving to the next. Natural order follows dependency graph: tokens → ast → error → lexer → parser → latex → main.

**Pros:** Clear progress tracking, each module independently testable
**Cons:** May not scale to complex interdependencies

### Feature-by-Feature
Migrates horizontal slices across all modules. Each feature (e.g., "addition operator") is migrated through lexer → parser → generator before moving to the next feature.

**Pros:** Incremental I/O validation, handles complex codebases better
**Cons:** Requires feature decomposition upfront

### Strategy Comparison Findings

1. **Identical outputs**: Both strategies produce byte-identical results for all test cases
2. **Production code**: Feature-by-feature produced 21% less production code (931 vs 1,184 lines)
3. **Test structure**: Module-by-module favors inline unit tests; feature-by-feature favors integration tests
4. **Dependencies**: Module-by-module used `clap`, `thiserror`; feature-by-feature used no dependencies

## Observations

1. Pre-capturing expected outputs (I/O contract) helped ensure behavioral equivalence
2. Structured prompts gave more predictable results than unstructured
3. Embedding source in prompts increased latency 4x
4. Both migration strategies are viable; choice depends on codebase characteristics
5. Feature-by-feature is better suited for larger codebases with cross-cutting concerns

## Limitations

- Trivial codebase (352 LOC, no dependencies)
- 21 test cases
- Single source language (Python)

## Next Steps

1. Test feature-by-feature strategy on txt2tex (~10k LOC)
2. Validate with external dependencies
3. Compare cost/time scaling between strategies

---

*December 2025*
*James Freeman, Pembroke College, University of Oxford*

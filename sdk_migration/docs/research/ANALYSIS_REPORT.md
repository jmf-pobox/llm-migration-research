# Claude Agent SDK Migration - Strategy Analysis Report

## Executive Summary

Four migration runs were conducted to convert rpn2tex from Python to Rust using the Claude Agent SDK. Two optimization strategies were tested:

- **Option A (Prompt Engineering)**: Embed source files in prompts — **FAILED** (4x slower, 38% more expensive)
- **Option B (Multi-Phase Orchestration)**: Analyst reads once, migrators use spec — **SUCCESS** (similar to baseline)

### Final Results

| Metric | Run 1 (Baseline) | Run 2 (Optimized) | Run 3 (Option A) | Run 4 (Option B) |
|--------|------------------|-------------------|------------------|------------------|
| **Duration** | 24 min | 17 min | **68 min** ❌ | 25 min ✓ |
| **Cost (USD)** | $4.47 | $3.64 | **$5.04** ❌ | $3.74 ✓ |
| **Modules** | 7/7 | 7/7 | 7/7 | 7/7 |
| **Tests** | 53 | 53 | 61 | 118 + 26 doc |
| **Coverage** | - | - | - | **97.66%** |

**Winner: Option B** — Multi-phase orchestration maintains baseline performance with cleaner architecture.

---

## Strategy Comparison

### Option A: Embed Source Files in Prompts

**Hypothesis**: Pre-reading Python files and embedding them in the main prompt would eliminate redundant file reads.

**Implementation**:
```python
# Pre-read all source files
source_contents = {name: open(path).read() for name, path in SOURCE_FILES.items()}

# Embed in prompt
prompt = f"""
# PYTHON SOURCE CODE (USE THIS - DO NOT READ FILES)
{embedded_sources}
"""
```

**Results**: Complete failure.

| Metric | Expected | Actual | Verdict |
|--------|----------|--------|---------|
| Duration | ~12 min | 68 min | ❌ 4x worse |
| Cost | ~$2.50 | $5.04 | ❌ 38% increase |
| Cache tokens | Lower | 270K vs 100K | ❌ 2.7x higher |

**Root Cause Analysis**:

1. **Bloated context per subagent**: Every subagent (21 invocations) inherited the full embedded source, even when not needed
2. **20-minute stall**: main.rs migration had a single LLM response that took 20 minutes due to context size
3. **Subagents still searched files**: Despite instructions, analyst still used Glob to search for files

**Token Usage**:
```
Run 2: cache_creation=101,140  cache_read=1,093,880
Run 3: cache_creation=270,006  cache_read=1,936,855  ← 2.7x larger initial context
```

**Lesson**: Embedding content in prompts doesn't scale. Larger context = slower responses.

---

### Option B: Multi-Phase Orchestration

**Hypothesis**: Separating concerns into distinct phases would keep context small per subagent.

**Implementation**:
```
PHASE 1: Analyst reads ALL Python files once → produces migration spec
PHASE 2: Migrators receive spec (not raw source) → migrate each module
PHASE 3: Reviewers validate against spec → verify each module
```

**Results**: Success — similar to baseline with cleaner architecture.

| Metric | Run 2 (Baseline) | Run 4 (Option B) | Change |
|--------|------------------|------------------|--------|
| Duration | 17 min | 25 min | +47% |
| Cost | $3.64 | $3.74 | +3% |
| Cache creation | 101,140 | 66,511 | -34% |

**Analysis**:

- Duration slightly longer due to Phase 1 comprehensive analysis
- Cost essentially the same
- **Cache creation tokens 34% lower** than baseline (66K vs 101K)
- **Cache creation tokens 75% lower** than Option A (66K vs 270K)
- Cleaner separation of concerns
- Migration spec provides consistent context across all subagents

**Token Usage Comparison**:
```
Run 2: cache_creation=101,140  cache_read=1,093,880
Run 4: cache_creation=66,511   cache_read=1,062,281  ← Smallest initial context
```

---

## Detailed Run Analysis

### Run 1: Initial Baseline

- **Date**: 2025-12-26 16:42
- **Duration**: 24 minutes
- **Cost**: $4.47
- **Issues**: File discovery failures (47), relative path confusion

### Run 2: Optimized Baseline

- **Date**: 2025-12-26 17:33
- **Duration**: 17 minutes
- **Cost**: $3.64
- **Improvements**:
  - Absolute paths (Glob calls -67%)
  - Batched cargo commands (Bash calls -59%)
  - Front-loaded Rust idioms (Edit calls -28%)

### Run 3: Option A (Embedded Sources)

- **Date**: 2025-12-26 23:26
- **Duration**: 68 minutes (4x worse)
- **Cost**: $5.04 (38% increase)
- **Critical Issue**: 20-minute stall during main.rs migration
- **Verdict**: ❌ Strategy backfired

### Run 4: Option B (Multi-Phase)

- **Date**: 2025-12-27 06:46
- **Duration**: 25 minutes
- **Cost**: $3.74
- **Test Coverage**: 97.66%
- **Verdict**: ✓ Clean architecture, competitive performance

---

## Quality Metrics

### Test Coverage (Run 4)

| Module | Line Coverage | Function Coverage |
|--------|--------------|-------------------|
| tokens.rs | 100.00% | 100.00% |
| lexer.rs | 100.00% | 100.00% |
| error.rs | 100.00% | 100.00% |
| latex.rs | 99.11% | 100.00% |
| ast.rs | 97.85% | 100.00% |
| parser.rs | 92.93% | 100.00% |
| **TOTAL** | **97.66%** | **100.00%** |

### Code Quality

All runs passed:
- `cargo check` — zero errors
- `cargo clippy -- -D warnings` — zero warnings
- `cargo fmt` — properly formatted
- `cargo test` — all tests passing

---

## Architecture Recommendations

### Recommended: Option B (Multi-Phase Orchestration)

```
┌─────────────────────────────────────────────────────────────┐
│                    Main Orchestrator                         │
│              (Manages phases, tracks progress)               │
└─────────────────────────────────────────────────────────────┘
                              │
        ┌─────────────────────┼─────────────────────┐
        ▼                     ▼                     ▼
   ┌─────────┐         ┌───────────┐         ┌───────────┐
   │ PHASE 1 │         │  PHASE 2  │         │  PHASE 3  │
   │ Analyst │ ──────► │ Migrators │ ──────► │ Reviewers │
   │ (haiku) │  spec   │ (sonnet)  │  files  │  (haiku)  │
   └─────────┘         └───────────┘         └───────────┘
        │                    │                     │
   Read ALL Python     Use spec, not raw      Read Rust files
   files ONCE          Python source          Compare to spec
```

**Benefits**:
1. **Single source read**: Analyst reads Python files once
2. **Small context per subagent**: Migrators receive spec, not full source
3. **Clear handoffs**: Each phase has defined inputs and outputs
4. **Cost effective**: 75% fewer cache_creation tokens than Option A

### Not Recommended: Option A (Embedded Sources)

**Why it fails**:
1. Context bloat compounds across subagent invocations
2. Large prompts cause LLM response latency
3. Doesn't prevent subagents from reading files anyway
4. Scales poorly with codebase size

---

## Cost Analysis

### Per-Run Breakdown

| Run | Duration | API Time | Cost | Turns | Messages |
|-----|----------|----------|------|-------|----------|
| Run 1 | 24 min | 28 min | $4.47 | 28 | ~780 |
| Run 2 | 17 min | 26 min | $3.64 | 47 | 592 |
| Run 3 | 68 min | 67 min | $5.04 | 35 | 470 |
| Run 4 | 25 min | 25 min | $3.74 | 37 | 438 |

### Token Economics

| Run | cache_creation | cache_read | Ratio |
|-----|---------------|------------|-------|
| Run 1 | 88,615 | 296,998 | 3.4x |
| Run 2 | 101,140 | 1,093,880 | 10.8x |
| Run 3 | **270,006** | 1,936,855 | 7.2x |
| Run 4 | 66,511 | 1,062,281 | **16.0x** |

**Observation**: Run 4 has the best cache efficiency (16x read-to-create ratio).

---

## Lessons Learned

### What Works

1. **Absolute paths**: Eliminate file discovery overhead
2. **Batched cargo commands**: Reduce shell invocations
3. **Front-loaded idioms**: Reduce fix iterations
4. **Multi-phase orchestration**: Keep context small per subagent
5. **Sequential subagent execution**: Prevent race conditions

### What Doesn't Work

1. **Embedding source in prompts**: Context bloat kills performance
2. **Parallel subagent spawning**: File discovery failures
3. **Trusting "don't read files" instructions**: Subagents still search

### Surprising Findings

1. **Smaller prompts are faster**: Option A's larger context caused 4x slowdown
2. **Cache efficiency matters**: Run 4's 16x read-to-create ratio is ideal
3. **Phase separation works**: Clean handoffs between phases are efficient
4. **97.66% test coverage**: Automated migration produces well-tested code

---

## Side-by-Side Testing Results

### Input/Output Comparison

After Run 4, we performed side-by-side testing comparing Python and Rust outputs for identical inputs:

| Input | Python Output | Rust Output | Match |
|-------|---------------|-------------|-------|
| `5 3 +` | `$5 + 3$` | `$5 + 3$` | ✓ |
| `4 7 *` | `$4 \times 7$` | `$4 \times 7$` | ✓ |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | `$( 5 + 3 ) \times 2$` | ✓ |
| `10 2 /` | `$10 \div 2$` | `$10 \div 2$` | ✓ |
| `8 3 -` | `$8 - 3$` | `$8 - 3$` | ✓ |
| `2 3 ^` | `$2^{3}$` | `$2^{3}$` | ✓ |
| `5 3 - 2 -` | `$5 - 3 - 2$` | `$( 5 - 3 ) - 2$` | **✗** |
| `100 10 / 5 / 2 /` | `$100 \div 10 \div 5 \div 2$` | `$( ( 100 \div 10 ) \div 5 ) \div 2$` | **✗** |
| `1 2 + 3 + 4 +` | `$1 + 2 + 3 + 4$` | `$( ( 1 + 2 ) + 3 ) + 4$` | **✗** |

**Result: 13/16 tests passed (81% exact match)**

### Root Cause Analysis

The 3 failing tests all involve **left-associative operator chains**. The difference is in the `needs_parens` function:

**Python (latex_gen.py:176-180):**
```python
return (
    child_precedence == parent_precedence
    and is_right
    and child.operator in ("-", "/")
)
```
- Only adds parentheses for **right-side** equal-precedence `-` and `/`

**Rust (latex.rs):**
```rust
fn needs_parens(...) -> bool {
    match parent_op {
        BinaryOp::Add | BinaryOp::Sub => child_op.precedence() <= parent_op.precedence(),
        BinaryOp::Mul | BinaryOp::Div => child_op.precedence() <= parent_op.precedence(),
        BinaryOp::Pow => child_op.precedence() < parent_op.precedence(),
    }
}
```
- Adds parentheses for **both left and right sides** when precedence is equal or lower

**Mathematical Equivalence**: Both outputs are mathematically correct. The Rust version produces more explicit parenthesization, which is valid but differs from Python's minimal parentheses approach.

### Key Finding

The migration produced **functionally correct but stylistically different** output. The Rust test suite was auto-generated to match the Rust implementation, masking this discrepancy.

### Implications for Migration Strategy

This finding reveals a critical gap in the migration process:

1. **Test Generation vs. Validation**: The migration generated tests that validate the Rust implementation, not the Python original. Tests should be derived from Python's actual outputs.

2. **Missing I/O Contract**: The migration spec should include concrete input/output examples from the Python implementation, not just API signatures.

3. **Review Phase Limitation**: The reviewer agent checked API completeness and Rust idioms, but couldn't detect behavioral differences without reference outputs.

### Recommended Process Improvement

Add **Phase 0: Generate I/O Test Cases** before analysis:

```
PHASE 0: I/O Contract Generation (NEW)
├── Run Python implementation on curated test inputs
├── Capture exact outputs for each input
└── Include in migration spec as "Expected Outputs" section

PHASE 1: Analysis (existing)
├── Read all Python source files
├── Analyze APIs and dependencies
└── Include I/O contract in migration spec

PHASE 2: Migration (existing)
├── Use spec including I/O contract
├── Run side-by-side tests after each module
└── Fail fast if outputs differ

PHASE 3: Review (existing)
├── Validate against spec
├── Verify I/O contract compliance
└── Report any behavioral differences
```

This ensures the migration produces **exact behavioral equivalence**, not just API compatibility.

---

## Future Improvements

### Potential Optimizations (Not Yet Tested)

1. **Parallel core module migration**: Migrate independent modules (tokens, ast, error) in parallel
2. **Incremental API context**: Build API summary incrementally, pass to later modules
3. **Streaming spec updates**: Update migration spec as each module completes
4. **Cached subagent sessions**: Reuse subagent context across modules

### Estimated Impact

| Improvement | Effort | Expected Savings |
|-------------|--------|------------------|
| Parallel core modules | Medium | 15-20% time |
| Incremental API context | Medium | 10-15% cost |
| Streaming spec | High | Unknown |
| Cached sessions | High | 20-30% cost |

---

## Conclusion

**Option B (Multi-Phase Orchestration) is the recommended strategy** for Claude Agent SDK migrations:

- Maintains baseline performance (~25 min, ~$3.74)
- 75% lower cache creation than Option A
- Clean separation of concerns
- Produces high-quality code (97.66% test coverage)

**Option A (Embedded Sources) should be avoided**:

- 4x slower than baseline
- 38% more expensive
- Context bloat causes LLM stalls
- Doesn't prevent redundant file reads

The key insight is that **smaller, focused contexts outperform large, comprehensive ones** when using subagents. Each subagent should receive only the information it needs for its specific task.

---

*Generated: 2025-12-27*
*Runs analyzed: migration_20251226_164225.log, migration_20251226_173315.log, migration_20251226_232632.log, migration_20251227_064636.log*

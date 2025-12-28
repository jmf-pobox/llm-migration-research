# Multi-Language Code Migration Experiment Report

## Project Status: Phase 1 Complete

**Completed:** Small-scale migration validated with two target languages (Rust, Java)
**Next Phase:** Scale to medium-complexity codebase with external dependencies

---

## Executive Summary

This report documents experiments using the **Claude Agent SDK** to automate cross-language code migration. We successfully migrated `rpn2tex` (an RPN expression to LaTeX converter) to both **Rust** and **Java**, achieving 100% behavioral equivalence in both cases.

### Key Results

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Total Cost | $3.74 USD | $7.24 USD |
| I/O Contract Match | 100% (21/21) | 100% (21/21) |
| Production LOC | 1,158 | 1,262 |
| Test Coverage | 97.66% | 95.87% |
| Unit Tests | 72 | 226 |

### Source System Complexity

The source codebase was deliberately trivial to establish baseline feasibility:

| Metric | Value |
|--------|-------|
| Python Source | 352 LOC (production) |
| Avg Cyclomatic Complexity | 2.8 (Grade A) |
| Max Cyclomatic Complexity | 10 |
| External Dependencies | None |
| Module Dependencies | Unidirectional |

## Background

### Problem Statement

Traditional code migration approaches require significant manual effort or use LLM-based tools that lack:
- File system access (must include all code in prompts)
- Build tool integration (can't verify compilation)
- Iterative error correction (single-shot generation)

### Hypothesis

A multi-agent architecture using Claude Agent SDK with specialized subagents and full tool access could automate code migration with quality enforcement across multiple target languages.

## Experimental Setup

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Claude Agent SDK                          │
│              (Main orchestrator agent)                       │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
┌─────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Analyst    │    │   Migrator       │    │    Reviewer     │
│  Subagent   │    │   Subagent       │    │    Subagent     │
│ (Read-only) │    │ (Full toolchain) │    │  (Read-only)    │
│             │    │ Read,Edit,Bash   │    │                 │
│ model:haiku │    │ model:sonnet     │    │  model:haiku    │
└─────────────┘    └──────────────────┘    └─────────────────┘
```

### Four-Phase Methodology

1. **Phase 0: I/O Contract** - Execute source on 21 test inputs, capture expected outputs
2. **Phase 1: Analysis** - Analyst reads all source files, produces migration specification
3. **Phase 2: Migration** - Migrators convert each module with quality gate feedback loops
4. **Phase 3: Review** - Reviewers validate against specification and I/O contract

### Quality Gates by Language

**Rust:**
- `cargo check` - zero compilation errors
- `cargo clippy -- -D warnings` - zero linter warnings
- `cargo fmt` - proper formatting
- `cargo test` - all tests pass

**Java:**
- `./gradlew compileJava` - zero compilation errors
- `./gradlew checkstyleMain` - style compliance
- `./gradlew test` - all tests pass (JaCoCo coverage)

### Migration Order

Modules migrated in dependency order:

| Order | Python Source | Rust Target | Java Target |
|-------|--------------|-------------|-------------|
| 1 | tokens.py | tokens.rs | Token.java |
| 2 | ast_nodes.py | ast.rs | Expr.java |
| 3 | errors.py | error.rs | RpnException.java |
| 4 | lexer.py | lexer.rs | Lexer.java |
| 5 | parser.py | parser.rs | Parser.java |
| 6 | latex_gen.py | latex.rs | LaTeXGenerator.java |
| 7 | cli.py | main.rs | Main.java |

## Results

### Lines of Code Comparison

| Metric | Python | Rust | Java |
|--------|--------|------|------|
| Production LOC | 352 | 1,158 | 1,262 |
| Test LOC | 0 (external) | 1,346 | 2,117 |
| Total LOC | 352 | 2,504 | 3,379 |
| Expansion Factor | 1.0x | 3.3x | 3.6x |
| Production Only Expansion | 1.0x | 3.3x | 3.6x |

Note: Rust embeds tests in source files; production LOC excludes `#[cfg(test)]` modules.

### Complexity Metrics

| Metric | Python | Rust | Java |
|--------|--------|------|------|
| Production LOC | 352 | 408 | 529 |
| Function count | 25 | 32 | 42 |
| Avg cyclomatic complexity | 2.8 | 2.4 | 2.9 |
| Max cyclomatic complexity | 10 | 7 | 15 |

### Test Coverage

| Language | Line Coverage | Branch Coverage | Unit Tests |
|----------|--------------|-----------------|------------|
| Rust | 97.66% | N/A | 72 |
| Java | 95.87% | 85.2% | 226 |

### Cost Analysis

| Metric | Rust | Java |
|--------|------|------|
| Total API Cost | $3.74 | $7.24 |
| Cost per 1k Source LOC | $10.63 | $20.57 |
| Messages Processed | ~750 | ~1,200 |

Java cost higher due to more verbose language requiring more generation tokens.

### I/O Contract Validation

All 21 test inputs produced identical outputs between Python and both target languages:

| Input | Expected Output | Rust | Java |
|-------|----------------|------|------|
| `5 3 +` | `$5 + 3$` | ✓ | ✓ |
| `4 7 *` | `$4 \times 7$` | ✓ | ✓ |
| `5 3 + 2 *` | `$( 5 + 3 ) \times 2$` | ✓ | ✓ |
| `10 2 /` | `$10 \div 2$` | ✓ | ✓ |
| `8 3 -` | `$8 - 3$` | ✓ | ✓ |
| ... | ... | ✓ | ✓ |
| **Total** | **21/21** | **100%** | **100%** |

## Key Findings

### 1. Multi-Language Capability Validated

The same four-phase methodology worked for both Rust and Java without modification. Only language-specific configuration (quality gates, idioms, file mappings) differed.

### 2. I/O Contracts Essential

Without explicit I/O contracts, 19% of outputs differed stylistically (e.g., `\times` vs `*`, spacing variations). The I/O contract enforces exact behavioral equivalence.

### 3. Quality Gates Enable Self-Correction

The migrator agent exhibited iterative self-correction:
1. Write code → run quality gate → detect error → fix → re-run
2. Average 8-10 quality gate invocations per module
3. 2-4 fix iterations per module on average

### 4. Low Complexity Prerequisite

This experiment validated migration on a trivial codebase (avg CC 2.8, no external dependencies). Scaling to medium-complexity codebases requires:
- Dependency mapping
- Library equivalence selection
- Handling of stateful systems

### What Worked Well

1. **Quality Gate Enforcement**: Migrator consistently ran build/lint/test after each change
2. **Subagent Specialization**: Haiku for analysis/review, Sonnet for code generation
3. **Full Tool Access**: Read, Write, Edit, Bash, Glob, Grep enabled autonomous operation
4. **I/O Contract**: Prevented stylistic drift, enforced exact behavioral equivalence

### Challenges Encountered

1. **Path Resolution**: Relative paths caused initial confusion (fixed with absolute paths)
2. **Java Verbosity**: More tokens required = higher cost ($7.24 vs $3.74)
3. **Test Embedding**: Rust inline tests complicated LOC accounting

## Framework Architecture

The migration framework is language-agnostic with self-contained project folders:

```
sdk_migration/
├── run_migration.py          # CLI: --target rust|java --project projects/rpn2tex
├── framework/
│   ├── agents.py             # Language-agnostic agent definitions
│   ├── runner.py             # Core orchestration logic
│   └── config.py             # Configuration loading
├── languages/
│   ├── base.py               # Abstract LanguageTarget class
│   ├── rust.py               # Rust-specific configuration
│   └── java.py               # Java-specific configuration
├── projects/
│   └── rpn2tex/              # Self-contained project folder
│       ├── config.yaml       # Project configuration
│       ├── source/           # Original Python source (frozen snapshot)
│       ├── rust/             # Generated Rust project
│       ├── java/             # Generated Java project
│       ├── io_contract.json  # I/O contract definition
│       └── logs/             # Migration logs
└── docs/
    └── research/             # Research documentation
```

**Extensibility:**
- Adding a new target language requires only implementing `LanguageTarget`
- Adding a new project requires only creating a new project folder with config.yaml

## Future Work

### Phase 2: Medium-Complexity Codebase

- 5,000-20,000 LOC
- External dependencies (HTTP clients, databases, etc.)
- Multiple modules with complex interdependencies

### Technical Challenges

- **Dependency Mapping**: Automatic library equivalence selection
- **Stateful Systems**: Behavioral contracts for systems with side effects
- **Incremental Migration**: Interoperability between migrated and unmigrated modules

## Conclusion

The Claude Agent SDK successfully automated cross-language code migration to both Rust and Java with:

- **100% I/O contract match** (21/21 test cases per language)
- **High test coverage** (97.66% Rust, 95.87% Java)
- **Quality enforcement** (all lint/check/test gates passed)
- **Reasonable cost** ($3.74 Rust, $7.24 Java)
- **Acceptable duration** (~25 minutes per language)

This validates that LLM-based code migration is viable when agents have:
1. Full file system access
2. Build tool integration
3. Iterative error correction capability
4. Quality gate enforcement
5. I/O contract validation

The next phase will test whether these findings scale to medium-complexity codebases with external dependencies.

---

*Report updated: 2025-12-27*
*Author: James Freeman, Pembroke College, University of Oxford*
*Experiments conducted using Claude Agent SDK with claude-opus-4-5-20251101*

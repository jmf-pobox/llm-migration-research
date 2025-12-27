# Claude Agent SDK Migration Experiment

Automated Python-to-Rust code migration using LLM-based multi-agent systems.

## Overview

This project demonstrates automated cross-language code migration using the Claude Agent SDK. The experiment migrated `rpn2tex` (a 990-line Python RPN-to-LaTeX converter) to Rust with 100% behavioral equivalence.

**Key Results:**
- Cost: $3.74 USD
- Duration: ~25 minutes
- I/O Match: 100% (21/21 tests)
- Test Coverage: 97.66%

## Quick Start

```bash
# Prerequisites
export ANTHROPIC_API_KEY=your-key
pip install claude-agent-sdk

# Run migration
python run_migration.py

# Dry run (preview only)
python run_migration.py --dry-run
```

## Project Structure

```
sdk_migration/
├── README.md                 # This file
├── run_migration.py          # Main orchestrator
├── agents.py                 # Subagent definitions
│
├── docs/
│   ├── research/
│   │   ├── COMPARATIVE_ANALYSIS.md   # Research paper
│   │   ├── ANALYSIS_REPORT.md        # Strategy comparison (4 runs)
│   │   └── MIGRATION_REPORT.md       # Run 1 experiment report
│   │
│   ├── io-contract/
│   │   ├── IO_CONTRACT.md            # Canonical I/O specification
│   │   └── TEST_EXECUTION_REPORT.md  # Detailed test results
│   │
│   └── migration/
│       ├── MIGRATION_SPEC.md         # Technical specification
│       └── MIGRATION_COMPLETE.md     # Final completion report
│
├── rpn2tex-rs/               # Generated Rust project
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── main.rs
│       ├── tokens.rs
│       ├── ast.rs
│       ├── error.rs
│       ├── lexer.rs
│       ├── parser.rs
│       └── latex.rs
│
└── logs/                     # Migration run logs
```

## Methodology

The migration uses a four-phase approach:

1. **Phase 0: I/O Contract** - Run Python on test inputs, capture expected outputs
2. **Phase 1: Analysis** - Analyst reads all Python files, produces migration spec
3. **Phase 2: Migration** - Migrators convert each module using the spec
4. **Phase 3: Review** - Reviewers validate against spec and I/O contract

## Agent Architecture

```
                    Main Orchestrator
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   io_contract        analyst           migrator         reviewer
   (haiku)           (haiku)           (sonnet)          (haiku)
```

**Model Selection:**
- Haiku: Analysis and validation (cost-optimized)
- Sonnet: Code generation (capability-optimized)

## Documentation

| Document | Description |
|----------|-------------|
| [COMPARATIVE_ANALYSIS.md](docs/research/COMPARATIVE_ANALYSIS.md) | Research paper with quantitative analysis |
| [IO_CONTRACT.md](docs/io-contract/IO_CONTRACT.md) | 21 test cases defining expected behavior |
| [MIGRATION_SPEC.md](docs/migration/MIGRATION_SPEC.md) | Technical specification for each module |
| [MIGRATION_COMPLETE.md](docs/migration/MIGRATION_COMPLETE.md) | Final status and quality metrics |

## Validation

```bash
cd rpn2tex-rs

# Build
cargo build --release

# Test
cargo test

# Verify I/O contract
echo "5 3 +" | ./target/release/rpn2tex -
# Output: $5 + 3$
```

## Key Findings

1. **Smaller contexts win** - Multi-phase orchestration outperforms embedded source (4x faster)
2. **I/O contracts are essential** - Without them, 19% of outputs differed stylistically
3. **Quality gates enable iteration** - Average 8.4 build verification cycles per module

## License

MIT

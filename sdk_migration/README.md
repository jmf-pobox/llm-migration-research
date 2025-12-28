# Multi-Language Code Migration Framework

Automated cross-language code migration using LLM-based multi-agent systems.

## Project Status: Phase 1 Complete

**Completed:** Small-scale migration validated with two target languages (Rust, Java)
**Next Phase:** Scale to medium-complexity codebase with external dependencies

## Overview

This project demonstrates automated cross-language code migration using the Claude Agent SDK. We migrated `rpn2tex` (a 990-line Python RPN-to-LaTeX converter) to both Rust and Java, achieving 100% behavioral equivalence in both cases.

### Key Results

| Metric | Rust | Java |
|--------|------|------|
| Duration | ~25 min | ~25 min |
| Cost | $3.74 USD | $7.24 USD |
| I/O Match | 100% (21/21) | 100% (21/21) |
| Production LOC | 1,158 | 1,262 |
| Test Coverage | 97.66% | 95.87% |

### Subject System Complexity

The source codebase was deliberately trivial to establish baseline feasibility:

| Metric | Value |
|--------|-------|
| Avg Cyclomatic Complexity | 2.8 (Grade A) |
| Max Cyclomatic Complexity | 10 |
| External Dependencies | None |
| Module Dependencies | Unidirectional |

## Quick Start

```bash
# Prerequisites
export ANTHROPIC_API_KEY=your-key
pip install claude-agent-sdk

# Run migration to Rust
python run_migration.py --target rust --project projects/rpn2tex

# Run migration to Java
python run_migration.py --target java --project projects/rpn2tex

# Dry run (preview only)
python run_migration.py --target rust --project projects/rpn2tex --dry-run
```

## Project Structure

```
sdk_migration/
├── README.md                 # This file
├── run_migration.py          # CLI entry point
│
├── framework/                # Core framework (language-agnostic)
│   ├── agents.py             # Agent definitions
│   ├── runner.py             # Orchestration logic
│   └── config.py             # Configuration loading
│
├── languages/                # Target language configurations
│   ├── base.py               # Abstract LanguageTarget interface
│   ├── rust.py               # Rust-specific settings
│   └── java.py               # Java-specific settings
│
├── projects/                 # Self-contained project folders
│   └── rpn2tex/              # rpn2tex migration project
│       ├── config.yaml       # Project configuration
│       ├── source/           # Original Python source (frozen snapshot)
│       ├── rust/             # Generated Rust project
│       ├── java/             # Generated Java project
│       ├── io_contract.json  # I/O contract definition
│       └── logs/             # Migration logs
│
└── docs/
    └── research/
        ├── comparative_analysis.tex   # LaTeX research paper
        └── COMPARATIVE_ANALYSIS.md    # Markdown research notes
```

## Methodology

The migration uses a four-phase approach:

1. **Phase 0: I/O Contract** - Execute source on test inputs, capture expected outputs
2. **Phase 1: Analysis** - Analyst reads all source files, produces migration specification
3. **Phase 2: Migration** - Migrators convert each module with quality gate feedback loops
4. **Phase 3: Review** - Reviewers validate against specification and I/O contract

## Agent Architecture

```
                    Main Orchestrator
                          │
        ┌─────────────────┼─────────────────┐
        │                 │                 │
   io_contract        analyst           migrator         reviewer
   (haiku)           (haiku)           (sonnet)          (haiku)
   Bash, Read        Read-only         Full access       Read, Bash
```

**Model Selection:**
- Haiku: Analysis and validation (cost-optimized)
- Sonnet: Code generation (capability-optimized)

## Adding a New Target Language

Implement the `LanguageTarget` interface in `languages/`:

```python
class NewLanguageTarget(LanguageTarget):
    name = "newlang"
    file_extension = ".nl"

    def get_quality_gates(self) -> list[str]:
        return ["newlang build", "newlang test"]

    def get_migrator_idioms(self) -> str:
        return "## NewLang idiom requirements..."

    # ... other abstract methods
```

## Key Findings

1. **Multi-language capability validated** - Same methodology works for Rust and Java
2. **Smaller contexts win** - Multi-phase orchestration 4x faster than embedded source
3. **I/O contracts essential** - Without them, 19% of outputs differed stylistically
4. **Low complexity prerequisite** - Trivial codebase (avg CC 2.8) migrates reliably

## Future Work

- **Medium-complexity codebase** - 5,000-20,000 LOC with external dependencies
- **Dependency mapping** - Automatic library equivalence selection
- **Stateful systems** - Behavioral contracts for systems with side effects
- **Incremental migration** - Interoperability between migrated and unmigrated modules

## Research Paper

The full research paper is available at:
- LaTeX: [`docs/research/comparative_analysis.tex`](docs/research/comparative_analysis.tex)
- PDF: `docs/research/comparative_analysis.pdf`

## License

MIT

# Claude Code: Project Context and Instructions

## Project Overview

`llm-migration` is a multi-agent LLM framework for cross-language code migration. It orchestrates Claude agents to migrate Python codebases to Rust, Java, or Go using either module-by-module or feature-by-feature strategies.

## Key Components

| Component | Location | Purpose |
|-----------|----------|---------|
| Agents | `src/migration/agents.py` | Prompt builders for each agent role |
| Config | `src/migration/config.py` | YAML config parsing and validation |
| Languages | `src/migration/languages/` | Target language support (rust, java, go) |
| Strategies | `src/migration/strategies/` | Migration strategies (module-by-module, feature-by-feature) |
| Reporting | `src/migration/reporting/` | Metrics collection, database, schema |
| Runner | `src/migration/runner.py` | Main migration orchestration |

## CRITICAL: Code Quality Standards (MANDATORY)

### Required Quality Gates (Run After EVERY Code Change)

```bash
hatch run type           # 1. ZERO MyPy errors (strict mode)
hatch run lint           # 2. ZERO Ruff violations
hatch run format-check   # 3. Perfect formatting
hatch run test           # 4. ALL tests pass
hatch run test-cov       # 5. Coverage >= 70%
```

### Combined Quality Check

```bash
hatch run check          # lint + type + test (fast)
hatch run check-cov      # lint + type + test-cov (with coverage)
```

### Code Standards (MANDATORY)

- **Type hints**: Full type annotations on all functions and methods
- **MyPy strict mode**: No `Any` types, no untyped definitions
- **Fail fast**: Raise exceptions on validation failure
- **No inline imports**: All imports at top of file, grouped by PEP 8
- **88 character line limit**: Enforced by ruff
- **Double quotes**: For strings (enforced by ruff format)

### Prohibited Patterns

- No `Any` types
- No inline import statements
- No `hasattr()` - use protocols instead
- No defensive coding or fallback logic unless explicitly requested
- No mock objects in production code (tests only)

## Workflow Commands

All commands defined in `pyproject.toml`:

```bash
# Testing
hatch run test                    # Run all tests
hatch run test-cov                # Run tests with coverage report

# Linting and Formatting
hatch run lint                    # Check for violations
hatch run format                  # Auto-format code
hatch run format-check            # Check formatting without changes

# Type Checking
hatch run type                    # Run MyPy strict mode

# Complexity Metrics
hatch run complexity              # Cyclomatic complexity with summary
hatch run complexity-detailed     # Per-function complexity
hatch run maintainability         # Maintainability index
hatch run raw-metrics             # LOC, comments, blanks
hatch run complexity-check        # Fail if complexity exceeds thresholds

# Combined Checks
hatch run check                   # lint + type + test
hatch run check-cov               # lint + type + test-cov
```

## Running Migrations

```bash
# Run a migration using the CLI
python run_migration.py configs/rpn2tex_rust_mbm.yaml

# Migration configs in configs/ directory define:
# - Source/target languages
# - Strategy (module-by-module or feature-by-feature)
# - IO contract location
# - Working directories
```

## Metrics Database

Migrations store metrics in `migrations.db` (SQLite):

```bash
# Query via CLI
python -m migration.reporting query --project rpn2tex --target rust

# Export to JSON
python -m migration.reporting export --output metrics.json

# Backfill from log files
python -m migration.reporting backfill logs/*.log --project rpn2tex
```

## Quality Thresholds

| Metric | Current | Target | Enforcement |
|--------|---------|--------|-------------|
| Test coverage | 74% | >= 70% | `hatch run test-cov` |
| Cyclomatic complexity | A (3.2 avg) | <= 20 per function | `hatch run complexity-check` |
| Maintainability index | A | >= B grade | `hatch run maintainability` |
| Ruff violations | 0 | 0 | `hatch run lint` |
| MyPy errors | 0 | 0 | `hatch run type` |

**Note**: Coverage threshold set to 70%. Remaining uncovered code is primarily in `runner.py` async functions that require the Claude SDK. One function (`run_migration`) has E complexity (39) due to async orchestration.

## Communication Standards

- Never claim "fixed" without user verification
- No buzzwords, jargon, or superlatives
- No exaggeration about unverified results
- State what changed and why
- Answer questions directly
- Modest, short commit messages

## Solution Standards

- **Proper solution first**: Identify and implement the correct solution immediately
- **No shortcuts or hacks**: Don't offer inferior alternatives to save time
- **Root causes are provable**: Do not present "likely" theories, only facts and data
- **If unsure**: Say "I do not know yet" rather than guessing

## Important Notes

### When Making Changes

1. Run `hatch run check` before every commit
2. Solve 100% of any issues reported
3. Success is defined as 100% - do not settle for lower standards
4. Always find the root cause through patience and tenacity
5. The user makes decisions - ask before making rationales and decisions

### Test Structure

Tests are located in `tests/` directory (203 tests, 74% coverage):

**Core module tests:**
- `tests/test_agents.py` - Agent builder functions
- `tests/test_config.py` - Config parsing and validation
- `tests/test_languages.py` - Language target implementations
- `tests/test_strategies.py` - Migration strategy logic
- `tests/test_runner.py` - LOC counting and migration orchestration

**Reporting tests:**
- `tests/reporting/test_analyzer.py` - Post-hoc analysis with mocked subprocess
- `tests/reporting/test_cli.py` - CLI command handlers
- `tests/reporting/test_collector.py` - Metrics collection
- `tests/reporting/test_database.py` - Database CRUD operations
- `tests/reporting/test_generator.py` - Report generation

### Four-Phase Migration Process

1. **IO Contract**: Defines input/output behavioral contracts
2. **Analyst**: Analyzes source code structure
3. **Migrator**: Generates target language code
4. **Reviewer**: Validates migration quality

### Quality Gates (Applied to Migrations)

Each migration must pass:
1. **Compilation**: Target language compiler (rustc, javac, go build)
2. **Linting**: Static analysis (Clippy, Checkstyle, go vet)
3. **Formatting**: Code style (rustfmt, google-java-format, gofmt)
4. **Testing**: All unit tests pass
5. **IO Contract**: Behavioral equivalence verified

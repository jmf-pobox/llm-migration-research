# LLM Code Migration Research

A research framework for studying LLM-assisted code migration strategies, collecting telemetry, and analyzing migration outcomes.

## What This Is

This project provides infrastructure for understanding **how** LLMs migrate code:

- **Migration Strategies** - Pluggable approaches (module-by-module, feature-by-feature) with different trade-offs
- **Telemetry Collection** - Standardized metrics: timing, cost, tokens, code quality
- **Analysis & Reporting** - Aggregation across runs, Markdown/LaTeX report generation

The framework uses the [Claude Agent SDK](https://github.com/anthropics/claude-code/tree/main/sdk) for execution, but the research focus is on strategy comparison and outcome analysis.

## Key Findings

Migrated `rpn2tex` (Python → Rust/Java, 352 LOC source):

| Metric | Rust (Module) | Rust (Feature) | Java |
|--------|---------------|----------------|------|
| Duration | 25 min | 43 min | 25 min |
| Cost | $3.74 | $6.07 | $7.24 |
| I/O Match | 100% | 100% | 100% |
| Production LOC | 1,184 | 931 | 1,547 |

**Insights:**
- Feature-by-feature produces 21% less code but takes 72% longer
- I/O contract validation catches behavioral regressions early
- Module-by-module is faster but generates more boilerplate

See [docs/research/](docs/research/) for detailed analysis.

## Project Structure

```
.
├── run_migration.py          # CLI entry point
├── framework/
│   ├── config.py             # YAML project configuration
│   ├── agents.py             # Sub-agent prompt templates
│   └── runner.py             # SDK orchestration + metrics integration
├── strategies/
│   ├── base.py               # Strategy interface
│   ├── module_by_module.py   # Migrate by source module
│   └── feature_by_feature.py # Migrate by feature slice
├── languages/
│   ├── base.py               # Language target interface
│   ├── rust.py               # Rust-specific config
│   └── java.py               # Java-specific config
├── reporting/
│   ├── schema.py             # MigrationMetrics dataclass
│   ├── collector.py          # Real-time metrics collection
│   ├── analyzer.py           # Post-hoc analysis (lizard, cloc)
│   ├── database.py           # SQLite storage & aggregation
│   ├── generator.py          # Jinja2 report rendering
│   ├── cli.py                # Reporting CLI
│   └── templates/            # Markdown & LaTeX templates
├── projects/
│   └── {project}/
│       ├── config.yaml       # Project definition
│       ├── source/           # Original source code
│       └── migrations/       # Output by target-strategy
└── docs/
    ├── REPORTING.md          # Telemetry framework docs
    ├── STRATEGIES.md         # Migration strategy guide
    ├── PROJECT_INDEX.md      # Links to all project reports
    └── research/             # Analysis papers
```

## Quick Start

```bash
# Install dependencies
pip install claude-agent-sdk jinja2

# Set API key
export ANTHROPIC_API_KEY=your-key

# Run a migration (module-by-module strategy)
python run_migration.py --target rust --project projects/rpn2tex

# Try feature-by-feature strategy
python run_migration.py --target rust --project projects/rpn2tex --strategy feature-by-feature

# Dry run (show prompt without executing)
python run_migration.py --target rust --project projects/rpn2tex --dry-run
```

## Telemetry & Reporting

Every migration automatically captures metrics:

| Category | Metrics |
|----------|---------|
| Timing | Wall clock, API time, phase durations |
| Cost | Total USD, token breakdown |
| Code | LOC, cyclomatic complexity, function count |
| Maintainability | Maintainability Index (0-100, higher is better) |
| Dependencies | External third-party dependency count |
| Quality | Compilation, linting, test results |
| Coverage | Line, function, and branch coverage percentages |
| I/O Contract | Test cases passed/failed, match rate |

Query and export with the reporting CLI:

```bash
# View database statistics
python -m reporting stats

# Query migrations
python -m reporting query --project rpn2tex --target rust

# Compare strategies
python -m reporting compare <run_id_1> <run_id_2>

# Export summary
python -m reporting export --format markdown
python -m reporting export --format latex
```

See [docs/REPORTING.md](docs/REPORTING.md) for the full API.

## Migration Strategies

### Module-by-Module (Default)

Migrates each source module in dependency order. Each module is fully translated before moving to the next.

**Pros:** Faster, simpler prompts, easier to debug
**Cons:** May produce redundant code, less cohesive architecture

### Feature-by-Feature

Migrates by feature slice, implementing end-to-end functionality incrementally. Validates I/O contract after each feature.

**Pros:** More cohesive code, catches integration issues early, 21% less code
**Cons:** 72% longer duration, more complex orchestration

See [docs/STRATEGIES.md](docs/STRATEGIES.md) for details.

## I/O Contract Validation

The framework generates an I/O contract from the source implementation before migration:

1. Run source on curated test inputs
2. Capture exact outputs
3. Include contract in migration spec
4. Validate target outputs match exactly

This ensures behavioral equivalence regardless of implementation approach.

## Adding a New Project

1. Create `projects/{name}/config.yaml`:

```yaml
name: myproject
source_language: python
source_directory: projects/myproject/source
source_files:
  - main.py
  - utils.py
modules:
  - source: utils.py
    phase: foundation
  - source: main.py
    phase: core
    depends_on: [utils.py]
test_inputs:
  - "input1"
  - "input2"
```

2. Place source code in `projects/{name}/source/`

3. Run migration:
```bash
python run_migration.py --target rust --project projects/myproject
```

## Adding a New Strategy

1. Create `strategies/{name}.py`:

```python
from strategies.base import MigrationStrategy

class MyStrategy(MigrationStrategy):
    name = "my-strategy"

    def get_prompt(self, config, target, project_dir):
        return f"Migration prompt for {config.name}..."
```

2. Register in `strategies/__init__.py`

3. Use with `--strategy my-strategy`

## Documentation

| Document | Description |
|----------|-------------|
| [docs/STRATEGIES.md](docs/STRATEGIES.md) | Migration strategy comparison |
| [docs/REPORTING.md](docs/REPORTING.md) | Telemetry framework reference |
| [docs/PROJECT_INDEX.md](docs/PROJECT_INDEX.md) | Links to all project reports |
| [docs/research/](docs/research/) | Analysis papers and findings |

## License

MIT

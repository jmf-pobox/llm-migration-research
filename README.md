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

| Target | Strategy | Duration | Cost | I/O Match |
|--------|----------|----------|------|-----------|
| Rust | Module-by-module | ~32 min | $6.53 | 100% |
| Rust | Feature-by-feature | ~32 min | $4.63 | 100% |
| Java | Module-by-module | ~26 min | $4.92 | 100% |
| Java | Feature-by-feature | ~51 min | $6.27 | 100% |

**Insights:**
- Strategy efficiency varies by target language
- For Rust: feature-by-feature saves ~30% cost
- For Java: module-by-module saves ~22% cost
- Both strategies produce identical behavioral output
- I/O contract validation ensures behavioral equivalence

See [docs/research/](docs/research/) for detailed analysis.

## Project Structure

```
.
├── run_migration.py          # CLI entry point
├── src/migration/            # Main package
│   ├── config.py             # YAML project configuration
│   ├── agents.py             # Sub-agent prompt templates
│   ├── runner.py             # SDK orchestration + metrics integration
│   ├── strategies/           # Migration approaches
│   │   ├── base.py           # Strategy interface
│   │   ├── module_by_module.py
│   │   └── feature_by_feature.py
│   ├── languages/            # Target language configs
│   │   ├── base.py           # Language target interface
│   │   ├── rust.py
│   │   └── java.py
│   └── reporting/            # Telemetry and analysis
│       ├── schema.py         # MigrationMetrics dataclass
│       ├── collector.py      # Real-time metrics collection
│       ├── analyzer.py       # Post-hoc analysis (lizard, cloc)
│       ├── database.py       # SQLite storage & aggregation
│       ├── generator.py      # Jinja2 report rendering
│       └── cli.py            # Reporting CLI
├── projects/
│   └── {project}/
│       ├── config.yaml       # Project definition
│       ├── source/           # Original source code
│       └── migrations/       # Output by target-strategy
└── docs/
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

**Pros:** Simpler prompts, easier to debug, more efficient for Java
**Cons:** May produce redundant code

### Feature-by-Feature

Migrates by feature slice, implementing end-to-end functionality incrementally. Validates I/O contract after each feature.

**Pros:** More cohesive code, catches integration issues early, more efficient for Rust
**Cons:** More complex orchestration

**Note:** Strategy efficiency varies by target language. See research findings above.

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
| [docs/research/](docs/research/) | Analysis papers and findings |

## License

MIT

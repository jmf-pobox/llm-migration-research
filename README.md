# Autonomous Cross-Language Code Migration via Behavioral Contracts

A multi-agent LLM framework for autonomous cross-language code migration. Orchestrates Claude agents to migrate Python codebases to Rust, Java, or Go using either module-by-module or feature-by-feature strategies, validated against I/O behavioral contracts.

## What This Is

This project provides infrastructure for **autonomous** code migration:

- **Multi-Agent Architecture** - Specialized agents for I/O contracts, analysis, migration, and review
- **Behavioral Contracts** - I/O contracts derived from source execution ensure behavioral equivalence
- **Migration Strategies** - Module-by-module (vertical slices) and feature-by-feature (horizontal slices)
- **Telemetry Collection** - Standardized metrics: timing, cost, tokens, code quality, coverage

The framework uses the [Claude Agent SDK](https://github.com/anthropics/claude-code/tree/main/sdk) for execution. Migrations run end-to-end without human intervention: agents read source code, generate target implementations, execute quality gates, and iterate on failures autonomously.

## Key Findings

Migrated `rpn2tex` (Python → Rust/Java/Go, 352 LOC source) across 18 runs (3 per configuration):

| Target | Strategy | Duration | Cost | Coverage | I/O Match |
|--------|----------|----------|------|----------|-----------|
| Rust | Module-by-module | 32 min | $8.62 | 95% | 100% |
| Rust | Feature-by-feature | 51 min | $8.24 | 95% | 100% |
| Java | Module-by-module | 45 min | $11.93 | 92% | 100% |
| Java | Feature-by-feature | 47 min | $7.96 | 80% | 100% |
| Go | Module-by-module | 49 min | $8.42 | 71% | 100% |
| Go | Feature-by-feature | 44 min | $6.32 | 67% | 100% |

*Values are means across 3 runs per configuration.*

**Key observations:**
- All 18 migrations achieve 100% behavioral equivalence on 21-case I/O contract
- Coverage varies by language: Rust 95%, Java 80-92%, Go 67-71%
- Cost ranges from $6.32 to $11.93 per migration
- Strategy effects are language-dependent (no universal winner)
- Agents generate 2-5x more test code than production code

See [docs/research/comparative_analysis.pdf](docs/research/comparative_analysis.pdf) for the full research paper.

## Project Structure

```
.
├── run_migration.py          # CLI entry point
├── src/migration/            # Main package
│   ├── config.py             # YAML project configuration
│   ├── agents.py             # Sub-agent prompt templates
│   ├── runner.py             # SDK orchestration + metrics integration
│   ├── checkpoint.py         # Resumable migration checkpoints
│   ├── strategies/           # Migration approaches
│   │   ├── base.py           # Strategy interface
│   │   ├── module_by_module.py
│   │   └── feature_by_feature.py
│   ├── languages/            # Target language configs
│   │   ├── base.py           # Language target interface
│   │   ├── rust.py
│   │   ├── java.py
│   │   └── go.py
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

# Resume a failed migration (target/strategy inferred from checkpoint)
python run_migration.py --resume projects/rpn2tex/migrations/rust-module-by-module-1 \
  --project projects/rpn2tex
```

## Telemetry & Reporting

Every migration automatically captures metrics and saves them to JSON files in the migration's `logs/` directory:

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

### Storing Metrics in Database

Metrics are **not** automatically inserted into `migrations.db`. After a migration completes, backfill the metrics from log files:

```bash
# Backfill a single migration
PYTHONPATH=src python -m migration.reporting backfill \
  projects/rpn2tex/migrations/rust-module-by-module-1/logs/*.log \
  --project rpn2tex --strategy module-by-module

# Backfill all migrations for a project
for log in projects/rpn2tex/migrations/*/logs/*.log; do
  PYTHONPATH=src python -m migration.reporting backfill "$log" \
    --project rpn2tex --strategy module-by-module
done
```

### Querying the Database

Once metrics are backfilled, query and export with the reporting CLI:

```bash
# View database statistics
PYTHONPATH=src python -m migration.reporting stats

# Query migrations
PYTHONPATH=src python -m migration.reporting query --project rpn2tex --target rust

# Compare strategies
PYTHONPATH=src python -m migration.reporting compare <run_id_1> <run_id_2>

# Export summary
PYTHONPATH=src python -m migration.reporting export --format markdown
PYTHONPATH=src python -m migration.reporting export --format latex
```

See [docs/REPORTING.md](docs/REPORTING.md) for the full API.

## Migration Strategies

### Module-by-Module (Default)

Migrates each source module in dependency order. Each module is fully translated before moving to the next.

**Characteristics:** Faster execution (32-37 min), simpler prompts, easier to debug

### Feature-by-Feature

Migrates by feature slice, implementing end-to-end functionality incrementally. Validates I/O contract after each feature.

**Characteristics:** More cohesive code, catches integration issues early, slower execution (55-60 min)

Both strategies produce correct output for all test cases. Strategy selection does not affect correctness—only timing and cost (which varies without clear pattern).

## I/O Contract Validation

The framework generates an I/O contract from the source implementation before migration:

1. Run source on curated test inputs
2. Capture exact outputs
3. Include contract in migration spec
4. Validate target outputs match exactly

This ensures behavioral equivalence regardless of implementation approach.

## Checkpoints and Resumable Migrations

Migrations automatically checkpoint progress to `.checkpoint/state.json` in the migration directory. If a migration fails mid-execution, resume from where it left off:

```bash
# Resume a failed migration
python run_migration.py --resume /path/to/migration/dir --project projects/rpn2tex

# Target and strategy are auto-inferred from checkpoint
# You can override them explicitly if needed:
python run_migration.py --resume /path/to/migration/dir --project projects/rpn2tex \
  --target rust --strategy module-by-module
```

**How it works:**

1. **Session-based resume**: Uses the SDK's `resume` option to continue the exact conversation state
2. **Feature-level resume**: Tracks completed features for reconstruction if the session expires

The checkpoint tracks:
- Project, target language, and strategy
- Current feature in progress
- Completed features list
- Failed feature and error message (if any)
- Session ID for SDK resume

On successful completion, the checkpoint is automatically cleared.

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
| [docs/research/comparative_analysis.pdf](docs/research/comparative_analysis.pdf) | Research paper (main artifact) |
| [docs/REPORTING.md](docs/REPORTING.md) | Telemetry and reporting framework |
| [CLAUDE.md](CLAUDE.md) | Development instructions |

## License

MIT

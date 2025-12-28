# Reporting Framework

The reporting framework provides standardized telemetry collection, storage, and report generation for migration runs.

## Architecture

```
Migration Run
     │
     ▼
┌─────────────────────────────────────────────────────────────┐
│  MetricsCollector (real-time instrumentation)               │
└─────────────────────────────────────────────────────────────┘
     │
     ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│ metrics.json    │ → │ SQLite DB       │ → │ ReportGenerator  │
│ (per-run)       │    │ (aggregation)   │    │ (MD/LaTeX)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Components

### Schema (`reporting/schema.py`)

Defines the canonical `MigrationMetrics` dataclass with 10 metric categories:

| Category | Fields |
|----------|--------|
| **Identity** | run_id, project_name, source/target_language, strategy, timestamps |
| **Timing** | wall_clock_duration_ms, api_duration_ms, phase_durations, module_durations |
| **Cost** | total_cost_usd, input_cost, output_cost, cache_cost |
| **Tokens** | input_tokens, output_tokens, cache_creation, cache_read, efficiency_ratio |
| **Agent** | total_turns, messages, tool_invocations, subagent_invocations, retries |
| **Source Metrics** | production_loc, test_loc, function_count, avg/max_cyclomatic_complexity |
| **Target Metrics** | production_loc, test_loc, function_count, avg/max_cyclomatic_complexity |
| **Quality Gates** | compilation, linting, formatting, unit_tests, coverage |
| **I/O Contract** | total_test_cases, passed, failed, unsupported, match_rate_pct |
| **Outcome** | status, modules_completed, modules_total, blocking_issues, notes |

### Collector (`reporting/collector.py`)

Real-time metrics collection during migration execution:

```python
from reporting.collector import MetricsCollector

collector = MetricsCollector(
    project_name="rpn2tex",
    source_language="python",
    target_language="rust",
    strategy="module-by-module",
)

# Track phases
collector.start_phase("analysis")
# ... do work ...
collector.end_phase("analysis")

# Track modules
collector.start_module("lexer")
# ... migrate module ...
collector.end_module("lexer", attempts=2)

# Record tool usage
collector.record_tool_use("Bash")
collector.record_tool_use("Read")

# At end of migration
metrics = collector.finalize()
metrics.save("metrics.json")
```

Also includes `LogParser` for backfilling metrics from existing log files.

### Analyzer (`reporting/analyzer.py`)

Post-hoc analysis using external tools:

```python
from reporting.analyzer import PostHocAnalyzer

analyzer = PostHocAnalyzer()

# Analyze Python source
source_metrics = analyzer.analyze_python_source(Path("source/"))

# Analyze Rust target
target_metrics = analyzer.analyze_rust_target(Path("migrations/rust/"))
quality = analyzer.analyze_rust_quality(Path("migrations/rust/"))
```

External tools used:
- **cloc**: Lines of code counting
- **lizard**: Cyclomatic complexity analysis
- **cargo check/clippy/fmt/test**: Rust quality gates
- **cargo-tarpaulin**: Rust code coverage

### Database (`reporting/database.py`)

SQLite storage for aggregation across runs:

```python
from reporting.database import MigrationDatabase

db = MigrationDatabase("migrations.db")

# Insert a run
db.insert(metrics)

# Query
results = db.query(project="rpn2tex", target="rust", strategy="module-by-module")

# Aggregate statistics
stats = db.aggregate(strategy="feature-by-feature")
print(f"Avg cost: ${stats.avg_cost_usd:.2f}")
print(f"Success rate: {stats.success_rate_pct:.0f}%")

# Group by field
by_strategy = db.group_by("strategy")
for name, stats in by_strategy.items():
    print(f"{name}: {stats.count} runs")
```

### Generator (`reporting/generator.py`)

Template-based report generation:

```python
from reporting.generator import ReportGenerator

generator = ReportGenerator()

# Single run report
markdown = generator.generate_run_report(metrics)

# Compare multiple runs
comparison = generator.generate_comparison([metrics1, metrics2])

# LaTeX table for papers
latex = generator.generate_latex_table([metrics1, metrics2], caption="Results")

# Aggregated summary from database
summary = generator.generate_summary(db)
```

### CLI (`reporting/cli.py`)

Command-line interface for all operations:

```bash
# Database statistics
python -m reporting stats

# Query migrations
python -m reporting query --project rpn2tex --target rust --limit 10

# Compare specific runs
python -m reporting compare run_id_1 run_id_2 --format markdown

# Export summary
python -m reporting export --format markdown > summary.md
python -m reporting export --format latex > table.tex
python -m reporting export --format json -o migrations.json

# Backfill from log file
python -m reporting backfill logs/migration.log --project rpn2tex --strategy module-by-module

# Analyze migration directory
python -m reporting analyze projects/rpn2tex/migrations/rust-module-by-module

# Generate report from metrics JSON
python -m reporting report metrics/run_20251228.json --format markdown
```

## Templates

Jinja2 templates in `reporting/templates/`:

| Template | Purpose |
|----------|---------|
| `run_report.md.j2` | Single migration run report |
| `comparison.md.j2` | Side-by-side comparison of runs |
| `summary.md.j2` | Aggregated summary from database |
| `table.tex.j2` | LaTeX table for academic papers |

## Integration

The framework is integrated into `framework/runner.py`. When `collect_metrics=True` (default):

1. MetricsCollector is initialized at migration start
2. Tool uses and phases are recorded during execution
3. On completion, metrics JSON is saved to `metrics/run_{timestamp}.json`
4. Report is generated to `reports/run_{timestamp}.md`

## Standardized Field Names

All reports use these canonical names:

| Metric | Field | Unit |
|--------|-------|------|
| Duration | `wall_clock_duration_ms` | milliseconds |
| Cost | `total_cost_usd` | USD |
| Lines of Code | `production_loc`, `test_loc` | integer |
| Complexity | `avg_cyclomatic_complexity`, `max_cyclomatic_complexity` | decimal/int |
| Coverage | `line_coverage_pct` | percentage |
| I/O Match | `match_rate_pct` | percentage |
| Expansion | `loc_expansion_ratio` | decimal ratio |

## Output Directory Structure

Each migration produces:

```
projects/{project}/migrations/{target}-{strategy}/
├── src/              # Migrated source code
├── logs/             # Execution logs
│   └── migration_{timestamp}.log
├── metrics/          # JSON metrics
│   └── run_{timestamp}.json
└── reports/          # Generated reports
    └── run_{timestamp}.md
```

## Database Schema

```sql
CREATE TABLE migrations (
    run_id TEXT PRIMARY KEY,
    project_name TEXT NOT NULL,
    source_language TEXT NOT NULL,
    target_language TEXT NOT NULL,
    strategy TEXT NOT NULL,
    started_at DATETIME NOT NULL,
    completed_at DATETIME,
    duration_ms INTEGER,
    cost_usd REAL,
    io_match_rate REAL,
    status TEXT,
    source_loc INTEGER,
    target_loc INTEGER,
    metrics_json TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX idx_project ON migrations(project_name);
CREATE INDEX idx_target ON migrations(target_language);
CREATE INDEX idx_strategy ON migrations(strategy);
CREATE INDEX idx_started ON migrations(started_at);
CREATE INDEX idx_status ON migrations(status);
```

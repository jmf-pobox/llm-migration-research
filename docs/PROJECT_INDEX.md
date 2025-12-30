# Documentation Index

## Primary Documentation

| Document | Description |
|----------|-------------|
| [comparative_analysis.pdf](research/comparative_analysis.pdf) | Research paper (main artifact) |
| [comparative_analysis.tex](research/comparative_analysis.tex) | LaTeX source |
| [REPORTING.md](REPORTING.md) | Telemetry and reporting framework |
| [STRATEGIES.md](STRATEGIES.md) | Migration strategy guide |

## Project: rpn2tex

**Source:** Python RPN-to-LaTeX converter (352 LOC)
**Target languages:** Rust, Java, Go
**Location:** `projects/rpn2tex/`

### Active Migrations

| Target | Strategy | Path |
|--------|----------|------|
| Rust | Module-by-module | `projects/rpn2tex/migrations/rust-module-by-module-1/` |
| Rust | Feature-by-feature | `projects/rpn2tex/migrations/rust-feature-by-feature-1/` |
| Java | Module-by-module | `projects/rpn2tex/migrations/java-module-by-module-1/` |
| Java | Feature-by-feature | `projects/rpn2tex/migrations/java-feature-by-feature-1/` |
| Go | Module-by-module | `projects/rpn2tex/migrations/go-module-by-module-1/` |
| Go | Feature-by-feature | `projects/rpn2tex/migrations/go-feature-by-feature-1/` |

### Archived Migrations

Earlier experimental runs are in `projects/rpn2tex/migrations_archive/`.

## Metrics Database

Query migrations with the reporting CLI:

```bash
python -m migration.reporting stats
python -m migration.reporting query --project rpn2tex
python -m migration.reporting export --format markdown
```

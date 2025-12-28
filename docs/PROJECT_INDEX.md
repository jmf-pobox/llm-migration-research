# Project Documentation Index

This document provides links to all project-specific reports and documentation generated during migrations.

## Research Documents

High-level analysis and findings:

| Document | Description |
|----------|-------------|
| [MIGRATION_REPORT.md](research/MIGRATION_REPORT.md) | Detailed migration outcomes |
| [COMPARATIVE_ANALYSIS.md](research/COMPARATIVE_ANALYSIS.md) | Strategy comparison |
| [ANALYSIS_REPORT.md](research/ANALYSIS_REPORT.md) | Code metrics analysis |
| [comparative_analysis.pdf](research/comparative_analysis.pdf) | LaTeX paper (PDF) |

## Framework Documentation

| Document | Description |
|----------|-------------|
| [REPORTING.md](REPORTING.md) | Telemetry and reporting framework |
| [STRATEGIES.md](STRATEGIES.md) | Migration strategy guide |

---

## Project: rpn2tex

**Source:** Python RPN-to-LaTeX converter (352 LOC)
**Migrations:** Rust (2 strategies), Java (1 strategy)

### I/O Contract

The I/O contract captures expected behavior from the source implementation:

| Document | Description |
|----------|-------------|
| [IO_CONTRACT.md](../projects/rpn2tex/IO_CONTRACT.md) | Main I/O contract |
| [VERIFIED_TEST_CASES.md](../projects/rpn2tex/VERIFIED_TEST_CASES.md) | 21 test cases with expected outputs |
| [PHASE_0_COMPLETION_REPORT.md](../projects/rpn2tex/PHASE_0_COMPLETION_REPORT.md) | I/O contract generation report |

### Migration: Rust (Module-by-Module)

| Artifact | Path |
|----------|------|
| Source code | `projects/rpn2tex/migrations/rust-module-by-module/src/` |
| Logs | `projects/rpn2tex/migrations/rust-module-by-module/logs/` |
| Metrics | `projects/rpn2tex/migrations/rust-module-by-module/logs/metrics_*.json` |

### Migration: Rust (Feature-by-Feature)

| Artifact | Path |
|----------|------|
| Source code | `projects/rpn2tex/migrations/rust-feature-by-feature/src/` |
| Logs | `projects/rpn2tex/migrations/rust-feature-by-feature/logs/` |

**Feature Reports:**

| Feature | Report | Status |
|---------|--------|--------|
| 1. Numbers | [FEATURE_1_NUMBERS_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_1_NUMBERS_REPORT.md) | Complete |
| 2. Addition | [FEATURE_2_ADDITION_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_2_ADDITION_REPORT.md) | Complete |
| 3. Subtraction | [FEATURE_3_SUBTRACTION_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_3_SUBTRACTION_REPORT.md) | Complete |
| 4. Multiplication | [FEATURE_4_MULTIPLICATION_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_4_MULTIPLICATION_REPORT.md) | Complete |
| 5. Division | [FEATURE_5_DIVISION_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_5_DIVISION_REPORT.md) | Complete |
| 6. Precedence | [FEATURE_6_PRECEDENCE_REPORT.md](../projects/rpn2tex/migrations/rust-feature-by-feature/FEATURE_6_PRECEDENCE_REPORT.md) | Complete |

[Migration Status](../projects/rpn2tex/migrations/rust-feature-by-feature/MIGRATION_STATUS.md) | [README](../projects/rpn2tex/migrations/rust-feature-by-feature/README.md)

### Migration: Java (Module-by-Module)

| Artifact | Path |
|----------|------|
| Source code | `projects/rpn2tex/migrations/java-module-by-module/src/` |
| Logs | `projects/rpn2tex/migrations/java-module-by-module/logs/` |

**Reports:**

| Document | Description |
|----------|-------------|
| [AST_MIGRATION_REPORT.md](../projects/rpn2tex/migrations/java-module-by-module/AST_MIGRATION_REPORT.md) | AST-based migration analysis |
| [MIGRATION_LOG.md](../projects/rpn2tex/migrations/java-module-by-module/MIGRATION_LOG.md) | Migration progress log |

---

## Project: txt2tex

**Status:** Pending migration
**Source:** Python LaTeX DSL compiler
**Location:** `projects/txt2tex/source/`

---

## Metrics Database

Aggregated metrics from all migrations are stored in `migrations.db` (SQLite).

Query with:
```bash
python -m reporting stats
python -m reporting query --project rpn2tex
python -m reporting export --format markdown
```

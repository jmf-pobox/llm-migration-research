"""Report generation from migration metrics.

This module renders Jinja2 templates to produce Markdown and
LaTeX reports from standardized migration metrics.
"""

from datetime import datetime
from pathlib import Path

try:
    from jinja2 import Environment, FileSystemLoader, select_autoescape
except ImportError as err:
    raise ImportError(
        "jinja2 is required for report generation. Install with: pip install jinja2"
    ) from err

from .database import MigrationDatabase
from .schema import MigrationMetrics

TEMPLATES_DIR = Path(__file__).parent / "templates"


class ReportGenerator:
    """Generates reports from migration metrics.

    Usage:
        generator = ReportGenerator()

        # Single run report
        md = generator.generate_run_report(metrics)

        # Comparison of multiple runs
        md = generator.generate_comparison([metrics1, metrics2])

        # LaTeX table for academic paper
        tex = generator.generate_latex_table([metrics1, metrics2])

        # Aggregated summary from database
        md = generator.generate_summary(db)
    """

    def __init__(self, templates_dir: Path | None = None):
        self.templates_dir = templates_dir or TEMPLATES_DIR
        self.env = Environment(
            loader=FileSystemLoader(self.templates_dir),
            autoescape=select_autoescape(["html", "xml"]),
            trim_blocks=True,
            lstrip_blocks=True,
        )

        # Add custom filters
        self.env.filters["duration"] = self._format_duration
        self.env.filters["cost"] = self._format_cost
        self.env.filters["pct"] = self._format_percentage
        self.env.filters["number"] = self._format_number

    def _format_duration(self, ms: int) -> str:
        """Format milliseconds as human-readable duration."""
        if ms < 1000:
            return f"{ms}ms"
        seconds = ms / 1000
        if seconds < 60:
            return f"{seconds:.1f}s"
        minutes = seconds / 60
        if minutes < 60:
            return f"{minutes:.1f}min"
        hours = minutes / 60
        return f"{hours:.1f}h"

    def _format_cost(self, usd: float) -> str:
        """Format USD cost."""
        return f"${usd:.2f}"

    def _format_percentage(self, value: float) -> str:
        """Format percentage."""
        return f"{value:.1f}%"

    def _format_number(self, value: int) -> str:
        """Format integer with thousands separator."""
        return f"{value:,}"

    def generate_run_report(self, metrics: MigrationMetrics) -> str:
        """Generate a Markdown report for a single migration run."""
        template = self.env.get_template("run_report.md.j2")
        return str(
            template.render(
                m=metrics,
                generated_at=datetime.now().isoformat(),
            )
        )

    def generate_comparison(
        self,
        runs: list[MigrationMetrics],
        title: str | None = None,
    ) -> str:
        """Generate a Markdown comparison of multiple runs."""
        template = self.env.get_template("comparison.md.j2")
        return str(
            template.render(
                runs=runs,
                title=title or "Migration Comparison",
                generated_at=datetime.now().isoformat(),
            )
        )

    def generate_latex_table(
        self,
        runs: list[MigrationMetrics],
        caption: str | None = None,
        label: str | None = None,
    ) -> str:
        """Generate a LaTeX table for inclusion in academic papers."""
        template = self.env.get_template("table.tex.j2")
        return str(
            template.render(
                runs=runs,
                caption=caption or "Migration Results",
                label=label or "tab:migrations",
            )
        )

    def generate_summary(
        self,
        db: MigrationDatabase,
        title: str | None = None,
    ) -> str:
        """Generate an aggregated summary from the database."""
        template = self.env.get_template("summary.md.j2")

        # Collect stats
        total_stats = db.aggregate()
        by_target = db.group_by("target_language")
        by_strategy = db.group_by("strategy")
        projects = db.list_projects()

        return str(
            template.render(
                title=title or "Migration Summary",
                total_count=db.count(),
                total_stats=total_stats,
                by_target=by_target,
                by_strategy=by_strategy,
                projects=projects,
                generated_at=datetime.now().isoformat(),
            )
        )


def create_default_templates() -> None:
    """Create default Jinja2 templates if they don't exist."""
    TEMPLATES_DIR.mkdir(parents=True, exist_ok=True)

    # Run report template
    run_report = TEMPLATES_DIR / "run_report.md.j2"
    if not run_report.exists():
        run_report.write_text(RUN_REPORT_TEMPLATE)

    # Comparison template
    comparison = TEMPLATES_DIR / "comparison.md.j2"
    if not comparison.exists():
        comparison.write_text(COMPARISON_TEMPLATE)

    # Summary template
    summary = TEMPLATES_DIR / "summary.md.j2"
    if not summary.exists():
        summary.write_text(SUMMARY_TEMPLATE)

    # LaTeX table template
    table_tex = TEMPLATES_DIR / "table.tex.j2"
    if not table_tex.exists():
        table_tex.write_text(LATEX_TABLE_TEMPLATE)


# Default templates
RUN_REPORT_TEMPLATE = """# Migration Report: {{ m.identity.project_name }}

**Run ID:** `{{ m.identity.run_id }}`
**Generated:** {{ generated_at }}

## Summary

| Metric | Value |
|--------|-------|
| Project | {{ m.identity.project_name }} |
| Source | {{ m.identity.source_language }} |
| Target | {{ m.identity.target_language }} |
| Strategy | {{ m.identity.strategy }} |
| Status | {{ m.outcome.status }} |
| Duration | {{ m.timing.wall_clock_duration_ms | duration }} |
| Cost | {{ m.cost.total_cost_usd | cost }} |
| I/O Match | {{ m.io_contract.match_rate_pct | pct }} |

## Timing

| Phase | Duration |
|-------|----------|
{% for phase, ms in m.timing.phase_durations_ms.items() %}
| {{ phase }} | {{ ms | duration }} |
{% endfor %}
| **Total** | {{ m.timing.wall_clock_duration_ms | duration }} |

## Code Metrics

| Metric | Source | Target |
|--------|--------|--------|
| Production LOC | {{ m.source_metrics.production_loc | number }} | {{ m.target_metrics.production_loc | number }} |
| Test LOC | {{ m.source_metrics.test_loc | number }} | {{ m.target_metrics.test_loc | number }} |
| Functions | {{ m.source_metrics.function_count }} | {{ m.target_metrics.function_count }} |
| Avg CC | {{ "%.1f" | format(m.source_metrics.avg_cyclomatic_complexity) }} | {{ "%.1f" | format(m.target_metrics.avg_cyclomatic_complexity) }} |
| Max CC | {{ m.source_metrics.max_cyclomatic_complexity }} | {{ m.target_metrics.max_cyclomatic_complexity }} |
| Expansion | - | {{ "%.2f" | format(m.loc_expansion_ratio) }}x |

## Token Usage

| Metric | Count |
|--------|-------|
| Input Tokens | {{ m.tokens.input_tokens | number }} |
| Output Tokens | {{ m.tokens.output_tokens | number }} |
| Cache Creation | {{ m.tokens.cache_creation_input_tokens | number }} |
| Cache Read | {{ m.tokens.cache_read_input_tokens | number }} |
| Cache Efficiency | {{ (m.tokens.cache_efficiency_ratio * 100) | pct }} |

## Quality Gates

| Gate | Status |
|------|--------|
| Compilation | {{ "PASS" if m.quality_gates.compilation.passed else "FAIL" }} |
| Linting | {{ "PASS" if m.quality_gates.linting.passed else "FAIL" }} ({{ m.quality_gates.linting.tool }}) |
| Formatting | {{ "PASS" if m.quality_gates.formatting.passed else "FAIL" }} |
| Tests | {{ "PASS" if m.quality_gates.unit_tests.passed else "FAIL" }} ({{ m.quality_gates.unit_tests.passed_count }}/{{ m.quality_gates.unit_tests.total }}) |
{% if m.quality_gates.coverage.line_coverage_pct %}
| Coverage | {{ m.quality_gates.coverage.line_coverage_pct | pct }} |
{% endif %}

## I/O Contract

| Metric | Value |
|--------|-------|
| Total Cases | {{ m.io_contract.total_test_cases }} |
| Passed | {{ m.io_contract.passed }} |
| Failed | {{ m.io_contract.failed }} |
| Unsupported | {{ m.io_contract.unsupported }} |
| Match Rate | {{ m.io_contract.match_rate_pct | pct }} |

{% if m.outcome.blocking_issues %}
## Blocking Issues

{% for issue in m.outcome.blocking_issues %}
- {{ issue }}
{% endfor %}
{% endif %}

{% if m.outcome.notes %}
## Notes

{{ m.outcome.notes }}
{% endif %}

---
*Schema version: {{ m.schema_version }}*
"""

COMPARISON_TEMPLATE = """# {{ title }}

**Generated:** {{ generated_at }}

## Overview

| Metric | {% for r in runs %}{{ r.identity.strategy }} | {% endfor %}
|--------|{% for r in runs %}--------|{% endfor %}
| Duration | {% for r in runs %}{{ r.timing.wall_clock_duration_ms | duration }} | {% endfor %}
| Cost | {% for r in runs %}{{ r.cost.total_cost_usd | cost }} | {% endfor %}
| Prod LOC | {% for r in runs %}{{ r.target_metrics.production_loc | number }} | {% endfor %}
| Tests | {% for r in runs %}{{ r.quality_gates.unit_tests.total }} | {% endfor %}
| I/O Match | {% for r in runs %}{{ r.io_contract.match_rate_pct | pct }} | {% endfor %}
| Status | {% for r in runs %}{{ r.outcome.status }} | {% endfor %}

## Token Usage

| Metric | {% for r in runs %}{{ r.identity.strategy }} | {% endfor %}
|--------|{% for r in runs %}--------|{% endfor %}
| Input | {% for r in runs %}{{ r.tokens.input_tokens | number }} | {% endfor %}
| Output | {% for r in runs %}{{ r.tokens.output_tokens | number }} | {% endfor %}
| Cache Create | {% for r in runs %}{{ r.tokens.cache_creation_input_tokens | number }} | {% endfor %}
| Cache Read | {% for r in runs %}{{ r.tokens.cache_read_input_tokens | number }} | {% endfor %}

## Code Metrics

| Metric | {% for r in runs %}{{ r.identity.strategy }} | {% endfor %}
|--------|{% for r in runs %}--------|{% endfor %}
| Prod LOC | {% for r in runs %}{{ r.target_metrics.production_loc | number }} | {% endfor %}
| Test LOC | {% for r in runs %}{{ r.target_metrics.test_loc | number }} | {% endfor %}
| Avg CC | {% for r in runs %}{{ "%.1f" | format(r.target_metrics.avg_cyclomatic_complexity) }} | {% endfor %}
| Max CC | {% for r in runs %}{{ r.target_metrics.max_cyclomatic_complexity }} | {% endfor %}

---
*{{ runs | length }} runs compared*
"""

SUMMARY_TEMPLATE = """# {{ title }}

**Generated:** {{ generated_at }}
**Total Migrations:** {{ total_count }}

## Overall Statistics

{% if total_stats %}
| Metric | Value |
|--------|-------|
| Total Runs | {{ total_stats.count }} |
| Avg Duration | {{ total_stats.avg_duration_ms | duration }} |
| Median Duration | {{ total_stats.median_duration_ms | duration }} |
| P95 Duration | {{ total_stats.p95_duration_ms | duration }} |
| Total Cost | {{ total_stats.total_cost_usd | cost }} |
| Avg Cost | {{ total_stats.avg_cost_usd | cost }} |
| Avg I/O Match | {{ total_stats.avg_io_match_rate | pct }} |
| Success Rate | {{ total_stats.success_rate_pct | pct }} |
| Avg LOC Expansion | {{ "%.2f" | format(total_stats.avg_loc_expansion) }}x |
{% endif %}

## By Target Language

| Target | Count | Avg Duration | Avg Cost | Success Rate |
|--------|-------|--------------|----------|--------------|
{% for target, stats in by_target.items() %}
| {{ target }} | {{ stats.count }} | {{ stats.avg_duration_ms | duration }} | {{ stats.avg_cost_usd | cost }} | {{ stats.success_rate_pct | pct }} |
{% endfor %}

## By Strategy

| Strategy | Count | Avg Duration | Avg Cost | Success Rate |
|----------|-------|--------------|----------|--------------|
{% for strat, stats in by_strategy.items() %}
| {{ strat }} | {{ stats.count }} | {{ stats.avg_duration_ms | duration }} | {{ stats.avg_cost_usd | cost }} | {{ stats.success_rate_pct | pct }} |
{% endfor %}

## Projects

{% for project in projects %}
- {{ project }}
{% endfor %}

---
*Aggregated from {{ total_count }} migration runs*
"""

LATEX_TABLE_TEMPLATE = r"""\begin{table}[h]
\centering
\caption{{ '{' }}{{ caption }}{{ '}' }}
\label{{ '{' }}{{ label }}{{ '}' }}
\begin{tabular}{@{}l{% for r in runs %}r{% endfor %}@{}}
\toprule
Metric {% for r in runs %}& {{ r.identity.strategy | replace("_", "-") }} {% endfor %}\\
\midrule
Duration {% for r in runs %}& {{ (r.timing.wall_clock_duration_ms / 60000) | round(0) | int }} min {% endfor %}\\
Cost {% for r in runs %}& \${{ "%.2f" | format(r.cost.total_cost_usd) }} {% endfor %}\\
Prod.\ LOC {% for r in runs %}& {{ r.target_metrics.production_loc | number }} {% endfor %}\\
Tests {% for r in runs %}& {{ r.quality_gates.unit_tests.total }} {% endfor %}\\
I/O Match {% for r in runs %}& {{ "%.0f" | format(r.io_contract.match_rate_pct) }}\% {% endfor %}\\
\bottomrule
\end{tabular}
\end{table}
"""

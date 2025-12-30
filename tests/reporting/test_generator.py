"""Unit tests for ReportGenerator."""

from unittest.mock import MagicMock

import pytest

from migration.reporting.generator import (
    TEMPLATES_DIR,
    ReportGenerator,
    create_default_templates,
)
from migration.reporting.schema import (
    AgentMetrics,
    CodeMetrics,
    CostMetrics,
    IdentityMetrics,
    IOContractMetrics,
    MigrationMetrics,
    OutcomeMetrics,
    QualityGates,
    TimingMetrics,
    TokenMetrics,
)


def create_test_metrics(
    run_id: str = "test-run",
    project: str = "test-project",
    strategy: str = "module-by-module",
) -> MigrationMetrics:
    """Create a test MigrationMetrics instance."""
    return MigrationMetrics(
        identity=IdentityMetrics(
            run_id=run_id,
            project_name=project,
            source_language="python",
            target_language="rust",
            strategy=strategy,
            started_at="2024-01-01T10:00:00",
            completed_at="2024-01-01T10:30:00",
        ),
        timing=TimingMetrics(
            wall_clock_duration_ms=1800000,
            api_duration_ms=1900000,
            phase_durations_ms={"analysis": 300000, "migration": 1500000},
            module_durations=[],
        ),
        cost=CostMetrics(total_cost_usd=2.50),
        tokens=TokenMetrics(
            input_tokens=10000,
            output_tokens=5000,
            cache_creation_input_tokens=20000,
            cache_read_input_tokens=80000,
        ),
        agent=AgentMetrics(
            total_turns=15,
            total_messages=30,
            tool_invocations={"Bash": 10, "Read": 20},
            subagent_invocations={"analyst": 1, "migrator": 5},
        ),
        io_contract=IOContractMetrics(total_test_cases=10, passed=10),
        quality_gates=QualityGates(),
        source_metrics=CodeMetrics(production_loc=500, test_loc=100),
        target_metrics=CodeMetrics(production_loc=600, test_loc=150),
        outcome=OutcomeMetrics(status="success"),
    )


class TestReportGenerator:
    """Tests for ReportGenerator class."""

    def test_init_creates_jinja_environment(self) -> None:
        """Test that init creates Jinja environment."""
        generator = ReportGenerator()
        assert generator.env is not None

    def test_init_registers_custom_filters(self) -> None:
        """Test that custom filters are registered."""
        generator = ReportGenerator()
        assert "duration" in generator.env.filters
        assert "cost" in generator.env.filters
        assert "pct" in generator.env.filters
        assert "number" in generator.env.filters


class TestFormatFilters:
    """Tests for custom Jinja2 filters."""

    def test_format_duration_milliseconds(self) -> None:
        """Test formatting milliseconds."""
        generator = ReportGenerator()
        assert generator._format_duration(500) == "500ms"

    def test_format_duration_seconds(self) -> None:
        """Test formatting seconds."""
        generator = ReportGenerator()
        assert generator._format_duration(5500) == "5.5s"

    def test_format_duration_minutes(self) -> None:
        """Test formatting minutes."""
        generator = ReportGenerator()
        assert generator._format_duration(90000) == "1.5min"

    def test_format_duration_hours(self) -> None:
        """Test formatting hours."""
        generator = ReportGenerator()
        assert generator._format_duration(7200000) == "2.0h"

    def test_format_cost(self) -> None:
        """Test formatting USD cost."""
        generator = ReportGenerator()
        assert generator._format_cost(5.5) == "$5.50"
        assert generator._format_cost(0.99) == "$0.99"

    def test_format_percentage(self) -> None:
        """Test formatting percentage."""
        generator = ReportGenerator()
        assert generator._format_percentage(92.567) == "92.6%"

    def test_format_number(self) -> None:
        """Test formatting number with thousands separator."""
        generator = ReportGenerator()
        assert generator._format_number(1234567) == "1,234,567"


class TestGenerateRunReport:
    """Tests for generate_run_report method."""

    def test_generate_run_report_contains_project_name(self) -> None:
        """Test that run report contains project name."""
        generator = ReportGenerator()
        metrics = create_test_metrics(project="my-project")

        report = generator.generate_run_report(metrics)

        assert "my-project" in report

    def test_generate_run_report_contains_run_id(self) -> None:
        """Test that run report contains run ID."""
        generator = ReportGenerator()
        metrics = create_test_metrics(run_id="run-123")

        report = generator.generate_run_report(metrics)

        assert "run-123" in report

    def test_generate_run_report_returns_string(self) -> None:
        """Test that run report returns a string."""
        generator = ReportGenerator()
        metrics = create_test_metrics()

        report = generator.generate_run_report(metrics)

        assert isinstance(report, str)
        assert len(report) > 0


class TestGenerateComparison:
    """Tests for generate_comparison method."""

    def test_generate_comparison_with_multiple_runs(self) -> None:
        """Test comparison report with multiple runs."""
        generator = ReportGenerator()
        runs = [
            create_test_metrics(run_id="run-1", strategy="module-by-module"),
            create_test_metrics(run_id="run-2", strategy="feature-by-feature"),
        ]

        report = generator.generate_comparison(runs)

        assert "module-by-module" in report
        assert "feature-by-feature" in report

    def test_generate_comparison_with_custom_title(self) -> None:
        """Test comparison report with custom title."""
        generator = ReportGenerator()
        runs = [create_test_metrics()]

        report = generator.generate_comparison(runs, title="Custom Title")

        assert "Custom Title" in report


class TestGenerateLatexTable:
    """Tests for generate_latex_table method."""

    def test_generate_latex_table_basic(self) -> None:
        """Test LaTeX table generation."""
        generator = ReportGenerator()
        runs = [create_test_metrics()]

        table = generator.generate_latex_table(runs)

        assert "\\begin{table}" in table
        assert "\\end{table}" in table

    def test_generate_latex_table_with_caption(self) -> None:
        """Test LaTeX table with custom caption."""
        generator = ReportGenerator()
        runs = [create_test_metrics()]

        table = generator.generate_latex_table(runs, caption="My Caption")

        assert "My Caption" in table

    def test_generate_latex_table_with_label(self) -> None:
        """Test LaTeX table with custom label."""
        generator = ReportGenerator()
        runs = [create_test_metrics()]

        table = generator.generate_latex_table(runs, label="tab:custom")

        assert "tab:custom" in table


class TestGenerateSummary:
    """Tests for generate_summary method."""

    def test_generate_summary_with_mock_database(self) -> None:
        """Test summary generation with mock database."""
        generator = ReportGenerator()

        # Create mock database
        mock_db = MagicMock()
        mock_db.count.return_value = 10
        mock_db.aggregate.return_value = None
        mock_db.group_by.return_value = {}
        mock_db.list_projects.return_value = ["project1", "project2"]

        summary = generator.generate_summary(mock_db)

        assert "10" in summary  # Total count
        assert "project1" in summary
        assert "project2" in summary


class TestCreateDefaultTemplates:
    """Tests for create_default_templates function."""

    def test_create_default_templates_creates_directory(self) -> None:
        """Test that templates directory is created."""
        # This tests the function runs without error
        # The actual template creation happens in the module
        create_default_templates()
        assert TEMPLATES_DIR.exists()

    def test_create_default_templates_creates_files(self) -> None:
        """Test that template files are created."""
        create_default_templates()

        expected_files = [
            "run_report.md.j2",
            "comparison.md.j2",
            "summary.md.j2",
            "table.tex.j2",
        ]
        for filename in expected_files:
            template_path = TEMPLATES_DIR / filename
            assert template_path.exists(), f"{filename} should exist"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

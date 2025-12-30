"""Unit tests for reporting CLI."""

import argparse
import sys
import tempfile
from pathlib import Path
from unittest.mock import patch

import pytest

from migration.reporting.cli import (
    cmd_backfill,
    cmd_compare,
    cmd_export,
    cmd_query,
    cmd_report,
    cmd_stats,
    main,
)
from migration.reporting.database import MigrationDatabase
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
) -> MigrationMetrics:
    """Create a test MigrationMetrics instance."""
    return MigrationMetrics(
        identity=IdentityMetrics(
            run_id=run_id,
            project_name=project,
            source_language="python",
            target_language="rust",
            strategy="module-by-module",
            started_at="2024-01-01T10:00:00",
            completed_at="2024-01-01T10:30:00",
        ),
        timing=TimingMetrics(
            wall_clock_duration_ms=1800000,
            api_duration_ms=1900000,
            phase_durations_ms={},
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
            tool_invocations={},
            subagent_invocations={},
        ),
        io_contract=IOContractMetrics(total_test_cases=10, passed=10),
        quality_gates=QualityGates(),
        source_metrics=CodeMetrics(production_loc=500),
        target_metrics=CodeMetrics(production_loc=600),
        outcome=OutcomeMetrics(status="success"),
    )


class TestCmdReport:
    """Tests for cmd_report command."""

    def test_report_file_not_found(self) -> None:
        """Test error when metrics file doesn't exist."""
        args = argparse.Namespace(
            metrics_json="/nonexistent/file.json",
            format="markdown",
            output=None,
        )

        result = cmd_report(args)

        assert result == 1

    def test_report_markdown_to_stdout(self, capsys: pytest.CaptureFixture) -> None:
        """Test generating markdown report to stdout."""
        metrics = create_test_metrics()

        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".json", delete=False
        ) as f:
            f.write(metrics.to_json())
            f.flush()

            args = argparse.Namespace(
                metrics_json=f.name,
                format="markdown",
                output=None,
            )

            result = cmd_report(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "test-project" in captured.out

    def test_report_latex_format(self, capsys: pytest.CaptureFixture) -> None:
        """Test generating LaTeX report."""
        metrics = create_test_metrics()

        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".json", delete=False
        ) as f:
            f.write(metrics.to_json())
            f.flush()

            args = argparse.Namespace(
                metrics_json=f.name,
                format="latex",
                output=None,
            )

            result = cmd_report(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "\\begin{table}" in captured.out

    def test_report_to_file(self) -> None:
        """Test writing report to file."""
        metrics = create_test_metrics()

        with (
            tempfile.NamedTemporaryFile(
                mode="w", suffix=".json", delete=False
            ) as metrics_file,
            tempfile.NamedTemporaryFile(
                mode="w", suffix=".md", delete=False
            ) as output_file,
        ):
            metrics_file.write(metrics.to_json())
            metrics_file.flush()

            args = argparse.Namespace(
                metrics_json=metrics_file.name,
                format="markdown",
                output=output_file.name,
            )

            result = cmd_report(args)

            assert result == 0
            content = Path(output_file.name).read_text()
            assert "test-project" in content


class TestCmdQuery:
    """Tests for cmd_query command."""

    def test_query_no_results(self, capsys: pytest.CaptureFixture) -> None:
        """Test query with no results."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            args = argparse.Namespace(
                database=Path(f.name),
                project=None,
                target=None,
                strategy=None,
                status=None,
                limit=20,
            )

            result = cmd_query(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "No matching migrations found" in captured.out

    def test_query_with_results(self, capsys: pytest.CaptureFixture) -> None:
        """Test query with results."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            db.insert(create_test_metrics(run_id="run-1"))
            db.insert(create_test_metrics(run_id="run-2"))

            args = argparse.Namespace(
                database=Path(f.name),
                project=None,
                target=None,
                strategy=None,
                status=None,
                limit=20,
            )

            result = cmd_query(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "Found 2 migration(s)" in captured.out

    def test_query_with_filter(self, capsys: pytest.CaptureFixture) -> None:
        """Test query with project filter."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            db.insert(create_test_metrics(run_id="run-1", project="project-a"))
            db.insert(create_test_metrics(run_id="run-2", project="project-b"))

            args = argparse.Namespace(
                database=Path(f.name),
                project="project-a",
                target=None,
                strategy=None,
                status=None,
                limit=20,
            )

            result = cmd_query(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "Found 1 migration(s)" in captured.out


class TestCmdExport:
    """Tests for cmd_export command."""

    def test_export_json(self) -> None:
        """Test JSON export."""
        with (
            tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file,
            tempfile.NamedTemporaryFile(suffix=".json", delete=False) as out_file,
        ):
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics())

            args = argparse.Namespace(
                database=Path(db_file.name),
                format="json",
                output=out_file.name,
            )

            result = cmd_export(args)

            assert result == 0
            content = Path(out_file.name).read_text()
            assert "test-project" in content

    def test_export_markdown_to_stdout(self, capsys: pytest.CaptureFixture) -> None:
        """Test markdown export to stdout."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics())

            args = argparse.Namespace(
                database=Path(db_file.name),
                format="markdown",
                output=None,
            )

            result = cmd_export(args)

        assert result == 0
        captured = capsys.readouterr()
        # Summary includes project list
        assert "test-project" in captured.out

    def test_export_latex_no_migrations(self) -> None:
        """Test LaTeX export with no migrations."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            MigrationDatabase(Path(db_file.name))

            args = argparse.Namespace(
                database=Path(db_file.name),
                format="latex",
                output=None,
            )

            result = cmd_export(args)

        assert result == 1  # Error - no migrations


class TestCmdCompare:
    """Tests for cmd_compare command."""

    def test_compare_not_enough_runs(self, capsys: pytest.CaptureFixture) -> None:
        """Test compare with fewer than 2 runs."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics(run_id="run-1"))

            args = argparse.Namespace(
                database=Path(db_file.name),
                run_ids=["run-1"],
                format="markdown",
                title=None,
                output=None,
            )

            result = cmd_compare(args)

        assert result == 1
        captured = capsys.readouterr()
        assert "Need at least 2 runs" in captured.err

    def test_compare_runs_found(self, capsys: pytest.CaptureFixture) -> None:
        """Test successful comparison."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics(run_id="run-1"))
            db.insert(create_test_metrics(run_id="run-2"))

            args = argparse.Namespace(
                database=Path(db_file.name),
                run_ids=["run-1", "run-2"],
                format="markdown",
                title="My Comparison",
                output=None,
            )

            result = cmd_compare(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "My Comparison" in captured.out

    def test_compare_run_not_found(self, capsys: pytest.CaptureFixture) -> None:
        """Test comparison with missing run."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics(run_id="run-1"))

            args = argparse.Namespace(
                database=Path(db_file.name),
                run_ids=["run-1", "nonexistent"],
                format="markdown",
                title=None,
                output=None,
            )

            result = cmd_compare(args)

        # Only 1 run found, should fail
        assert result == 1
        captured = capsys.readouterr()
        assert "Warning: Run not found" in captured.err


class TestCmdBackfill:
    """Tests for cmd_backfill command."""

    def test_backfill_file_not_found(self) -> None:
        """Test backfill with nonexistent file."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            args = argparse.Namespace(
                database=Path(db_file.name),
                log_file="/nonexistent/log.log",
                project=None,
                strategy=None,
                metrics_output=None,
            )

            result = cmd_backfill(args)

        assert result == 1

    def test_backfill_with_project_override(
        self, capsys: pytest.CaptureFixture
    ) -> None:
        """Test backfill with project name override."""
        # Create a minimal log file
        log_content = (
            "[10:00:00] Starting Migration: unknown -> rust\n"
            "[10:00:00] Strategy: module-by-module\n"
            "[10:00:05] MSG #1\n"
            "ResultMessage(duration_ms=60000, duration_api_ms=70000, "
            "num_turns=5, total_cost_usd=1.50, subtype='success')\n"
        )
        with (
            tempfile.NamedTemporaryFile(
                mode="w", suffix=".log", delete=False
            ) as log_file,
            tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file,
            tempfile.NamedTemporaryFile(suffix=".json", delete=False) as metrics_file,
        ):
            log_file.write(log_content)
            log_file.flush()

            args = argparse.Namespace(
                database=Path(db_file.name),
                log_file=log_file.name,
                project="my-project",
                strategy="feature-by-feature",
                metrics_output=metrics_file.name,
            )

            result = cmd_backfill(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "my-project" in captured.out
        assert "feature-by-feature" in captured.out


class TestCmdStats:
    """Tests for cmd_stats command."""

    def test_stats_empty_database(self, capsys: pytest.CaptureFixture) -> None:
        """Test stats on empty database."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            MigrationDatabase(Path(db_file.name))

            args = argparse.Namespace(database=Path(db_file.name))

            result = cmd_stats(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "Database is empty" in captured.out

    def test_stats_with_data(self, capsys: pytest.CaptureFixture) -> None:
        """Test stats with data."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics(run_id="run-1"))
            db.insert(create_test_metrics(run_id="run-2"))

            args = argparse.Namespace(database=Path(db_file.name))

            result = cmd_stats(args)

        assert result == 0
        captured = capsys.readouterr()
        assert "Total Migrations: 2" in captured.out
        assert "By Target Language" in captured.out
        assert "rust" in captured.out


class TestMain:
    """Tests for main entry point."""

    def test_main_requires_command(self) -> None:
        """Test that main requires a command."""
        with pytest.raises(SystemExit), patch.object(sys, "argv", ["prog"]):
            main()

    def test_main_stats_command(self) -> None:
        """Test main with stats command."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file:
            db_name = db_file.name
        with patch.object(sys, "argv", ["prog", "--database", db_name, "stats"]):
            result = main()
            assert result == 0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

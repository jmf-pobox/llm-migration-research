"""Unit tests for MigrationDatabase."""

import tempfile
from pathlib import Path

import pytest

from migration.reporting.database import MigrationDatabase
from migration.reporting.schema import (
    AgentMetrics,
    CodeMetrics,
    CostMetrics,
    CoverageResult,
    IdentityMetrics,
    IOContractMetrics,
    MigrationMetrics,
    OutcomeMetrics,
    QualityGates,
    TestOutcomeResult,
    TimingMetrics,
    TokenMetrics,
)


def create_test_metrics(
    run_id: str = "test-run-1",
    project: str = "test-project",
    target: str = "rust",
    strategy: str = "module-by-module",
    status: str = "success",
    duration_ms: int = 60000,
    cost_usd: float = 1.50,
    io_match_rate: float = 100.0,
    coverage_pct: float = 85.0,
    source_loc: int = 500,
    target_loc: int = 600,
) -> MigrationMetrics:
    """Create a test MigrationMetrics instance."""
    return MigrationMetrics(
        identity=IdentityMetrics(
            run_id=run_id,
            project_name=project,
            source_language="python",
            target_language=target,
            strategy=strategy,
            started_at="2024-01-01T10:00:00",
            completed_at="2024-01-01T10:01:00",
        ),
        timing=TimingMetrics(
            wall_clock_duration_ms=duration_ms,
            api_duration_ms=duration_ms + 1000,
            phase_durations_ms={},
            module_durations=[],
        ),
        cost=CostMetrics(total_cost_usd=cost_usd),
        tokens=TokenMetrics(
            input_tokens=1000,
            output_tokens=500,
            cache_creation_input_tokens=100,
            cache_read_input_tokens=200,
        ),
        agent=AgentMetrics(
            total_turns=10,
            total_messages=20,
            tool_invocations={},
            subagent_invocations={},
        ),
        io_contract=IOContractMetrics(
            total_test_cases=10,
            passed=int(io_match_rate / 10),
        ),
        quality_gates=QualityGates(
            compilation=None,
            linting=None,
            formatting=None,
            unit_tests=TestOutcomeResult(
                passed=True, total=5, passed_count=5, failed_count=0, skipped_count=0
            ),
            coverage=CoverageResult(
                line_coverage_pct=coverage_pct, branch_coverage_pct=0
            ),
        ),
        source_metrics=CodeMetrics(production_loc=source_loc, test_loc=100),
        target_metrics=CodeMetrics(production_loc=target_loc, test_loc=150),
        outcome=OutcomeMetrics(status=status),
    )


class TestMigrationDatabase:
    """Tests for MigrationDatabase class."""

    def test_init_creates_tables(self) -> None:
        """Test that init creates the required tables."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            assert db.count() == 0

    def test_insert_and_get(self) -> None:
        """Test inserting and retrieving a migration."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            metrics = create_test_metrics()

            db.insert(metrics)
            assert db.count() == 1

            retrieved = db.get("test-run-1")
            assert retrieved is not None
            assert retrieved.identity.run_id == "test-run-1"
            assert retrieved.identity.project_name == "test-project"

    def test_get_nonexistent(self) -> None:
        """Test getting a non-existent migration returns None."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            assert db.get("nonexistent") is None

    def test_insert_replaces_existing(self) -> None:
        """Test that inserting with same run_id replaces existing."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            metrics1 = create_test_metrics(cost_usd=1.00)
            db.insert(metrics1)

            metrics2 = create_test_metrics(cost_usd=2.00)
            db.insert(metrics2)

            assert db.count() == 1
            retrieved = db.get("test-run-1")
            assert retrieved is not None
            assert retrieved.cost.total_cost_usd == 2.00

    def test_query_by_project(self) -> None:
        """Test querying by project name."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", project="project-a"))
            db.insert(create_test_metrics(run_id="run-2", project="project-b"))
            db.insert(create_test_metrics(run_id="run-3", project="project-a"))

            results = db.query(project="project-a")
            assert len(results) == 2

    def test_query_by_target(self) -> None:
        """Test querying by target language."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", target="rust"))
            db.insert(create_test_metrics(run_id="run-2", target="java"))
            db.insert(create_test_metrics(run_id="run-3", target="rust"))

            results = db.query(target="rust")
            assert len(results) == 2

    def test_query_by_strategy(self) -> None:
        """Test querying by migration strategy."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", strategy="module-by-module"))
            db.insert(
                create_test_metrics(run_id="run-2", strategy="feature-by-feature")
            )

            results = db.query(strategy="module-by-module")
            assert len(results) == 1

    def test_query_by_status(self) -> None:
        """Test querying by migration status."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", status="success"))
            db.insert(create_test_metrics(run_id="run-2", status="failure"))

            results = db.query(status="success")
            assert len(results) == 1

    def test_query_with_limit(self) -> None:
        """Test query respects limit parameter."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            for i in range(10):
                db.insert(create_test_metrics(run_id=f"run-{i}"))

            results = db.query(limit=5)
            assert len(results) == 5

    def test_aggregate_empty_db(self) -> None:
        """Test aggregate on empty database returns None."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            stats = db.aggregate()
            assert stats is None

    def test_aggregate_all(self) -> None:
        """Test aggregate statistics across all migrations."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(
                create_test_metrics(run_id="run-1", duration_ms=60000, cost_usd=1.00)
            )
            db.insert(
                create_test_metrics(run_id="run-2", duration_ms=120000, cost_usd=2.00)
            )
            db.insert(
                create_test_metrics(run_id="run-3", duration_ms=180000, cost_usd=3.00)
            )

            stats = db.aggregate()
            assert stats is not None
            assert stats.count == 3
            assert stats.total_cost_usd == 6.00
            assert stats.avg_cost_usd == 2.00
            assert stats.avg_duration_ms == 120000.0

    def test_aggregate_filtered(self) -> None:
        """Test aggregate with filters."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", target="rust", cost_usd=1.00))
            db.insert(create_test_metrics(run_id="run-2", target="java", cost_usd=5.00))
            db.insert(create_test_metrics(run_id="run-3", target="rust", cost_usd=3.00))

            stats = db.aggregate(target="rust")
            assert stats is not None
            assert stats.count == 2
            assert stats.total_cost_usd == 4.00

    def test_delete(self) -> None:
        """Test deleting a migration."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            db.insert(create_test_metrics())

            assert db.count() == 1
            result = db.delete("test-run-1")
            assert result is True
            assert db.count() == 0

    def test_delete_nonexistent(self) -> None:
        """Test deleting non-existent migration returns False."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))
            result = db.delete("nonexistent")
            assert result is False

    def test_list_projects(self) -> None:
        """Test listing unique project names."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", project="alpha"))
            db.insert(create_test_metrics(run_id="run-2", project="beta"))
            db.insert(create_test_metrics(run_id="run-3", project="alpha"))

            projects = db.list_projects()
            assert len(projects) == 2
            assert "alpha" in projects
            assert "beta" in projects

    def test_list_targets(self) -> None:
        """Test listing unique target languages."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", target="rust"))
            db.insert(create_test_metrics(run_id="run-2", target="java"))
            db.insert(create_test_metrics(run_id="run-3", target="go"))

            targets = db.list_targets()
            assert len(targets) == 3
            assert set(targets) == {"rust", "java", "go"}

    def test_group_by_target(self) -> None:
        """Test grouping statistics by target language."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            db.insert(create_test_metrics(run_id="run-1", target="rust", cost_usd=1.00))
            db.insert(create_test_metrics(run_id="run-2", target="rust", cost_usd=2.00))
            db.insert(create_test_metrics(run_id="run-3", target="java", cost_usd=5.00))

            grouped = db.group_by("target_language")
            assert "rust" in grouped
            assert "java" in grouped
            assert grouped["rust"].count == 2
            assert grouped["java"].count == 1

    def test_group_by_invalid_field(self) -> None:
        """Test group_by with invalid field raises ValueError."""
        with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
            db = MigrationDatabase(Path(f.name))

            with pytest.raises(ValueError):
                db.group_by("invalid_field")

    def test_export_to_json(self) -> None:
        """Test exporting to JSON file."""
        with (
            tempfile.NamedTemporaryFile(suffix=".db", delete=False) as db_file,
            tempfile.NamedTemporaryFile(suffix=".json", delete=False) as json_file,
        ):
            db = MigrationDatabase(Path(db_file.name))
            db.insert(create_test_metrics())

            db.export_to_json(Path(json_file.name))

            # Verify file was written
            content = Path(json_file.name).read_text()
            assert "test-run-1" in content
            assert "test-project" in content


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

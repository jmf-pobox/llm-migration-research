"""Unit tests for MetricsCollector."""

import time
from typing import Any

import pytest

from migration.reporting.collector import MetricsCollector


class MockResultMessage:
    """Mock ResultMessage from Claude Agent SDK."""

    def __init__(
        self,
        subtype: str = "success",
        duration_ms: int = 1500000,
        duration_api_ms: int = 1900000,
        num_turns: int = 15,
        total_cost_usd: float = 3.50,
        usage: dict[str, int] | None = None,
    ) -> None:
        self.subtype = subtype
        self.duration_ms = duration_ms
        self.duration_api_ms = duration_api_ms
        self.num_turns = num_turns
        self.total_cost_usd = total_cost_usd
        self.usage: dict[str, Any] = usage or {
            "input_tokens": 100,
            "cache_creation_input_tokens": 50000,
            "cache_read_input_tokens": 350000,
            "output_tokens": 15000,
        }


class TestMetricsCollector:
    """Tests for MetricsCollector class."""

    def test_message_counting(self) -> None:
        """Test that messages are properly counted via record_message()."""
        collector = MetricsCollector("test", "python", "rust", "test")

        for _ in range(100):
            collector.record_message()

        metrics = collector.finalize()
        assert metrics.agent.total_messages == 100

    def test_tool_invocations(self) -> None:
        """Test that tool invocations are tracked."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_tool_use("Bash")
        collector.record_tool_use("Bash")
        collector.record_tool_use("Read")
        collector.record_tool_use("Write")

        metrics = collector.finalize()
        assert metrics.agent.tool_invocations["Bash"] == 2
        assert metrics.agent.tool_invocations["Read"] == 1
        assert metrics.agent.tool_invocations["Write"] == 1

    def test_subagent_invocations(self) -> None:
        """Test that subagent invocations are tracked."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_subagent("analyst")
        collector.record_subagent("migrator")
        collector.record_subagent("migrator")
        collector.record_subagent("reviewer")

        metrics = collector.finalize()
        assert metrics.agent.subagent_invocations["analyst"] == 1
        assert metrics.agent.subagent_invocations["migrator"] == 2
        assert metrics.agent.subagent_invocations["reviewer"] == 1

    def test_record_result_extracts_timing(self) -> None:
        """Test that record_result extracts timing from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(duration_ms=1234567, duration_api_ms=2345678)
        collector.record_result(result)

        assert collector.timing.wall_clock_duration_ms == 1234567
        assert collector.timing.api_duration_ms == 2345678

    def test_record_result_extracts_cost(self) -> None:
        """Test that record_result extracts cost from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(total_cost_usd=5.75)
        collector.record_result(result)

        assert collector.cost.total_cost_usd == 5.75

    def test_record_result_extracts_tokens(self) -> None:
        """Test that record_result extracts token usage from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        usage = {
            "input_tokens": 50,
            "cache_creation_input_tokens": 10000,
            "cache_read_input_tokens": 200000,
            "output_tokens": 25000,
        }
        result = MockResultMessage(usage=usage)
        collector.record_result(result)

        assert collector.tokens.input_tokens == 50
        assert collector.tokens.cache_creation_input_tokens == 10000
        assert collector.tokens.cache_read_input_tokens == 200000
        assert collector.tokens.output_tokens == 25000

    def test_record_result_extracts_turns(self) -> None:
        """Test that record_result extracts turns from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(num_turns=42)
        collector.record_result(result)

        assert collector.agent.total_turns == 42

    def test_record_result_success_status(self) -> None:
        """Test that record_result sets success status correctly."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(subtype="success")
        collector.record_result(result)

        assert collector.outcome.status == "success"

    def test_record_result_failure_status(self) -> None:
        """Test that record_result sets failure status correctly."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(subtype="error")
        collector.record_result(result)

        assert collector.outcome.status == "failure"

    def test_phase_timing(self) -> None:
        """Test that phase durations are tracked."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.start_phase("setup")
        time.sleep(0.1)  # 100ms
        collector.end_phase("setup")

        metrics = collector.finalize()
        assert "setup" in metrics.timing.phase_durations_ms
        assert metrics.timing.phase_durations_ms["setup"] >= 100

    def test_finalize_sets_completed_at(self) -> None:
        """Test that finalize sets completed_at timestamp."""
        collector = MetricsCollector("test", "python", "rust", "test")

        metrics = collector.finalize()
        assert metrics.identity.completed_at is not None

    def test_cache_efficiency_ratio(self) -> None:
        """Test cache efficiency ratio calculation."""
        collector = MetricsCollector("test", "python", "rust", "test")

        usage = {
            "input_tokens": 1000,
            "cache_creation_input_tokens": 5000,
            "cache_read_input_tokens": 9000,
            "output_tokens": 500,
        }
        result = MockResultMessage(usage=usage)
        collector.record_result(result)

        metrics = collector.finalize()
        # Formula: cache_read / (cache_read + input) = 9000 / 10000 = 0.9
        assert metrics.tokens.cache_efficiency_ratio == 0.9

    def test_module_timing(self) -> None:
        """Test that module durations are tracked."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.start_module("lexer")
        time.sleep(0.05)  # 50ms
        collector.end_module("lexer")

        collector.start_module("parser")
        time.sleep(0.05)
        collector.end_module("parser", attempts=2)

        metrics = collector.finalize()
        assert len(metrics.timing.module_durations) == 2
        assert metrics.timing.module_durations[0].module_name == "lexer"
        assert metrics.timing.module_durations[0].duration_ms >= 50
        assert metrics.timing.module_durations[1].attempts == 2

    def test_record_error_recovery(self) -> None:
        """Test recording error recovery events."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_error_recovery()
        collector.record_error_recovery()

        metrics = collector.finalize()
        assert metrics.agent.error_recovery_events == 2

    def test_record_retry(self) -> None:
        """Test recording retry attempts."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_retry()
        collector.record_retry()
        collector.record_retry()

        metrics = collector.finalize()
        assert metrics.agent.retry_count == 3

    def test_record_coverage(self) -> None:
        """Test recording coverage percentage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_coverage(85.5)

        metrics = collector.finalize()
        assert metrics.quality_gates.coverage is not None
        assert metrics.quality_gates.coverage.line_coverage_pct == 85.5

    def test_record_coverage_none(self) -> None:
        """Test recording None coverage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_coverage(None)

        metrics = collector.finalize()
        assert metrics.quality_gates.coverage is not None
        assert metrics.quality_gates.coverage.line_coverage_pct is None

    def test_record_idiomaticness(self) -> None:
        """Test recording idiomaticness score."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_idiomaticness("Idiomatic", "Code follows Rust conventions")

        metrics = collector.finalize()
        assert metrics.quality_gates.idiomaticness is not None
        assert metrics.quality_gates.idiomaticness.score == "Idiomatic"
        assert metrics.quality_gates.idiomaticness.reasoning is not None

    def test_record_source_loc(self) -> None:
        """Test recording source LOC metrics."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_source_loc(
            production_loc=500,
            test_loc=200,
            module_count=5,
            function_count=30,
        )

        metrics = collector.finalize()
        assert metrics.source_metrics.production_loc == 500
        assert metrics.source_metrics.test_loc == 200
        assert metrics.source_metrics.total_loc == 700
        assert metrics.source_metrics.module_count == 5
        assert metrics.source_metrics.function_count == 30

    def test_record_target_loc(self) -> None:
        """Test recording target LOC metrics."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.record_target_loc(
            production_loc=600,
            test_loc=250,
            module_count=6,
            function_count=35,
        )

        metrics = collector.finalize()
        assert metrics.target_metrics.production_loc == 600
        assert metrics.target_metrics.test_loc == 250
        assert metrics.target_metrics.total_loc == 850

    def test_set_outcome(self) -> None:
        """Test setting final outcome."""
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.set_outcome(
            status="partial",
            modules_completed=3,
            modules_total=5,
            blocking_issues=["Type error in parser"],
            notes="Migration partially complete",
        )

        metrics = collector.finalize()
        assert metrics.outcome.status == "partial"
        assert metrics.outcome.modules_completed == 3
        assert metrics.outcome.modules_total == 5
        assert len(metrics.outcome.blocking_issues) == 1
        assert metrics.outcome.notes == "Migration partially complete"

    def test_record_result_with_dict(self) -> None:
        """Test record_result with dict input instead of object."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result_dict = {
            "duration_ms": 5000,
            "duration_api_ms": 6000,
            "total_cost_usd": 2.50,
            "num_turns": 10,
            "subtype": "success",
            "usage": {
                "input_tokens": 100,
                "output_tokens": 50,
                "cache_creation_input_tokens": 1000,
                "cache_read_input_tokens": 2000,
            },
        }
        collector.record_result(result_dict)

        assert collector.timing.wall_clock_duration_ms == 5000
        assert collector.cost.total_cost_usd == 2.50
        assert collector.tokens.input_tokens == 100
        assert collector.outcome.status == "success"

    def test_finalize_calculates_wall_clock_if_not_set(self) -> None:
        """Test that finalize calculates wall clock from start time if not set."""
        collector = MetricsCollector("test", "python", "rust", "test")

        time.sleep(0.05)  # 50ms

        metrics = collector.finalize()
        # Should have calculated duration from start time
        assert metrics.timing.wall_clock_duration_ms >= 50


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

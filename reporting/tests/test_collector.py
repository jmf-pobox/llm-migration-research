"""Unit tests for MetricsCollector."""

import pytest
from reporting.collector import MetricsCollector


class MockResultMessage:
    """Mock ResultMessage from Claude Agent SDK."""

    def __init__(
        self,
        subtype='success',
        duration_ms=1500000,
        duration_api_ms=1900000,
        num_turns=15,
        total_cost_usd=3.50,
        usage=None
    ):
        self.subtype = subtype
        self.duration_ms = duration_ms
        self.duration_api_ms = duration_api_ms
        self.num_turns = num_turns
        self.total_cost_usd = total_cost_usd
        self.usage = usage or {
            'input_tokens': 100,
            'cache_creation_input_tokens': 50000,
            'cache_read_input_tokens': 350000,
            'output_tokens': 15000
        }


class TestMetricsCollector:
    """Tests for MetricsCollector class."""

    def test_message_counting(self):
        """Test that messages are properly counted via record_message()."""
        collector = MetricsCollector("test", "python", "rust", "test")

        for _ in range(100):
            collector.record_message()

        metrics = collector.finalize()
        assert metrics.agent.total_messages == 100

    def test_tool_invocations(self):
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

    def test_subagent_invocations(self):
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

    def test_record_result_extracts_timing(self):
        """Test that record_result extracts timing from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(duration_ms=1234567, duration_api_ms=2345678)
        collector.record_result(result)

        assert collector.timing.wall_clock_duration_ms == 1234567
        assert collector.timing.api_duration_ms == 2345678

    def test_record_result_extracts_cost(self):
        """Test that record_result extracts cost from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(total_cost_usd=5.75)
        collector.record_result(result)

        assert collector.cost.total_cost_usd == 5.75

    def test_record_result_extracts_tokens(self):
        """Test that record_result extracts token usage from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        usage = {
            'input_tokens': 50,
            'cache_creation_input_tokens': 10000,
            'cache_read_input_tokens': 200000,
            'output_tokens': 25000
        }
        result = MockResultMessage(usage=usage)
        collector.record_result(result)

        assert collector.tokens.input_tokens == 50
        assert collector.tokens.cache_creation_input_tokens == 10000
        assert collector.tokens.cache_read_input_tokens == 200000
        assert collector.tokens.output_tokens == 25000

    def test_record_result_extracts_turns(self):
        """Test that record_result extracts turns from ResultMessage."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(num_turns=42)
        collector.record_result(result)

        assert collector.agent.total_turns == 42

    def test_record_result_success_status(self):
        """Test that record_result sets success status correctly."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(subtype='success')
        collector.record_result(result)

        assert collector.outcome.status == "success"

    def test_record_result_failure_status(self):
        """Test that record_result sets failure status correctly."""
        collector = MetricsCollector("test", "python", "rust", "test")

        result = MockResultMessage(subtype='error')
        collector.record_result(result)

        assert collector.outcome.status == "failure"

    def test_phase_timing(self):
        """Test that phase durations are tracked."""
        import time
        collector = MetricsCollector("test", "python", "rust", "test")

        collector.start_phase("setup")
        time.sleep(0.1)  # 100ms
        collector.end_phase("setup")

        metrics = collector.finalize()
        assert "setup" in metrics.timing.phase_durations_ms
        assert metrics.timing.phase_durations_ms["setup"] >= 100

    def test_finalize_sets_completed_at(self):
        """Test that finalize sets completed_at timestamp."""
        collector = MetricsCollector("test", "python", "rust", "test")

        metrics = collector.finalize()
        assert metrics.identity.completed_at is not None

    def test_cache_efficiency_ratio(self):
        """Test cache efficiency ratio calculation."""
        collector = MetricsCollector("test", "python", "rust", "test")

        usage = {
            'input_tokens': 1000,
            'cache_creation_input_tokens': 5000,
            'cache_read_input_tokens': 9000,
            'output_tokens': 500
        }
        result = MockResultMessage(usage=usage)
        collector.record_result(result)

        metrics = collector.finalize()
        # Formula: cache_read / (cache_read + input) = 9000 / 10000 = 0.9
        assert metrics.tokens.cache_efficiency_ratio == 0.9


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

"""Real-time metrics collection during migration.

This module provides instrumentation hooks to capture timing,
tool usage, and other metrics during migration execution.
"""

import re
import time
from datetime import datetime
from pathlib import Path
from typing import Optional, Any

from .schema import (
    MigrationMetrics,
    IdentityMetrics,
    TimingMetrics,
    CostMetrics,
    TokenMetrics,
    AgentMetrics,
    ModuleTiming,
    OutcomeMetrics,
)


class MetricsCollector:
    """Collects metrics during migration execution.

    Usage:
        collector = MetricsCollector(
            project_name="rpn2tex",
            source_language="python",
            target_language="rust",
            strategy="module-by-module",
        )

        collector.start_phase("analysis")
        # ... do analysis ...
        collector.end_phase("analysis")

        collector.start_module("lexer")
        # ... migrate lexer ...
        collector.end_module("lexer", attempts=2)

        collector.record_tool_use("Bash")
        collector.record_tool_use("Read")

        # At end of migration:
        collector.record_result(result_message)
        metrics = collector.finalize()
        metrics.save("metrics.json")
    """

    def __init__(
        self,
        project_name: str,
        source_language: str,
        target_language: str,
        strategy: str,
    ):
        self.identity = IdentityMetrics.create(
            project_name=project_name,
            source_language=source_language,
            target_language=target_language,
            strategy=strategy,
        )
        self.timing = TimingMetrics()
        self.cost = CostMetrics()
        self.tokens = TokenMetrics()
        self.agent = AgentMetrics()
        self.outcome = OutcomeMetrics()

        # Internal tracking
        self._phase_start_times: dict[str, float] = {}
        self._module_start_times: dict[str, float] = {}
        self._module_attempts: dict[str, int] = {}
        self._start_time = time.time()
        self._message_count = 0

    def start_phase(self, phase_name: str) -> None:
        """Mark the start of a migration phase."""
        self._phase_start_times[phase_name] = time.time()

    def end_phase(self, phase_name: str) -> None:
        """Mark the end of a migration phase."""
        if phase_name in self._phase_start_times:
            duration_ms = int((time.time() - self._phase_start_times[phase_name]) * 1000)
            self.timing.phase_durations_ms[phase_name] = duration_ms

    def start_module(self, module_name: str) -> None:
        """Mark the start of a module migration."""
        self._module_start_times[module_name] = time.time()
        self._module_attempts[module_name] = self._module_attempts.get(module_name, 0) + 1

    def end_module(self, module_name: str, attempts: Optional[int] = None) -> None:
        """Mark the end of a module migration."""
        if module_name in self._module_start_times:
            duration_ms = int((time.time() - self._module_start_times[module_name]) * 1000)
            actual_attempts = attempts or self._module_attempts.get(module_name, 1)
            self.timing.module_durations.append(
                ModuleTiming(
                    module_name=module_name,
                    duration_ms=duration_ms,
                    attempts=actual_attempts,
                )
            )

    def record_tool_use(self, tool_name: str) -> None:
        """Record a tool invocation."""
        self.agent.tool_invocations[tool_name] = (
            self.agent.tool_invocations.get(tool_name, 0) + 1
        )

    def record_subagent(self, agent_type: str) -> None:
        """Record a subagent invocation."""
        self.agent.subagent_invocations[agent_type] = (
            self.agent.subagent_invocations.get(agent_type, 0) + 1
        )

    def record_message(self) -> None:
        """Record a message processed."""
        self._message_count += 1

    def record_error_recovery(self) -> None:
        """Record an error recovery event."""
        self.agent.error_recovery_events += 1

    def record_retry(self) -> None:
        """Record a retry attempt."""
        self.agent.retry_count += 1

    def record_result(self, result: Any) -> None:
        """Record the final ResultMessage from the SDK.

        Extracts timing, cost, and token metrics from the result.
        """
        # Handle ResultMessage object or dict
        if hasattr(result, "__dict__"):
            # Object with attributes
            self.timing.wall_clock_duration_ms = getattr(result, "duration_ms", 0)
            self.timing.api_duration_ms = getattr(result, "duration_api_ms", 0)
            self.cost.total_cost_usd = getattr(result, "total_cost_usd", 0.0)
            self.agent.total_turns = getattr(result, "num_turns", 0)

            usage = getattr(result, "usage", {})
            if isinstance(usage, dict):
                self.tokens.input_tokens = usage.get("input_tokens", 0)
                self.tokens.output_tokens = usage.get("output_tokens", 0)
                self.tokens.cache_creation_input_tokens = usage.get(
                    "cache_creation_input_tokens", 0
                )
                self.tokens.cache_read_input_tokens = usage.get(
                    "cache_read_input_tokens", 0
                )

            # Set outcome based on subtype
            subtype = getattr(result, "subtype", "failure")
            self.outcome.status = "success" if subtype == "success" else "failure"

        elif isinstance(result, dict):
            # Dictionary format
            self.timing.wall_clock_duration_ms = result.get("duration_ms", 0)
            self.timing.api_duration_ms = result.get("duration_api_ms", 0)
            self.cost.total_cost_usd = result.get("total_cost_usd", 0.0)
            self.agent.total_turns = result.get("num_turns", 0)

            usage = result.get("usage", {})
            self.tokens.input_tokens = usage.get("input_tokens", 0)
            self.tokens.output_tokens = usage.get("output_tokens", 0)
            self.tokens.cache_creation_input_tokens = usage.get(
                "cache_creation_input_tokens", 0
            )
            self.tokens.cache_read_input_tokens = usage.get(
                "cache_read_input_tokens", 0
            )

            self.outcome.status = (
                "success" if result.get("subtype") == "success" else "failure"
            )

    def set_outcome(
        self,
        status: str,
        modules_completed: int = 0,
        modules_total: int = 0,
        blocking_issues: Optional[list[str]] = None,
        notes: Optional[str] = None,
    ) -> None:
        """Set the final outcome of the migration."""
        self.outcome.status = status
        self.outcome.modules_completed = modules_completed
        self.outcome.modules_total = modules_total
        self.outcome.blocking_issues = blocking_issues or []
        self.outcome.notes = notes

    def finalize(self) -> MigrationMetrics:
        """Finalize and return the complete metrics.

        Call this at the end of the migration to get the final metrics object.
        """
        self.identity.completed_at = datetime.now().isoformat()
        self.agent.total_messages = self._message_count

        # If wall clock wasn't set from result, calculate from start time
        if self.timing.wall_clock_duration_ms == 0:
            self.timing.wall_clock_duration_ms = int(
                (time.time() - self._start_time) * 1000
            )

        return MigrationMetrics(
            identity=self.identity,
            timing=self.timing,
            cost=self.cost,
            tokens=self.tokens,
            agent=self.agent,
            outcome=self.outcome,
        )


class LogParser:
    """Parse existing log files to extract metrics.

    This is used for backfilling metrics from logs created before
    the MetricsCollector was integrated.
    """

    # Regex patterns for log parsing
    RESULT_MESSAGE_PATTERN = re.compile(
        r"ResultMessage\("
        r".*?duration_ms=(\d+)"
        r".*?duration_api_ms=(\d+)"
        r".*?num_turns=(\d+)"
        r".*?total_cost_usd=([\d.]+)"
    )
    USAGE_PATTERN = re.compile(
        r"'input_tokens': (\d+).*?"
        r"'cache_creation_input_tokens': (\d+).*?"
        r"'cache_read_input_tokens': (\d+)"
    )
    OUTPUT_TOKENS_PATTERN = re.compile(r"'output_tokens': (\d+)")
    TOOL_USE_PATTERN = re.compile(r"ToolUseBlock\(.*?name='(\w+)'")
    TASK_PATTERN = re.compile(r"subagent_type='(\w+)'")
    MSG_PATTERN = re.compile(r"\[[\d:]+\] MSG #(\d+)")
    TIMESTAMP_PATTERN = re.compile(r"\[([\d:]+)\]")
    START_PATTERN = re.compile(r"Starting Migration: (\w+) -> (\w+)")
    STRATEGY_PATTERN = re.compile(r"Strategy: ([\w-]+)")

    def parse_log(self, log_path: Path) -> MigrationMetrics:
        """Parse a log file and extract metrics."""
        content = log_path.read_text()
        lines = content.split("\n")

        # Extract basic info
        project_name = "unknown"
        target_language = "unknown"
        strategy = "unknown"

        for line in lines[:20]:  # Check first 20 lines for header
            start_match = self.START_PATTERN.search(line)
            if start_match:
                project_name = start_match.group(1)
                target_language = start_match.group(2)

            strat_match = self.STRATEGY_PATTERN.search(line)
            if strat_match:
                strategy = strat_match.group(1)

        # Create collector for accumulating metrics
        collector = MetricsCollector(
            project_name=project_name,
            source_language="python",
            target_language=target_language,
            strategy=strategy,
        )

        # Parse the file
        for line in lines:
            # Count messages
            if self.MSG_PATTERN.search(line):
                collector.record_message()

            # Extract tool uses
            tool_match = self.TOOL_USE_PATTERN.search(line)
            if tool_match:
                collector.record_tool_use(tool_match.group(1))

            # Extract subagent invocations
            task_match = self.TASK_PATTERN.search(line)
            if task_match:
                collector.record_subagent(task_match.group(1))

            # Extract ResultMessage data
            if "ResultMessage" in line:
                result_match = self.RESULT_MESSAGE_PATTERN.search(line)
                if result_match:
                    collector.timing.wall_clock_duration_ms = int(result_match.group(1))
                    collector.timing.api_duration_ms = int(result_match.group(2))
                    collector.agent.total_turns = int(result_match.group(3))
                    collector.cost.total_cost_usd = float(result_match.group(4))

                usage_match = self.USAGE_PATTERN.search(line)
                if usage_match:
                    collector.tokens.input_tokens = int(usage_match.group(1))
                    collector.tokens.cache_creation_input_tokens = int(
                        usage_match.group(2)
                    )
                    collector.tokens.cache_read_input_tokens = int(usage_match.group(3))

                output_match = self.OUTPUT_TOKENS_PATTERN.search(line)
                if output_match:
                    collector.tokens.output_tokens = int(output_match.group(1))

                # Set success status
                if "subtype='success'" in line:
                    collector.outcome.status = "success"

        # Extract timestamps for start/end
        timestamps = self.TIMESTAMP_PATTERN.findall(content)
        if timestamps:
            # Get date from log filename
            date_match = re.search(r"migration_(\d{8})_(\d{6})", log_path.name)
            if date_match:
                date_str = date_match.group(1)
                time_str = timestamps[0]
                collector.identity.started_at = (
                    f"{date_str[:4]}-{date_str[4:6]}-{date_str[6:8]}T{time_str}"
                )
                if len(timestamps) > 1:
                    time_str = timestamps[-1]
                    collector.identity.completed_at = (
                        f"{date_str[:4]}-{date_str[4:6]}-{date_str[6:8]}T{time_str}"
                    )

        # Set run_id from session_id in log if available
        session_match = re.search(r"session_id='([^']+)'", content)
        if session_match:
            collector.identity.run_id = session_match.group(1)

        return collector.finalize()

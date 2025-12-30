from unittest.mock import MagicMock, patch

import pytest

from migration.config import ProjectConfig
from migration.languages.base import LanguageTarget
from migration.runner import run_migration


class MockToolUseBlock:
    def __init__(self, name, input):
        self.name = name
        self.input = input
        self.__name__ = "ToolUseBlock"


class MockMessage:
    def __init__(self, content):
        self.content = content
        self.type = "tool_use"


class MockResultMessage:
    def __init__(self, subtype="success"):
        self.subtype = subtype
        self.duration_ms = 100
        self.duration_api_ms = 50
        self.num_turns = 1
        self.total_cost_usd = 0.1
        self.usage = {
            "input_tokens": 10,
            "output_tokens": 5,
            "cache_creation_input_tokens": 0,
            "cache_read_input_tokens": 0,
        }
        self.__name__ = "ResultMessage"


@pytest.mark.asyncio
async def test_module_timing_and_outcome_tracking(tmp_path):
    # Setup mocks
    config = MagicMock(spec=ProjectConfig)
    config.name = "test_project"
    config.source_language = "python"
    config.source_directory = "src"
    config.source_files = ["lexer.py", "parser.py"]
    config.modules = [MagicMock(name="mod1"), MagicMock(name="mod2")]
    config.test_inputs = ["test1"]

    target = MagicMock(spec=LanguageTarget)
    target.name = "rust"
    target.get_quality_gates.return_value = []
    target.get_source_dir.return_value = "src"

    # Mock the SDK query
    mock_messages = [
        # Setup phase
        MockMessage(
            content=[
                MockToolUseBlock(
                    "Task",
                    {"subagent_type": "io_contract", "task": "Generate I/O contract"},
                )
            ]
        ),
        # Migration phase - Module 1
        MockMessage(
            content=[
                MockToolUseBlock(
                    "Task",
                    {"subagent_type": "migrator", "task": "Migrate module 'lexer.py'"},
                )
            ]
        ),
        # Review phase - Module 1
        MockMessage(
            content=[
                MockToolUseBlock(
                    "Task",
                    {"subagent_type": "reviewer", "task": "Review module 'lexer.py'"},
                )
            ]
        ),
        # Migration phase - Module 2
        MockMessage(
            content=[
                MockToolUseBlock(
                    "Task",
                    {"subagent_type": "migrator", "task": "Migrate module 'parser.py'"},
                )
            ]
        ),
        # End
        MockResultMessage(subtype="success"),
    ]

    async def mock_query(*args, **kwargs):
        for msg in mock_messages:
            yield msg

    # We patch migration.runner.query which is imported from claude_agent_sdk
    # also mock sys.modules to prevent run_migration from trying to import it
    sdk_mock = MagicMock()
    sdk_mock.query.side_effect = mock_query

    # ToolUseBlock and ResultMessage need to be types so that type(item).__name__ works
    class ToolUseBlock:
        pass

    class ResultMessage:
        def __init__(self, subtype="success"):
            self.subtype = subtype
            self.duration_ms = 100
            self.duration_api_ms = 50
            self.num_turns = 1
            self.total_cost_usd = 0.1
            self.usage = {
                "input_tokens": 10,
                "output_tokens": 5,
                "cache_creation_input_tokens": 0,
                "cache_read_input_tokens": 0,
            }
            self.session_id = "test_session"

    def mock_type(obj):
        return getattr(obj, "__type__", type(obj))

    with (
        patch.dict("sys.modules", {"claude_agent_sdk": sdk_mock}),
        patch("migration.runner.measure_loc", return_value=(100, 20, 5)),
        patch("migration.runner.measure_coverage", return_value=90.0),
        patch(
            "migration.runner.evaluate_idiomaticness",
            return_value=("Idiomatic", "Good"),
        ),
        patch("migration.runner.Path.mkdir"),
        patch("migration.runner.Path.iterdir", return_value=[]),
        patch("migration.runner.CheckpointManager", spec=True),
        patch("builtins.open", MagicMock()),
        patch("migration.runner.type", side_effect=mock_type, create=True),
    ):
        # Prepare messages with proper types
        for msg in mock_messages:
            if hasattr(msg, "content") and msg.content:
                for item in msg.content:
                    item.__type__ = ToolUseBlock
            if isinstance(msg, MockResultMessage):
                msg.__type__ = ResultMessage

        collector = await run_migration(
            config=config,
            target=target,
            base_dir=str(tmp_path),
            collect_metrics=True,
        )

        assert collector is not None
        metrics = collector.finalize()

        # Verify Bug #2: Module timings recorded
        assert len(metrics.timing.module_durations) == 2
        assert metrics.timing.module_durations[0].module_name == "lexer.py"
        assert metrics.timing.module_durations[1].module_name == "parser.py"

        # Verify Bug #5: Modules completed tracked
        assert metrics.outcome.modules_completed == 2
        assert metrics.outcome.modules_total == 2
        assert metrics.outcome.status == "success"


if __name__ == "__main__":
    pytest.main([__file__])

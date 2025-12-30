"""Unit tests for agent builder functions."""

import pytest

from migration.agents import (
    build_agents,
    build_analyst_agent,
    build_io_contract_agent,
    build_migrator_agent,
    build_reviewer_agent,
)
from migration.config import IOContract, ModuleConfig, ProjectConfig
from migration.languages import RustTarget


def create_test_config() -> ProjectConfig:
    """Create a minimal ProjectConfig for testing."""
    return ProjectConfig(
        name="test-project",
        description="Test project",
        source_language="python",
        source_directory="/src",
        source_files=["lexer.py", "parser.py"],
        modules=[
            ModuleConfig(
                name="lexer",
                source="lexer.py",
                phase="foundation",
                description="Tokenizer",
            ),
        ],
        test_inputs=["2 3 +", "4 5 *"],
        io_contract=IOContract(command_template="python -m rpn {input}"),
    )


class TestBuildIOContractAgent:
    """Tests for build_io_contract_agent."""

    def test_returns_dict_with_required_keys(self) -> None:
        """Test that agent has all required keys."""
        config = create_test_config()
        agent = build_io_contract_agent(config, "/project")

        assert "description" in agent
        assert "prompt" in agent
        assert "tools" in agent
        assert "model" in agent

    def test_prompt_contains_test_inputs(self) -> None:
        """Test that prompt includes test inputs."""
        config = create_test_config()
        agent = build_io_contract_agent(config, "/project")

        assert "2 3 +" in agent["prompt"]
        assert "4 5 *" in agent["prompt"]

    def test_prompt_contains_output_path(self) -> None:
        """Test that prompt specifies output location."""
        config = create_test_config()
        agent = build_io_contract_agent(config, "/project")

        assert "/project/artifacts/PHASE_0_IO_CONTRACT.md" in agent["prompt"]


class TestBuildAnalystAgent:
    """Tests for build_analyst_agent."""

    def test_returns_dict_with_required_keys(self) -> None:
        """Test that agent has all required keys."""
        config = create_test_config()
        target = RustTarget()
        agent = build_analyst_agent(config, target, "/project")

        assert "description" in agent
        assert "prompt" in agent
        assert "tools" in agent
        assert "model" in agent

    def test_prompt_contains_source_files(self) -> None:
        """Test that prompt includes source files."""
        config = create_test_config()
        target = RustTarget()
        agent = build_analyst_agent(config, target, "/project")

        assert "lexer.py" in agent["prompt"]
        assert "parser.py" in agent["prompt"]

    def test_prompt_contains_target_language(self) -> None:
        """Test that prompt mentions target language."""
        config = create_test_config()
        target = RustTarget()
        agent = build_analyst_agent(config, target, "/project")

        assert "Rust" in agent["prompt"]


class TestBuildMigratorAgent:
    """Tests for build_migrator_agent."""

    def test_returns_dict_with_required_keys(self) -> None:
        """Test that agent has all required keys."""
        config = create_test_config()
        target = RustTarget()
        agent = build_migrator_agent(config, target, "/project")

        assert "description" in agent
        assert "prompt" in agent
        assert "tools" in agent
        assert "model" in agent

    def test_prompt_contains_quality_gates(self) -> None:
        """Test that prompt includes quality gate commands."""
        config = create_test_config()
        target = RustTarget()
        agent = build_migrator_agent(config, target, "/project")

        assert "cargo" in agent["prompt"]


class TestBuildReviewerAgent:
    """Tests for build_reviewer_agent."""

    def test_returns_dict_with_required_keys(self) -> None:
        """Test that agent has all required keys."""
        config = create_test_config()
        target = RustTarget()
        agent = build_reviewer_agent(config, target, "/project")

        assert "description" in agent
        assert "prompt" in agent
        assert "tools" in agent
        assert "model" in agent

    def test_prompt_contains_output_location(self) -> None:
        """Test that prompt specifies review output location."""
        config = create_test_config()
        target = RustTarget()
        agent = build_reviewer_agent(config, target, "/project")

        assert "/project/artifacts/PHASE_3_REVIEW.md" in agent["prompt"]


class TestBuildAgents:
    """Tests for build_agents orchestration function."""

    def test_returns_all_four_agents(self) -> None:
        """Test that build_agents returns all agent types."""
        config = create_test_config()
        target = RustTarget()
        agents = build_agents(config, target, "/project")

        assert "io_contract" in agents
        assert "analyst" in agents
        assert "migrator" in agents
        assert "reviewer" in agents

    def test_each_agent_has_required_structure(self) -> None:
        """Test that each returned agent has required keys."""
        config = create_test_config()
        target = RustTarget()
        agents = build_agents(config, target, "/project")

        for name, agent in agents.items():
            assert "description" in agent, f"{name} missing description"
            assert "prompt" in agent, f"{name} missing prompt"
            assert "tools" in agent, f"{name} missing tools"
            assert "model" in agent, f"{name} missing model"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

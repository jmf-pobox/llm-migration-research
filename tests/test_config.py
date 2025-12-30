"""Unit tests for config loading."""

import tempfile
from pathlib import Path

import pytest

from migration.config import (
    IOContract,
    ProjectConfig,
    load_project_config,
)

VALID_CONFIG_YAML = """
name: test-project
description: A test project

source:
  language: python
  directory: /src
  files:
    - lexer.py
    - parser.py

modules:
  - name: lexer
    source: lexer.py
    phase: foundation
    description: Tokenizes input
  - name: parser
    source: parser.py
    phase: core
    depends_on:
      - lexer

test_inputs:
  - "2 3 +"
  - "4 5 *"

io_contract:
  command_template: python -m rpn {input}
  expected_outputs:
    "2 3 +": "5"
  error_inputs:
    - "invalid"

features:
  - name: addition
    description: Addition operator
    touches:
      - lexer.py
      - parser.py
    test_cases:
      - input: "2 3 +"
        output: "5"
    depends_on: []
"""


class TestLoadProjectConfig:
    """Tests for load_project_config function."""

    def test_load_valid_config(self) -> None:
        """Test loading a valid YAML config."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(VALID_CONFIG_YAML)
            f.flush()

            config = load_project_config(f.name)

            assert config.name == "test-project"
            assert config.description == "A test project"
            assert config.source_language == "python"
            assert config.source_directory == "/src"
            assert len(config.source_files) == 2

    def test_load_parses_modules(self) -> None:
        """Test that modules are parsed correctly."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(VALID_CONFIG_YAML)
            f.flush()

            config = load_project_config(f.name)

            assert len(config.modules) == 2
            assert config.modules[0].name == "lexer"
            assert config.modules[0].source == "lexer.py"
            assert config.modules[0].phase == "foundation"
            assert config.modules[1].depends_on == ["lexer"]

    def test_load_parses_io_contract(self) -> None:
        """Test that I/O contract is parsed correctly."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(VALID_CONFIG_YAML)
            f.flush()

            config = load_project_config(f.name)

            assert config.io_contract.command_template == "python -m rpn {input}"
            assert config.io_contract.expected_outputs == {"2 3 +": "5"}
            assert config.io_contract.error_inputs == ["invalid"]

    def test_load_parses_features(self) -> None:
        """Test that features are parsed correctly."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(VALID_CONFIG_YAML)
            f.flush()

            config = load_project_config(f.name)

            assert len(config.features) == 1
            assert config.features[0].name == "addition"
            assert len(config.features[0].test_cases) == 1
            assert config.features[0].test_cases[0].input == "2 3 +"

    def test_load_missing_file_raises(self) -> None:
        """Test that missing file raises FileNotFoundError."""
        with pytest.raises(FileNotFoundError, match="Config file not found"):
            load_project_config("/nonexistent/path.yaml")

    def test_load_accepts_path_object(self) -> None:
        """Test that Path objects are accepted."""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(VALID_CONFIG_YAML)
            f.flush()

            config = load_project_config(Path(f.name))
            assert config.name == "test-project"


class TestProjectConfig:
    """Tests for ProjectConfig dataclass."""

    def test_source_dir_property(self) -> None:
        """Test source_dir property returns source_directory."""
        config = ProjectConfig(
            name="test",
            description="",
            source_language="python",
            source_directory="/my/source/dir",
            source_files=[],
            modules=[],
            test_inputs=[],
            io_contract=IOContract(command_template=""),
        )

        assert config.source_dir == "/my/source/dir"


class TestMinimalConfig:
    """Test loading minimal configs with defaults."""

    def test_minimal_config_uses_defaults(self) -> None:
        """Test that missing optional fields use defaults."""
        minimal_yaml = """
name: minimal
source:
  files: []
modules: []
"""
        with tempfile.NamedTemporaryFile(mode="w", suffix=".yaml", delete=False) as f:
            f.write(minimal_yaml)
            f.flush()

            config = load_project_config(f.name)

            assert config.name == "minimal"
            assert config.description == ""
            assert config.source_language == "python"  # default
            assert config.features == []
            assert config.test_inputs == []


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

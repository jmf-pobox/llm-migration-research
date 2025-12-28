"""Project configuration loading from YAML."""

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import yaml


@dataclass
class ModuleConfig:
    """Configuration for a single module to migrate."""

    name: str
    source: str
    phase: str
    description: str = ""
    depends_on: list[str] = field(default_factory=list)


@dataclass
class IOContract:
    """I/O contract configuration for behavioral validation."""

    command_template: str
    expected_outputs: dict[str, str] = field(default_factory=dict)
    error_inputs: list[str] = field(default_factory=list)


@dataclass
class ProjectConfig:
    """Complete project configuration."""

    name: str
    description: str
    source_language: str
    source_directory: str
    source_files: list[str]
    modules: list[ModuleConfig]
    test_inputs: list[str]
    io_contract: IOContract

    @property
    def source_dir(self) -> str:
        """Alias for source_directory."""
        return self.source_directory


def load_project_config(path: str | Path) -> ProjectConfig:
    """Load project configuration from YAML file.

    Args:
        path: Path to the YAML configuration file

    Returns:
        ProjectConfig instance

    Raises:
        FileNotFoundError: If config file doesn't exist
        ValueError: If config is invalid
    """
    path = Path(path)
    if not path.exists():
        raise FileNotFoundError(f"Config file not found: {path}")

    with open(path) as f:
        data = yaml.safe_load(f)

    return _parse_config(data)


def _parse_config(data: dict[str, Any]) -> ProjectConfig:
    """Parse raw YAML data into ProjectConfig."""
    # Parse source section
    source = data.get("source", {})

    # Parse modules
    modules = [
        ModuleConfig(
            name=m["name"],
            source=m["source"],
            phase=m["phase"],
            description=m.get("description", ""),
            depends_on=m.get("depends_on", []),
        )
        for m in data.get("modules", [])
    ]

    # Parse I/O contract
    io_data = data.get("io_contract", {})
    io_contract = IOContract(
        command_template=io_data.get("command_template", ""),
        expected_outputs=io_data.get("expected_outputs", {}),
        error_inputs=io_data.get("error_inputs", []),
    )

    return ProjectConfig(
        name=data["name"],
        description=data.get("description", ""),
        source_language=source.get("language", "python"),
        source_directory=source.get("directory", ""),
        source_files=source.get("files", []),
        modules=modules,
        test_inputs=data.get("test_inputs", []),
        io_contract=io_contract,
    )

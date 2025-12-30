"""Base class for migration strategies."""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any

from ..config import ProjectConfig
from ..languages.base import LanguageTarget


@dataclass
class MigrationSlice:
    """A slice of work to migrate.

    For module-by-module: one module.
    For feature-by-feature: one feature (may span multiple files).
    """

    name: str
    description: str
    source_files: list[str]  # Files this slice touches
    test_inputs: list[dict[str, Any]]  # {input: str, output: str} for this slice


class MigrationStrategy(ABC):
    """Abstract base class for migration strategies.

    A strategy determines:
    1. How to divide the migration into slices
    2. What prompt to generate for each slice
    """

    name: str

    @abstractmethod
    def get_prompt(
        self,
        config: ProjectConfig,
        target: LanguageTarget,
        project_dir: str,
    ) -> str:
        """Generate the main migration prompt."""
        pass

    @abstractmethod
    def get_slices(self, config: ProjectConfig) -> list[MigrationSlice]:
        """Return ordered list of migration slices."""
        pass

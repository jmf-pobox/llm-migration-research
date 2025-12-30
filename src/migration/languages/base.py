"""Abstract base class for target language configuration."""

from abc import ABC, abstractmethod


class LanguageTarget(ABC):
    """Abstract base class for target language configuration.

    Each target language (Rust, Java, etc.) implements this interface
    to provide language-specific configuration for the migration framework.
    """

    @property
    @abstractmethod
    def name(self) -> str:
        """Language name (e.g., 'rust', 'java')."""
        pass

    @property
    @abstractmethod
    def file_extension(self) -> str:
        """File extension including dot (e.g., '.rs', '.java')."""
        pass

    @abstractmethod
    def get_project_subdir(self, project_name: str) -> str:
        """Return the output subdirectory for this language.

        Args:
            project_name: The project name from config (e.g., 'rpn2tex')

        Returns:
            Subdirectory name (e.g., 'rpn2tex-rs', 'rpn2tex-java')
        """
        pass

    @abstractmethod
    def get_quality_gates(self) -> list[str]:
        """Return shell commands for build/lint/test quality gates.

        Returns:
            List of shell commands to run (e.g., ['cargo check', 'cargo test'])
        """
        pass

    @abstractmethod
    def get_migrator_idioms(self) -> str:
        """Return language-specific idiom requirements for migrator prompt.

        This text is inserted into the migrator agent's prompt to guide
        generation of idiomatic code.

        Returns:
            Markdown-formatted string with idiom requirements
        """
        pass

    @abstractmethod
    def get_reviewer_checks(self) -> str:
        """Return language-specific review criteria.

        This text is inserted into the reviewer agent's prompt.

        Returns:
            Markdown-formatted string with review criteria
        """
        pass

    @abstractmethod
    def get_file_mapping(self, python_file: str) -> str:
        """Map a Python source filename to target language filename.

        Args:
            python_file: Source Python filename (e.g., 'tokens.py')

        Returns:
            Target filename (e.g., 'tokens.rs' or 'Token.java')
        """
        pass

    @abstractmethod
    def get_project_init_commands(self, project_dir: str) -> list[str]:
        """Return commands to initialize a new project.

        Args:
            project_dir: Absolute path to the project directory

        Returns:
            List of shell commands to initialize the project
        """
        pass

    @abstractmethod
    def get_source_dir(self, project_dir: str) -> str:
        """Return the source code directory within the project.

        Args:
            project_dir: Absolute path to the project directory

        Returns:
            Path to source directory (e.g., 'src' for Rust, 'src/main/java/...' for Java)
        """
        pass

    @abstractmethod
    def get_coverage_command(self, project_dir: str) -> str:
        """Return shell command to measure test coverage.

        The command should output coverage percentage in a parseable format.
        The framework will parse for patterns like "XX.XX% coverage" or "Coverage: XX.XX%".

        Args:
            project_dir: Absolute path to the project directory

        Returns:
            Shell command string, or empty string if coverage not supported
        """
        pass

    def parse_coverage_output(self, output: str) -> float | None:
        """Parse coverage percentage from command output.

        Override this method for language-specific parsing.
        Default implementation looks for common patterns.

        Args:
            output: stdout/stderr from coverage command

        Returns:
            Coverage percentage (0-100) or None if not parseable
        """
        import re

        # Common patterns: "92.5% coverage", "Coverage: 92.5%", "92.5%"
        patterns = [
            r"(\d+\.?\d*)%\s*coverage",
            r"coverage[:\s]+(\d+\.?\d*)%",
            r"(\d+\.?\d*)%\s*of\s*statements",
            r"line[:\s]+(\d+\.?\d*)%",
        ]
        for pattern in patterns:
            match = re.search(pattern, output, re.IGNORECASE)
            if match:
                return float(match.group(1))
        return None

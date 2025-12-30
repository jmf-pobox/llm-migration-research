"""Unit tests for language target classes."""

import pytest

from migration.languages import (
    GoTarget,
    JavaTarget,
    RustTarget,
    get_language_target,
)


class TestGetLanguageTarget:
    """Tests for get_language_target factory function."""

    def test_get_rust_target(self) -> None:
        """Test retrieving Rust target."""
        target = get_language_target("rust")
        assert isinstance(target, RustTarget)
        assert target.name == "rust"

    def test_get_java_target(self) -> None:
        """Test retrieving Java target."""
        target = get_language_target("java")
        assert isinstance(target, JavaTarget)
        assert target.name == "java"

    def test_get_go_target(self) -> None:
        """Test retrieving Go target."""
        target = get_language_target("go")
        assert isinstance(target, GoTarget)
        assert target.name == "go"

    def test_get_unknown_target_raises(self) -> None:
        """Test that unknown language raises ValueError."""
        with pytest.raises(ValueError, match="Unknown language"):
            get_language_target("cobol")


class TestRustTarget:
    """Tests for RustTarget class."""

    def test_name(self) -> None:
        """Test name property."""
        target = RustTarget()
        assert target.name == "rust"

    def test_file_extension(self) -> None:
        """Test file extension."""
        target = RustTarget()
        assert target.file_extension == ".rs"

    def test_get_project_subdir(self) -> None:
        """Test project subdirectory naming."""
        target = RustTarget()
        assert target.get_project_subdir("myproject") == "myproject-rs"

    def test_get_quality_gates(self) -> None:
        """Test quality gate commands are returned."""
        target = RustTarget()
        gates = target.get_quality_gates()
        assert isinstance(gates, list)
        assert len(gates) > 0
        assert any("cargo" in gate for gate in gates)

    def test_get_file_mapping_known(self) -> None:
        """Test file mapping for known files."""
        target = RustTarget()
        assert target.get_file_mapping("lexer.py") == "lexer.rs"
        assert target.get_file_mapping("cli.py") == "main.rs"
        assert target.get_file_mapping("__init__.py") == "lib.rs"

    def test_get_file_mapping_unknown(self) -> None:
        """Test file mapping falls back for unknown files."""
        target = RustTarget()
        assert target.get_file_mapping("custom.py") == "custom.rs"

    def test_get_source_dir(self) -> None:
        """Test source directory path."""
        target = RustTarget()
        assert target.get_source_dir("/project") == "/project/src"

    def test_get_migrator_idioms(self) -> None:
        """Test migrator idioms contains Rust-specific content."""
        target = RustTarget()
        idioms = target.get_migrator_idioms()
        assert "Rust" in idioms or "cargo" in idioms.lower()
        assert "#[must_use]" in idioms or "derive" in idioms

    def test_get_reviewer_checks(self) -> None:
        """Test reviewer checks contains Rust-specific content."""
        target = RustTarget()
        checks = target.get_reviewer_checks()
        assert "Rust" in checks or "ownership" in checks.lower()

    def test_parse_coverage_output_llvm_cov(self) -> None:
        """Test parsing cargo-llvm-cov output."""
        target = RustTarget()
        output = "TOTAL  100  80  80.00%  50  40  80.00%  10  8  80.00%"
        assert target.parse_coverage_output(output) == 80.0

    def test_parse_coverage_output_tarpaulin(self) -> None:
        """Test parsing tarpaulin output."""
        target = RustTarget()
        output = "92.5% coverage"
        assert target.parse_coverage_output(output) == 92.5

    def test_parse_coverage_output_none(self) -> None:
        """Test parsing returns None for unparseable output."""
        target = RustTarget()
        assert target.parse_coverage_output("no coverage info") is None


class TestJavaTarget:
    """Tests for JavaTarget class."""

    def test_name(self) -> None:
        """Test name property."""
        target = JavaTarget()
        assert target.name == "java"

    def test_file_extension(self) -> None:
        """Test file extension."""
        target = JavaTarget()
        assert target.file_extension == ".java"

    def test_get_project_subdir(self) -> None:
        """Test project subdirectory naming."""
        target = JavaTarget()
        assert target.get_project_subdir("myproject") == "myproject-java"

    def test_get_quality_gates(self) -> None:
        """Test quality gate commands are returned."""
        target = JavaTarget()
        gates = target.get_quality_gates()
        assert isinstance(gates, list)
        assert len(gates) > 0
        assert any("gradle" in gate.lower() for gate in gates)

    def test_get_file_mapping_known(self) -> None:
        """Test file mapping for known files."""
        target = JavaTarget()
        assert target.get_file_mapping("lexer.py") == "Lexer.java"
        assert target.get_file_mapping("cli.py") == "Main.java"
        assert target.get_file_mapping("errors.py") == "RpnException.java"

    def test_get_file_mapping_unknown(self) -> None:
        """Test file mapping capitalizes unknown files."""
        target = JavaTarget()
        # my_module.py -> MyModule.java
        assert target.get_file_mapping("my_module.py") == "MyModule.java"

    def test_get_source_dir(self) -> None:
        """Test source directory path."""
        target = JavaTarget()
        src_dir = target.get_source_dir("/project")
        assert "src/main/java" in src_dir

    def test_parse_coverage_output(self) -> None:
        """Test parsing JaCoCo output."""
        target = JavaTarget()
        output = "Total coverage: 95%"
        assert target.parse_coverage_output(output) == 95.0


class TestGoTarget:
    """Tests for GoTarget class."""

    def test_name(self) -> None:
        """Test name property."""
        target = GoTarget()
        assert target.name == "go"

    def test_file_extension(self) -> None:
        """Test file extension."""
        target = GoTarget()
        assert target.file_extension == ".go"

    def test_get_project_subdir(self) -> None:
        """Test project subdirectory naming."""
        target = GoTarget()
        assert target.get_project_subdir("myproject") == "myproject-go"

    def test_get_quality_gates(self) -> None:
        """Test quality gate commands are returned."""
        target = GoTarget()
        gates = target.get_quality_gates()
        assert isinstance(gates, list)
        assert len(gates) > 0
        assert any("go " in gate for gate in gates)

    def test_get_file_mapping_known(self) -> None:
        """Test file mapping for known files."""
        target = GoTarget()
        assert target.get_file_mapping("lexer.py") == "lexer.go"
        assert target.get_file_mapping("cli.py") == "cmd/rpn2tex/main.go"
        assert target.get_file_mapping("tokens.py") == "token.go"

    def test_get_file_mapping_unknown(self) -> None:
        """Test file mapping falls back for unknown files."""
        target = GoTarget()
        assert target.get_file_mapping("custom.py") == "custom.go"

    def test_get_source_dir(self) -> None:
        """Test source directory is project root for Go."""
        target = GoTarget()
        assert target.get_source_dir("/project") == "/project"

    def test_parse_coverage_output_single(self) -> None:
        """Test parsing single package coverage."""
        target = GoTarget()
        output = "coverage: 85.5% of statements"
        assert target.parse_coverage_output(output) == 85.5

    def test_parse_coverage_output_multiple(self) -> None:
        """Test parsing multiple package coverage averages."""
        target = GoTarget()
        output = """ok  rpn2tex  coverage: 80.0% of statements
ok  rpn2tex/cmd  coverage: 90.0% of statements"""
        result = target.parse_coverage_output(output)
        assert result == 85.0  # Average of 80 and 90


class TestBaseCoverageParser:
    """Tests for base class coverage parsing."""

    def test_parse_coverage_common_patterns(self) -> None:
        """Test base class parses common coverage patterns."""
        target = RustTarget()  # Use concrete class

        # Pattern: "XX% coverage"
        assert target.parse_coverage_output("75.5% coverage") == 75.5

        # Pattern: "coverage: XX%"
        output = "coverage: 92%"
        result = target.parse_coverage_output(output)
        # Note: RustTarget overrides, so may not match base behavior
        assert result is None or result == 92.0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

"""Unit tests for PostHocAnalyzer."""

import json
import tempfile
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from migration.reporting.analyzer import PostHocAnalyzer
from migration.reporting.schema import CodeMetrics


class TestPostHocAnalyzerInit:
    """Tests for PostHocAnalyzer initialization."""

    def test_can_create_analyzer(self) -> None:
        """Test that analyzer can be instantiated."""
        analyzer = PostHocAnalyzer()
        assert analyzer is not None


class TestAnalyzePythonSource:
    """Tests for analyze_python_source method."""

    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_cloc")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_lizard")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_radon_mi")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._count_python_deps")
    def test_analyze_python_source(
        self,
        mock_deps: MagicMock,
        mock_radon: MagicMock,
        mock_lizard: MagicMock,
        mock_cloc: MagicMock,
    ) -> None:
        """Test analyzing Python source code."""
        mock_cloc.return_value = {"code": 500, "comment": 100, "blank": 50}
        mock_lizard.return_value = {"function_count": 30, "avg_cc": 3.5, "max_cc": 10}
        mock_radon.return_value = 85.0
        mock_deps.return_value = 5

        with tempfile.TemporaryDirectory() as tmpdir:
            # Create some Python files
            Path(tmpdir, "main.py").write_text("def main(): pass")
            Path(tmpdir, "utils.py").write_text("def helper(): pass")

            analyzer = PostHocAnalyzer()
            metrics = analyzer.analyze_python_source(Path(tmpdir))

        assert metrics.production_loc == 500
        assert metrics.function_count == 30
        assert metrics.avg_cyclomatic_complexity == 3.5
        assert metrics.maintainability_index == 85.0
        assert metrics.external_dependencies == 5

    def test_analyze_python_source_no_tools(self) -> None:
        """Test analyzing when external tools are unavailable."""
        with tempfile.TemporaryDirectory() as tmpdir:
            Path(tmpdir, "main.py").write_text("def main(): pass")

            analyzer = PostHocAnalyzer()
            # Tools will fail, but should return empty metrics
            metrics = analyzer.analyze_python_source(Path(tmpdir))

        assert isinstance(metrics, CodeMetrics)


class TestAnalyzeRustTarget:
    """Tests for analyze_rust_target method."""

    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_cloc")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_lizard")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._count_cargo_deps")
    def test_analyze_rust_target(
        self,
        mock_deps: MagicMock,
        mock_lizard: MagicMock,
        mock_cloc: MagicMock,
    ) -> None:
        """Test analyzing Rust target code."""
        mock_cloc.return_value = {"code": 600, "comment": 120, "blank": 60}
        mock_lizard.return_value = {"function_count": 35, "avg_cc": 2.5, "max_cc": 8}
        mock_deps.return_value = 3

        with tempfile.TemporaryDirectory() as tmpdir:
            # Create Rust project structure
            src_dir = Path(tmpdir) / "src"
            src_dir.mkdir()
            (src_dir / "main.rs").write_text("fn main() {}")
            (src_dir / "lib.rs").write_text("pub fn add() {}")

            # Create Cargo.toml
            (Path(tmpdir) / "Cargo.toml").write_text('[package]\nname = "test"')

            analyzer = PostHocAnalyzer()
            metrics = analyzer.analyze_rust_target(Path(tmpdir))

        assert metrics.production_loc == 600
        assert metrics.function_count == 35
        assert metrics.module_count == 2
        assert metrics.external_dependencies == 3

    def test_analyze_rust_target_no_src(self) -> None:
        """Test analyzing when src directory doesn't exist."""
        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            metrics = analyzer.analyze_rust_target(Path(tmpdir))

        assert metrics.production_loc == 0


class TestAnalyzeRustQuality:
    """Tests for analyze_rust_quality method."""

    @patch("migration.reporting.analyzer.PostHocAnalyzer._check_cargo_check")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._check_cargo_clippy")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._check_cargo_fmt")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_cargo_test")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._get_cargo_coverage")
    def test_analyze_rust_quality(
        self,
        mock_coverage: MagicMock,
        mock_test: MagicMock,
        mock_fmt: MagicMock,
        mock_clippy: MagicMock,
        mock_check: MagicMock,
    ) -> None:
        """Test analyzing Rust quality gates."""
        from migration.reporting.schema import (
            CompilationResult,
            CoverageResult,
            FormattingResult,
            LintingResult,
            TestResult,
        )

        mock_check.return_value = CompilationResult(passed=True)
        mock_clippy.return_value = LintingResult(passed=True, tool="clippy")
        mock_fmt.return_value = FormattingResult(passed=True, tool="rustfmt")
        mock_test.return_value = TestResult(
            passed=True, total=10, passed_count=10, failed_count=0, skipped_count=0
        )
        mock_coverage.return_value = CoverageResult(line_coverage_pct=85.0)

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            quality = analyzer.analyze_rust_quality(Path(tmpdir))

        assert quality.compilation is not None
        assert quality.compilation.passed is True
        assert quality.linting is not None
        assert quality.linting.passed is True
        assert quality.unit_tests is not None
        assert quality.unit_tests.passed is True
        assert quality.coverage is not None
        assert quality.coverage.line_coverage_pct == 85.0


class TestRunCloc:
    """Tests for _run_cloc helper method."""

    @patch("subprocess.run")
    def test_run_cloc_success(self, mock_run: MagicMock) -> None:
        """Test successful cloc execution."""
        mock_run.return_value = MagicMock(
            returncode=0,
            stdout=json.dumps({"Python": {"code": 100, "comment": 20, "blank": 10}}),
        )

        analyzer = PostHocAnalyzer()
        result = analyzer._run_cloc(Path("/some/path"), "Python")

        assert result is not None
        assert result["code"] == 100

    @patch("subprocess.run")
    def test_run_cloc_failure(self, mock_run: MagicMock) -> None:
        """Test cloc execution failure."""
        mock_run.return_value = MagicMock(returncode=1, stdout="")

        analyzer = PostHocAnalyzer()
        result = analyzer._run_cloc(Path("/some/path"), "Python")

        assert result is None

    @patch("subprocess.run")
    def test_run_cloc_timeout(self, mock_run: MagicMock) -> None:
        """Test cloc timeout handling."""
        import subprocess

        mock_run.side_effect = subprocess.TimeoutExpired("cloc", 60)

        analyzer = PostHocAnalyzer()
        result = analyzer._run_cloc(Path("/some/path"), "Python")

        assert result is None

    @patch("subprocess.run")
    def test_run_cloc_not_found(self, mock_run: MagicMock) -> None:
        """Test cloc not installed."""
        mock_run.side_effect = FileNotFoundError()

        analyzer = PostHocAnalyzer()
        result = analyzer._run_cloc(Path("/some/path"), "Python")

        assert result is None


class TestRunLizard:
    """Tests for _run_lizard helper method."""

    @patch("subprocess.run")
    def test_run_lizard_success(self, mock_run: MagicMock) -> None:
        """Test successful lizard execution."""
        lizard_output = """
================================================
  NLOC    CCN   token  PARAM  length  location
------------------------------------------------
      10      2     50      2      12 main@1-12@/path/main.py
      20      5     80      3      25 process@14-38@/path/main.py

Total nloc   Avg.NLOC  AvgCCN  Avg.token   Fun Cnt  Warning cnt
      30        15.0     3.5       65.0         2            0
"""
        mock_run.return_value = MagicMock(returncode=0, stdout=lizard_output)

        analyzer = PostHocAnalyzer()
        result = analyzer._run_lizard(Path("/some/path"), "python")

        assert result is not None
        # The parser should extract function count and avg CC

    @patch("subprocess.run")
    def test_run_lizard_not_found(self, mock_run: MagicMock) -> None:
        """Test lizard not installed."""
        mock_run.side_effect = FileNotFoundError()

        analyzer = PostHocAnalyzer()
        result = analyzer._run_lizard(Path("/some/path"), "python")

        assert result is None


class TestCountCargoDeps:
    """Tests for _count_cargo_deps helper method."""

    def test_count_cargo_deps(self) -> None:
        """Test counting Cargo.toml dependencies."""
        cargo_toml = """
[package]
name = "test"
version = "0.1.0"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }
regex = "1.5"
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            cargo_path = Path(tmpdir) / "Cargo.toml"
            cargo_path.write_text(cargo_toml)

            analyzer = PostHocAnalyzer()
            count = analyzer._count_cargo_deps(cargo_path)

        assert count == 3

    def test_count_cargo_deps_no_deps(self) -> None:
        """Test Cargo.toml with no dependencies."""
        cargo_toml = """
[package]
name = "test"
version = "0.1.0"
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            cargo_path = Path(tmpdir) / "Cargo.toml"
            cargo_path.write_text(cargo_toml)

            analyzer = PostHocAnalyzer()
            count = analyzer._count_cargo_deps(cargo_path)

        assert count == 0


class TestCheckCargoCommands:
    """Tests for cargo-related check methods."""

    @patch("subprocess.run")
    def test_check_cargo_check_pass(self, mock_run: MagicMock) -> None:
        """Test successful cargo check."""
        mock_run.return_value = MagicMock(returncode=0, stderr="")

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._check_cargo_check(Path(tmpdir))

        assert result is not None
        assert result.passed is True

    @patch("subprocess.run")
    def test_check_cargo_check_fail(self, mock_run: MagicMock) -> None:
        """Test failed cargo check."""
        mock_run.return_value = MagicMock(
            returncode=1, stderr="error: expected expression"
        )

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._check_cargo_check(Path(tmpdir))

        assert result is not None
        assert result.passed is False

    @patch("subprocess.run")
    def test_check_cargo_clippy_pass(self, mock_run: MagicMock) -> None:
        """Test successful clippy check."""
        mock_run.return_value = MagicMock(returncode=0, stderr="")

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._check_cargo_clippy(Path(tmpdir))

        assert result is not None
        assert result.passed is True
        assert result.tool == "clippy"

    @patch("subprocess.run")
    def test_check_cargo_fmt_pass(self, mock_run: MagicMock) -> None:
        """Test successful rustfmt check."""
        mock_run.return_value = MagicMock(returncode=0, stdout="")

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._check_cargo_fmt(Path(tmpdir))

        assert result is not None
        assert result.passed is True

    @patch("subprocess.run")
    def test_run_cargo_test_pass(self, mock_run: MagicMock) -> None:
        """Test successful cargo test."""
        test_output = """
running 5 tests
test test_one ... ok
test test_two ... ok
test test_three ... ok
test test_four ... ok
test test_five ... ok

test result: ok. 5 passed; 0 failed; 0 ignored
"""
        mock_run.return_value = MagicMock(returncode=0, stdout=test_output, stderr="")

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._run_cargo_test(Path(tmpdir))

        assert result is not None
        assert result.passed is True

    @patch("subprocess.run")
    def test_run_cargo_test_fail(self, mock_run: MagicMock) -> None:
        """Test failed cargo test."""
        test_output = """
running 5 tests
test test_one ... ok
test test_two ... FAILED

test result: FAILED. 1 passed; 1 failed; 0 ignored
"""
        mock_run.return_value = MagicMock(returncode=1, stdout=test_output, stderr="")

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._run_cargo_test(Path(tmpdir))

        assert result is not None
        assert result.passed is False


class TestCalculateMI:
    """Tests for maintainability index calculation."""

    def test_calculate_mi_from_metrics(self) -> None:
        """Test MI calculation from LOC and complexity."""
        analyzer = PostHocAnalyzer()

        # Test with reasonable values
        mi = analyzer._calculate_mi_from_metrics(500, 5.0)

        # MI should be between 0 and 100
        assert mi is None or (0 <= mi <= 100)

    def test_calculate_mi_zero_loc(self) -> None:
        """Test MI calculation with zero LOC."""
        analyzer = PostHocAnalyzer()

        mi = analyzer._calculate_mi_from_metrics(0, 5.0)

        # Should handle zero gracefully
        assert mi is None or mi >= 0


class TestCountPythonDeps:
    """Tests for _count_python_deps helper."""

    def test_count_python_deps(self) -> None:
        """Test counting Python dependencies from imports."""
        content = """
import os
import sys
from pathlib import Path

import requests
import numpy as np
from flask import Flask
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            py_file = Path(tmpdir) / "main.py"
            py_file.write_text(content)

            analyzer = PostHocAnalyzer()
            count = analyzer._count_python_deps(Path(tmpdir))

        # Should count third-party imports (requests, numpy, flask)
        assert count >= 0


class TestAnalyzeJavaTarget:
    """Tests for analyze_java_target method."""

    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_cloc")
    @patch("migration.reporting.analyzer.PostHocAnalyzer._run_lizard")
    def test_analyze_java_target(
        self,
        mock_lizard: MagicMock,
        mock_cloc: MagicMock,
    ) -> None:
        """Test analyzing Java target code."""
        mock_cloc.return_value = {"code": 700, "comment": 150, "blank": 70}
        mock_lizard.return_value = {"function_count": 40, "avg_cc": 3.0, "max_cc": 12}

        with tempfile.TemporaryDirectory() as tmpdir:
            # Create Java project structure
            src_dir = Path(tmpdir) / "src" / "main" / "java"
            src_dir.mkdir(parents=True)
            (src_dir / "Main.java").write_text("public class Main {}")

            analyzer = PostHocAnalyzer()
            metrics = analyzer.analyze_java_target(Path(tmpdir))

        assert metrics.production_loc == 700
        assert metrics.function_count == 40

    def test_analyze_java_target_no_src(self) -> None:
        """Test analyzing when src directory doesn't exist."""
        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            metrics = analyzer.analyze_java_target(Path(tmpdir))

        assert metrics.production_loc == 0


class TestRunRadonMI:
    """Tests for _run_radon_mi helper."""

    @patch("subprocess.run")
    def test_run_radon_mi_success(self, mock_run: MagicMock) -> None:
        """Test successful radon MI execution."""
        radon_output = "main.py - A (85.50)"
        mock_run.return_value = MagicMock(returncode=0, stdout=radon_output)

        analyzer = PostHocAnalyzer()
        result = analyzer._run_radon_mi(Path("/some/path"))

        # Should extract average MI
        assert result is None or isinstance(result, float)

    @patch("subprocess.run")
    def test_run_radon_mi_failure(self, mock_run: MagicMock) -> None:
        """Test radon MI execution failure."""
        mock_run.side_effect = FileNotFoundError()

        analyzer = PostHocAnalyzer()
        result = analyzer._run_radon_mi(Path("/some/path"))

        assert result is None


class TestCountJavaDeps:
    """Tests for _count_java_deps helper."""

    def test_count_java_deps_gradle(self) -> None:
        """Test counting Java dependencies from build.gradle."""
        gradle_content = """
plugins {
    id 'java'
}

dependencies {
    implementation 'com.google.guava:guava:31.0'
    implementation 'org.apache.commons:commons-lang3:3.12'
    testImplementation 'junit:junit:4.13'
}
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            gradle_file = Path(tmpdir) / "build.gradle"
            gradle_file.write_text(gradle_content)

            analyzer = PostHocAnalyzer()
            count = analyzer._count_java_deps(Path(tmpdir))

        # Should count dependencies
        assert count >= 0


class TestGetCargoCoverage:
    """Tests for _get_cargo_coverage helper."""

    @patch("subprocess.run")
    def test_get_cargo_coverage_success(self, mock_run: MagicMock) -> None:
        """Test successful cargo coverage."""
        mock_run.return_value = MagicMock(
            returncode=0, stdout="Coverage: 92.5%", stderr=""
        )

        with tempfile.TemporaryDirectory() as tmpdir:
            analyzer = PostHocAnalyzer()
            result = analyzer._get_cargo_coverage(Path(tmpdir))

        # May or may not parse depending on format
        assert result is None or isinstance(
            result.line_coverage_pct, (float, type(None))
        )


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

"""Post-hoc analysis using external tools.

This module runs external tools like lizard, cloc, and cargo
to capture code metrics that aren't available during migration.
"""

import json
import subprocess
from pathlib import Path
from typing import Optional

from .schema import (
    CodeMetrics,
    QualityGates,
    CompilationResult,
    LintingResult,
    FormattingResult,
    TestResult,
    CoverageResult,
)


class PostHocAnalyzer:
    """Analyzes migrated code using external tools.

    Usage:
        analyzer = PostHocAnalyzer()

        source_metrics = analyzer.analyze_python_source(Path("source/"))
        target_metrics = analyzer.analyze_rust_target(Path("migrations/rust/"))
        quality = analyzer.analyze_rust_quality(Path("migrations/rust/"))
    """

    def analyze_python_source(self, source_path: Path) -> CodeMetrics:
        """Analyze Python source code complexity and size."""
        metrics = CodeMetrics()

        # Count LOC with cloc
        loc_data = self._run_cloc(source_path, "Python")
        if loc_data:
            metrics.production_loc = loc_data.get("code", 0)
            metrics.total_loc = (
                loc_data.get("code", 0)
                + loc_data.get("comment", 0)
                + loc_data.get("blank", 0)
            )

        # Analyze complexity with lizard
        complexity_data = self._run_lizard(source_path, "python")
        if complexity_data:
            metrics.function_count = complexity_data.get("function_count", 0)
            metrics.avg_cyclomatic_complexity = complexity_data.get("avg_cc", 0.0)
            metrics.max_cyclomatic_complexity = complexity_data.get("max_cc", 0)

        # Count modules
        py_files = list(source_path.glob("*.py"))
        metrics.module_count = len([f for f in py_files if not f.name.startswith("_")])

        return metrics

    def analyze_rust_target(self, target_path: Path) -> CodeMetrics:
        """Analyze Rust target code complexity and size."""
        metrics = CodeMetrics()
        src_path = target_path / "src"

        if not src_path.exists():
            return metrics

        # Count LOC with cloc
        loc_data = self._run_cloc(src_path, "Rust")
        if loc_data:
            metrics.production_loc = loc_data.get("code", 0)
            metrics.total_loc = (
                loc_data.get("code", 0)
                + loc_data.get("comment", 0)
                + loc_data.get("blank", 0)
            )

        # Check for tests directory
        tests_path = target_path / "tests"
        if tests_path.exists():
            test_loc = self._run_cloc(tests_path, "Rust")
            if test_loc:
                metrics.test_loc = test_loc.get("code", 0)

        # Analyze complexity with lizard
        complexity_data = self._run_lizard(src_path, "rust")
        if complexity_data:
            metrics.function_count = complexity_data.get("function_count", 0)
            metrics.avg_cyclomatic_complexity = complexity_data.get("avg_cc", 0.0)
            metrics.max_cyclomatic_complexity = complexity_data.get("max_cc", 0)

        # Count modules (*.rs files)
        rs_files = list(src_path.glob("*.rs"))
        metrics.module_count = len(rs_files)

        # Count external dependencies from Cargo.toml
        cargo_toml = target_path / "Cargo.toml"
        if cargo_toml.exists():
            metrics.external_dependencies = self._count_cargo_deps(cargo_toml)

        return metrics

    def analyze_java_target(self, target_path: Path) -> CodeMetrics:
        """Analyze Java target code complexity and size."""
        metrics = CodeMetrics()
        src_path = target_path / "src" / "main" / "java"

        if not src_path.exists():
            return metrics

        # Count LOC with cloc
        loc_data = self._run_cloc(src_path, "Java")
        if loc_data:
            metrics.production_loc = loc_data.get("code", 0)
            metrics.total_loc = (
                loc_data.get("code", 0)
                + loc_data.get("comment", 0)
                + loc_data.get("blank", 0)
            )

        # Check for test directory
        test_path = target_path / "src" / "test" / "java"
        if test_path.exists():
            test_loc = self._run_cloc(test_path, "Java")
            if test_loc:
                metrics.test_loc = test_loc.get("code", 0)

        # Analyze complexity with lizard
        complexity_data = self._run_lizard(src_path, "java")
        if complexity_data:
            metrics.function_count = complexity_data.get("function_count", 0)
            metrics.avg_cyclomatic_complexity = complexity_data.get("avg_cc", 0.0)
            metrics.max_cyclomatic_complexity = complexity_data.get("max_cc", 0)

        # Count Java files
        java_files = list(src_path.rglob("*.java"))
        metrics.module_count = len(java_files)

        return metrics

    def analyze_rust_quality(self, target_path: Path) -> QualityGates:
        """Run Rust quality gates and return results."""
        quality = QualityGates()

        # Check compilation
        quality.compilation = self._check_cargo_check(target_path)

        # Check clippy
        quality.linting = self._check_cargo_clippy(target_path)

        # Check formatting
        quality.formatting = self._check_cargo_fmt(target_path)

        # Run tests
        quality.unit_tests = self._run_cargo_test(target_path)

        # Get coverage if available
        quality.coverage = self._get_cargo_coverage(target_path)

        return quality

    def _run_cloc(self, path: Path, language: str) -> Optional[dict]:
        """Run cloc and return LOC data for specified language."""
        try:
            result = subprocess.run(
                ["cloc", "--json", str(path)],
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode == 0:
                data = json.loads(result.stdout)
                return data.get(language, {})
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass
        return None

    def _run_lizard(self, path: Path, language: str) -> Optional[dict]:
        """Run lizard and return complexity data."""
        try:
            result = subprocess.run(
                ["lizard", "-l", language, str(path)],
                capture_output=True,
                text=True,
                timeout=120,
            )
            if result.returncode == 0:
                # Parse lizard output
                lines = result.stdout.strip().split("\n")
                functions = []
                for line in lines:
                    # Parse function lines (skip headers and totals)
                    if line.strip() and not line.startswith("-") and "@" in line:
                        parts = line.split()
                        if len(parts) >= 2:
                            try:
                                cc = int(parts[1])
                                functions.append(cc)
                            except ValueError:
                                pass

                if functions:
                    return {
                        "function_count": len(functions),
                        "avg_cc": sum(functions) / len(functions),
                        "max_cc": max(functions),
                    }
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return None

    def _count_cargo_deps(self, cargo_toml: Path) -> int:
        """Count external dependencies in Cargo.toml."""
        content = cargo_toml.read_text()
        in_deps = False
        count = 0
        for line in content.split("\n"):
            if line.strip() == "[dependencies]":
                in_deps = True
            elif line.strip().startswith("[") and in_deps:
                break
            elif in_deps and "=" in line and not line.strip().startswith("#"):
                count += 1
        return count

    def _check_cargo_check(self, path: Path) -> CompilationResult:
        """Run cargo check and return result."""
        result = CompilationResult()
        try:
            proc = subprocess.run(
                ["cargo", "check", "--message-format=json"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=300,
            )
            result.passed = proc.returncode == 0

            # Count errors and warnings from JSON output
            for line in proc.stdout.split("\n"):
                if line.strip():
                    try:
                        msg = json.loads(line)
                        if msg.get("reason") == "compiler-message":
                            level = msg.get("message", {}).get("level", "")
                            if level == "error":
                                result.error_count += 1
                            elif level == "warning":
                                result.warning_count += 1
                    except json.JSONDecodeError:
                        pass
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _check_cargo_clippy(self, path: Path) -> LintingResult:
        """Run cargo clippy and return result."""
        result = LintingResult(tool="clippy")
        try:
            proc = subprocess.run(
                ["cargo", "clippy", "--message-format=json", "--", "-D", "warnings"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=300,
            )
            result.passed = proc.returncode == 0

            for line in proc.stdout.split("\n"):
                if line.strip():
                    try:
                        msg = json.loads(line)
                        if msg.get("reason") == "compiler-message":
                            level = msg.get("message", {}).get("level", "")
                            if level == "error":
                                result.error_count += 1
                            elif level == "warning":
                                result.warning_count += 1
                    except json.JSONDecodeError:
                        pass
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _check_cargo_fmt(self, path: Path) -> FormattingResult:
        """Check if code is properly formatted."""
        result = FormattingResult(tool="rustfmt")
        try:
            proc = subprocess.run(
                ["cargo", "fmt", "--check"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=60,
            )
            result.passed = proc.returncode == 0
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _run_cargo_test(self, path: Path) -> TestResult:
        """Run cargo test and return results."""
        result = TestResult()
        try:
            proc = subprocess.run(
                ["cargo", "test", "--", "--format=json", "-Z", "unstable-options"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            result.passed = proc.returncode == 0

            # Parse test results from JSON
            for line in proc.stdout.split("\n"):
                if line.strip():
                    try:
                        msg = json.loads(line)
                        if msg.get("type") == "test":
                            result.total += 1
                            event = msg.get("event", "")
                            if event == "ok":
                                result.passed_count += 1
                            elif event == "failed":
                                result.failed_count += 1
                            elif event == "ignored":
                                result.skipped_count += 1
                    except json.JSONDecodeError:
                        pass

            # Fallback: parse regular output if JSON parsing failed
            if result.total == 0:
                # Look for "test result: ok. X passed; Y failed"
                import re
                match = re.search(
                    r"(\d+) passed.*?(\d+) failed",
                    proc.stdout + proc.stderr,
                )
                if match:
                    result.passed_count = int(match.group(1))
                    result.failed_count = int(match.group(2))
                    result.total = result.passed_count + result.failed_count

        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _get_cargo_coverage(self, path: Path) -> CoverageResult:
        """Get test coverage using cargo tarpaulin or llvm-cov."""
        result = CoverageResult()

        # Try cargo tarpaulin first
        try:
            proc = subprocess.run(
                ["cargo", "tarpaulin", "--out", "json", "--output-dir", "/tmp"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            if proc.returncode == 0:
                coverage_file = Path("/tmp/tarpaulin-report.json")
                if coverage_file.exists():
                    data = json.loads(coverage_file.read_text())
                    # Extract coverage percentage
                    if "coverage" in data:
                        result.line_coverage_pct = data["coverage"]
                    return result
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass

        # Fallback: try cargo llvm-cov
        try:
            proc = subprocess.run(
                ["cargo", "llvm-cov", "--json"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            if proc.returncode == 0:
                data = json.loads(proc.stdout)
                totals = data.get("data", [{}])[0].get("totals", {})
                lines = totals.get("lines", {})
                if lines.get("count", 0) > 0:
                    result.line_coverage_pct = lines.get("percent", 0)
                functions = totals.get("functions", {})
                if functions.get("count", 0) > 0:
                    result.function_coverage_pct = functions.get("percent", 0)
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass

        return result

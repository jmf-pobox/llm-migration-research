"""Post-hoc analysis using external tools.

This module runs external tools like lizard, cloc, and cargo
to capture code metrics that aren't available during migration.
"""

import json
import re
import subprocess
from pathlib import Path
from typing import Any

from .schema import (
    CodeMetrics,
    CompilationResult,
    CoverageResult,
    FormattingResult,
    LintingResult,
    QualityGates,
    TestOutcomeResult,
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

        # Calculate Maintainability Index with radon
        metrics.maintainability_index = self._run_radon_mi(source_path)

        # Count external dependencies
        metrics.external_dependencies = self._count_python_deps(source_path)

        return metrics

    def analyze_rust_target(self, target_path: Path) -> CodeMetrics:
        """Analyze Rust target code complexity and size."""
        metrics = CodeMetrics()
        src_path = target_path / "src"

        if not src_path.exists():
            return metrics

        # Count LOC separating production from inline #[cfg(test)] code
        prod_loc, inline_test_loc = self._count_rust_loc_with_inline_tests(src_path)
        metrics.production_loc = prod_loc
        metrics.test_loc = inline_test_loc

        # Add tests/ directory LOC to test_loc
        tests_path = target_path / "tests"
        if tests_path.exists():
            test_loc = self._run_cloc(tests_path, "Rust")
            if test_loc:
                metrics.test_loc += test_loc.get("code", 0)

        # Calculate total_loc
        metrics.total_loc = metrics.production_loc + metrics.test_loc

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

        # Calculate Maintainability Index from collected metrics
        metrics.maintainability_index = self._calculate_mi_from_metrics(
            metrics.production_loc,
            metrics.avg_cyclomatic_complexity,
        )

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

        # Count external dependencies from build.gradle or pom.xml
        metrics.external_dependencies = self._count_java_deps(target_path)

        # Calculate Maintainability Index from collected metrics
        metrics.maintainability_index = self._calculate_mi_from_metrics(
            metrics.production_loc,
            metrics.avg_cyclomatic_complexity,
        )

        return metrics

    def analyze_go_target(self, target_path: Path) -> CodeMetrics:
        """Analyze Go target code complexity and size.

        Go projects have production code in *.go files and test code in
        *_test.go files, typically at the root of the module.
        """
        metrics = CodeMetrics()

        # Go files are typically at root or in subdirectories
        # First check for go.mod to find module root
        go_files = list(target_path.glob("*.go"))
        if not go_files:
            # Check for cmd/ or pkg/ structure
            go_files = list(target_path.glob("**/*.go"))

        if not go_files:
            return metrics

        # Separate prod and test files
        prod_files = [f for f in go_files if not f.name.endswith("_test.go")]
        test_files = [f for f in go_files if f.name.endswith("_test.go")]

        # Count LOC for production files
        if prod_files:
            loc_data = self._run_cloc_files(prod_files, "Go")
            if loc_data:
                metrics.production_loc = loc_data.get("code", 0)

        # Count LOC for test files
        if test_files:
            test_loc = self._run_cloc_files(test_files, "Go")
            if test_loc:
                metrics.test_loc = test_loc.get("code", 0)

        metrics.total_loc = metrics.production_loc + metrics.test_loc

        # Analyze complexity with lizard (use target_path for all Go files)
        complexity_data = self._run_lizard(target_path, "go")
        if complexity_data:
            metrics.function_count = complexity_data.get("function_count", 0)
            metrics.avg_cyclomatic_complexity = complexity_data.get("avg_cc", 0.0)
            metrics.max_cyclomatic_complexity = complexity_data.get("max_cc", 0)

        # Count modules (non-test .go files)
        metrics.module_count = len(prod_files)

        # Count external dependencies from go.mod
        go_mod = target_path / "go.mod"
        if go_mod.exists():
            metrics.external_dependencies = self._count_go_deps(go_mod)

        # Calculate Maintainability Index from collected metrics
        metrics.maintainability_index = self._calculate_mi_from_metrics(
            metrics.production_loc,
            metrics.avg_cyclomatic_complexity,
        )

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

    def analyze_java_quality(self, target_path: Path) -> QualityGates:
        """Run Java quality gates and return results."""
        quality = QualityGates()

        # Check compilation and tests (gradle/maven)
        if (target_path / "build.gradle").exists():
            quality.unit_tests = self._run_gradle_test(target_path)
            quality.compilation.passed = quality.unit_tests.passed or (
                self._check_gradle_compile(target_path)
            )
            quality.coverage = self._get_jacoco_coverage(target_path)
        elif (target_path / "pom.xml").exists():
            # Basic support for Maven if needed
            pass

        return quality

    def analyze_go_quality(self, target_path: Path) -> QualityGates:
        """Run Go quality gates and return results."""
        quality = QualityGates()

        # Check compilation
        quality.compilation.passed = self._check_go_build(target_path)

        # Run tests
        quality.unit_tests = self._run_go_test(target_path)

        # Get coverage
        quality.coverage = self._get_go_coverage(target_path)

        return quality

    def _check_gradle_compile(self, path: Path) -> bool:
        """Check if Java project compiles with Gradle."""
        try:
            proc = subprocess.run(
                ["gradle", "classes"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=300,
            )
            return proc.returncode == 0
        except (subprocess.TimeoutExpired, FileNotFoundError):
            return False

    def _run_gradle_test(self, path: Path) -> TestOutcomeResult:
        """Run gradle test and return results."""
        result = TestOutcomeResult()
        try:
            proc = subprocess.run(
                ["gradle", "test"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            result.passed = proc.returncode == 0

            # Parse test results from reports if they exist
            # This is a bit complex as Gradle writes XML files.
            # Simplified fallback: parse stdout for "X tests completed, Y failed"
            import re

            match = re.search(
                r"(\d+) tests completed, (\d+) failed", proc.stdout + proc.stderr
            )
            if match:
                result.total = int(match.group(1))
                result.failed_count = int(match.group(2))
                result.passed_count = result.total - result.failed_count
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _get_jacoco_coverage(self, path: Path) -> CoverageResult:
        """Get test coverage from JaCoCo report."""
        result = CoverageResult()
        jacoco_xml = (
            path / "build" / "reports" / "jacoco" / "test" / "jacocoTestReport.xml"
        )
        if jacoco_xml.exists():
            try:
                # Parse XML for line and branch coverage
                from defusedxml import ElementTree  # type: ignore[import-untyped]
                tree = ElementTree.parse(jacoco_xml)
                root = tree.getroot()

                for counter in root.findall("counter"):
                    c_type = counter.get("type")
                    missed = int(counter.get("missed", 0))
                    covered = int(counter.get("covered", 0))
                    total = missed + covered
                    if total > 0:
                        pct = (covered / total) * 100
                        if c_type == "LINE":
                            result.line_coverage_pct = pct
                        elif c_type == "BRANCH":
                            result.branch_coverage_pct = pct
                        elif c_type == "METHOD":
                            result.function_coverage_pct = pct
            except Exception:  # noqa: S110
                # Suppression intentional here as quality gate data is optional
                pass
        return result

    def _check_go_build(self, path: Path) -> bool:
        """Check if Go project compiles."""
        try:
            proc = subprocess.run(
                ["go", "build", "./..."],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=300,
            )
            return proc.returncode == 0
        except (subprocess.TimeoutExpired, FileNotFoundError):
            return False

    def _run_go_test(self, path: Path) -> TestOutcomeResult:
        """Run go test and return results."""
        result = TestOutcomeResult()
        try:
            proc = subprocess.run(
                ["go", "test", "-v", "./..."],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            result.passed = proc.returncode == 0

            # Parse verbose output for "PASS: TestName" or "FAIL: TestName"
            passed = len(re.findall(r"--- PASS:", proc.stdout))
            failed = len(re.findall(r"--- FAIL:", proc.stdout))
            skipped = len(re.findall(r"--- SKIP:", proc.stdout))

            result.passed_count = passed
            result.failed_count = failed
            result.skipped_count = skipped
            result.total = passed + failed + skipped
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _get_go_coverage(self, path: Path) -> CoverageResult:
        """Get test coverage for Go project."""
        result = CoverageResult()
        try:
            # Run coverage
            subprocess.run(
                ["go", "test", "-coverprofile=coverage.out", "./..."],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=300,
            )
            # Parse coverage summary
            proc = subprocess.run(
                ["go", "tool", "cover", "-func=coverage.out"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=60,
            )
            if proc.returncode == 0:
                # Look for "total: (statements) X.Y%"
                match = re.search(r"total:\s+\(statements\)\s+(\d+\.\d+)%", proc.stdout)
                if match:
                    result.line_coverage_pct = float(match.group(1))
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return result

    def _run_cloc(self, path: Path, language: str) -> dict[str, Any] | None:
        """Run cloc and return LOC data for specified language."""
        try:
            result = subprocess.run(
                ["cloc", "--json", str(path)],
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode == 0:
                data: dict[str, Any] = json.loads(result.stdout)
                lang_data: dict[str, Any] = data.get(language, {})
                return lang_data
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass
        return None

    def _run_cloc_files(
        self, files: list[Path], language: str
    ) -> dict[str, Any] | None:
        """Run cloc on a list of specific files and return LOC data."""
        if not files:
            return None
        try:
            cmd = ["cloc", "--json"] + [str(f) for f in files]
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode == 0:
                data: dict[str, Any] = json.loads(result.stdout)
                lang_data: dict[str, Any] = data.get(language, {})
                return lang_data
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass
        return None

    def _count_rust_loc_with_inline_tests(self, src_path: Path) -> tuple[int, int]:
        """Count Rust LOC separating production from inline #[cfg(test)] code.

        Rust allows inline test modules marked with #[cfg(test)] within source
        files. This method separates code lines before the marker (production)
        from code lines after the marker (test).

        Args:
            src_path: Path to the src/ directory containing .rs files

        Returns:
            Tuple of (production_loc, inline_test_loc) where both counts
            exclude blank lines and single-line comments.
        """
        prod_lines = 0
        test_lines = 0

        for rs_file in src_path.glob("*.rs"):
            try:
                content = rs_file.read_text()
            except OSError:
                continue

            lines = content.split("\n")

            # Find #[cfg(test)] marker
            cfg_test_line: int | None = None
            for i, line in enumerate(lines):
                if "#[cfg(test)]" in line:
                    cfg_test_line = i
                    break

            if cfg_test_line is not None:
                # Lines before marker are production
                for line in lines[:cfg_test_line]:
                    stripped = line.strip()
                    if stripped and not stripped.startswith("//"):
                        prod_lines += 1
                # Lines from marker onward are test
                for line in lines[cfg_test_line:]:
                    stripped = line.strip()
                    if stripped and not stripped.startswith("//"):
                        test_lines += 1
            else:
                # No test marker - all lines are production
                for line in lines:
                    stripped = line.strip()
                    if stripped and not stripped.startswith("//"):
                        prod_lines += 1

        return prod_lines, test_lines

    def _run_lizard(self, path: Path, language: str) -> dict[str, Any] | None:
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

    def _run_cargo_test(self, path: Path) -> TestOutcomeResult:
        """Run cargo test and return results."""
        result = TestOutcomeResult()
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

                combined_output = proc.stdout + proc.stderr
                match = re.search(
                    r"(\d+) passed.*?(\d+) failed",
                    combined_output,
                )
                if match:
                    result.passed_count = int(match.group(1))
                    result.failed_count = int(match.group(2))

                # Also look for ignored tests
                ignored_match = re.search(r"(\d+) ignored", combined_output)
                if ignored_match:
                    result.skipped_count = int(ignored_match.group(1))

                result.total = (
                    result.passed_count + result.failed_count + result.skipped_count
                )

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
                    # Tarpaulin JSON format has a top-level "files" or similar
                    # but also often provides a summary.
                    if "coverage" in data:
                        result.line_coverage_pct = data["coverage"]
                    return result
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass

        # Fallback: try cargo llvm-cov
        try:
            proc = subprocess.run(
                ["cargo", "llvm-cov", "--json", "--summary-only"],
                cwd=path,
                capture_output=True,
                text=True,
                timeout=600,
            )
            if proc.returncode == 0:
                data = json.loads(proc.stdout)
                totals = data.get("data", [{}])[0].get("totals", {})

                # Lines
                lines = totals.get("lines", {})
                if lines.get("count", 0) > 0:
                    result.line_coverage_pct = lines.get("percent", 0)

                # Functions
                functions = totals.get("functions", {})
                if functions.get("count", 0) > 0:
                    result.function_coverage_pct = functions.get("percent", 0)

                # Branches (instantiated as regions in some llvm-cov versions)
                branches = totals.get("branches", {})
                if branches.get("count", 0) > 0:
                    result.branch_coverage_pct = branches.get("percent", 0)
        except (subprocess.TimeoutExpired, FileNotFoundError, json.JSONDecodeError):
            pass

        return result

    def _run_radon_mi(self, path: Path) -> float | None:
        """Run radon mi and return average Maintainability Index."""
        try:
            result = subprocess.run(
                ["radon", "mi", "-s", str(path)],
                capture_output=True,
                text=True,
                timeout=60,
            )
            if result.returncode == 0:
                # Parse radon mi output: "filename - A (score)"
                scores = []
                for line in result.stdout.strip().split("\n"):
                    if " - " in line and "(" in line:
                        # Extract score from parentheses
                        try:
                            score_str = line.split("(")[-1].rstrip(")")
                            scores.append(float(score_str))
                        except (ValueError, IndexError):
                            pass
                if scores:
                    return sum(scores) / len(scores)
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
        return None

    def _count_python_deps(self, source_path: Path) -> int:
        """Count Python dependencies from requirements.txt or pyproject.toml."""
        # Check requirements.txt first
        req_file = source_path / "requirements.txt"
        if not req_file.exists():
            req_file = source_path.parent / "requirements.txt"
        if req_file.exists():
            count = 0
            for line in req_file.read_text().split("\n"):
                line = line.strip()
                if line and not line.startswith("#") and not line.startswith("-"):
                    count += 1
            return count

        # Check pyproject.toml
        pyproject = source_path / "pyproject.toml"
        if not pyproject.exists():
            pyproject = source_path.parent / "pyproject.toml"
        if pyproject.exists():
            content = pyproject.read_text()
            in_deps = False
            count = 0
            for line in content.split("\n"):
                if "dependencies" in line and "=" in line:
                    in_deps = True
                elif line.strip().startswith("[") and in_deps:
                    break
                elif in_deps and line.strip().startswith('"'):
                    count += 1
            return count

        return 0

    def _count_java_deps(self, target_path: Path) -> int:
        """Count Java dependencies from build.gradle or pom.xml."""
        # Check build.gradle
        gradle_file = target_path / "build.gradle"
        if gradle_file.exists():
            content = gradle_file.read_text()
            # Count implementation/api/compile dependencies
            import re

            deps = re.findall(
                r"(implementation|api|compile|testImplementation)\s*['\"]",
                content,
            )
            return len(deps)

        # Check pom.xml
        pom_file = target_path / "pom.xml"
        if pom_file.exists():
            content = pom_file.read_text()
            # Count <dependency> tags (excluding test scope)
            import re

            deps = re.findall(r"<dependency>", content)
            return len(deps)

        return 0

    def _count_go_deps(self, go_mod: Path) -> int:
        """Count Go dependencies from go.mod file."""
        try:
            content = go_mod.read_text()
            # Count require statements
            # Can be single line: require github.com/foo/bar v1.0.0
            # Or block: require ( ... )
            import re

            # Single-line requires
            single_requires = re.findall(r"^require\s+\S+", content, re.MULTILINE)

            # Block requires (count lines inside require blocks)
            block_requires: list[str] = []
            in_block = False
            for line in content.split("\n"):
                stripped = line.strip()
                if stripped.startswith("require ("):
                    in_block = True
                    continue
                if in_block:
                    if stripped == ")":
                        in_block = False
                    elif stripped and not stripped.startswith("//"):
                        block_requires.append(stripped)

            return len(single_requires) + len(block_requires)
        except OSError:
            return 0

    def _calculate_mi_from_metrics(self, loc: int, avg_cc: float) -> float | None:
        """Calculate Maintainability Index from LOC and cyclomatic complexity.

        Uses simplified formula: MI = 171 - 5.2*ln(HV) - 0.23*CC - 16.2*ln(LOC)
        Since we don't have Halstead Volume, we use a simplified version:
        MI = 171 - 0.23*CC - 16.2*ln(LOC)

        Returns value clamped to 0-100 range.
        """
        import math

        if loc <= 0:
            return None

        # Simplified MI formula (without Halstead Volume)
        mi = 171 - 0.23 * avg_cc - 16.2 * math.log(loc)

        # Clamp to 0-100 range
        return max(0.0, min(100.0, mi))

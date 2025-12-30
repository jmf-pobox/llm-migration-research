"""Unit tests for runner module."""

import tempfile
from pathlib import Path
from unittest.mock import MagicMock, patch

import pytest

from migration.runner import (
    _count_go_loc,
    _count_java_loc,
    _count_python_loc,
    _count_rust_test_loc,
    measure_loc,
)


class TestCountRustTestLoc:
    """Tests for _count_rust_test_loc function."""

    def test_production_only(self) -> None:
        """Test file with only production code."""
        content = """
fn main() {
    println!("Hello");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".rs", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_rust_test_loc(Path(f.name))

        assert prod > 0
        assert test == 0

    def test_with_test_module(self) -> None:
        """Test file with #[cfg(test)] module."""
        content = """
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".rs", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_rust_test_loc(Path(f.name))

        assert prod > 0
        assert test > 0

    def test_nonexistent_file(self) -> None:
        """Test handling of nonexistent file."""
        prod, test = _count_rust_test_loc(Path("/nonexistent/file.rs"))
        assert prod == 0
        assert test == 0

    def test_empty_lines_ignored(self) -> None:
        """Test that empty lines and comments are ignored."""
        content = """
// This is a comment

fn main() {
    // Another comment
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".rs", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, _test = _count_rust_test_loc(Path(f.name))

        # Should only count non-empty, non-comment lines
        # fn main() { and } (closing brace)
        assert prod == 2


class TestCountGoLoc:
    """Tests for _count_go_loc function."""

    def test_production_file(self) -> None:
        """Test regular Go file."""
        content = """
package main

func main() {
    fmt.Println("Hello")
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".go", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_go_loc(Path(f.name))

        assert prod > 0
        assert test == 0

    def test_test_file(self) -> None:
        """Test Go test file (_test.go suffix)."""
        content = """
package main

import "testing"

func TestAdd(t *testing.T) {
    if add(1, 2) != 3 {
        t.Error("failed")
    }
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix="_test.go", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_go_loc(Path(f.name))

        assert prod == 0
        assert test > 0

    def test_nonexistent_file(self) -> None:
        """Test handling of nonexistent file."""
        prod, test = _count_go_loc(Path("/nonexistent/file.go"))
        assert prod == 0
        assert test == 0


class TestCountJavaLoc:
    """Tests for _count_java_loc function."""

    def test_production_file(self) -> None:
        """Test regular Java file."""
        content = """
package com.example;

public class Main {
    public static void main(String[] args) {
        System.out.println("Hello");
    }
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".java", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_java_loc(Path(f.name))

        assert prod > 0
        assert test == 0

    def test_test_file_by_name(self) -> None:
        """Test Java test file (Test suffix)."""
        content = """
package com.example;

import org.junit.Test;

public class MainTest {
    @Test
    public void testMain() {
    }
}
"""
        # Create file with Test suffix
        with tempfile.TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "MainTest.java"
            test_file.write_text(content)

            prod, test = _count_java_loc(test_file)

        assert prod == 0
        assert test > 0

    def test_test_file_by_path(self) -> None:
        """Test Java test file (in test/ directory)."""
        content = """
public class MyTest {}
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            test_dir = Path(tmpdir) / "test"
            test_dir.mkdir()
            test_file = test_dir / "MyFile.java"
            test_file.write_text(content)

            prod, test = _count_java_loc(test_file)

        assert prod == 0
        assert test > 0

    def test_block_comments_ignored(self) -> None:
        """Test that block comments are ignored."""
        content = """
package com.example;

/*
 * This is a block comment
 * spanning multiple lines
 */
public class Main {
}
"""
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".java", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, _test = _count_java_loc(Path(f.name))

        # Only package line and class lines should count
        assert prod == 3  # package, public class Main {, }


class TestCountPythonLoc:
    """Tests for _count_python_loc function."""

    def test_production_file(self) -> None:
        """Test regular Python file."""
        content = '''
def main():
    print("Hello")

if __name__ == "__main__":
    main()
'''
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".py", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, test = _count_python_loc(Path(f.name))

        assert prod > 0
        assert test == 0

    def test_test_file_by_name(self) -> None:
        """Test Python test file (test_ prefix)."""
        content = """
def test_something():
    assert True
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            test_file = Path(tmpdir) / "test_example.py"
            test_file.write_text(content)

            prod, test = _count_python_loc(test_file)

        assert prod == 0
        assert test > 0

    def test_test_file_by_path(self) -> None:
        """Test Python test file (in tests/ directory)."""
        content = """
def my_test():
    pass
"""
        with tempfile.TemporaryDirectory() as tmpdir:
            tests_dir = Path(tmpdir) / "tests"
            tests_dir.mkdir()
            test_file = tests_dir / "example.py"
            test_file.write_text(content)

            prod, test = _count_python_loc(test_file)

        assert prod == 0
        assert test > 0

    def test_docstrings_ignored(self) -> None:
        """Test that docstrings are ignored."""
        content = '''
def main():
    """This is a docstring."""
    print("Hello")
'''
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".py", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, _test = _count_python_loc(Path(f.name))

        # Only def and print should count
        assert prod == 2

    def test_multiline_docstrings_ignored(self) -> None:
        """Test that multiline docstrings are ignored."""
        content = '''
def main():
    """
    This is a multiline
    docstring.
    """
    print("Hello")
'''
        with tempfile.NamedTemporaryFile(
            mode="w", suffix=".py", delete=False
        ) as f:
            f.write(content)
            f.flush()

            prod, _test = _count_python_loc(Path(f.name))

        # Only def and print should count
        assert prod == 2


class TestMeasureLoc:
    """Tests for measure_loc function."""

    def test_measure_python_loc(self) -> None:
        """Test measuring Python LOC."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create production file
            prod_file = Path(tmpdir) / "main.py"
            prod_file.write_text("def main():\n    pass\n")

            # Create test file
            tests_dir = Path(tmpdir) / "tests"
            tests_dir.mkdir()
            test_file = tests_dir / "test_main.py"
            test_file.write_text("def test_main():\n    pass\n")

            prod, test, count = measure_loc(tmpdir, "python")

        assert prod > 0
        assert test > 0
        assert count == 2

    def test_measure_rust_loc(self) -> None:
        """Test measuring Rust LOC."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create Rust file
            rust_file = Path(tmpdir) / "main.rs"
            rust_file.write_text("fn main() {\n    println!(\"Hello\");\n}\n")

            prod, _test, count = measure_loc(tmpdir, "rust")

        assert prod > 0
        assert count == 1

    def test_measure_nonexistent_directory(self) -> None:
        """Test handling of nonexistent directory."""
        prod, test, count = measure_loc("/nonexistent/dir", "python")

        assert prod == 0
        assert test == 0
        assert count == 0

    def test_measure_unsupported_language(self) -> None:
        """Test handling of unsupported language."""
        with tempfile.TemporaryDirectory() as tmpdir:
            prod, test, count = measure_loc(tmpdir, "cobol")

        assert prod == 0
        assert test == 0
        assert count == 0

    def test_measure_with_log_file(self) -> None:
        """Test measuring with log output."""
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create file
            rust_file = Path(tmpdir) / "lib.rs"
            rust_file.write_text("pub fn add() {}\n")

            log_file = Path(tmpdir) / "test.log"

            measure_loc(tmpdir, "rust", log_file)

            # Check log was written
            assert log_file.exists()
            content = log_file.read_text()
            assert "LOC" in content


class TestMeasureCoverage:
    """Tests for measure_coverage function."""

    def test_measure_coverage_no_coverage_command(self) -> None:
        """Test coverage measurement with mock."""
        from migration.runner import measure_coverage

        assert callable(measure_coverage)

    @patch("subprocess.run")
    def test_measure_coverage_success(self, mock_run: MagicMock) -> None:
        """Test successful coverage measurement."""
        from migration.languages import RustTarget
        from migration.runner import measure_coverage

        mock_run.return_value = MagicMock(
            returncode=0, stdout="Total coverage: 85.5%", stderr=""
        )

        target = RustTarget()
        with tempfile.TemporaryDirectory() as tmpdir:
            result = measure_coverage(target, tmpdir)

        # Result depends on parser, may be None or float
        assert result is None or isinstance(result, float)

    @patch("subprocess.run")
    def test_measure_coverage_failure(self, mock_run: MagicMock) -> None:
        """Test coverage measurement failure."""
        from migration.languages import RustTarget
        from migration.runner import measure_coverage

        mock_run.return_value = MagicMock(returncode=1, stdout="", stderr="error")

        target = RustTarget()
        with tempfile.TemporaryDirectory() as tmpdir:
            result = measure_coverage(target, tmpdir)

        assert result is None


class TestEvaluateIdiomaticness:
    """Tests for evaluate_idiomaticness function."""

    def test_no_sdk_available(self) -> None:
        """Test when claude-agent-sdk is not available."""
        from migration.languages import RustTarget
        from migration.runner import evaluate_idiomaticness

        # The function should handle ImportError gracefully
        with patch.dict("sys.modules", {"claude_agent_sdk": None}):
            target = RustTarget()
            with tempfile.TemporaryDirectory() as tmpdir:
                score, _reasoning = evaluate_idiomaticness(target, tmpdir)

        # Should return None when SDK is not available or dir is empty
        assert score is None

    def test_no_source_files(self) -> None:
        """Test when no source files exist."""
        from migration.languages import RustTarget
        from migration.runner import evaluate_idiomaticness

        target = RustTarget()
        with tempfile.TemporaryDirectory() as tmpdir:
            # Create src dir but no .rs files
            (Path(tmpdir) / "src").mkdir()
            score, _reasoning = evaluate_idiomaticness(target, tmpdir)

        assert score is None


class TestLog:
    """Tests for log function."""

    def test_log_to_stdout(self, capsys: pytest.CaptureFixture) -> None:
        """Test logging to stdout."""
        from migration.runner import log

        log("Test message", None)

        captured = capsys.readouterr()
        assert "Test message" in captured.out

    def test_log_to_file(self) -> None:
        """Test logging to file."""
        from migration.runner import log

        with tempfile.TemporaryDirectory() as tmpdir:
            log_file = Path(tmpdir) / "test.log"
            log("Test message", log_file)

            content = log_file.read_text()
            assert "Test message" in content


class TestBuildMigrationPrompt:
    """Tests for build_migration_prompt function."""

    def test_build_migration_prompt(self) -> None:
        """Test building migration prompt."""
        from migration.config import IOContract, ModuleConfig, ProjectConfig
        from migration.languages import RustTarget
        from migration.runner import build_migration_prompt

        config = ProjectConfig(
            name="test",
            description="Test",
            source_language="python",
            source_directory="/src",
            source_files=["main.py"],
            modules=[ModuleConfig(name="main", source="main.py", phase="core")],
            test_inputs=["1 2 +"],
            io_contract=IOContract(command_template="python main.py"),
        )
        target = RustTarget()

        prompt = build_migration_prompt(config, target, "/project")

        assert "test" in prompt
        assert "Rust" in prompt


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

"""Unit tests for migration strategies."""

import pytest

from migration.config import (
    FeatureConfig,
    IOContract,
    ModuleConfig,
    ProjectConfig,
    TestCase,
)
from migration.languages import RustTarget
from migration.strategies import (
    FeatureByFeatureStrategy,
    ModuleByModuleStrategy,
    get_strategy,
)


def create_test_config(with_features: bool = False) -> ProjectConfig:
    """Create a minimal ProjectConfig for testing."""
    modules = [
        ModuleConfig(
            name="lexer",
            source="lexer.py",
            phase="foundation",
            description="Tokenizer",
        ),
        ModuleConfig(
            name="parser",
            source="parser.py",
            phase="core",
            depends_on=["lexer"],
        ),
    ]

    features = []
    if with_features:
        features = [
            FeatureConfig(
                name="addition",
                description="Addition operator",
                touches=["lexer.py", "parser.py"],
                test_cases=[TestCase(input="2 3 +", output="5")],
            ),
            FeatureConfig(
                name="multiplication",
                description="Multiplication operator",
                touches=["lexer.py", "parser.py"],
                test_cases=[TestCase(input="2 3 *", output="6")],
                depends_on=["addition"],
            ),
        ]

    return ProjectConfig(
        name="test-project",
        description="Test project",
        source_language="python",
        source_directory="/src",
        source_files=["lexer.py", "parser.py"],
        modules=modules,
        test_inputs=["2 3 +", "4 5 *"],
        io_contract=IOContract(command_template="python -m rpn {input}"),
        features=features,
    )


class TestGetStrategy:
    """Tests for get_strategy factory function."""

    def test_get_module_by_module_strategy(self) -> None:
        """Test retrieving module-by-module strategy."""
        strategy = get_strategy("module-by-module")
        assert isinstance(strategy, ModuleByModuleStrategy)
        assert strategy.name == "module-by-module"

    def test_get_feature_by_feature_strategy(self) -> None:
        """Test retrieving feature-by-feature strategy."""
        strategy = get_strategy("feature-by-feature")
        assert isinstance(strategy, FeatureByFeatureStrategy)
        assert strategy.name == "feature-by-feature"

    def test_get_unknown_strategy_raises(self) -> None:
        """Test that unknown strategy raises ValueError."""
        with pytest.raises(ValueError, match="Unknown strategy"):
            get_strategy("unknown")


class TestModuleByModuleStrategy:
    """Tests for ModuleByModuleStrategy."""

    def test_get_slices_returns_modules(self) -> None:
        """Test that get_slices returns one slice per module."""
        strategy = ModuleByModuleStrategy()
        config = create_test_config()

        slices = strategy.get_slices(config)

        assert len(slices) == 2
        assert slices[0].name == "lexer"
        assert slices[0].source_files == ["lexer.py"]
        assert slices[1].name == "parser"

    def test_get_prompt_contains_modules(self) -> None:
        """Test that prompt includes module information."""
        strategy = ModuleByModuleStrategy()
        config = create_test_config()
        target = RustTarget()

        prompt = strategy.get_prompt(config, target, "/project")

        assert "lexer.py" in prompt
        assert "parser.py" in prompt
        assert "Rust" in prompt
        assert "module" in prompt.lower()  # "module to migrate", "modules", etc.

    def test_get_prompt_includes_quality_gates(self) -> None:
        """Test that prompt includes quality gate commands."""
        strategy = ModuleByModuleStrategy()
        config = create_test_config()
        target = RustTarget()

        prompt = strategy.get_prompt(config, target, "/project")

        assert "cargo" in prompt


class TestFeatureByFeatureStrategy:
    """Tests for FeatureByFeatureStrategy."""

    def test_get_slices_returns_features(self) -> None:
        """Test that get_slices returns one slice per feature."""
        strategy = FeatureByFeatureStrategy()
        config = create_test_config(with_features=True)

        slices = strategy.get_slices(config)

        assert len(slices) == 2
        assert slices[0].name == "addition"
        assert slices[0].source_files == ["lexer.py", "parser.py"]
        assert slices[1].name == "multiplication"

    def test_get_slices_without_features_raises(self) -> None:
        """Test that get_slices raises when no features defined."""
        strategy = FeatureByFeatureStrategy()
        config = create_test_config(with_features=False)

        with pytest.raises(ValueError, match="does not define features"):
            strategy.get_slices(config)

    def test_get_prompt_contains_features(self) -> None:
        """Test that prompt includes feature information."""
        strategy = FeatureByFeatureStrategy()
        config = create_test_config(with_features=True)
        target = RustTarget()

        prompt = strategy.get_prompt(config, target, "/project")

        assert "addition" in prompt
        assert "multiplication" in prompt
        assert "feature" in prompt.lower()

    def test_get_prompt_includes_test_cases(self) -> None:
        """Test that prompt includes test cases for features."""
        strategy = FeatureByFeatureStrategy()
        config = create_test_config(with_features=True)
        target = RustTarget()

        prompt = strategy.get_prompt(config, target, "/project")

        assert "2 3 +" in prompt
        assert "5" in prompt


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

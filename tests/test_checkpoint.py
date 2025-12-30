"""Tests for checkpoint module."""

import json
from pathlib import Path

import pytest

from migration.checkpoint import CheckpointManager, CheckpointState


class TestCheckpointState:
    """Tests for CheckpointState dataclass."""

    def test_default_values(self) -> None:
        """Test default values for optional fields."""
        state = CheckpointState(
            project="test-project",
            target="rust",
            strategy="feature-by-feature",
        )
        assert state.project == "test-project"
        assert state.target == "rust"
        assert state.strategy == "feature-by-feature"
        assert state.session_id == ""
        assert state.current_feature is None
        assert state.completed_features == []
        assert state.failed_feature is None
        assert state.error_message is None
        assert state.last_checkpoint_time == ""

    def test_to_dict(self) -> None:
        """Test serialization to dictionary."""
        state = CheckpointState(
            project="rpn2tex",
            target="go",
            strategy="module-by-module",
            session_id="abc-123",
            current_feature="addition",
            completed_features=["numbers"],
            failed_feature=None,
            error_message=None,
            last_checkpoint_time="2025-01-01T00:00:00Z",
        )
        result = state.to_dict()
        assert result["project"] == "rpn2tex"
        assert result["target"] == "go"
        assert result["strategy"] == "module-by-module"
        assert result["session_id"] == "abc-123"
        assert result["current_feature"] == "addition"
        assert result["completed_features"] == ["numbers"]
        assert result["failed_feature"] is None
        assert result["error_message"] is None
        assert result["last_checkpoint_time"] == "2025-01-01T00:00:00Z"

    def test_from_dict(self) -> None:
        """Test deserialization from dictionary."""
        data: dict[str, str | list[str] | None] = {
            "project": "rpn2tex",
            "target": "java",
            "strategy": "feature-by-feature",
            "session_id": "xyz-789",
            "current_feature": None,
            "completed_features": ["numbers", "addition"],
            "failed_feature": "subtraction",
            "error_message": "Command failed",
            "last_checkpoint_time": "2025-01-02T12:00:00Z",
        }
        state = CheckpointState.from_dict(data)
        assert state.project == "rpn2tex"
        assert state.target == "java"
        assert state.strategy == "feature-by-feature"
        assert state.session_id == "xyz-789"
        assert state.current_feature is None
        assert state.completed_features == ["numbers", "addition"]
        assert state.failed_feature == "subtraction"
        assert state.error_message == "Command failed"

    def test_from_dict_missing_fields(self) -> None:
        """Test deserialization handles missing fields gracefully."""
        data: dict[str, str | list[str] | None] = {
            "project": "test",
            "target": "rust",
            "strategy": "module-by-module",
        }
        state = CheckpointState.from_dict(data)
        assert state.project == "test"
        assert state.session_id == ""
        assert state.completed_features == []
        assert state.current_feature is None

    def test_from_dict_empty(self) -> None:
        """Test deserialization of empty dict."""
        data: dict[str, str | list[str] | None] = {}
        state = CheckpointState.from_dict(data)
        assert state.project == ""
        assert state.target == ""
        assert state.strategy == ""
        assert state.completed_features == []

    def test_roundtrip(self) -> None:
        """Test to_dict/from_dict roundtrip preserves data."""
        original = CheckpointState(
            project="rpn2tex",
            target="rust",
            strategy="feature-by-feature",
            session_id="session-abc",
            current_feature="multiplication",
            completed_features=["numbers", "addition", "subtraction"],
            failed_feature=None,
            error_message=None,
            last_checkpoint_time="2025-01-03T08:30:00Z",
        )
        restored = CheckpointState.from_dict(original.to_dict())
        assert restored.project == original.project
        assert restored.target == original.target
        assert restored.strategy == original.strategy
        assert restored.session_id == original.session_id
        assert restored.current_feature == original.current_feature
        assert restored.completed_features == original.completed_features
        assert restored.failed_feature == original.failed_feature
        assert restored.error_message == original.error_message


class TestCheckpointManager:
    """Tests for CheckpointManager."""

    def test_exists_no_checkpoint(self, tmp_path: Path) -> None:
        """Test exists returns False when no checkpoint."""
        manager = CheckpointManager(tmp_path)
        assert not manager.exists()

    def test_load_no_checkpoint(self, tmp_path: Path) -> None:
        """Test load returns None when no checkpoint."""
        manager = CheckpointManager(tmp_path)
        assert manager.load() is None

    def test_save_and_load(self, tmp_path: Path) -> None:
        """Test save and load roundtrip."""
        manager = CheckpointManager(tmp_path)
        state = CheckpointState(
            project="test",
            target="rust",
            strategy="feature-by-feature",
            session_id="test-session",
        )
        manager.save(state)

        assert manager.exists()
        loaded = manager.load()
        assert loaded is not None
        assert loaded.project == "test"
        assert loaded.target == "rust"
        assert loaded.session_id == "test-session"
        # last_checkpoint_time should be set
        assert loaded.last_checkpoint_time != ""

    def test_save_creates_directory(self, tmp_path: Path) -> None:
        """Test save creates checkpoint directory."""
        manager = CheckpointManager(tmp_path)
        state = CheckpointState(
            project="test", target="go", strategy="module-by-module"
        )
        manager.save(state)

        assert manager.checkpoint_dir.exists()
        assert manager.state_file.exists()

    def test_clear(self, tmp_path: Path) -> None:
        """Test clear removes checkpoint."""
        manager = CheckpointManager(tmp_path)
        state = CheckpointState(
            project="test", target="rust", strategy="feature-by-feature"
        )
        manager.save(state)
        assert manager.exists()

        manager.clear()
        assert not manager.exists()
        assert manager.load() is None

    def test_clear_nonexistent(self, tmp_path: Path) -> None:
        """Test clear handles nonexistent checkpoint gracefully."""
        manager = CheckpointManager(tmp_path)
        manager.clear()  # Should not raise
        assert not manager.exists()

    def test_create_initial(self, tmp_path: Path) -> None:
        """Test create_initial creates and saves state."""
        manager = CheckpointManager(tmp_path)
        state = manager.create_initial(
            project="rpn2tex",
            target="java",
            strategy="feature-by-feature",
        )

        assert state.project == "rpn2tex"
        assert state.target == "java"
        assert state.strategy == "feature-by-feature"
        assert manager.exists()

        loaded = manager.load()
        assert loaded is not None
        assert loaded.project == "rpn2tex"

    def test_mark_feature_started(self, tmp_path: Path) -> None:
        """Test mark_feature_started updates state."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")

        manager.mark_feature_started("addition", "session-123")

        state = manager.load()
        assert state is not None
        assert state.current_feature == "addition"
        assert state.session_id == "session-123"
        assert state.failed_feature is None

    def test_mark_feature_started_no_state(self, tmp_path: Path) -> None:
        """Test mark_feature_started raises when no state exists."""
        manager = CheckpointManager(tmp_path)

        with pytest.raises(ValueError, match="No checkpoint state exists"):
            manager.mark_feature_started("addition", "session-123")

    def test_mark_feature_completed(self, tmp_path: Path) -> None:
        """Test mark_feature_completed updates state."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        manager.mark_feature_started("numbers", "session-1")

        manager.mark_feature_completed("numbers", "session-2")

        state = manager.load()
        assert state is not None
        assert state.current_feature is None
        assert "numbers" in state.completed_features
        assert state.session_id == "session-2"

    def test_mark_feature_completed_no_duplicate(self, tmp_path: Path) -> None:
        """Test mark_feature_completed doesn't add duplicates."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")

        manager.mark_feature_completed("numbers", "session-1")
        manager.mark_feature_completed("numbers", "session-2")

        state = manager.load()
        assert state is not None
        assert state.completed_features.count("numbers") == 1

    def test_mark_feature_completed_no_state(self, tmp_path: Path) -> None:
        """Test mark_feature_completed raises when no state exists."""
        manager = CheckpointManager(tmp_path)

        with pytest.raises(ValueError, match="No checkpoint state exists"):
            manager.mark_feature_completed("numbers", "session-123")

    def test_mark_feature_failed(self, tmp_path: Path) -> None:
        """Test mark_feature_failed updates state."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        manager.mark_feature_started("subtraction", "session-1")

        manager.mark_feature_failed("subtraction", "Command failed", "session-1")

        state = manager.load()
        assert state is not None
        assert state.current_feature is None
        assert state.failed_feature == "subtraction"
        assert state.error_message == "Command failed"
        assert state.session_id == "session-1"

    def test_mark_feature_failed_no_state(self, tmp_path: Path) -> None:
        """Test mark_feature_failed raises when no state exists."""
        manager = CheckpointManager(tmp_path)

        with pytest.raises(ValueError, match="No checkpoint state exists"):
            manager.mark_feature_failed("subtraction", "error", "session-123")

    def test_get_resume_info_no_state(self, tmp_path: Path) -> None:
        """Test get_resume_info returns empty tuple when no state."""
        manager = CheckpointManager(tmp_path)
        session_id, completed, failed = manager.get_resume_info()

        assert session_id == ""
        assert completed == []
        assert failed is None

    def test_get_resume_info_with_progress(self, tmp_path: Path) -> None:
        """Test get_resume_info returns correct data."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        manager.mark_feature_completed("numbers", "session-1")
        manager.mark_feature_completed("addition", "session-2")
        manager.mark_feature_failed("subtraction", "error", "session-3")

        session_id, completed, failed = manager.get_resume_info()

        assert session_id == "session-3"
        assert completed == ["numbers", "addition"]
        assert failed == "subtraction"

    def test_can_resume_no_state(self, tmp_path: Path) -> None:
        """Test can_resume returns False when no state."""
        manager = CheckpointManager(tmp_path)
        assert not manager.can_resume()

    def test_can_resume_fresh_state(self, tmp_path: Path) -> None:
        """Test can_resume returns False for fresh state."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        assert not manager.can_resume()

    def test_can_resume_with_completed(self, tmp_path: Path) -> None:
        """Test can_resume returns True with completed features."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        manager.mark_feature_completed("numbers", "session-123")

        assert manager.can_resume()

    def test_can_resume_with_failed(self, tmp_path: Path) -> None:
        """Test can_resume returns True with failed feature."""
        manager = CheckpointManager(tmp_path)
        manager.create_initial("test", "rust", "feature-by-feature")
        manager.mark_feature_failed("numbers", "error", "session-123")

        assert manager.can_resume()

    def test_state_file_is_valid_json(self, tmp_path: Path) -> None:
        """Test that saved state file is valid JSON."""
        manager = CheckpointManager(tmp_path)
        state = CheckpointState(
            project="test",
            target="rust",
            strategy="feature-by-feature",
            completed_features=["a", "b"],
        )
        manager.save(state)

        content = manager.state_file.read_text()
        parsed = json.loads(content)  # Should not raise
        assert parsed["project"] == "test"
        assert parsed["completed_features"] == ["a", "b"]

"""Checkpoint management for resumable migrations.

Provides two resume strategies:
1. Session-based resume: Uses SDK's resume option to continue exact conversation
2. Feature-level resume: Tracks completed features for reconstruction after expiry
"""

import json
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path


@dataclass
class CheckpointState:
    """Persistent state for migration checkpoints.

    Attributes:
        project: Project name (e.g., "rpn2tex")
        target: Target language (rust, java, go)
        strategy: Migration strategy (feature-by-feature, module-by-module)
        session_id: Last successful Claude session ID for resume
        current_feature: Feature currently in progress (None if between features)
        completed_features: List of successfully completed feature names
        failed_feature: Feature that failed (None if no failure)
        error_message: Error message from last failure
        last_checkpoint_time: ISO timestamp of last checkpoint update
    """

    project: str
    target: str
    strategy: str
    session_id: str = ""
    current_feature: str | None = None
    completed_features: list[str] = field(default_factory=list)
    failed_feature: str | None = None
    error_message: str | None = None
    last_checkpoint_time: str = ""

    def to_dict(self) -> dict[str, str | list[str] | None]:
        """Serialize to dictionary for JSON storage."""
        return {
            "project": self.project,
            "target": self.target,
            "strategy": self.strategy,
            "session_id": self.session_id,
            "current_feature": self.current_feature,
            "completed_features": self.completed_features,
            "failed_feature": self.failed_feature,
            "error_message": self.error_message,
            "last_checkpoint_time": self.last_checkpoint_time,
        }

    @classmethod
    def from_dict(cls, data: dict[str, str | list[str] | None]) -> "CheckpointState":
        """Deserialize from dictionary."""
        completed = data.get("completed_features")
        if completed is None or not isinstance(completed, list):
            completed = []

        current = data.get("current_feature")
        current_feature: str | None = (
            str(current) if current and isinstance(current, str) else None
        )

        failed = data.get("failed_feature")
        failed_feature: str | None = (
            str(failed) if failed and isinstance(failed, str) else None
        )

        error = data.get("error_message")
        error_message: str | None = (
            str(error) if error and isinstance(error, str) else None
        )

        return cls(
            project=str(data.get("project", "")),
            target=str(data.get("target", "")),
            strategy=str(data.get("strategy", "")),
            session_id=str(data.get("session_id", "")),
            current_feature=current_feature,
            completed_features=[str(f) for f in completed],
            failed_feature=failed_feature,
            error_message=error_message,
            last_checkpoint_time=str(data.get("last_checkpoint_time", "")),
        )


class CheckpointManager:
    """Manages checkpoint state for a migration directory.

    Checkpoints are stored in .checkpoint/state.json within the migration directory.
    """

    CHECKPOINT_DIR = ".checkpoint"
    STATE_FILE = "state.json"

    def __init__(self, migration_dir: Path) -> None:
        """Initialize checkpoint manager.

        Args:
            migration_dir: Path to the migration output directory
        """
        self.migration_dir = migration_dir
        self.checkpoint_dir = migration_dir / self.CHECKPOINT_DIR
        self.state_file = self.checkpoint_dir / self.STATE_FILE

    def exists(self) -> bool:
        """Check if a checkpoint exists."""
        return self.state_file.exists()

    def load(self) -> CheckpointState | None:
        """Load checkpoint state from disk.

        Returns:
            CheckpointState if checkpoint exists, None otherwise
        """
        if not self.state_file.exists():
            return None

        content = self.state_file.read_text()
        data = json.loads(content)
        return CheckpointState.from_dict(data)

    def save(self, state: CheckpointState) -> None:
        """Save checkpoint state to disk.

        Args:
            state: CheckpointState to persist
        """
        self.checkpoint_dir.mkdir(parents=True, exist_ok=True)
        state.last_checkpoint_time = datetime.now(timezone.utc).isoformat()
        content = json.dumps(state.to_dict(), indent=2)
        self.state_file.write_text(content)

    def clear(self) -> None:
        """Remove checkpoint state."""
        if self.state_file.exists():
            self.state_file.unlink()

    def create_initial(
        self,
        project: str,
        target: str,
        strategy: str,
    ) -> CheckpointState:
        """Create and save initial checkpoint state.

        Args:
            project: Project name
            target: Target language
            strategy: Migration strategy

        Returns:
            New CheckpointState
        """
        state = CheckpointState(
            project=project,
            target=target,
            strategy=strategy,
        )
        self.save(state)
        return state

    def mark_feature_started(self, feature: str, session_id: str) -> None:
        """Mark a feature as started.

        Args:
            feature: Feature name being started
            session_id: Current session ID
        """
        state = self.load()
        if state is None:
            raise ValueError("No checkpoint state exists")

        state.current_feature = feature
        state.session_id = session_id
        state.failed_feature = None
        state.error_message = None
        self.save(state)

    def mark_feature_completed(self, feature: str, session_id: str) -> None:
        """Mark a feature as successfully completed.

        Args:
            feature: Feature name that completed
            session_id: Session ID to save for potential resume
        """
        state = self.load()
        if state is None:
            raise ValueError("No checkpoint state exists")

        if feature not in state.completed_features:
            state.completed_features.append(feature)
        state.current_feature = None
        state.session_id = session_id
        state.failed_feature = None
        state.error_message = None
        self.save(state)

    def mark_feature_failed(self, feature: str, error: str, session_id: str) -> None:
        """Mark a feature as failed.

        Args:
            feature: Feature name that failed
            error: Error message
            session_id: Session ID for potential resume attempt
        """
        state = self.load()
        if state is None:
            raise ValueError("No checkpoint state exists")

        state.current_feature = None
        state.failed_feature = feature
        state.error_message = error
        state.session_id = session_id
        self.save(state)

    def get_resume_info(self) -> tuple[str, list[str], str | None]:
        """Get information needed to resume a migration.

        Returns:
            Tuple of (session_id, completed_features, failed_feature)
        """
        state = self.load()
        if state is None:
            return ("", [], None)

        return (state.session_id, state.completed_features, state.failed_feature)

    def can_resume(self) -> bool:
        """Check if migration can be resumed.

        Returns:
            True if there's a valid session_id and either completed features
            or a failed feature to resume from
        """
        state = self.load()
        if state is None:
            return False

        has_session = bool(state.session_id)
        has_progress = bool(state.completed_features) or bool(state.failed_feature)
        return has_session and has_progress

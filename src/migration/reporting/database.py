"""SQLite database for migration metrics aggregation.

This module provides storage and querying capabilities for
analyzing migration metrics across many runs.
"""

import sqlite3
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path
from typing import Iterator, Optional

from .schema import MigrationMetrics


DEFAULT_DB_PATH = Path("migrations.db")


@dataclass
class AggregateStats:
    """Aggregated statistics across multiple migrations."""
    count: int
    avg_duration_ms: float
    median_duration_ms: float
    p95_duration_ms: float
    total_cost_usd: float
    avg_cost_usd: float
    avg_io_match_rate: float
    success_rate_pct: float
    avg_loc_expansion: float
    avg_coverage_pct: Optional[float] = None


class MigrationDatabase:
    """SQLite database for storing and querying migration metrics.

    Usage:
        db = MigrationDatabase("migrations.db")
        db.insert(metrics)

        # Query
        results = db.query(project="rpn2tex", target="rust")
        stats = db.aggregate(strategy="feature-by-feature")

        # Export
        db.export_summary("summary.json")
    """

    SCHEMA = """
    CREATE TABLE IF NOT EXISTS migrations (
        run_id TEXT PRIMARY KEY,
        project_name TEXT NOT NULL,
        source_language TEXT NOT NULL,
        target_language TEXT NOT NULL,
        strategy TEXT NOT NULL,
        started_at DATETIME NOT NULL,
        completed_at DATETIME,
        duration_ms INTEGER,
        cost_usd REAL,
        io_match_rate REAL,
        line_coverage_pct REAL,
        status TEXT,
        source_loc INTEGER,
        target_loc INTEGER,
        metrics_json TEXT NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

    CREATE INDEX IF NOT EXISTS idx_project ON migrations(project_name);
    CREATE INDEX IF NOT EXISTS idx_target ON migrations(target_language);
    CREATE INDEX IF NOT EXISTS idx_strategy ON migrations(strategy);
    CREATE INDEX IF NOT EXISTS idx_started ON migrations(started_at);
    CREATE INDEX IF NOT EXISTS idx_status ON migrations(status);
    """

    MIGRATION_V2 = """
    ALTER TABLE migrations ADD COLUMN line_coverage_pct REAL;
    """

    def __init__(self, db_path: Path = DEFAULT_DB_PATH):
        self.db_path = db_path
        self._init_db()

    def _init_db(self) -> None:
        """Initialize database schema."""
        with self._connect() as conn:
            conn.executescript(self.SCHEMA)
            # Run migrations for existing databases
            self._run_migrations(conn)

    def _run_migrations(self, conn: sqlite3.Connection) -> None:
        """Run schema migrations for existing databases."""
        # Check if line_coverage_pct column exists
        cursor = conn.execute("PRAGMA table_info(migrations)")
        columns = [row[1] for row in cursor.fetchall()]
        if "line_coverage_pct" not in columns:
            try:
                conn.execute(self.MIGRATION_V2)
            except sqlite3.OperationalError:
                pass  # Column already exists

    @contextmanager
    def _connect(self) -> Iterator[sqlite3.Connection]:
        """Context manager for database connections."""
        conn = sqlite3.connect(self.db_path)
        conn.row_factory = sqlite3.Row
        try:
            yield conn
            conn.commit()
        finally:
            conn.close()

    def insert(self, metrics: MigrationMetrics) -> None:
        """Insert a migration run into the database."""
        with self._connect() as conn:
            conn.execute(
                """
                INSERT OR REPLACE INTO migrations (
                    run_id, project_name, source_language, target_language,
                    strategy, started_at, completed_at, duration_ms, cost_usd,
                    io_match_rate, line_coverage_pct, status, source_loc, target_loc, metrics_json
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                """,
                (
                    metrics.identity.run_id,
                    metrics.identity.project_name,
                    metrics.identity.source_language,
                    metrics.identity.target_language,
                    metrics.identity.strategy,
                    metrics.identity.started_at,
                    metrics.identity.completed_at,
                    metrics.timing.wall_clock_duration_ms,
                    metrics.cost.total_cost_usd,
                    metrics.io_contract.match_rate_pct,
                    metrics.quality_gates.coverage.line_coverage_pct,
                    metrics.outcome.status,
                    metrics.source_metrics.production_loc,
                    metrics.target_metrics.production_loc,
                    metrics.to_json(),
                ),
            )

    def get(self, run_id: str) -> Optional[MigrationMetrics]:
        """Get a single migration by run_id."""
        with self._connect() as conn:
            row = conn.execute(
                "SELECT metrics_json FROM migrations WHERE run_id = ?",
                (run_id,),
            ).fetchone()
            if row:
                return MigrationMetrics.from_json(row["metrics_json"])
        return None

    def query(
        self,
        project: Optional[str] = None,
        target: Optional[str] = None,
        strategy: Optional[str] = None,
        status: Optional[str] = None,
        since: Optional[datetime] = None,
        until: Optional[datetime] = None,
        limit: int = 100,
    ) -> list[MigrationMetrics]:
        """Query migrations with filters."""
        conditions = []
        params = []

        if project:
            conditions.append("project_name = ?")
            params.append(project)
        if target:
            conditions.append("target_language = ?")
            params.append(target)
        if strategy:
            conditions.append("strategy = ?")
            params.append(strategy)
        if status:
            conditions.append("status = ?")
            params.append(status)
        if since:
            conditions.append("started_at >= ?")
            params.append(since.isoformat())
        if until:
            conditions.append("started_at <= ?")
            params.append(until.isoformat())

        where_clause = " AND ".join(conditions) if conditions else "1=1"
        params.append(limit)

        with self._connect() as conn:
            rows = conn.execute(
                f"""
                SELECT metrics_json FROM migrations
                WHERE {where_clause}
                ORDER BY started_at DESC
                LIMIT ?
                """,
                params,
            ).fetchall()

        return [MigrationMetrics.from_json(row["metrics_json"]) for row in rows]

    def aggregate(
        self,
        project: Optional[str] = None,
        target: Optional[str] = None,
        strategy: Optional[str] = None,
    ) -> Optional[AggregateStats]:
        """Compute aggregate statistics for filtered migrations."""
        conditions = []
        params = []

        if project:
            conditions.append("project_name = ?")
            params.append(project)
        if target:
            conditions.append("target_language = ?")
            params.append(target)
        if strategy:
            conditions.append("strategy = ?")
            params.append(strategy)

        where_clause = " AND ".join(conditions) if conditions else "1=1"

        with self._connect() as conn:
            # Basic aggregates
            row = conn.execute(
                f"""
                SELECT
                    COUNT(*) as count,
                    AVG(duration_ms) as avg_duration_ms,
                    SUM(cost_usd) as total_cost_usd,
                    AVG(cost_usd) as avg_cost_usd,
                    AVG(io_match_rate) as avg_io_match_rate,
                    AVG(line_coverage_pct) as avg_coverage_pct,
                    AVG(CASE WHEN status = 'success' THEN 1.0 ELSE 0.0 END) * 100 as success_rate_pct,
                    AVG(CAST(target_loc AS REAL) / NULLIF(source_loc, 0)) as avg_loc_expansion
                FROM migrations
                WHERE {where_clause}
                """,
                params,
            ).fetchone()

            if not row or row["count"] == 0:
                return None

            # Median and p95 require window functions or sorting
            durations = conn.execute(
                f"""
                SELECT duration_ms FROM migrations
                WHERE {where_clause} AND duration_ms IS NOT NULL
                ORDER BY duration_ms
                """,
                params,
            ).fetchall()

            duration_values = [d["duration_ms"] for d in durations]
            median = self._percentile(duration_values, 50)
            p95 = self._percentile(duration_values, 95)

            return AggregateStats(
                count=row["count"],
                avg_duration_ms=row["avg_duration_ms"] or 0,
                median_duration_ms=median,
                p95_duration_ms=p95,
                total_cost_usd=row["total_cost_usd"] or 0,
                avg_cost_usd=row["avg_cost_usd"] or 0,
                avg_io_match_rate=row["avg_io_match_rate"] or 0,
                success_rate_pct=row["success_rate_pct"] or 0,
                avg_loc_expansion=row["avg_loc_expansion"] or 0,
                avg_coverage_pct=row["avg_coverage_pct"],
            )

    def _percentile(self, values: list[int], percentile: int) -> float:
        """Calculate percentile from sorted list."""
        if not values:
            return 0.0
        k = (len(values) - 1) * percentile / 100
        f = int(k)
        c = f + 1 if f + 1 < len(values) else f
        return values[f] + (values[c] - values[f]) * (k - f)

    def group_by(
        self,
        group_field: str,
    ) -> dict[str, AggregateStats]:
        """Get aggregate stats grouped by a field."""
        valid_fields = ["project_name", "target_language", "strategy"]
        if group_field not in valid_fields:
            raise ValueError(f"Invalid group field. Must be one of: {valid_fields}")

        results = {}
        with self._connect() as conn:
            groups = conn.execute(
                f"SELECT DISTINCT {group_field} FROM migrations"
            ).fetchall()

            for group in groups:
                value = group[0]
                kwargs = {group_field.replace("_language", "").replace("_name", ""): value}
                stats = self.aggregate(**kwargs)
                if stats:
                    results[value] = stats

        return results

    def list_projects(self) -> list[str]:
        """List all unique project names."""
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT DISTINCT project_name FROM migrations ORDER BY project_name"
            ).fetchall()
            return [row[0] for row in rows]

    def list_targets(self) -> list[str]:
        """List all unique target languages."""
        with self._connect() as conn:
            rows = conn.execute(
                "SELECT DISTINCT target_language FROM migrations ORDER BY target_language"
            ).fetchall()
            return [row[0] for row in rows]

    def count(self) -> int:
        """Count total migrations in database."""
        with self._connect() as conn:
            row = conn.execute("SELECT COUNT(*) FROM migrations").fetchone()
            return row[0]

    def delete(self, run_id: str) -> bool:
        """Delete a migration by run_id."""
        with self._connect() as conn:
            cursor = conn.execute(
                "DELETE FROM migrations WHERE run_id = ?",
                (run_id,),
            )
            return cursor.rowcount > 0

    def export_to_json(self, output_path: Path) -> None:
        """Export all migrations to a JSON file."""
        import json

        metrics_list = self.query(limit=10000)
        data = {
            "exported_at": datetime.now().isoformat(),
            "count": len(metrics_list),
            "migrations": [m.to_dict() for m in metrics_list],
        }
        with open(output_path, "w") as f:
            json.dump(data, f, indent=2)

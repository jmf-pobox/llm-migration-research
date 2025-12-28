"""Command-line interface for migration reporting.

Usage:
    python -m reporting analyze <migration_dir>
    python -m reporting report <metrics_json> [--format markdown|latex]
    python -m reporting query [--project NAME] [--target LANG] [--strategy STRAT]
    python -m reporting export [--since DATE] [--format json|markdown|latex]
    python -m reporting backfill <log_file> [--project NAME]
"""

import argparse
import json
import sys
from datetime import datetime
from pathlib import Path
from typing import Optional

from .schema import MigrationMetrics
from .collector import LogParser
from .analyzer import PostHocAnalyzer
from .database import MigrationDatabase
from .generator import ReportGenerator


DEFAULT_DB_PATH = Path("migrations.db")


def cmd_analyze(args: argparse.Namespace) -> int:
    """Analyze a completed migration directory."""
    migration_dir = Path(args.migration_dir)

    if not migration_dir.exists():
        print(f"Error: Directory not found: {migration_dir}", file=sys.stderr)
        return 1

    # Look for log files
    log_dir = migration_dir / "logs"
    if not log_dir.exists():
        log_dir = migration_dir  # Try migration_dir itself

    log_files = list(log_dir.glob("*.log")) + list(log_dir.glob("*.txt"))
    if not log_files:
        print(f"Error: No log files found in {log_dir}", file=sys.stderr)
        return 1

    # Use the most recent log file
    log_file = max(log_files, key=lambda p: p.stat().st_mtime)
    print(f"Analyzing: {log_file}")

    # Parse log file
    parser = LogParser()
    metrics = parser.parse_log(log_file)

    # Run post-hoc analysis if source/target dirs exist
    analyzer = PostHocAnalyzer()

    source_dir = migration_dir.parent.parent / "source"
    if source_dir.exists():
        print(f"Analyzing source: {source_dir}")
        source_metrics = analyzer.analyze_python_source(source_dir)
        metrics.source_metrics = source_metrics

    target_dir = migration_dir / "src"
    if target_dir.exists():
        print(f"Analyzing target: {target_dir}")
        target_metrics = analyzer.analyze_rust_target(target_dir.parent)
        metrics.target_metrics = target_metrics

        quality = analyzer.analyze_rust_quality(target_dir.parent)
        metrics.quality_gates = quality

    # Save metrics
    metrics_dir = migration_dir / "metrics"
    metrics_dir.mkdir(exist_ok=True)

    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    metrics_file = metrics_dir / f"run_{timestamp}.json"

    with open(metrics_file, "w") as f:
        f.write(metrics.to_json())

    print(f"Metrics saved: {metrics_file}")

    # Insert into database
    db = MigrationDatabase(args.database)
    db.insert(metrics)
    print(f"Inserted into database: {args.database}")

    # Generate report
    generator = ReportGenerator()
    report = generator.generate_run_report(metrics)

    reports_dir = migration_dir / "reports"
    reports_dir.mkdir(exist_ok=True)

    report_file = reports_dir / f"run_{timestamp}.md"
    with open(report_file, "w") as f:
        f.write(report)

    print(f"Report saved: {report_file}")

    return 0


def cmd_report(args: argparse.Namespace) -> int:
    """Generate a report from metrics JSON."""
    metrics_file = Path(args.metrics_json)

    if not metrics_file.exists():
        print(f"Error: File not found: {metrics_file}", file=sys.stderr)
        return 1

    with open(metrics_file) as f:
        metrics = MigrationMetrics.from_json(f.read())

    generator = ReportGenerator()

    if args.format == "latex":
        output = generator.generate_latex_table([metrics])
    else:
        output = generator.generate_run_report(metrics)

    if args.output:
        with open(args.output, "w") as f:
            f.write(output)
        print(f"Report saved: {args.output}")
    else:
        print(output)

    return 0


def cmd_query(args: argparse.Namespace) -> int:
    """Query the migrations database."""
    db = MigrationDatabase(args.database)

    results = db.query(
        project=args.project,
        target=args.target,
        strategy=args.strategy,
        status=args.status,
        limit=args.limit,
    )

    if not results:
        print("No matching migrations found.")
        return 0

    print(f"Found {len(results)} migration(s):\n")

    for m in results:
        status_icon = "✓" if m.outcome.status == "success" else "✗"
        duration = f"{m.timing.wall_clock_duration_ms / 60000:.1f}min"
        print(f"  {status_icon} {m.identity.run_id[:8]}  "
              f"{m.identity.project_name} → {m.identity.target_language} "
              f"({m.identity.strategy})  "
              f"{duration}  ${m.cost.total_cost_usd:.2f}  "
              f"I/O: {m.io_contract.match_rate_pct:.0f}%")

    # Show aggregates if multiple results
    if len(results) > 1:
        stats = db.aggregate(
            project=args.project,
            target=args.target,
            strategy=args.strategy,
        )
        if stats:
            print(f"\nAggregate Statistics:")
            print(f"  Count: {stats.count}")
            print(f"  Avg Duration: {stats.avg_duration_ms / 60000:.1f}min")
            print(f"  Avg Cost: ${stats.avg_cost_usd:.2f}")
            print(f"  Success Rate: {stats.success_rate_pct:.0f}%")
            print(f"  Avg I/O Match: {stats.avg_io_match_rate:.0f}%")

    return 0


def cmd_export(args: argparse.Namespace) -> int:
    """Export database to various formats."""
    db = MigrationDatabase(args.database)

    if args.format == "json":
        output_path = Path(args.output or "migrations_export.json")
        db.export_to_json(output_path)
        print(f"Exported to: {output_path}")
        return 0

    # For markdown/latex, generate summary
    generator = ReportGenerator()

    if args.format == "latex":
        results = db.query(limit=100)
        if not results:
            print("No migrations to export.")
            return 1
        output = generator.generate_latex_table(results)
    else:
        output = generator.generate_summary(db)

    if args.output:
        with open(args.output, "w") as f:
            f.write(output)
        print(f"Exported to: {args.output}")
    else:
        print(output)

    return 0


def cmd_compare(args: argparse.Namespace) -> int:
    """Compare multiple migration runs."""
    db = MigrationDatabase(args.database)

    runs = []
    for run_id in args.run_ids:
        metrics = db.get(run_id)
        if metrics:
            runs.append(metrics)
        else:
            print(f"Warning: Run not found: {run_id}", file=sys.stderr)

    if len(runs) < 2:
        print("Error: Need at least 2 runs to compare.", file=sys.stderr)
        return 1

    generator = ReportGenerator()

    if args.format == "latex":
        output = generator.generate_latex_table(runs, caption=args.title)
    else:
        output = generator.generate_comparison(runs, title=args.title)

    if args.output:
        with open(args.output, "w") as f:
            f.write(output)
        print(f"Comparison saved: {args.output}")
    else:
        print(output)

    return 0


def cmd_backfill(args: argparse.Namespace) -> int:
    """Backfill metrics from an existing log file."""
    log_file = Path(args.log_file)

    if not log_file.exists():
        print(f"Error: File not found: {log_file}", file=sys.stderr)
        return 1

    print(f"Parsing: {log_file}")

    parser = LogParser()
    metrics = parser.parse_log(log_file)

    # Override project name if specified
    if args.project:
        metrics.identity.project_name = args.project

    # Override strategy if specified
    if args.strategy:
        metrics.identity.strategy = args.strategy

    # Save metrics JSON
    if args.metrics_output:
        output_path = Path(args.metrics_output)
    else:
        output_path = log_file.parent / f"metrics_{log_file.stem}.json"

    with open(output_path, "w") as f:
        f.write(metrics.to_json())

    print(f"Metrics saved: {output_path}")

    # Insert into database
    db = MigrationDatabase(args.database)
    db.insert(metrics)
    print(f"Inserted into database: {args.database}")

    # Print summary
    print(f"\nBackfill Summary:")
    print(f"  Run ID: {metrics.identity.run_id}")
    print(f"  Project: {metrics.identity.project_name}")
    print(f"  Strategy: {metrics.identity.strategy}")
    print(f"  Duration: {metrics.timing.wall_clock_duration_ms / 60000:.1f}min")
    print(f"  Cost: ${metrics.cost.total_cost_usd:.2f}")
    print(f"  I/O Match: {metrics.io_contract.match_rate_pct:.0f}%")

    return 0


def cmd_stats(args: argparse.Namespace) -> int:
    """Show database statistics."""
    db = MigrationDatabase(args.database)

    total = db.count()
    if total == 0:
        print("Database is empty.")
        return 0

    print(f"Total Migrations: {total}\n")

    # By target language
    by_target = db.group_by("target_language")
    if by_target:
        print("By Target Language:")
        for target, stats in by_target.items():
            print(f"  {target}: {stats.count} runs, "
                  f"avg ${stats.avg_cost_usd:.2f}, "
                  f"{stats.success_rate_pct:.0f}% success")

    # By strategy
    by_strategy = db.group_by("strategy")
    if by_strategy:
        print("\nBy Strategy:")
        for strat, stats in by_strategy.items():
            print(f"  {strat}: {stats.count} runs, "
                  f"avg ${stats.avg_cost_usd:.2f}, "
                  f"{stats.success_rate_pct:.0f}% success")

    # Projects
    projects = db.list_projects()
    if projects:
        print(f"\nProjects: {', '.join(projects)}")

    return 0


def main() -> int:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Migration reporting tools",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--database", "-d",
        type=Path,
        default=DEFAULT_DB_PATH,
        help="Path to SQLite database (default: migrations.db)",
    )

    subparsers = parser.add_subparsers(dest="command", required=True)

    # analyze command
    p_analyze = subparsers.add_parser(
        "analyze",
        help="Analyze a completed migration directory",
    )
    p_analyze.add_argument("migration_dir", help="Path to migration directory")
    p_analyze.set_defaults(func=cmd_analyze)

    # report command
    p_report = subparsers.add_parser(
        "report",
        help="Generate a report from metrics JSON",
    )
    p_report.add_argument("metrics_json", help="Path to metrics JSON file")
    p_report.add_argument(
        "--format", "-f",
        choices=["markdown", "latex"],
        default="markdown",
        help="Output format (default: markdown)",
    )
    p_report.add_argument(
        "--output", "-o",
        help="Output file path (default: stdout)",
    )
    p_report.set_defaults(func=cmd_report)

    # query command
    p_query = subparsers.add_parser(
        "query",
        help="Query the migrations database",
    )
    p_query.add_argument("--project", "-p", help="Filter by project name")
    p_query.add_argument("--target", "-t", help="Filter by target language")
    p_query.add_argument("--strategy", "-s", help="Filter by strategy")
    p_query.add_argument("--status", help="Filter by status (success/failure)")
    p_query.add_argument(
        "--limit", "-n",
        type=int,
        default=20,
        help="Maximum results (default: 20)",
    )
    p_query.set_defaults(func=cmd_query)

    # export command
    p_export = subparsers.add_parser(
        "export",
        help="Export database to various formats",
    )
    p_export.add_argument(
        "--format", "-f",
        choices=["json", "markdown", "latex"],
        default="markdown",
        help="Output format (default: markdown)",
    )
    p_export.add_argument(
        "--output", "-o",
        help="Output file path (default: stdout for markdown/latex)",
    )
    p_export.set_defaults(func=cmd_export)

    # compare command
    p_compare = subparsers.add_parser(
        "compare",
        help="Compare multiple migration runs",
    )
    p_compare.add_argument(
        "run_ids",
        nargs="+",
        help="Run IDs to compare",
    )
    p_compare.add_argument(
        "--format", "-f",
        choices=["markdown", "latex"],
        default="markdown",
        help="Output format (default: markdown)",
    )
    p_compare.add_argument("--title", help="Report title")
    p_compare.add_argument(
        "--output", "-o",
        help="Output file path (default: stdout)",
    )
    p_compare.set_defaults(func=cmd_compare)

    # backfill command
    p_backfill = subparsers.add_parser(
        "backfill",
        help="Backfill metrics from an existing log file",
    )
    p_backfill.add_argument("log_file", help="Path to log file")
    p_backfill.add_argument("--project", "-p", help="Project name override")
    p_backfill.add_argument("--strategy", "-s", help="Strategy override")
    p_backfill.add_argument(
        "--metrics-output", "-m",
        help="Metrics JSON output path",
    )
    p_backfill.set_defaults(func=cmd_backfill)

    # stats command
    p_stats = subparsers.add_parser(
        "stats",
        help="Show database statistics",
    )
    p_stats.set_defaults(func=cmd_stats)

    args = parser.parse_args()
    return args.func(args)


if __name__ == "__main__":
    sys.exit(main())

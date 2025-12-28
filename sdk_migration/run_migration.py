#!/usr/bin/env python3
"""
Claude Agent SDK Migration Runner - Multi-Language Version

Uses a 4-phase approach to ensure behavioral equivalence:
- Phase 0: I/O contract generator runs source implementation, captures outputs
- Phase 1: Analyst reads ALL source files ONCE, includes I/O contract in spec
- Phase 2: For each module, migrator uses spec and validates I/O contract
- Phase 3: For each module, reviewer validates against spec + I/O contract

Usage:
    python run_migration.py --target rust --project projects/rpn2tex.yaml
    python run_migration.py --target java --project projects/rpn2tex.yaml
    python run_migration.py --target rust --project projects/rpn2tex.yaml --dry-run
"""

import argparse
import asyncio
import os
import sys
from pathlib import Path

# Check for required environment variable
if not os.environ.get("ANTHROPIC_API_KEY"):
    print("Error: ANTHROPIC_API_KEY environment variable not set")
    print("Set it with: export ANTHROPIC_API_KEY=your-api-key")
    sys.exit(1)

# Add parent directory to path for imports
sys.path.insert(0, str(Path(__file__).parent))

from framework import load_project_config, run_migration
from languages import get_language_target, LANGUAGE_REGISTRY


def list_languages() -> None:
    """Print available target languages."""
    print("Available target languages:")
    for name in sorted(LANGUAGE_REGISTRY.keys()):
        target = LANGUAGE_REGISTRY[name]()
        print(f"  {name}: {target.file_extension} files")


def list_projects() -> None:
    """Print available project configurations."""
    projects_dir = Path(__file__).parent / "projects"
    if not projects_dir.exists():
        print("No projects directory found.")
        return

    yaml_files = list(projects_dir.glob("*.yaml")) + list(projects_dir.glob("*.yml"))
    if not yaml_files:
        print("No project configurations found in projects/")
        return

    print("Available project configurations:")
    for path in sorted(yaml_files):
        print(f"  projects/{path.name}")


async def main_async(target: str, project: str, dry_run: bool, base_dir: str) -> None:
    """Async main entry point."""
    # Load configuration
    config = load_project_config(project)
    lang_target = get_language_target(target)

    print(f"Migration: {config.name} ({config.source_language}) -> {lang_target.name}")
    print(f"Project config: {project}")
    print(f"Output directory: {base_dir}")
    print()

    # Run migration
    await run_migration(
        config=config,
        target=lang_target,
        base_dir=base_dir,
        dry_run=dry_run,
    )


def main() -> None:
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Migrate code to target language using Claude Agent SDK",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Migrate rpn2tex to Rust
  python run_migration.py --target rust --project projects/rpn2tex.yaml

  # Migrate rpn2tex to Java
  python run_migration.py --target java --project projects/rpn2tex.yaml

  # Dry run (show what would happen)
  python run_migration.py --target rust --project projects/rpn2tex.yaml --dry-run

  # List available targets and projects
  python run_migration.py --list-targets
  python run_migration.py --list-projects
""",
    )

    parser.add_argument(
        "--target",
        "-t",
        choices=list(LANGUAGE_REGISTRY.keys()),
        help="Target language for migration",
    )
    parser.add_argument(
        "--project",
        "-p",
        type=str,
        help="Path to project configuration YAML file",
    )
    parser.add_argument(
        "--base-dir",
        "-b",
        type=str,
        default=str(Path(__file__).parent),
        help="Base directory for output (default: sdk_migration/)",
    )
    parser.add_argument(
        "--dry-run",
        "-n",
        action="store_true",
        help="Show what would be done without executing",
    )
    parser.add_argument(
        "--list-targets",
        action="store_true",
        help="List available target languages",
    )
    parser.add_argument(
        "--list-projects",
        action="store_true",
        help="List available project configurations",
    )

    args = parser.parse_args()

    # Handle list commands
    if args.list_targets:
        list_languages()
        return

    if args.list_projects:
        list_projects()
        return

    # Validate required arguments for migration
    if not args.target:
        parser.error("--target is required (use --list-targets to see options)")

    if not args.project:
        parser.error("--project is required (use --list-projects to see options)")

    # Validate project file exists
    project_path = Path(args.project)
    if not project_path.exists():
        # Try relative to script directory
        alt_path = Path(__file__).parent / args.project
        if alt_path.exists():
            project_path = alt_path
        else:
            print(f"Error: Project file not found: {args.project}")
            sys.exit(1)

    # Run migration
    asyncio.run(main_async(
        target=args.target,
        project=str(project_path),
        dry_run=args.dry_run,
        base_dir=args.base_dir,
    ))


if __name__ == "__main__":
    main()

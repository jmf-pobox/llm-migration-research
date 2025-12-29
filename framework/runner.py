"""Core migration orchestration logic."""

import sys
from datetime import datetime
from pathlib import Path
from typing import Any, Optional, TYPE_CHECKING

from languages.base import LanguageTarget
from framework.agents import build_agents
from framework.config import ProjectConfig

if TYPE_CHECKING:
    from strategies.base import MigrationStrategy
    from reporting.collector import MetricsCollector
    from reporting.database import MigrationDatabase


def log(msg: str, log_file: Path | None = None, also_print: bool = True) -> None:
    """Log a message with timestamp."""
    timestamp = datetime.now().strftime("%H:%M:%S")
    formatted = f"[{timestamp}] {msg}"

    if log_file:
        with open(log_file, "a") as f:
            f.write(formatted + "\n")
            f.flush()

    if also_print:
        print(formatted)
        sys.stdout.flush()


def build_migration_prompt(
    config: ProjectConfig, target: LanguageTarget, project_dir: str
) -> str:
    """Build the multi-phase migration prompt."""
    modules_list = "\n".join(
        f"    {i+1}. {m.source} -> {target.get_file_mapping(m.source)} ({m.phase} phase)"
        for i, m in enumerate(config.modules)
    )

    source_files_list = "\n".join(
        f"    - {f}: {config.source_directory}/{f}" for f in config.source_files
    )

    target_dir = target.get_source_dir(project_dir)
    target_files_list = "\n".join(
        f"    - {target.get_file_mapping(f)}: {target_dir}/{target.get_file_mapping(f)}"
        for f in config.source_files
    )

    test_inputs_list = "\n".join(f'    - "{inp}"' for inp in config.test_inputs)

    quality_gates = "\n".join(
        f"- `{cmd}`" for cmd in target.get_quality_gates()
    )

    return f"""
Migrate the {config.name} {config.source_language.title()} codebase to {target.name.title()} using a multi-phase approach with I/O validation.

## IMPORTANT: Multi-Phase Orchestration with I/O Contract

This migration uses FOUR distinct phases to ensure behavioral equivalence:

### PHASE 0: I/O Contract Generation (DO THIS FIRST)

Spawn the **io_contract** agent ONCE to:
1. Run the {config.source_language.title()} implementation on curated test inputs
2. Capture EXACT outputs for each input
3. Produce an I/O contract document

Test inputs to run:
{test_inputs_list}

SAVE the I/O contract - you will include it in the migration spec.

### PHASE 1: Comprehensive Analysis

Spawn the **analyst** agent ONCE to analyze ALL {config.source_language.title()} modules:
- The analyst will read all source files
- It will produce a comprehensive migration specification
- INCLUDE the I/O contract from Phase 0 in the spec
- This spec guides all subsequent phases

### PHASE 2: Sequential Migration

For EACH module in dependency order:
1. Spawn the **migrator** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The specific module to migrate
2. Migrator MUST validate outputs match I/O contract
3. Migrator should NOT read source files directly - use the spec instead

### PHASE 3: Sequential Review

For EACH module, AFTER its migration completes:
1. Spawn the **reviewer** agent with:
   - The relevant section of the migration spec (including I/O contract)
   - The {target.name.title()} file path to review
2. Reviewer MUST verify I/O contract compliance

## Source Files (analyst will read these in Phase 1):
{source_files_list}

## Target Files (migrators write to these exact paths):
{target_files_list}

## Project Directory:
    {project_dir}

## Migration Order (respect dependencies):
{modules_list}

## Quality Gates (MUST pass before next module):
{quality_gates}
- **I/O contract validation** - outputs must match {config.source_language.title()} EXACTLY

## Key Points:
1. Phase 0 generates I/O contract from {config.source_language.title()} - critical for validation
2. Phase 1 reads source files ONCE and includes I/O contract in spec
3. Migrators validate against I/O contract
4. I/O contract violations are BLOCKERS - fix before proceeding
5. Run all build commands from: {project_dir}

Begin with Phase 0: spawn the io_contract agent to generate the I/O contract.
"""


async def run_migration(
    config: ProjectConfig,
    target: LanguageTarget,
    base_dir: str,
    dry_run: bool = False,
    strategy: "MigrationStrategy | None" = None,
    collect_metrics: bool = True,
) -> Optional["MetricsCollector"]:
    """Run the migration using Claude Agent SDK.

    Args:
        config: Project configuration
        target: Target language configuration
        base_dir: Base directory for output (project will be created as subdirectory)
        dry_run: If True, only print what would be done
        strategy: Migration strategy to use (defaults to module-by-module)
        collect_metrics: If True, collect metrics during migration

    Returns:
        MetricsCollector with migration metrics if collect_metrics is True, else None
    """
    # Import here to avoid import errors if SDK not installed
    try:
        from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition
    except ImportError:
        print("Error: claude-agent-sdk not installed")
        print("Install with: pip install claude-agent-sdk")
        sys.exit(1)

    # Determine migration directory
    # Structure: {base_dir}/projects/{project_name}/migrations/{target}-{strategy}-{run_number}/
    strategy_name = strategy.name if strategy else "module-by-module"
    migration_base = f"{target.name}-{strategy_name}"

    # Find next available run number
    migrations_dir = Path(base_dir) / "projects" / config.name / "migrations"
    migrations_dir.mkdir(parents=True, exist_ok=True)

    existing_runs = [
        d.name for d in migrations_dir.iterdir()
        if d.is_dir() and d.name.startswith(migration_base)
    ]

    # Extract run numbers from existing directories
    run_numbers = []
    for name in existing_runs:
        # Check for suffix like "-1", "-2", etc.
        if name == migration_base:
            run_numbers.append(0)  # Original unnumbered directory
        elif name.startswith(f"{migration_base}-"):
            try:
                suffix = name[len(migration_base) + 1:]
                run_numbers.append(int(suffix))
            except ValueError:
                pass

    next_run = max(run_numbers, default=0) + 1
    migration_name = f"{migration_base}-{next_run}"
    project_dir = f"{base_dir}/projects/{config.name}/migrations/{migration_name}"

    # Build prompt using strategy (or default to built-in prompt)
    if strategy:
        prompt = strategy.get_prompt(config, target, project_dir)
    else:
        prompt = build_migration_prompt(config, target, project_dir)
    agents_config = build_agents(config, target, project_dir)

    if dry_run:
        print("=" * 60)
        print(f"DRY RUN - Strategy: {strategy_name}")
        print("=" * 60)
        print(prompt)
        print("=" * 60)
        print("\nAgents configured:")
        for name, agent in agents_config.items():
            print(f"  - {name}: {agent['description'][:60]}...")
        print("\nTarget language:", target.name)
        print("Migration directory:", project_dir)
        print("Logs directory:", f"{project_dir}/logs/")
        return None

    # Initialize metrics collector
    collector = None
    if collect_metrics:
        try:
            from reporting.collector import MetricsCollector
            collector = MetricsCollector(
                project_name=config.name,
                source_language=config.source_language,
                target_language=target.name,
                strategy=strategy_name,
            )
            collector.start_phase("setup")
        except ImportError:
            log("Warning: reporting module not available, metrics collection disabled", None)

    # Setup logging - logs go in the migration's logs directory
    log_dir = Path(project_dir) / "logs"
    log_dir.mkdir(parents=True, exist_ok=True)
    log_file = log_dir / f"migration_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"

    log("=" * 60, log_file)
    log(f"Starting Migration: {config.name} -> {target.name.title()}", log_file)
    log(f"Strategy: {strategy_name}", log_file)
    log("=" * 60, log_file)
    log(f"Source: {config.source_directory}", log_file)
    log(f"Target: {project_dir}", log_file)
    log(f"Modules: {len(config.modules)}", log_file)
    log(f"Log file: {log_file}", log_file)
    log("=" * 60, log_file)

    # Create AgentDefinition objects
    agents = {
        name: AgentDefinition(
            description=agent["description"],
            prompt=agent["prompt"],
            tools=agent["tools"],
            model=agent["model"],
        )
        for name, agent in agents_config.items()
    }

    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Edit", "Bash", "Glob", "Grep", "Task"],
        agents=agents,
        permission_mode="acceptEdits",
    )

    if collector:
        collector.end_phase("setup")
        collector.start_phase("migration")

    message_count = 0
    migration_status = "success"
    result_message = None
    try:
        async for message in query(prompt=prompt, options=options):
            message_count += 1
            _log_message(message, message_count, log_file)

            # Record message for metrics
            if collector:
                collector.record_message()

            # Record tool uses for metrics - check content for ToolUseBlocks
            if collector and hasattr(message, "content"):
                content = message.content
                if isinstance(content, list):
                    for item in content:
                        if type(item).__name__ == "ToolUseBlock":
                            tool_name = getattr(item, "name", "unknown")
                            collector.record_tool_use(tool_name)
                            # Track subagent invocations for Task tool
                            if tool_name == "Task":
                                tool_input = getattr(item, "input", {})
                                if isinstance(tool_input, dict):
                                    subagent_type = tool_input.get("subagent_type", "unknown")
                                    collector.record_subagent(subagent_type)

            # Capture ResultMessage for metrics extraction
            if type(message).__name__ == "ResultMessage":
                result_message = message

    except KeyboardInterrupt:
        log("\n\nMigration interrupted by user.", log_file)
        migration_status = "interrupted"
        if collector:
            collector.record_result(type("Result", (), {"status": "interrupted"})())
        sys.exit(1)
    except Exception as e:
        log(f"\n\nError during migration: {e}", log_file)
        migration_status = "failure"
        if collector:
            collector.record_result(type("Result", (), {"status": "failure", "error": str(e)})())
        raise

    if collector:
        collector.end_phase("migration")
        # Use actual ResultMessage if captured, otherwise dummy for status only
        if result_message:
            collector.record_result(result_message)
        elif migration_status == "success":
            collector.record_result(type("Result", (), {"status": "success"})())

    log("", log_file)
    log("=" * 60, log_file)
    log("Migration Complete", log_file)
    log("=" * 60, log_file)
    log(f"Total messages processed: {message_count}", log_file)
    log("", log_file)
    log("Next steps:", log_file)
    for i, cmd in enumerate(target.get_quality_gates(), 1):
        log(f"{i}. Run: {cmd}", log_file)

    # Finalize and save metrics
    if collector:
        collector.start_phase("reporting")
        metrics = collector.finalize()

        # Save metrics JSON
        metrics_dir = Path(project_dir) / "metrics"
        metrics_dir.mkdir(exist_ok=True)
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        metrics_file = metrics_dir / f"run_{timestamp}.json"

        with open(metrics_file, "w") as f:
            f.write(metrics.to_json())
        log(f"Metrics saved: {metrics_file}", log_file)

        # Generate report
        try:
            from reporting.generator import ReportGenerator
            generator = ReportGenerator()
            report = generator.generate_run_report(metrics)

            reports_dir = Path(project_dir) / "reports"
            reports_dir.mkdir(exist_ok=True)
            report_file = reports_dir / f"run_{timestamp}.md"

            with open(report_file, "w") as f:
                f.write(report)
            log(f"Report saved: {report_file}", log_file)
        except Exception as e:
            log(f"Warning: Could not generate report: {e}", log_file)

        collector.end_phase("reporting")
        return collector

    return None


def _log_message(message: Any, count: int, log_file: Path) -> None:
    """Log a message from the SDK."""
    log(f"MSG #{count}: type={type(message).__name__}", log_file, also_print=False)

    if hasattr(message, "type"):
        msg_type = message.type
        if msg_type == "text":
            text = getattr(message, "text", "")
            log(f"[TEXT] {text}", log_file, also_print=True)
        elif msg_type == "tool_use":
            tool_name = getattr(message, "name", "unknown")
            tool_input = getattr(message, "input", {})
            if isinstance(tool_input, dict):
                if "file_path" in tool_input:
                    log(f"[TOOL] {tool_name}: {tool_input.get('file_path', '')}", log_file)
                elif "command" in tool_input:
                    cmd = tool_input.get("command", "")[:80]
                    log(f"[TOOL] {tool_name}: {cmd}...", log_file)
                else:
                    log(f"[TOOL] {tool_name}: {str(tool_input)[:100]}", log_file)
            else:
                log(f"[TOOL] {tool_name}", log_file)
        elif msg_type == "tool_result":
            result = getattr(message, "result", "")
            if isinstance(result, str):
                log(f"[RESULT] ({len(result)} chars)", log_file, also_print=False)
            else:
                log(f"[RESULT] {type(result).__name__}", log_file, also_print=False)
        else:
            log(f"[{msg_type.upper()}] {str(message)[:200]}", log_file)
    elif hasattr(message, "content"):
        content = message.content
        log(f"[CONTENT] {str(content)[:200]}", log_file)
    else:
        msg_str = str(message)
        if len(msg_str) > 300:
            log(f"[RAW] {msg_str[:300]}...", log_file)
        else:
            log(f"[RAW] {msg_str}", log_file)

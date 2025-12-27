#!/usr/bin/env python3
"""
Claude Agent SDK Migration Runner - Option B: Multi-Phase Orchestration

Uses a 3-phase approach to minimize context size per subagent:
- Phase 1: Analyst reads ALL Python files ONCE, produces migration spec
- Phase 2: For each module, migrator uses spec (not raw source)
- Phase 3: For each module, reviewer validates against spec

Usage:
    python run_migration.py
    python run_migration.py --dry-run
"""

import argparse
import asyncio
import os
import sys
from datetime import datetime
from pathlib import Path

# Check for required environment variable
if not os.environ.get("ANTHROPIC_API_KEY"):
    print("Error: ANTHROPIC_API_KEY environment variable not set")
    print("Set it with: export ANTHROPIC_API_KEY=your-api-key")
    sys.exit(1)

try:
    from claude_agent_sdk import query, ClaudeAgentOptions, AgentDefinition
except ImportError:
    print("Error: claude-agent-sdk not installed")
    print("Install with: pip install claude-agent-sdk")
    print("Also ensure Claude Code is installed: curl -fsSL https://claude.ai/install.sh | sh")
    sys.exit(1)

from agents import (
    ANALYST_AGENT, MIGRATOR_AGENT, REVIEWER_AGENT, MIGRATION_CONFIG,
    SOURCE_FILES, TARGET_FILES, PROJECT_DIR
)


# Setup logging
LOG_DIR = Path(__file__).parent / "logs"
LOG_DIR.mkdir(exist_ok=True)
LOG_FILE = LOG_DIR / f"migration_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"


def log(msg: str, also_print: bool = True) -> None:
    """Log a message with timestamp to file and optionally stdout."""
    timestamp = datetime.now().strftime("%H:%M:%S")
    formatted = f"[{timestamp}] {msg}"

    # Always write to log file
    with open(LOG_FILE, "a") as f:
        f.write(formatted + "\n")
        f.flush()

    # Optionally print to stdout
    if also_print:
        print(formatted)
        sys.stdout.flush()


def build_migration_prompt() -> str:
    """Build the multi-phase migration prompt (Option B)."""
    modules_list = "\n".join(
        f"    {i+1}. {m['python']} -> {m['rust']} ({m['phase']} phase)"
        for i, m in enumerate(MIGRATION_CONFIG["modules"])
    )

    source_files_list = "\n".join(
        f"    - {k}: {v}" for k, v in SOURCE_FILES.items()
    )

    target_files_list = "\n".join(
        f"    - {k}: {v}" for k, v in TARGET_FILES.items()
    )

    return f"""
Migrate the rpn2tex Python codebase to Rust using a multi-phase approach.

## IMPORTANT: Multi-Phase Orchestration

This migration uses THREE distinct phases to minimize redundant file reads:

### PHASE 1: Comprehensive Analysis (DO THIS FIRST)

Spawn the **analyst** agent ONCE to analyze ALL Python modules:
- The analyst will read all 7 Python source files
- It will produce a comprehensive migration specification
- This spec contains all APIs, dependencies, and migration notes
- SAVE this specification - you will use it for all subsequent phases

### PHASE 2: Sequential Migration

For EACH module in dependency order:
1. Spawn the **migrator** agent with:
   - The relevant section of the migration spec (from Phase 1)
   - The specific module to migrate (python file -> rust file)
2. WAIT for migrator to complete and verify the .rs file exists
3. Migrator should NOT read Python files - use the spec instead

### PHASE 3: Sequential Review

For EACH module, AFTER its migration completes:
1. Spawn the **reviewer** agent with:
   - The relevant section of the migration spec
   - The Rust file path to review
2. Reviewer reads Rust file and compares against spec

## Source Files (analyst will read these in Phase 1):
{source_files_list}

## Target Files (migrators write to these exact paths):
{target_files_list}

## Project Directory:
    {PROJECT_DIR}

## Migration Order (respect dependencies):
{modules_list}

## Quality Gates (MUST pass before next module):
- `cargo check && cargo clippy -- -D warnings` - zero errors/warnings
- `cargo fmt` - properly formatted
- `cargo test` - tests pass

## Key Points:
1. Phase 1 reads Python files ONCE - no redundant reads
2. Migrators use the spec, not raw Python source
3. Each module must pass quality gates before proceeding
4. Update lib.rs after adding each new module
5. Run all cargo commands from: {PROJECT_DIR}

Begin with Phase 1: spawn the analyst to analyze ALL modules.
"""


async def run_migration(dry_run: bool = False) -> None:
    """Run the migration using Claude Agent SDK with multi-phase orchestration."""
    prompt = build_migration_prompt()

    if dry_run:
        print("=" * 60)
        print("DRY RUN - Would execute with the following prompt:")
        print("=" * 60)
        print(prompt)
        print("=" * 60)
        print("\nAgents configured:")
        print(f"  - analyst: {ANALYST_AGENT['description'][:60]}...")
        print(f"  - migrator: {MIGRATOR_AGENT['description'][:60]}...")
        print(f"  - reviewer: {REVIEWER_AGENT['description'][:60]}...")
        return

    log("=" * 60)
    log("Starting Claude Agent SDK Migration (Option B: Multi-Phase)")
    log("=" * 60)
    log(f"Source: {MIGRATION_CONFIG['source_dir']}")
    log(f"Target: {MIGRATION_CONFIG['target_dir']}")
    log(f"Modules: {len(MIGRATION_CONFIG['modules'])}")
    log(f"Log file: {LOG_FILE}")
    log("=" * 60)

    # Create AgentDefinition objects
    agents = {
        "analyst": AgentDefinition(
            description=ANALYST_AGENT["description"],
            prompt=ANALYST_AGENT["prompt"],
            tools=ANALYST_AGENT["tools"],
            model=ANALYST_AGENT["model"],
        ),
        "migrator": AgentDefinition(
            description=MIGRATOR_AGENT["description"],
            prompt=MIGRATOR_AGENT["prompt"],
            tools=MIGRATOR_AGENT["tools"],
            model=MIGRATOR_AGENT["model"],
        ),
        "reviewer": AgentDefinition(
            description=REVIEWER_AGENT["description"],
            prompt=REVIEWER_AGENT["prompt"],
            tools=REVIEWER_AGENT["tools"],
            model=REVIEWER_AGENT["model"],
        ),
    }

    options = ClaudeAgentOptions(
        allowed_tools=["Read", "Write", "Edit", "Bash", "Glob", "Grep", "Task"],
        agents=agents,
        permission_mode="acceptEdits",  # Auto-accept file edits
    )

    message_count = 0
    try:
        async for message in query(prompt=prompt, options=options):
            message_count += 1

            # Log ALL messages for debugging
            log(f"MSG #{message_count}: type={type(message).__name__}", also_print=False)

            # Try to extract useful info from the message
            if hasattr(message, "type"):
                msg_type = message.type
                if msg_type == "text":
                    text = getattr(message, "text", "")
                    log(f"[TEXT] {text}", also_print=True)
                elif msg_type == "tool_use":
                    tool_name = getattr(message, "name", "unknown")
                    tool_input = getattr(message, "input", {})
                    if isinstance(tool_input, dict):
                        if "file_path" in tool_input:
                            log(f"[TOOL] {tool_name}: {tool_input.get('file_path', '')}")
                        elif "command" in tool_input:
                            cmd = tool_input.get("command", "")[:80]
                            log(f"[TOOL] {tool_name}: {cmd}...")
                        else:
                            log(f"[TOOL] {tool_name}: {str(tool_input)[:100]}")
                    else:
                        log(f"[TOOL] {tool_name}")
                elif msg_type == "tool_result":
                    result = getattr(message, "result", "")
                    if isinstance(result, str):
                        log(f"[RESULT] ({len(result)} chars)", also_print=False)
                    else:
                        log(f"[RESULT] {type(result).__name__}", also_print=False)
                else:
                    log(f"[{msg_type.upper()}] {str(message)[:200]}")
            elif hasattr(message, "content"):
                content = message.content
                log(f"[CONTENT] {str(content)[:200]}")
            else:
                msg_str = str(message)
                if len(msg_str) > 300:
                    log(f"[RAW] {msg_str[:300]}...")
                else:
                    log(f"[RAW] {msg_str}")

    except KeyboardInterrupt:
        log("\n\nMigration interrupted by user.")
        sys.exit(1)
    except Exception as e:
        log(f"\n\nError during migration: {e}")
        raise

    log("")
    log("=" * 60)
    log("Migration Complete")
    log("=" * 60)
    log(f"Total messages processed: {message_count}")
    log("")
    log("Next steps:")
    log("1. Run: cargo test")
    log("2. Run: cargo clippy")
    log("3. Compare with Python implementation")


def main():
    parser = argparse.ArgumentParser(
        description="Migrate Python code to Rust using Claude Agent SDK (Multi-Phase)"
    )
    parser.add_argument(
        "--dry-run",
        "-n",
        action="store_true",
        help="Show what would be done without executing",
    )

    args = parser.parse_args()

    asyncio.run(run_migration(dry_run=args.dry_run))


if __name__ == "__main__":
    main()

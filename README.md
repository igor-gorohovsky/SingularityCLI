# Singularity CLI

> This entire project — every line of code, every test, this very README — was written by [Claude Code](https://claude.ai/code). No humans were mass-unemployed in the making of this CLI. Your job is safe. Probably.

A command-line interface for the [Singularity](https://singularity-app.com) task manager, built in Rust.

Manage your projects, tasks, task groups, and tags directly from the terminal or through AI agent integrations.

## Installation

```bash
cargo install singularity-cli
```

Or build from source:

```bash
git clone https://github.com/igor-gorohovsky/SingularityCLI.git
cd SingularityCLI
cargo build --release
# binary is at ./target/release/singularity
```

## Quick Start

```bash
# Set your API token (get it from the Singularity app)
singularity config set-token <YOUR_TOKEN>

# List your projects
singularity project list

# Create a task
singularity task create --title "Review PR" --priority high --project-id P-xxx

# Complete a task
singularity task update T-xxx --checked checked

# Get today's tasks
singularity task list --start-from 2026-01-01T00:00:00.000Z --start-to 2026-01-01T23:59:59.999Z
```

## Authentication

Two options, env var takes precedence:

| Method | Details |
|--------|---------|
| Config file | `singularity config set-token <TOKEN>` saves to `~/.config/singularity/config.toml` |
| Environment variable | `export SINGULARITY_TOKEN=<TOKEN>` |

## Commands

### Projects

```bash
singularity project list [--max-count N] [--offset N] [--include-removed] [--include-archived]
singularity project get <P-uuid>
singularity project create --title "My Project" [--note "..."] [--parent P-uuid] [--color "#FF0000"] [--emoji "..."] [--start DATE] [--end DATE] [--notebook]
singularity project update <P-uuid> [--title "..."] [--note "..."] [--color "..."] ...
singularity project delete <P-uuid>
```

### Tasks

```bash
singularity task list [--project-id P-uuid] [--parent T-uuid] [--start-from DATE] [--start-to DATE] [--max-count N] [--offset N] [--include-removed] [--include-archived]
singularity task get <T-uuid>
singularity task create --title "My Task" [--priority high|normal|low] [--project-id P-uuid] [--deadline DATE] [--tags tag1,tag2] ...
singularity task update <T-uuid> [--checked empty|checked|cancelled] [--priority high|normal|low] [--title "..."] ...
singularity task delete <T-uuid>
```

### Task Groups

```bash
singularity task-group list [--parent P-uuid] [--max-count N] [--offset N] [--include-removed]
singularity task-group get <Q-uuid>
singularity task-group create --title "Sprint 1" --parent <P-uuid> [--order 1.0]
singularity task-group update <Q-uuid> [--title "..."] [--parent P-uuid] [--order 2.0]
singularity task-group delete <Q-uuid>
```

### Tags

```bash
singularity tag list [--parent <tag-id>] [--max-count N] [--offset N] [--include-removed]
singularity tag get <tag-id>
singularity tag create --title "urgent" [--parent <tag-id>] [--order 1.0]
singularity tag update <tag-id> [--title "..."] [--parent <tag-id>] [--order 2.0]
singularity tag delete <tag-id>
```

## Output Formats

By default, list commands output a formatted table:

```
+---------+-------------------+----------+---------+
| ID      | TITLE             | PRIORITY | CHECKED |
+---------+-------------------+----------+---------+
| T-abc.. | Review PR         | high     | empty   |
+---------+-------------------+----------+---------+
| T-def.. | Write tests       | normal   | checked |
+---------+-------------------+----------+---------+
```

Use `--json` on any command for machine-readable JSON output:

```bash
singularity --json task list --project-id P-xxx
```

## ID Formats

Singularity uses prefixed UUIDs for entity identification:

| Entity | Format | Example |
|--------|--------|---------|
| Project | `P-<uuid>` | `P-a1b2c3d4-e5f6-...` |
| Task | `T-<uuid>` | `T-f7e8d9c0-b1a2-...` |
| Task Group | `Q-<uuid>` | `Q-1a2b3c4d-5e6f-...` |

## Agent Integration

This CLI is designed to work well with AI agents. It includes:

- Descriptive `--help` on every command and flag
- `--json` output for programmatic parsing
- A [Claude Code skill file](.claude/skills/singularity.md) for seamless agent integration

## Development

```bash
# Run tests
cargo test

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

## License

Apache-2.0 - see [LICENSE](LICENSE) for details.


# Singularity CLI

CLI wrapper for the Singularity task manager API. Use it to manage projects, tasks, task groups, and tags.

## Setup

Run interactive setup to configure API token and timezone:
```bash
singularity setup
```

Or configure individually:
```bash
singularity config set-token <TOKEN>
singularity config set-timezone Europe/Kyiv
```

Token is stored in `~/.config/singularity/config.toml`. Alternatively set the `SINGULARITY_TOKEN` env var (takes precedence over config file).

Timezone (IANA format) ensures date filters and display use correct local time instead of UTC. Without it, dates are treated as UTC.

## ID Formats

- Projects: `P-<uuid>` (e.g. `P-a1b2c3d4-...`)
- Tasks: `T-<uuid>`
- Task groups: `Q-<uuid>`

## Global Flags

- `--json` — output raw JSON instead of formatted tables. Use this when you need to parse the output programmatically.

## Commands

### Projects

```bash
singularity project list [--max-count N] [--offset N] [--include-removed] [--include-archived]
singularity project get <P-uuid>
singularity project create --title "My Project" [--note "..."] [--parent P-uuid] [--color "#FF0000"] [--emoji "📁"] [--start 2025-01-01] [--end 2025-12-31] [--notebook]
singularity project update <P-uuid> [--title "New Title"] [--note "..."] [--parent P-uuid] [--color "..."] [--emoji "..."] [--start ...] [--end ...] [--notebook true|false]
singularity project delete <P-uuid>
```

### Tasks

```bash
singularity task list [--project-id P-uuid] [--parent T-uuid] [--start-from 2025-01-01] [--start-to 2025-12-31] [--max-count N] [--offset N] [--include-removed] [--include-archived]
singularity task get <T-uuid>
singularity task create --title "My Task" [--note "..."] [--priority high|normal|low] [--project-id P-uuid] [--parent T-uuid] [--group Q-uuid] [--deadline 2025-06-01] [--start 2025-01-01] [--tags tag1,tag2]
singularity task update <T-uuid> [--title "..."] [--note "..."] [--priority high|normal|low] [--checked empty|checked|cancelled] [--project-id P-uuid] [--parent T-uuid] [--group Q-uuid] [--deadline ...] [--start ...] [--tags tag1,tag2]
singularity task delete <T-uuid>
```

Priority values: `high`, `normal`, `low`
Checked values: `empty` (not done), `checked` (completed), `cancelled`

### Task Groups

Task groups organize tasks within a project.

```bash
singularity task-group list [--parent P-uuid] [--max-count N] [--offset N] [--include-removed]
singularity task-group get <Q-uuid>
singularity task-group create --title "Group Name" --parent <P-uuid> [--order 1.0]
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

## Common Workflows

### Create a project with tasks
```bash
singularity project create --title "Sprint 1"
# note the returned P-uuid
singularity task create --title "Design API" --project-id P-xxx --priority high
singularity task create --title "Implement endpoints" --project-id P-xxx --priority normal
```

### Complete a task
```bash
singularity task update T-xxx --checked checked
```

### List all tasks in a project
```bash
singularity task list --project-id P-xxx
```

### Get full task details as JSON
```bash
singularity --json task get T-xxx
```

## Pagination

All list commands support `--max-count` (max 1000) and `--offset` for pagination. Default returns all results up to API limits.

## Error Handling

- Missing token: prints instruction to set it
- 401 Unauthorized: token is invalid or expired
- Other API errors: prints HTTP status code and error body

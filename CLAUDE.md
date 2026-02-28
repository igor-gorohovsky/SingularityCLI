# Singularity CLI

Rust CLI wrapper for the Singularity task manager REST API (`https://api.singularity-app.com/v2/`).

## Project Structure

```
src/
├── main.rs              # Entry point, top-level clap CLI definition
├── lib.rs               # Library crate re-exports for integration tests
├── client.rs            # HTTP client (reqwest blocking, bearer auth)
├── config.rs            # Config management (token, timezone resolution)
├── models/              # Serde DTOs (response, create, update per resource)
│   ├── project.rs
│   ├── task.rs
│   ├── task_group.rs
│   └── tag.rs
└── commands/            # Clap subcommand enums + run() handlers
    ├── config.rs
    ├── setup.rs
    ├── project.rs
    ├── task.rs
    ├── task_group.rs
    └── tag.rs
tests/
└── api_client.rs        # Integration tests using mockito mock server
```

## Key Patterns

- Each command module is self-contained: clap enum + execution logic + help texts
- Models use `#[serde(skip_serializing_if = "Option::is_none")]` for partial updates
- `ApiClient::with_base_url()` exists for pointing tests at mockito
- Binary name is `singularity`, crate name is `singularity-cli`

## Commands

```
cargo build              # Build
cargo test               # Unit + integration tests
cargo fmt                # Format
cargo clippy -D warnings # Lint
```

## API Resources (currently supported)

Projects (`P-<uuid>`), Tasks (`T-<uuid>`), Task Groups (`Q-<uuid>`), Tags — each with full CRUD.

## Auth & Config

Token from `SINGULARITY_TOKEN` env var or `~/.config/singularity/config.toml`.

Config file also stores `timezone` (IANA format, e.g. `Europe/Kyiv`). Used to convert date filters from local time to UTC before querying the API, and to display API dates in local time. Run `singularity setup` for interactive configuration.

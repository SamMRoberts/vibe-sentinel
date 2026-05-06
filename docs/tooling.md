# Tooling and Validation

## Tech stack conventions

- Language: Rust.
- Package manager and build tool: Cargo.
- CLI: parse commands separately from application behavior.
- TUI: ratatui, with render/state logic isolated for tests.
- MCP: thin protocol adapters over mockable application services.
- Repository validation helpers may use Python 3 when they do not become runtime dependencies.

## Validation commands

- `cargo fmt --check`: verify Rust formatting.
- `cargo clippy --all-targets --all-features -- -D warnings`: verify Rust lint quality.
- `cargo test --all`: run all Rust tests.
- `cargo build --all-targets`: verify build targets.
- `python3 scripts/validate_tdd_workflow.py`: verify active feature plans contain required modified TDD artifacts.
- `python3 /Users/samroberts/.codex/plugins/cache/local-marketplace/agentic-coding-harness/0.1.0/scripts/validate_harness.py`: validate harness structure when the plugin cache path is available.

## Command reporting

When changing code, report which commands ran, which failed, and which were skipped. Do not invent successful validation.

## Missing command rule

If a relevant validation command is missing, add a note to `docs/exec-plans/tech-debt-tracker.md` or create a plan to add it.

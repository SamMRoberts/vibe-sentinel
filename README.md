# vibe-sentinel

`vibe-sentinel` is a Rust CLI, ratatui TUI, and MCP product built under the
repository's modified TDD harness.

## Current CLI

Run the first vertical slice from the repository root:

```bash
cargo run -- status
```

For machine-readable output, run:

```bash
cargo run -- status --json
```

For the read-only terminal UI, run:

```bash
cargo run -- status --tui
```

The command prints deterministic local readiness checks for harness docs, active
execution plans, and Cargo workspace presence. JSON output includes the project
name, aggregate readiness, and the ordered readiness checks. TUI mode renders the
same readiness report and exits on `q` or `Esc`.

## Validation

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --all-targets
python3 scripts/validate_tdd_workflow.py
```
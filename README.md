# vibe-sentinel

`vibe-sentinel` is a Rust CLI, ratatui TUI, and MCP product built under the
repository's modified TDD harness.

## Current CLI

Run the first vertical slice from the repository root:

```bash
cargo run -- status
```

The command prints deterministic local readiness checks for harness docs, active
execution plans, and Cargo workspace presence.

## Validation

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
cargo build --all-targets
python3 scripts/validate_tdd_workflow.py
```
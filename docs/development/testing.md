# Testing Strategy

Testing should evolve with the product while keeping fast local feedback.

## Unit tests

Use for:

- domain invariants;
- graph traversal;
- config validation;
- rule evaluation.

## Fixture/integration tests

Use tiny repository fixtures for:

- project discovery;
- package adapters;
- graph construction;
- boundaries;
- affected analysis.

Fixtures should:

- require no network access;
- require no package installation;
- be small enough to understand by inspection;
- include valid and invalid cases.

## CLI tests

Use for:

- command contracts;
- exit codes;
- human-readable diagnostics;
- JSON/machine-readable output.

## Standard checks

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace
cargo test --workspace
```

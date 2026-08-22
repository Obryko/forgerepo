# Rust rules

Canonical architecture rules live in `CLAUDE.md` and `docs/architecture/`.

- Keep `forge-core` ecosystem-neutral.
- Prefer stable Rust.
- Prefer explicit domain types for IDs/invariants.
- Avoid panic-based normal error paths.
- Keep graph traversal deterministic.
- Keep CLI formatting out of domain crates.
- Run fmt, clippy, check, and tests.

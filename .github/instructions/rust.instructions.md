---
applyTo: "**/*.rs,**/Cargo.toml"
---

# Rust-specific instructions

Follow `CLAUDE.md` and the architecture documents under `docs/architecture/`.

- Prefer idiomatic stable Rust.
- Favor explicit domain types when they protect invariants.
- Avoid `unwrap()`/`expect()` in normal production error paths.
- Keep graph traversal deterministic.
- Keep CLI formatting out of domain crates.
- Do not introduce async runtime dependencies without a demonstrated need.
- Run fmt, clippy, check, and tests for Rust changes.

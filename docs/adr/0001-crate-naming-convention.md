# ADR 0001: Crate naming convention

- Status: Accepted
- Date: 2026-08-22
- Related issue: #2

## Context

Early docs (README, CLAUDE.md, `docs/architecture/crate-boundaries.md`) sketched crate names as `forge-core`, `forge-config`, `forge-boundaries`, `forge-cli`. When the workspace was actually initialized (#2), the crates were published under `forgerepo-*` package names instead (`forgerepo-core`, `forgerepo-config`, `forgerepo-boundaries`), with the CLI crate kept as bare `forgerepo`. Docs and implementation disagreed and needed a single source of truth.

## Decision

Library crates use the `forgerepo-*` package-name prefix, matching the repository/project name:

- `forgerepo-core`
- `forgerepo-config`
- `forgerepo-boundaries`

The CLI crate is the exception: its package (and binary) name is `forgerepo`, not `forgerepo-cli`. It is the primary user-facing entrypoint, typed directly on the command line — an extra `-cli` suffix only adds noise for the one crate users actually invoke.

Directory names under `crates/` stay short (`cli/`, `core/`, `config/`, `boundaries/`); the `forgerepo-*` prefix lives on the Cargo package name, not the path.

## Alternatives considered

### Option A: keep `forge-*` as originally sketched

Pros:

- Shorter names.

Cons:

- Doesn't match the repository/project name (`forgerepo`), which is confusing on crates.io and in error messages.
- Would require renaming already-implemented crates from #2 for no functional benefit.

### Option B: `forgerepo-*` everywhere, including `forgerepo-cli`

Pros:

- Fully uniform prefix across all crates.

Cons:

- Makes the primary binary name `forgerepo-cli` instead of `forgerepo`, which is worse UX for the one crate end users type directly.

## Consequences

- Docs (`README.md`, `CLAUDE.md`, `docs/architecture/crate-boundaries.md`, `docs/product/principles.md`) now reference `forgerepo-core` / `forgerepo-config` / `forgerepo-boundaries` / `forgerepo` (CLI).
- Future crates (adapters, etc.) should default to the `forgerepo-*` prefix unless they are a directly user-invoked binary.

## Follow-up work

None — naming already matches implementation as of #2.

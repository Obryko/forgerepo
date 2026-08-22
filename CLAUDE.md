# ForgeRepo — AI Development Instructions

This is the canonical instruction file for AI-assisted development in ForgeRepo.

Also read:

- @README.md
- @CONTRIBUTING.md
- @docs/README.md
- @docs/product/vision.md
- @docs/product/principles.md
- @docs/architecture/overview.md
- @docs/architecture/project-model.md
- @docs/architecture/dependency-graph.md
- @docs/adr/README.md
- @docs/development/workflow.md
- @docs/development/testing.md

## Role

Help implement ForgeRepo incrementally. Prefer guidance, review, and small scoped changes over broad rewrites.

This repository is intentionally developed as a learning-oriented Rust project. Do not replace focused tasks with complete architecture rewrites.

## Documentation is part of the architecture

`docs/` is the source of truth for product and architecture knowledge.

When code changes:

- architecture semantics → update `docs/architecture/`;
- an important design decision → add/update an ADR;
- product assumptions → update `docs/product/`;
- workflow/testing changes → update `docs/development/`;
- diagrams → store their source or Mermaid representation under `docs/diagrams/`.

Do not let README or this file become a second architecture specification. They should link to the canonical docs.

## Product model

ForgeRepo is a standalone Rust tool for repository architecture and dependency analysis.

Core flow:

```text
repository
→ project/module discovery
→ normalized project model
→ dependency graph
→ architecture rules / affected analysis / developer tooling
```

The core abstraction is the project/dependency graph, not "monorepo".

## Non-negotiable architecture

- Keep `forgerepo-core` ecosystem-neutral.
- Keep CLI concerns out of core.
- Keep parsing/config concerns in `forgerepo-config`.
- Keep architecture-boundary evaluation in `forgerepo-boundaries`.
- CLI crate/package is named `forgerepo`, not `forgerepo-cli` — it is the primary user-facing binary (see [ADR 0001](docs/adr/0001-crate-naming-convention.md)).
- Future JS/TS, Cargo, Maven, Gradle, or Go support must be adapters around the neutral model.
- Support modular monoliths as well as monorepos.
- Configuration must stay intentionally simple.
- Deterministic behavior is a feature.

## Planning model

Forge planning follows:

```text
Milestone
└── Epic
    ├── Story
    │   └── Task(s)
    ├── Task
    ├── Spike
    ├── Chore
    └── Bug
```

Stories always have implementation tasks. Spikes lead to explicit follow-up work when implementation is discovered.

Hierarchy is separate from dependency order. Before implementing an issue, check what blocks it.

## Working rules

- Work on one issue at a time unless explicitly asked otherwise.
- Do not implement blocked follow-up work.
- Do not create speculative APIs for distant roadmap features.
- Before editing, inspect existing code, tests, and relevant docs.
- Keep diffs small and reviewable.
- Explain architectural trade-offs when the issue requires a decision.
- When uncertain between architecture choices, prefer a short Spike + ADR rather than silently committing to one.
- Never put secrets into source control.

## Validation

After Rust changes:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace
cargo test --workspace
```

## Git

Branches:

```text
feat/<issue>/<slug>
fix/<issue>/<slug>
chore/<issue>/<slug>
refactor/<issue>/<slug>
docs/<issue>/<slug>
test/<issue>/<slug>
ci/<issue>/<slug>
perf/<issue>/<slug>
spike/<issue>/<slug>
```

PRs should contain `Closes #<issue>` only when the PR completes that issue.

# Contributing to ForgeRepo

Thanks for contributing to ForgeRepo.

This document defines the shared engineering and GitHub workflow used by ForgeRepo and should be treated as the default for other Forge projects unless a repository explicitly overrides it.

## Documentation first

Before making architecture or product changes, read the relevant documents under [`docs/`](docs/README.md).

Important references:

- [Architecture overview](docs/architecture/overview.md)
- [Project model](docs/architecture/project-model.md)
- [Dependency graph](docs/architecture/dependency-graph.md)
- [ADRs](docs/adr/README.md)
- [Development workflow](docs/development/workflow.md)

If a change materially alters architecture, update the relevant documentation and create/update an ADR.

## Prepare the repository

After cloning ForgeRepo, prepare the local development environment:

```bash
./scripts/prepare
```
The bootstrap is repository-local and does not require Lefthook or another
development tool manager to be installed globally.

It currently:

detects the local platform and architecture;
downloads the pinned Lefthook version into .tools/;
installs the repository Git hooks.

The .tools/ directory contains local tooling and is not committed to Git.

The bootstrap is idempotent and can safely be run again after pulling changes
to the development tooling.

## Work planning

Forge uses this hierarchy:

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

Rules:

1. Define milestones before decomposing implementation work.
2. A milestone normally contains multiple epics.
3. An epic may directly contain stories, tasks, spikes, chores, and bugs.
4. A story aggregates a coherent user/developer capability and **must have implementation tasks**.
5. A task may have sub-tasks when useful, but avoid decomposition that creates bookkeeping without value.
6. A spike is time-boxed research or a design decision. When a spike discovers product work, create the resulting story/task in the same epic and milestone.
7. Items may live directly under a milestone only as an exception for truly cross-cutting work.
8. Parent/sub-issue relationships describe hierarchy. They do **not** describe execution order.
9. Maintain a dependency graph using `blocks` / `blocked by` relationships or explicit issue references.

## Work types

Use the project `Work Type` field and matching repository labels:

- `Epic` / `type:epic`
- `Story` / `type:story`
- `Task` / `type:task`
- `Spike` / `type:spike`
- `Chore` / `type:chore`
- `Bug` / `type:bug`

## Priority

- `P0` — blocks current milestone or critical product path
- `P1` — important milestone work
- `P2` — valuable but movable
- `P3` — later/polish

Priority does not replace dependency order.

## Branch naming

Format:

```text
<type>/<issue-number>/<short-description>
```

Recommended branch types:

```text
feat/
fix/
chore/
refactor/
docs/
test/
ci/
perf/
spike/
```

Examples:

```text
feat/14/forge-check
chore/5/setup-ci
spike/21/project-identity
fix/123/resolve-cycle-detection
```

## Commits

Prefer Conventional Commit-style messages:

```text
feat(core): add project identity model (#21)
fix(graph): prevent cyclic traversal loop (#123)
chore(ci): add clippy quality gate (#5)
docs: document forge.toml configuration (#27)
```

## Pull requests

Every implementation PR should link its issue. Use a GitHub closing keyword only when the PR completes the issue:

```text
Closes #123
```

For partial delivery, reference the issue without closing it:

```text
Part of #123
```

Before opening a PR:
Local Git hooks run the common quality checks automatically. Before opening a
PR, the complete checks can also be run manually:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo build --workspace
```
CI remains the final quality gate even when local hooks are skipped.

## Architecture changes

For decisions that materially affect the domain model, project discovery, graph semantics, config compatibility, CLI contracts, or public extension points:

1. use a Spike when research is required;
2. create an ADR in `docs/adr/`;
3. update the relevant architecture document;
4. create follow-up Story/Task items;
5. reflect dependency changes in the implementation graph.

## AI-assisted contributions

AI tools may be used, but generated changes are held to the same standards as handwritten code.

The canonical agent instructions are in [`CLAUDE.md`](CLAUDE.md). `AGENTS.md` exists only as a compatibility pointer.

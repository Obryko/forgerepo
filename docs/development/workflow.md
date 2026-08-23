# Development Workflow

## Planning

Work should originate from an existing GitHub issue in the Forge planning hierarchy.

Before implementation, determine:

- Milestone;
- parent Epic/Story;
- Work Type;
- acceptance criteria;
- blockers;
- downstream work.

## Branches

```text
<type>/<issue>/<slug>
```

Examples:

```text
feat/14/forge-check
spike/21/project-identity
chore/5/setup-ci
```

## Pull requests

A PR that completes an issue must include:

```text
Closes #<issue>
```

Partial work uses:

```text
Part of #<issue>
```

## Dependency order

Parent/sub-issue hierarchy does not imply execution order.

Use dependency relationships to identify:

- ready work;
- blocked work;
- parallel work;
- critical path.

## Documentation

Architecture/product changes update `docs/` in the same PR.

## Local development hooks

Repository-local development tooling is prepared with:

```bash
./scripts/prepare
```
The bootstrap downloads the pinned Lefthook binary into the gitignored
.tools/ directory and installs the configured Git hooks. No global Lefthook
installation is required.
The current hooks are:
#### Pre-commit:
- `cargo fmt --all` — formats the workspace automatically
- `cargo check --workspace --all-targets`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
#### Pre-push:
- `cargo test --workspace --all-targets`

Hooks provide fast local feedback only. They do not replace CI and may not be treated as a merge gate.

Lefthook can also be invoked through the repository-local binary:
```bash
./.tools/lefthook run pre-commit
```

## Continuous integration

`.github/workflows/ci.yml` runs on every push to `main` and on every pull request, as five independent jobs:

- **Formatting** — `cargo fmt --all -- --check`
- **Clippy** — `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- **Check** — `cargo check --workspace --all-targets`
- **Test** — `cargo test --workspace --all-targets`
- **Build** — `cargo build --workspace`

Check and Test add `--all-targets` (covering tests, examples, and benches, not just library/binary code) beyond the local [standard checks](testing.md#standard-checks); run with `--all-targets` locally to match CI exactly.

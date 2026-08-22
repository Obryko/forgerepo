# ForgeRepo

ForgeRepo is a fast, standalone repository architecture and dependency-analysis tool written in Rust.

The long-term goal is to provide a small, runtime-independent toolkit for understanding repository structure, building a project graph, enforcing architecture boundaries, and powering affected analysis and scalable CI.

> ForgeRepo is under active development. The public API, configuration format, and CLI may change before `1.0.0`.

## Why ForgeRepo?

Modern repositories often rely on large framework-specific toolchains just to answer basic questions:

- What projects or modules exist?
- What depends on what?
- Is this dependency allowed?
- Why does project A depend on project B?
- Which projects are affected by this change?

ForgeRepo aims to answer these questions with one fast Rust binary and an ecosystem-neutral core.

## Product principles

- **Architecture first.** The project graph is the central abstraction.
- **Ecosystem neutral.** Core concepts must not depend on Nx, pnpm, Cargo, Maven, Gradle, or another specific build system.
- **Simple configuration.** `forge.toml` should be understandable without reading a manual.
- **Useful outside monorepos.** Modular monoliths and layered applications are first-class use cases.
- **Fast feedback.** Commands should be suitable for local development and CI.
- **No runtime baggage.** Distributed releases should run without Node.js or another language runtime.

## Planned CLI

```bash
forge init
forge check
forge graph
forge deps <project>
forge why <source> <target>
forge affected --base <ref> --head <ref>
```

## Documentation

The `docs/` directory is the source of truth for ForgeRepo's product and engineering documentation.

Start here:

- [Documentation index](docs/README.md)
- [Product vision](docs/product/vision.md)
- [Product principles](docs/product/principles.md)
- [Architecture overview](docs/architecture/overview.md)
- [Project model](docs/architecture/project-model.md)
- [Dependency graph](docs/architecture/dependency-graph.md)
- [Architecture Decision Records](docs/adr/README.md)
- [Development workflow](docs/development/workflow.md)
- [Testing strategy](docs/development/testing.md)
- [Diagram conventions](docs/diagrams/README.md)

This structure is intentionally suitable for future publication as GitHub Wiki or a generated documentation site.

## Repository layout

The initial workspace is expected to evolve toward:

```text
crates/
├── forge-cli/
├── forge-core/
├── forge-config/
└── forge-boundaries/

fixtures/
docs/
```

Crate boundaries are intentional:

- `forge-core` owns ecosystem-neutral domain concepts and graph primitives.
- `forge-config` owns ForgeRepo configuration loading and validation.
- `forge-boundaries` owns architecture-boundary rules and evaluation.
- `forge-cli` owns CLI argument parsing and presentation, not domain logic.

## Development

Prerequisites:

- Rust stable
- Cargo
- Git

Once the workspace is initialized, the standard local checks are expected to be:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo check --workspace
cargo test --workspace
```

See:

- [CONTRIBUTING.md](CONTRIBUTING.md)
- [Development workflow](docs/development/workflow.md)
- [CLAUDE.md](CLAUDE.md) for AI-assisted development rules

## Planning model

Forge projects use:

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

A milestone normally contains multiple epics. Stories always contain implementation tasks. Parent/sub-issue hierarchy describes ownership; dependency relationships describe implementation order.

## Status

The project is currently pre-`0.1.0`.

See the repository Issues, Milestones, and GitHub Project for the active roadmap.

## License

MIT. See [LICENSE](LICENSE).

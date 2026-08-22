# ForgeRepo Documentation

This directory is the canonical source of truth for ForgeRepo product, architecture, engineering decisions, diagrams, and development practices.

It is intentionally organized so the same Markdown files can later be published as:

- GitHub Wiki;
- a generated static documentation site;
- repository-native documentation.

## Sections

### Product

- [Vision](product/vision.md)
- [Product principles](product/principles.md)
- [Roadmap model](product/roadmap-model.md)

### Architecture

- [Architecture overview](architecture/overview.md)
- [Project model](architecture/project-model.md)
- [Dependency graph](architecture/dependency-graph.md)
- [Configuration architecture](architecture/configuration.md)
- [Crate boundaries](architecture/crate-boundaries.md)

### Architecture Decision Records

- [ADR index](adr/README.md)
- [ADR template](adr/0000-template.md)

### Development

- [Workflow](development/workflow.md)
- [Testing strategy](development/testing.md)
- [Release strategy](development/releases.md)

### Diagrams

- [Diagram conventions](diagrams/README.md)

## Documentation rules

- Prefer one canonical document for each concept.
- Link instead of duplicating.
- Architecture-affecting changes must update documentation in the same PR.
- Important irreversible or compatibility-sensitive decisions should have an ADR.
- Prefer Mermaid for diagrams when possible so diagrams remain reviewable as text.

# Project Model

A `Project` is ForgeRepo's logical unit of repository ownership and dependency analysis.

The exact model is intentionally still evolving.

## Requirements

The model must support:

- package-based monorepos;
- Cargo workspaces;
- modular monolith modules;
- layered applications;
- nested project structures where explicitly supported;
- ecosystem-specific metadata without leaking it into core identity.

## Open questions

These should be resolved through the relevant Spike/ADR before the model stabilizes:

- Can projects be nested?
- Can one file belong to multiple projects?
- How is project identity different from package-manager identity?
- How are root-level/unowned files represented?
- How are generated/vendor directories handled?
- What constitutes a project in a modular monolith?

Do not silently answer these questions through implementation details.

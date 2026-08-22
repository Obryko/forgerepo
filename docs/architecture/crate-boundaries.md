# Crate Boundaries

The initial workspace is expected to contain:

```text
crates/
├── forge-cli
├── forge-core
├── forge-config
└── forge-boundaries
```

## forge-core

Owns:

- project/domain types;
- dependency graph;
- ecosystem-neutral algorithms.

Must not depend on:

- CLI presentation;
- package-manager-specific concepts;
- boundary-specific policy.

## forge-config

Owns:

- locating `forge.toml`;
- deserialization;
- defaults;
- semantic configuration validation.

## forge-boundaries

Owns:

- boundary rule model;
- boundary evaluation;
- violation domain objects.

## forge-cli

Owns:

- argument parsing;
- command orchestration;
- human/machine output;
- exit-code mapping.

Business logic should live below the CLI boundary.

# Crate Boundaries

The initial workspace is expected to contain:

```text
crates/
├── cli/         (package: forgerepo)
├── core/        (package: forgerepo-core)
├── config/      (package: forgerepo-config)
└── boundaries/  (package: forgerepo-boundaries)
```

Directory names are short; package names carry the `forgerepo-*` prefix, except the CLI crate — see [ADR 0001](../adr/0001-crate-naming-convention.md).

## forgerepo-core

Owns:

- project/domain types;
- dependency graph;
- ecosystem-neutral algorithms.

Must not depend on:

- CLI presentation;
- package-manager-specific concepts;
- boundary-specific policy.

## forgerepo-config

Owns:

- locating `forge.toml`;
- deserialization;
- defaults;
- semantic configuration validation.

## forgerepo-boundaries

Owns:

- boundary rule model;
- boundary evaluation;
- violation domain objects.

## forgerepo (cli)

Owns:

- argument parsing;
- command orchestration;
- human/machine output;
- exit-code mapping.

Business logic should live below the CLI boundary.

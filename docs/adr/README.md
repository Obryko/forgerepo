# Architecture Decision Records

ADRs capture important architectural decisions that would otherwise be hidden in code or issue discussions.

## When to create an ADR

Create an ADR when a decision materially affects:

- Project identity/ownership semantics;
- dependency graph semantics;
- crate boundaries;
- configuration compatibility;
- CLI/output contracts;
- extension/adapter architecture;
- a difficult-to-reverse technical choice.

Not every implementation detail needs an ADR.

## Naming

Use sequential names:

```text
0001-project-identity.md
0002-graph-representation.md
0003-configuration-schema.md
```

Use [0000-template.md](0000-template.md) as the starting point.

## Status

Recommended statuses:

- Proposed
- Accepted
- Superseded
- Rejected

When an ADR is superseded, link both directions.

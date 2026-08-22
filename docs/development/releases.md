# Release Strategy

ForgeRepo is pre-`1.0.0`.

## Target Release vs Milestone

These are intentionally separate concepts.

A Target Release may group one or several Milestones.

Example:

```text
v0.1.0
├── M1
├── M2
├── M3
└── M4
```

After `1.0.0`, release strategy may evolve toward stricter semantic-versioning rules.

## Pre-1.0 guidance

- breaking changes are allowed but should be intentional;
- user-visible config/CLI changes should be documented;
- release notes should summarize behavior and migration impact.

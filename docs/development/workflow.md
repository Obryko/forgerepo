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

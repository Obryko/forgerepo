# Planning and Roadmap Model

Forge projects use the following hierarchy:

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

## Milestone

A coherent product/engineering stage. A milestone normally contains multiple Epics.

## Epic

A large coherent outcome within a Milestone.

## Story

Aggregates a concrete capability/value and always contains implementation Tasks.

## Task

A concrete implementable unit of work.

## Spike

Time-boxed research or decision work. If it discovers implementation work, create the resulting Story/Task in the same Epic and Milestone.

## Chore

Tooling, CI, release, maintenance, or housekeeping work.

## Bug

A defect attached to the closest meaningful Story/Epic.

## Dependency graph

Hierarchy does not define order.

Separately track:

- blocks;
- blocked by;
- ready;
- parallel work;
- critical path.

Priority is not a substitute for dependencies.

## Target releases

Target Release is independent from Milestone. A release may contain one or several milestones.

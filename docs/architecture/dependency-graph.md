# Dependency Graph

The Project Graph is the central data structure of ForgeRepo.

## Model

A graph contains:

- project/module nodes;
- directed dependency edges;
- deterministic traversal behavior.

Expected operations include:

- direct dependencies;
- direct dependents;
- transitive dependencies/dependents;
- dependency-path explanation;
- cycle-safe traversal.

## Requirements

- ecosystem-neutral representation;
- deterministic output;
- cycle-safe algorithms;
- reusable API for boundaries, inspection, and affected analysis;
- no CLI formatting inside graph logic.

## Why this matters

Boundary validation, `forge why`, and affected analysis should all consume the same graph rather than maintain separate dependency models.

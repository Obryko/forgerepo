# Product Vision

ForgeRepo is a fast, standalone repository architecture and dependency-analysis tool written in Rust.

## Problem

Repositories become difficult to reason about as they grow across packages, modules, languages, teams, and build systems.

Developers need reliable answers to questions such as:

- What logical projects/modules exist?
- What depends on what?
- Why does one project depend on another?
- Are architecture boundaries being violated?
- What is affected by a change?

Existing solutions are often coupled to a specific ecosystem or require large runtime/tooling stacks.

## Vision

ForgeRepo should provide one small binary that understands repository structure through an ecosystem-neutral model and exposes reusable analysis capabilities.

It should work for:

- monorepos;
- modular monoliths;
- layered applications;
- polyglot repositories.

## Initial product path

1. repository/project discovery;
2. normalized Project Graph;
3. dependency boundaries;
4. graph inspection;
5. affected analysis;
6. later: task/build intelligence and richer developer tooling.

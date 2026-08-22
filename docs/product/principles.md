# Product Principles

## Architecture first

The Project Graph is the central abstraction. Features should build on the graph instead of introducing parallel models.

## Ecosystem neutral core

`forgerepo-core` must not assume a specific package manager, framework, or build tool.

## Simple configuration

`forge.toml` should remain readable and unsurprising.

## Useful beyond monorepos

A modular monolith is a first-class ForgeRepo use case.

## Deterministic behavior

Discovery, traversal, diagnostics, and machine-readable output should be deterministic.

## Fast local feedback

ForgeRepo should feel natural in developer workflows and CI.

## Standalone distribution

Users should not need Node.js, Python, JVM, or another runtime just to execute ForgeRepo.

## Progressive capability

Do not implement remote cache, task runners, IDE integrations, or cloud services before the graph/boundary model is proven.

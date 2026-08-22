---
name: plan-work
description: Plan Forge work using Milestone → Epic → Story/Task/Spike/Chore plus a separate dependency DAG.
---

Produce:

1. Work breakdown hierarchy:
   - Milestone
   - multiple Epics
   - Story → mandatory Tasks
   - direct Task/Spike/Chore/Bug under Epic when appropriate
   - direct Milestone items only exceptionally

2. Dependency DAG:
   - blocked by
   - blocks
   - ready now
   - parallel work
   - critical path where useful

Document significant planning/architecture assumptions under `docs/` when they become project-wide conventions.

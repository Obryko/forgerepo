---
name: review-pr
description: Review a ForgeRepo PR against its issue, docs, architecture constraints, and Rust standards.
---

1. Identify linked issue.
2. Read relevant docs/ADRs.
3. Compare diff to acceptance criteria.
4. Check crate boundaries, error handling, deterministic behavior, tests, and docs.
5. Flag undocumented architecture changes.
6. Return findings ordered by severity.

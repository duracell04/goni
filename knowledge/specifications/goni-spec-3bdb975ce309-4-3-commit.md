---
id: GONI-SPEC-3BDB975CE309
title: 4.3 Commit
type: specification
status: draft
implementation_state: specified_only
proposition: write durable state deltas, append receipt with transaction outcome, release unused budget reservation.
domains:
- kernel
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/kernel/SPEC-TXN-01-transactional-tools.md
  heading: 4.3 Commit
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 4.3 Commit

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.3 Commit
- write durable state deltas,
- append receipt with transaction outcome,
- release unused budget reservation.

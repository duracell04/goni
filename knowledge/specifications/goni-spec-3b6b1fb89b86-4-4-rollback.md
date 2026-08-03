---
id: GONI-SPEC-3B6B1FB89B86
title: 4.4 Rollback
type: specification
status: draft
implementation_state: specified_only
proposition: undo staged local deltas, append rollback receipt, mark external effects for compensation if needed.
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
  heading: 4.4 Rollback
  revision: c93972edd18e2b5ad118be428d6c83042f8702eb
---

# 4.4 Rollback

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

### 4.4 Rollback
- undo staged local deltas,
- append rollback receipt,
- mark external effects for compensation if needed.

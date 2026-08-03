---
id: GONI-PRINCIPLE-772A7FA7ECEF
title: 9. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**No bypass**: packages cannot be installed without signature verification.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agents/agent-store.md
  heading: 9. Invariants
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 9. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 9. Invariants

- **No bypass**: packages cannot be installed without signature verification.
- **Policy binding**: installs bind to a capability policy snapshot.
- **Auditability**: all store actions emit receipts.

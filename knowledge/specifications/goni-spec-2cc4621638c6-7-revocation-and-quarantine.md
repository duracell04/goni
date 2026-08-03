---
id: GONI-SPEC-2CC4621638C6
title: 7. Revocation and quarantine
type: specification
status: draft
implementation_state: specified_only
proposition: 'The system MUST support: publisher revocation lists.'
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
  heading: 7. Revocation and quarantine
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 7. Revocation and quarantine

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 7. Revocation and quarantine

The system MUST support:
- publisher revocation lists.
- package quarantine status.
- forced disablement for revoked signatures.
- a safe rollback path to last known-good versions.

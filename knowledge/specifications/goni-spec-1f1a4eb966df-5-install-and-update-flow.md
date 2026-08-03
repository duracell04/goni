---
id: GONI-SPEC-1F1A4EB966DF
title: 5. Install and update flow
type: specification
status: draft
implementation_state: specified_only
proposition: 'Install MUST: record a receipt for the install action.'
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
  heading: 5. Install and update flow
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 5. Install and update flow

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Install and update flow

Install MUST:
- record a receipt for the install action.
- pin the exact artifact digest and publisher signature.
- bind the install to a capability policy snapshot.

Update MUST:
- record a receipt for the update action.
- enforce update_policy and allow rollback.
- require explicit confirmation for new capabilities.

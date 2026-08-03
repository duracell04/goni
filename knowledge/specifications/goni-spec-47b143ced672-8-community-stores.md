---
id: GONI-SPEC-47B143CED672
title: 8. Community stores
type: specification
status: draft
implementation_state: specified_only
proposition: Community stores MUST be labeled as untrusted by default and require explicit opt-in.
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
  heading: 8. Community stores
  revision: 17d060e8ac309d5a25765a07f3a00da85d0739e1
---

# 8. Community stores

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 8. Community stores

Community stores MUST be labeled as untrusted by default and require explicit
opt-in. The runtime SHOULD allow per-store policies, including:
- allowlist and denylist controls.
- stricter sandbox defaults.
- blocked auto-updates.

---
id: GONI-PRINCIPLE-A7D58080EBA1
title: 3. Invariants
type: principle
status: draft
implementation_state: specified_only
proposition: '**No ambient authority:** capabilities must be explicitly listed.'
domains:
- agent
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/agent-manifest.md
  heading: 3. Invariants
  revision: b0cc5f3b78265e3c4ecefaeb94209ce1e0e251e3
---

# 3. Invariants

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 3. Invariants

- **No ambient authority:** capabilities must be explicitly listed.
- **Budget required:** an agent without budgets is invalid.
- **Default-off network:** network access is false unless explicitly granted.
- **UI integration is explicit:** any UI surface must be declared in the manifest.
- **Remote access is explicit:** remote presence requires an explicit grant.
- **Policy intersection:** effective capabilities are the intersection of
  manifest requests and active policy.

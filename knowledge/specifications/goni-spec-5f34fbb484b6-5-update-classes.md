---
id: GONI-SPEC-5F34FBB484B6
title: 5. Update classes
type: specification
status: draft
implementation_state: specified_only
proposition: 'CDC MUST gate updates by evidence strength and governance centrality: | Signal | Default update | | Single correction | Scoped hypothesis, pending review or short TTL | | Repeated correction | Scoped preference candidate | | Accepted learning card | Policy or memory candidate with receipt | | High-confidence repeated rule | Stable default after replay coverage |'
domains:
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/correction-delta-compiler.md
  heading: 5. Update classes
  revision: e3e487b4f8de4b5cdd83d5be45e0f966f2cb4a8a
---

# 5. Update classes

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 5. Update classes

CDC MUST gate updates by evidence strength and governance centrality:

| Signal | Default update |
| --- | --- |
| Single correction | Scoped hypothesis, pending review or short TTL |
| Repeated correction | Scoped preference candidate |
| Accepted learning card | Policy or memory candidate with receipt |
| High-confidence repeated rule | Stable default after replay coverage |
| Constitutional or high-risk rule | Explicit approval and slow promotion |

The update rate must be conservative. A single angry or context-specific edit
must not become a global rule.

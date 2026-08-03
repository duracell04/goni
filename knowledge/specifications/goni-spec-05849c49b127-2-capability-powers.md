---
id: GONI-SPEC-05849C49B127
title: 2. Capability powers
type: specification
status: draft
implementation_state: specified_only
proposition: 'Goni separates four powers: | Power | Meaning | Boundary question | | observation | The agent may see a screen, window, tab, app, frame, stream, accessibility tree, or event.'
domains:
- memory
- specs
aliases: []
relations: []
sources: []
artifacts: []
uncertainty: Preserved from the legacy draft without status promotion or newly inferred evidence strength.
legacy:
- path: blueprint/30-specs/vision-memory-actuation-boundaries.md
  heading: 2. Capability powers
  revision: f5439552442ae66d1f6739d2f853a46006372771
---

# 2. Capability powers

> Status boundary: this is a migrated draft. For `specified_only` nodes, present-tense or enforcement language below states intended contract behavior, not observed implementation, verification, or non-bypassability.

## 2. Capability powers

Goni separates four powers:

| Power | Meaning | Boundary question |
| --- | --- | --- |
| `observation` | The agent may see a screen, window, tab, app, frame, stream, accessibility tree, or event. | What can be observed, for how long, and under which user/session scope? |
| `extraction` | The agent may parse, OCR, summarize, classify, embed, transform, or send observed content to a model. | What can be derived from observation, and may it leave the local node? |
| `memory` | The agent may store, index, consolidate, retrieve, sync, or reuse extracted facts or artifacts. | What memory class may be written or read, and with what expiry/review policy? |
| `actuation` | The agent may click, type, scroll, submit, delete, move, publish, run commands, or call tools. | What side effects are allowed, under which tool token, sandbox, corridor, and receipt? |

Granting one power MUST NOT imply any other power.

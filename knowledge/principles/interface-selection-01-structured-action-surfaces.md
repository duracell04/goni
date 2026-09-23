---
id: INTERFACE-SELECTION-01
title: Prefer action surfaces with explicit structure and observable state
type: principle
status: draft
implementation_state: specified_only
proposition: When multiple interfaces can perform the same action, Goni should prefer the interface that minimizes semantic ambiguity and exposes the clearest preconditions, postconditions, and state, rather than applying an absolute API-over-CLI-over-GUI rule.
domains:
- system
- interfaces
- tools
aliases:
- API CLI GUI selection
relations:
- type: refines
  target: TOOL-EFFECT-01
sources:
- SRC-XIE2024-OSWORLD
artifacts: []
uncertainty: API, CLI, and GUI quality varies by system; this is a risk-sensitive engineering heuristic rather than a universal ordering.
legacy: []
---

# Prefer action surfaces with explicit structure and observable state

A common heuristic is:

```text
structured API -> CLI -> GUI automation
```

but Goni does not treat that sequence as a law.

The selection criterion is the quality of the action and observation contract:

- Is the action space typed and bounded?
- Are preconditions explicit?
- Does the interface expose machine-readable resulting state?
- Can the operation be retried idempotently?
- Can failure be distinguished from partial success?
- Is provenance available?
- Can the effect be rolled back or compensated?
- How much perception or grounding uncertainty is introduced?

A well-designed CLI can be superior to a weak API. GUI/computer use can be necessary when no structured interface exists. Pixel-level interaction generally requires additional perception, grounding, and postcondition verification, which Goni should reflect in risk and verification policy.
